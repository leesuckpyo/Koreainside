use crate::credentials::{connection_result, read_vercel_access_token, VercelConnectionStatus};
use reqwest::{header::HeaderMap, redirect::Policy, StatusCode};
use serde::Serialize;
use std::{
    sync::{Mutex, MutexGuard, OnceLock},
    time::{Duration, Instant},
};
use time::{
    format_description, format_description::well_known::Rfc3339, OffsetDateTime, UtcOffset,
};

const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_RESPONSE_BYTES: usize = 64 * 1024;
const MAX_JAVASCRIPT_SAFE_INTEGER: u64 = 9_007_199_254_740_991;
const CACHE_TTL: Duration = Duration::from_secs(5 * 60);
const FAILURE_COOLDOWN: Duration = Duration::from_secs(10);
const MAX_CREDENTIAL_GENERATION_ATTEMPTS: usize = 2;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AnalyticsPeriod {
    Hours24,
    Days7,
    Days30,
}

impl AnalyticsPeriod {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "24h" => Some(Self::Hours24),
            "7d" => Some(Self::Days7),
            "30d" => Some(Self::Days30),
            _ => None,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Hours24 => "24h",
            Self::Days7 => "7d",
            Self::Days30 => "30d",
        }
    }

    fn duration_ms(self) -> i64 {
        match self {
            Self::Hours24 => 86_400_000,
            Self::Days7 => 604_800_000,
            Self::Days30 => 2_592_000_000,
        }
    }

    fn index(self) -> usize {
        match self {
            Self::Hours24 => 0,
            Self::Days7 => 1,
            Self::Days30 => 2,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TimeRange {
    since_ms: i64,
    until_ms: i64,
}

impl TimeRange {
    fn rolling(period: AnalyticsPeriod, until_ms: i64) -> Option<Self> {
        Some(Self {
            since_ms: until_ms.checked_sub(period.duration_ms())?,
            until_ms,
        })
    }
}

#[derive(Clone, Debug)]
struct NormalizedSummary {
    period: AnalyticsPeriod,
    range: TimeRange,
    fetched_at_ms: i64,
    pageviews: u64,
    visitors: u64,
}

#[derive(Clone, Debug)]
struct CachedSummary {
    summary: NormalizedSummary,
    stored_at: Instant,
}

#[derive(Clone, Debug)]
struct Cooldown {
    until: Instant,
    retry_at: String,
    error_code: &'static str,
    message: &'static str,
}

#[derive(Default)]
struct PeriodRequestState {
    in_flight: Option<RequestIdentity>,
    cache: Option<CachedSummary>,
    cooldown: Option<Cooldown>,
}

#[derive(Default)]
struct AnalyticsRequestState {
    generation: u64,
    next_request_id: u64,
    periods: [PeriodRequestState; 3],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RequestIdentity {
    generation: u64,
    request_id: u64,
}

enum BeginRequest {
    Start(RequestIdentity),
    Cached(NormalizedSummary),
    InFlight,
    CoolingDown(Cooldown),
}

impl AnalyticsRequestState {
    fn begin(&mut self, period: AnalyticsPeriod, now: Instant) -> BeginRequest {
        let state = &mut self.periods[period.index()];

        if let Some(cache) = &state.cache {
            if now.saturating_duration_since(cache.stored_at) < CACHE_TTL {
                return BeginRequest::Cached(cache.summary.clone());
            }
            state.cache = None;
        }

        if state.in_flight.is_some() {
            return BeginRequest::InFlight;
        }

        if let Some(cooldown) = &state.cooldown {
            if now < cooldown.until {
                return BeginRequest::CoolingDown(cooldown.clone());
            }
            state.cooldown = None;
        }

        self.next_request_id = self.next_request_id.wrapping_add(1);
        let identity = RequestIdentity {
            generation: self.generation,
            request_id: self.next_request_id,
        };
        self.periods[period.index()].in_flight = Some(identity);
        BeginRequest::Start(identity)
    }

    fn finish_success(
        &mut self,
        period: AnalyticsPeriod,
        identity: RequestIdentity,
        summary: NormalizedSummary,
        now: Instant,
    ) -> bool {
        if self.generation != identity.generation
            || self.periods[period.index()].in_flight != Some(identity)
        {
            return false;
        }
        let state = &mut self.periods[period.index()];
        state.in_flight = None;
        state.cooldown = None;
        state.cache = Some(CachedSummary {
            summary,
            stored_at: now,
        });
        true
    }

    fn finish_failure(
        &mut self,
        period: AnalyticsPeriod,
        identity: RequestIdentity,
        now: Instant,
        cooldown_duration: Duration,
        retry_at: String,
        error: &AnalyticsQueryError,
    ) -> bool {
        if self.generation != identity.generation
            || self.periods[period.index()].in_flight != Some(identity)
        {
            return false;
        }
        let state = &mut self.periods[period.index()];
        state.in_flight = None;
        state.cooldown = Some(Cooldown {
            until: now + cooldown_duration,
            retry_at,
            error_code: error.code(),
            message: error.message(),
        });
        true
    }

    fn release(&mut self, period: AnalyticsPeriod, identity: RequestIdentity) -> bool {
        if self.generation == identity.generation
            && self.periods[period.index()].in_flight == Some(identity)
        {
            self.periods[period.index()].in_flight = None;
            true
        } else {
            false
        }
    }

    fn invalidate(&mut self) {
        self.generation = self.generation.wrapping_add(1);
        self.periods = Default::default();
    }
}

fn request_state() -> &'static Mutex<AnalyticsRequestState> {
    static STATE: OnceLock<Mutex<AnalyticsRequestState>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(AnalyticsRequestState::default()))
}

fn lock_request_state(
    state: &'static Mutex<AnalyticsRequestState>,
) -> MutexGuard<'static, AnalyticsRequestState> {
    state
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

pub(crate) fn invalidate_analytics_runtime_state() {
    lock_request_state(request_state()).invalidate();
}

struct RequestLease {
    state: &'static Mutex<AnalyticsRequestState>,
    period: AnalyticsPeriod,
    identity: RequestIdentity,
    active: bool,
}

impl RequestLease {
    fn new(
        state: &'static Mutex<AnalyticsRequestState>,
        period: AnalyticsPeriod,
        identity: RequestIdentity,
    ) -> Self {
        Self {
            state,
            period,
            identity,
            active: true,
        }
    }

    fn finish_success(&mut self, summary: NormalizedSummary, now: Instant) -> bool {
        let applied =
            lock_request_state(self.state).finish_success(self.period, self.identity, summary, now);
        self.active = false;
        applied
    }

    fn finish_failure(
        &mut self,
        now: Instant,
        cooldown_duration: Duration,
        retry_at: String,
        error: &AnalyticsQueryError,
    ) -> bool {
        let applied = lock_request_state(self.state).finish_failure(
            self.period,
            self.identity,
            now,
            cooldown_duration,
            retry_at,
            error,
        );
        self.active = false;
        applied
    }

    fn release(&mut self) -> bool {
        if self.active {
            let released = lock_request_state(self.state).release(self.period, self.identity);
            self.active = false;
            released
        } else {
            false
        }
    }
}

impl Drop for RequestLease {
    fn drop(&mut self) {
        let _ = self.release();
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CredentialAccessError {
    NotConfigured,
    ReadFailed,
}

enum PreparedRequest {
    Start { token: String, lease: RequestLease },
    Cached(NormalizedSummary),
    InFlight,
    CoolingDown(Cooldown),
    CredentialChanged,
}

fn prepare_summary_request<F>(
    state_mutex: &'static Mutex<AnalyticsRequestState>,
    period: AnalyticsPeriod,
    mut read_credential: F,
) -> Result<PreparedRequest, CredentialAccessError>
where
    F: FnMut() -> Result<String, CredentialAccessError>,
{
    for _ in 0..MAX_CREDENTIAL_GENERATION_ATTEMPTS {
        let generation = lock_request_state(state_mutex).generation;
        let token = read_credential()?;
        let mut state = lock_request_state(state_mutex);
        if state.generation != generation {
            drop(state);
            drop(token);
            continue;
        }

        return Ok(match state.begin(period, Instant::now()) {
            BeginRequest::Start(identity) => PreparedRequest::Start {
                token,
                lease: RequestLease::new(state_mutex, period, identity),
            },
            BeginRequest::Cached(summary) => PreparedRequest::Cached(summary),
            BeginRequest::InFlight => PreparedRequest::InFlight,
            BeginRequest::CoolingDown(cooldown) => PreparedRequest::CoolingDown(cooldown),
        });
    }

    Ok(PreparedRequest::CredentialChanged)
}

#[derive(Clone, Debug)]
enum AnalyticsQueryError {
    NotConfigured,
    CredentialReadFailed,
    HttpClientFailed,
    InvalidRequest,
    Unauthorized,
    PlanOrBillingRequired,
    Forbidden,
    NotFound,
    RateLimited { retry_after: Duration },
    ServiceUnavailable,
    Timeout,
    Network,
    RequestFailed,
    ResponseTooLarge,
    InvalidResponse,
    ApiError,
}

impl AnalyticsQueryError {
    fn code(&self) -> &'static str {
        match self {
            Self::NotConfigured => "not_configured",
            Self::CredentialReadFailed => "credential_read_failed",
            Self::HttpClientFailed => "http_client_failed",
            Self::InvalidRequest => "invalid_request",
            Self::Unauthorized => "unauthorized",
            Self::PlanOrBillingRequired => "plan_or_billing_required",
            Self::Forbidden => "forbidden",
            Self::NotFound => "not_found",
            Self::RateLimited { .. } => "rate_limited",
            Self::ServiceUnavailable => "service_unavailable",
            Self::Timeout => "timeout",
            Self::Network => "network_error",
            Self::RequestFailed => "request_failed",
            Self::ResponseTooLarge => "response_too_large",
            Self::InvalidResponse => "invalid_response",
            Self::ApiError => "api_error",
        }
    }

    fn message(&self) -> &'static str {
        match self {
            Self::NotConfigured => "저장된 Vercel Access Token이 없습니다.",
            Self::CredentialReadFailed => "Windows 자격 증명 관리자에서 토큰을 읽을 수 없습니다.",
            Self::HttpClientFailed => "보안 HTTP 연결을 준비할 수 없습니다.",
            Self::InvalidRequest => "Vercel Analytics 요청이 올바르지 않습니다.",
            Self::Unauthorized => "Vercel 자격 증명이 유효하지 않습니다.",
            Self::PlanOrBillingRequired => "Vercel 플랜 또는 결제 상태를 확인해 주십시오.",
            Self::Forbidden => "Vercel Analytics를 읽을 권한이 없습니다.",
            Self::NotFound => "Vercel 프로젝트 또는 Analytics 데이터를 확인할 수 없습니다.",
            Self::RateLimited { .. } => "Vercel 요청 제한 상태입니다.",
            Self::ServiceUnavailable => "Vercel 서비스가 요청을 처리하지 못했습니다.",
            Self::Timeout => "Vercel 연결 시간이 초과되었습니다.",
            Self::Network => "Vercel API에 연결할 수 없습니다.",
            Self::RequestFailed => "Vercel Analytics 요청을 완료할 수 없습니다.",
            Self::ResponseTooLarge => "Vercel Analytics 응답 크기가 허용 범위를 초과했습니다.",
            Self::InvalidResponse => "Vercel Analytics 응답 형식을 확인할 수 없습니다.",
            Self::ApiError => "Vercel Analytics 요청 결과를 확인할 수 없습니다.",
        }
    }

    fn cooldown_duration(&self) -> Option<Duration> {
        match self {
            Self::NotConfigured | Self::CredentialReadFailed | Self::HttpClientFailed => None,
            Self::RateLimited { retry_after } => Some(*retry_after),
            _ => Some(FAILURE_COOLDOWN),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyticsSummaryResponse {
    status: &'static str,
    period: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    range_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fetched_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pageviews: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visitors: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cached: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_code: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_at: Option<String>,
}

impl AnalyticsSummaryResponse {
    fn success(summary: NormalizedSummary, cached: bool) -> Self {
        let timestamps = (
            format_utc_timestamp(summary.range.since_ms),
            format_utc_timestamp(summary.range.until_ms),
            format_utc_timestamp(summary.fetched_at_ms),
        );
        match timestamps {
            (Some(range_start), Some(range_end), Some(fetched_at)) => Self {
                status: "ok",
                period: summary.period.as_str(),
                range_start: Some(range_start),
                range_end: Some(range_end),
                fetched_at: Some(fetched_at),
                pageviews: Some(summary.pageviews),
                visitors: Some(summary.visitors),
                cached: Some(cached),
                error_code: None,
                message: None,
                retry_at: None,
            },
            _ => Self::error(
                summary.period.as_str(),
                "invalid_response",
                "Vercel Analytics 응답 형식을 확인할 수 없습니다.",
                None,
            ),
        }
    }

    fn error(
        period: &'static str,
        error_code: &'static str,
        message: &'static str,
        retry_at: Option<String>,
    ) -> Self {
        Self {
            status: "error",
            period,
            range_start: None,
            range_end: None,
            fetched_at: None,
            pageviews: None,
            visitors: None,
            cached: None,
            error_code: Some(error_code),
            message: Some(message),
            retry_at,
        }
    }
}

#[tauri::command]
pub async fn get_vercel_analytics_summary(period: String) -> AnalyticsSummaryResponse {
    let Some(period) = AnalyticsPeriod::parse(&period) else {
        return AnalyticsSummaryResponse::error(
            "invalid",
            "invalid_period",
            "조회 기간은 24h, 7d, 30d 중 하나여야 합니다.",
            None,
        );
    };

    let prepared = prepare_summary_request(request_state(), period, || {
        read_vercel_access_token().map_err(|error| match error {
            keyring_core::Error::NoEntry => CredentialAccessError::NotConfigured,
            _ => CredentialAccessError::ReadFailed,
        })
    });

    let (token, mut lease) = match prepared {
        Err(CredentialAccessError::NotConfigured) => {
            return response_from_error(period, AnalyticsQueryError::NotConfigured, None);
        }
        Err(CredentialAccessError::ReadFailed) => {
            return response_from_error(period, AnalyticsQueryError::CredentialReadFailed, None);
        }
        Ok(PreparedRequest::Cached(summary)) => {
            return AnalyticsSummaryResponse::success(summary, true);
        }
        Ok(PreparedRequest::InFlight) => {
            return AnalyticsSummaryResponse::error(
                period.as_str(),
                "request_in_progress",
                "같은 기간의 Analytics 조회가 이미 진행 중입니다.",
                None,
            );
        }
        Ok(PreparedRequest::CoolingDown(cooldown)) => {
            return AnalyticsSummaryResponse::error(
                period.as_str(),
                cooldown.error_code,
                cooldown.message,
                Some(cooldown.retry_at),
            );
        }
        Ok(PreparedRequest::CredentialChanged) => {
            return AnalyticsSummaryResponse::error(
                period.as_str(),
                "credential_changed",
                "자격 증명이 변경되어 요청을 시작하지 않았습니다. 다시 시도해 주십시오.",
                None,
            );
        }
        Ok(PreparedRequest::Start { token, lease }) => (token, lease),
    };

    let until_ms = current_utc_milliseconds();
    let Some(range) = TimeRange::rolling(period, until_ms) else {
        let error = AnalyticsQueryError::InvalidResponse;
        let _ = lease.release();
        return response_from_error(period, error, None);
    };

    match vercel_adapter::fetch_count(&token, &range).await {
        Ok(count) => {
            let summary = NormalizedSummary {
                period,
                range,
                fetched_at_ms: current_utc_milliseconds(),
                pageviews: count.pageviews,
                visitors: count.visitors,
            };
            if lease.finish_success(summary.clone(), Instant::now()) {
                AnalyticsSummaryResponse::success(summary, false)
            } else {
                credential_changed_response(period)
            }
        }
        Err(error) => {
            let (retry_at, applied) = finish_failed_request(
                &mut lease,
                Instant::now(),
                current_utc_milliseconds(),
                &error,
            );
            if applied {
                response_from_error(period, error, retry_at)
            } else {
                credential_changed_response(period)
            }
        }
    }
}

fn finish_failed_request(
    lease: &mut RequestLease,
    now: Instant,
    now_ms: i64,
    error: &AnalyticsQueryError,
) -> (Option<String>, bool) {
    let Some(duration) = error.cooldown_duration() else {
        return (None, lease.release());
    };
    let retry_at_ms = now_ms.saturating_add(duration.as_millis().min(i64::MAX as u128) as i64);
    let Some(retry_at) = format_utc_timestamp(retry_at_ms) else {
        let released = lease.release();
        return (None, released);
    };
    let applied = lease.finish_failure(now, duration, retry_at.clone(), error);
    (applied.then_some(retry_at), applied)
}

fn credential_changed_response(period: AnalyticsPeriod) -> AnalyticsSummaryResponse {
    AnalyticsSummaryResponse::error(
        period.as_str(),
        "credential_changed",
        "자격 증명이 변경되어 요청 결과를 사용하지 않았습니다. 다시 시도해 주십시오.",
        None,
    )
}

fn response_from_error(
    period: AnalyticsPeriod,
    error: AnalyticsQueryError,
    retry_at: Option<String>,
) -> AnalyticsSummaryResponse {
    AnalyticsSummaryResponse::error(period.as_str(), error.code(), error.message(), retry_at)
}

#[tauri::command]
pub async fn test_vercel_analytics_connection() -> VercelConnectionStatus {
    test_connection_with(
        || {
            read_vercel_access_token().map_err(|error| match error {
                keyring_core::Error::NoEntry => CredentialAccessError::NotConfigured,
                _ => CredentialAccessError::ReadFailed,
            })
        },
        |token, range| async move { vercel_adapter::fetch_count(&token, &range).await },
    )
    .await
}

async fn test_connection_with<R, F, Fut>(read_credential: R, fetch: F) -> VercelConnectionStatus
where
    R: FnOnce() -> Result<String, CredentialAccessError>,
    F: FnOnce(String, TimeRange) -> Fut,
    Fut: std::future::Future<Output = Result<vercel_adapter::CountResult, AnalyticsQueryError>>,
{
    let token = match read_credential() {
        Ok(token) => token,
        Err(CredentialAccessError::NotConfigured) => {
            return connection_result(
                "not_configured",
                false,
                None,
                Some("TOKEN_NOT_CONFIGURED"),
                Some("저장된 Vercel Access Token이 없습니다."),
            );
        }
        Err(CredentialAccessError::ReadFailed) => {
            return connection_result(
                "error",
                false,
                None,
                Some("CREDENTIAL_READ_FAILED"),
                Some("Windows 자격 증명 관리자에서 토큰을 읽을 수 없습니다."),
            );
        }
    };

    let checked_at = current_kst_timestamp();
    let until_ms = current_utc_milliseconds();
    let Some(range) = TimeRange::rolling(AnalyticsPeriod::Hours24, until_ms) else {
        return invalid_connection_response(checked_at);
    };

    match fetch(token, range).await {
        Ok(_) => connection_result("connected", true, checked_at, None, None),
        Err(error) => connection_status_from_error(error, checked_at),
    }
}

fn connection_status_from_error(
    error: AnalyticsQueryError,
    checked_at: Option<String>,
) -> VercelConnectionStatus {
    let status = if matches!(error, AnalyticsQueryError::RateLimited { .. }) {
        "rate_limited"
    } else {
        "error"
    };
    connection_result(
        status,
        true,
        checked_at,
        Some(error.code()),
        Some(error.message()),
    )
}

fn invalid_connection_response(checked_at: Option<String>) -> VercelConnectionStatus {
    connection_status_from_error(AnalyticsQueryError::InvalidResponse, checked_at)
}

fn current_utc_milliseconds() -> i64 {
    let value = OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000;
    i64::try_from(value).unwrap_or(i64::MAX)
}

fn format_utc_timestamp(milliseconds: i64) -> Option<String> {
    OffsetDateTime::from_unix_timestamp_nanos(i128::from(milliseconds) * 1_000_000)
        .ok()?
        .format(&Rfc3339)
        .ok()
}

fn current_kst_timestamp() -> Option<String> {
    let offset = UtcOffset::from_hms(9, 0, 0).ok()?;
    let description = format_description::parse_borrowed::<2>(
        "[year]-[month]-[day]T[hour]:[minute]:[second]+09:00",
    )
    .ok()?;
    OffsetDateTime::now_utc()
        .to_offset(offset)
        .format(&description)
        .ok()
}

mod vercel_adapter {
    use super::*;
    use serde::Deserialize;

    pub(super) const ENDPOINT: &str = "https://api.vercel.com/v1/query/web-analytics/visits/count";
    pub(super) const PROJECT_ID: &str = "prj_E6IPJCgBEvh3J7Ga8AMNXeq3Gb4M";
    pub(super) const TEAM_ID: &str = "team_C5MoIigwWP5u47OShZDSyz4Z";
    const DEFAULT_RATE_LIMIT_COOLDOWN: Duration = Duration::from_secs(60);
    const MAX_RATE_LIMIT_COOLDOWN: Duration = Duration::from_secs(24 * 60 * 60);

    #[derive(Deserialize)]
    struct VercelCountResponse {
        version: u64,
        data: VercelCountData,
    }

    #[derive(Deserialize)]
    struct VercelCountData {
        pageviews: u64,
        visitors: u64,
    }

    pub(super) struct CountResult {
        pub(super) pageviews: u64,
        pub(super) visitors: u64,
    }

    pub(super) async fn fetch_count(
        token: &str,
        range: &TimeRange,
    ) -> Result<CountResult, AnalyticsQueryError> {
        let client = secure_http_client()?;
        let response = client
            .get(ENDPOINT)
            .bearer_auth(token)
            .query(&[
                ("projectId", PROJECT_ID.to_string()),
                ("teamId", TEAM_ID.to_string()),
                ("since", range.since_ms.to_string()),
                ("until", range.until_ms.to_string()),
            ])
            .send()
            .await
            .map_err(map_request_error)?;

        if response.status() != StatusCode::OK {
            return Err(map_http_status(response.status(), response.headers()));
        }

        if response
            .content_length()
            .is_some_and(|length| length > MAX_RESPONSE_BYTES as u64)
        {
            return Err(AnalyticsQueryError::ResponseTooLarge);
        }

        let body = read_limited_body(response).await?;
        parse_count_response(&body)
    }

    fn secure_http_client() -> Result<reqwest::Client, AnalyticsQueryError> {
        reqwest::Client::builder()
            .user_agent(concat!("Korea-Inside-Admin/", env!("CARGO_PKG_VERSION")))
            .https_only(true)
            .redirect(Policy::none())
            .referer(false)
            .connect_timeout(CONNECT_TIMEOUT)
            .timeout(REQUEST_TIMEOUT)
            .retry(reqwest::retry::never())
            .build()
            .map_err(|_| AnalyticsQueryError::HttpClientFailed)
    }

    async fn read_limited_body(
        mut response: reqwest::Response,
    ) -> Result<Vec<u8>, AnalyticsQueryError> {
        let mut body = Vec::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|_| AnalyticsQueryError::InvalidResponse)?
        {
            if body.len().saturating_add(chunk.len()) > MAX_RESPONSE_BYTES {
                return Err(AnalyticsQueryError::ResponseTooLarge);
            }
            body.extend_from_slice(&chunk);
        }
        Ok(body)
    }

    fn parse_count_response(body: &[u8]) -> Result<CountResult, AnalyticsQueryError> {
        if body.len() > MAX_RESPONSE_BYTES {
            return Err(AnalyticsQueryError::ResponseTooLarge);
        }
        let response: VercelCountResponse =
            serde_json::from_slice(body).map_err(|_| AnalyticsQueryError::InvalidResponse)?;
        let _version = response.version;
        if response.data.pageviews > MAX_JAVASCRIPT_SAFE_INTEGER
            || response.data.visitors > MAX_JAVASCRIPT_SAFE_INTEGER
        {
            return Err(AnalyticsQueryError::InvalidResponse);
        }
        Ok(CountResult {
            pageviews: response.data.pageviews,
            visitors: response.data.visitors,
        })
    }

    fn map_request_error(error: reqwest::Error) -> AnalyticsQueryError {
        if error.is_timeout() {
            AnalyticsQueryError::Timeout
        } else if error.is_connect() {
            AnalyticsQueryError::Network
        } else {
            AnalyticsQueryError::RequestFailed
        }
    }

    fn map_http_status(status: StatusCode, headers: &HeaderMap) -> AnalyticsQueryError {
        match status {
            StatusCode::BAD_REQUEST => AnalyticsQueryError::InvalidRequest,
            StatusCode::UNAUTHORIZED => AnalyticsQueryError::Unauthorized,
            StatusCode::PAYMENT_REQUIRED => AnalyticsQueryError::PlanOrBillingRequired,
            StatusCode::FORBIDDEN => AnalyticsQueryError::Forbidden,
            StatusCode::NOT_FOUND => AnalyticsQueryError::NotFound,
            StatusCode::TOO_MANY_REQUESTS => AnalyticsQueryError::RateLimited {
                retry_after: rate_limit_cooldown_at(headers, current_utc_milliseconds()),
            },
            _ if status.is_server_error() => AnalyticsQueryError::ServiceUnavailable,
            _ => AnalyticsQueryError::ApiError,
        }
    }

    fn rate_limit_cooldown_at(headers: &HeaderMap, now_ms: i64) -> Duration {
        let retry_after = headers
            .get(reqwest::header::RETRY_AFTER)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<u64>().ok())
            .map(Duration::from_secs);

        let reset_after = headers
            .get("x-ratelimit-reset")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<i64>().ok())
            .and_then(|reset| {
                let reset_ms = if reset > 10_000_000_000 {
                    reset
                } else {
                    reset.saturating_mul(1_000)
                };
                let remaining_ms = reset_ms.saturating_sub(now_ms);
                (remaining_ms >= 0).then(|| Duration::from_millis(remaining_ms as u64))
            });

        match (retry_after, reset_after) {
            (Some(retry_after), Some(reset_after)) => Some(retry_after.max(reset_after)),
            (Some(retry_after), None) => Some(retry_after),
            (None, Some(reset_after)) => Some(reset_after),
            (None, None) => None,
        }
        .unwrap_or(DEFAULT_RATE_LIMIT_COOLDOWN)
        .min(MAX_RATE_LIMIT_COOLDOWN)
    }

    #[cfg(test)]
    pub(super) fn parse_for_test(body: &[u8]) -> Result<CountResult, AnalyticsQueryError> {
        parse_count_response(body)
    }

    #[cfg(test)]
    pub(super) fn map_status_for_test(
        status: StatusCode,
        headers: &HeaderMap,
    ) -> AnalyticsQueryError {
        map_http_status(status, headers)
    }

    #[cfg(test)]
    pub(super) fn rate_limit_for_test(headers: &HeaderMap, now_ms: i64) -> Duration {
        rate_limit_cooldown_at(headers, now_ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_summary(period: AnalyticsPeriod, pageviews: u64, visitors: u64) -> NormalizedSummary {
        NormalizedSummary {
            period,
            range: TimeRange::rolling(period, 1_750_000_000_000).unwrap(),
            fetched_at_ms: 1_750_000_000_000,
            pageviews,
            visitors,
        }
    }

    fn leaked_state() -> &'static Mutex<AnalyticsRequestState> {
        Box::leak(Box::new(Mutex::new(AnalyticsRequestState::default())))
    }

    fn start_identity(
        state: &mut AnalyticsRequestState,
        period: AnalyticsPeriod,
        now: Instant,
    ) -> RequestIdentity {
        match state.begin(period, now) {
            BeginRequest::Start(identity) => identity,
            _ => panic!("request should start"),
        }
    }

    fn store_sample_cache(
        state: &mut AnalyticsRequestState,
        period: AnalyticsPeriod,
        now: Instant,
    ) {
        let identity = start_identity(state, period, now);
        assert!(state.finish_success(period, identity, sample_summary(period, 10, 8), now,));
    }

    #[test]
    fn parses_only_supported_periods() {
        assert_eq!(
            AnalyticsPeriod::parse("24h"),
            Some(AnalyticsPeriod::Hours24)
        );
        assert_eq!(AnalyticsPeriod::parse("7d"), Some(AnalyticsPeriod::Days7));
        assert_eq!(AnalyticsPeriod::parse("30d"), Some(AnalyticsPeriod::Days30));
        assert_eq!(AnalyticsPeriod::parse("1h"), None);
        assert_eq!(AnalyticsPeriod::parse("custom"), None);
    }

    #[test]
    fn calculates_utc_rolling_windows() {
        let until = 2_000_000_000_000;
        for (period, expected_duration) in [
            (AnalyticsPeriod::Hours24, 86_400_000),
            (AnalyticsPeriod::Days7, 604_800_000),
            (AnalyticsPeriod::Days30, 2_592_000_000),
        ] {
            let range = TimeRange::rolling(period, until).unwrap();
            assert_eq!(range.until_ms, until);
            assert_eq!(range.until_ms - range.since_ms, expected_duration);
        }
    }

    #[test]
    fn normalizes_valid_counts_and_allows_zero_and_unknown_fields() {
        for response in [
            br#"{"version":1,"data":{"pageviews":42,"visitors":30,"future":true},"future":true}"#
                .as_slice(),
            br#"{"version":1,"data":{"pageviews":0,"visitors":0}}"#.as_slice(),
        ] {
            let count = vercel_adapter::parse_for_test(response).unwrap();
            assert!(count.pageviews <= 42);
            assert!(count.visitors <= 30);
        }
    }

    #[test]
    fn rejects_missing_invalid_and_trailing_response_data() {
        for response in [
            br#"{"version":1,"data":{"pageviews":42}}"#.as_slice(),
            br#"{"version":1,"data":{"pageviews":-1,"visitors":1}}"#.as_slice(),
            br#"{"version":1,"data":{"pageviews":"1","visitors":1}}"#.as_slice(),
            br#"{"version":1,"data":{"pageviews":1.5,"visitors":1}}"#.as_slice(),
            br#"{"version":1,"data":{"pageviews":1,"visitors":1}} trailing"#.as_slice(),
            b"not-json".as_slice(),
        ] {
            assert!(matches!(
                vercel_adapter::parse_for_test(response),
                Err(AnalyticsQueryError::InvalidResponse)
            ));
        }
    }

    #[test]
    fn rejects_values_above_javascript_safe_integer() {
        let response = format!(
            r#"{{"version":1,"data":{{"pageviews":{},"visitors":1}}}}"#,
            MAX_JAVASCRIPT_SAFE_INTEGER + 1
        );
        assert!(matches!(
            vercel_adapter::parse_for_test(response.as_bytes()),
            Err(AnalyticsQueryError::InvalidResponse)
        ));
    }

    #[test]
    fn enforces_response_size_limit() {
        let response = vec![b' '; MAX_RESPONSE_BYTES + 1];
        assert!(matches!(
            vercel_adapter::parse_for_test(&response),
            Err(AnalyticsQueryError::ResponseTooLarge)
        ));
    }

    #[test]
    fn maps_required_http_statuses() {
        let headers = HeaderMap::new();
        let cases = [
            (StatusCode::BAD_REQUEST, "invalid_request"),
            (StatusCode::UNAUTHORIZED, "unauthorized"),
            (StatusCode::PAYMENT_REQUIRED, "plan_or_billing_required"),
            (StatusCode::FORBIDDEN, "forbidden"),
            (StatusCode::NOT_FOUND, "not_found"),
            (StatusCode::TOO_MANY_REQUESTS, "rate_limited"),
            (StatusCode::INTERNAL_SERVER_ERROR, "service_unavailable"),
        ];
        for (status, expected) in cases {
            assert_eq!(
                vercel_adapter::map_status_for_test(status, &headers).code(),
                expected
            );
        }
    }

    #[test]
    fn returns_cache_hit_and_expires_cache() {
        let now = Instant::now();
        let mut state = AnalyticsRequestState::default();
        store_sample_cache(&mut state, AnalyticsPeriod::Hours24, now);

        assert!(matches!(
            state.begin(
                AnalyticsPeriod::Hours24,
                now + CACHE_TTL - Duration::from_secs(1)
            ),
            BeginRequest::Cached(_)
        ));
        assert!(matches!(
            state.begin(AnalyticsPeriod::Hours24, now + CACHE_TTL),
            BeginRequest::Start(_)
        ));
    }

    #[test]
    fn keeps_period_caches_separate() {
        let now = Instant::now();
        let mut state = AnalyticsRequestState::default();
        store_sample_cache(&mut state, AnalyticsPeriod::Hours24, now);

        assert!(matches!(
            state.begin(AnalyticsPeriod::Hours24, now),
            BeginRequest::Cached(_)
        ));
        assert!(matches!(
            state.begin(AnalyticsPeriod::Days7, now),
            BeginRequest::Start(_)
        ));
    }

    #[test]
    fn prevents_duplicate_in_flight_requests_for_same_period() {
        let now = Instant::now();
        let mut state = AnalyticsRequestState::default();
        assert!(matches!(
            state.begin(AnalyticsPeriod::Days7, now),
            BeginRequest::Start(_)
        ));
        assert!(matches!(
            state.begin(AnalyticsPeriod::Days7, now),
            BeginRequest::InFlight
        ));
        assert!(matches!(
            state.begin(AnalyticsPeriod::Days30, now),
            BeginRequest::Start(_)
        ));
    }

    #[test]
    fn enforces_rate_limit_cooldown() {
        let now = Instant::now();
        let error = AnalyticsQueryError::RateLimited {
            retry_after: Duration::from_secs(60),
        };
        let mut state = AnalyticsRequestState::default();
        let identity = start_identity(&mut state, AnalyticsPeriod::Hours24, now);
        assert!(state.finish_failure(
            AnalyticsPeriod::Hours24,
            identity,
            now,
            Duration::from_secs(60),
            "2026-07-11T00:01:00Z".to_string(),
            &error,
        ));

        assert!(matches!(
            state.begin(AnalyticsPeriod::Hours24, now + Duration::from_secs(59)),
            BeginRequest::CoolingDown(_)
        ));
        assert!(matches!(
            state.begin(AnalyticsPeriod::Hours24, now + Duration::from_secs(60)),
            BeginRequest::Start(_)
        ));
    }

    #[test]
    fn cached_result_requires_current_credential() {
        let state = leaked_state();
        store_sample_cache(
            &mut lock_request_state(state),
            AnalyticsPeriod::Hours24,
            Instant::now(),
        );

        let result = prepare_summary_request(state, AnalyticsPeriod::Hours24, || {
            Err(CredentialAccessError::NotConfigured)
        });
        assert!(matches!(result, Err(CredentialAccessError::NotConfigured)));

        let result = prepare_summary_request(state, AnalyticsPeriod::Hours24, || {
            Ok("test-token".to_string())
        });
        assert!(matches!(result, Ok(PreparedRequest::Cached(_))));
    }

    #[test]
    fn not_configured_does_not_create_cooldown() {
        let state = leaked_state();
        let result = prepare_summary_request(state, AnalyticsPeriod::Days7, || {
            Err(CredentialAccessError::NotConfigured)
        });

        assert!(matches!(result, Err(CredentialAccessError::NotConfigured)));
        let state = lock_request_state(state);
        assert!(state.periods[AnalyticsPeriod::Days7.index()]
            .cooldown
            .is_none());
        assert!(state.periods[AnalyticsPeriod::Days7.index()]
            .in_flight
            .is_none());
    }

    #[test]
    fn credential_read_error_does_not_create_cooldown() {
        let state = leaked_state();
        let result = prepare_summary_request(state, AnalyticsPeriod::Days7, || {
            Err(CredentialAccessError::ReadFailed)
        });

        assert!(matches!(result, Err(CredentialAccessError::ReadFailed)));
        let state = lock_request_state(state);
        assert!(state.periods[AnalyticsPeriod::Days7.index()]
            .cooldown
            .is_none());
        assert!(state.periods[AnalyticsPeriod::Days7.index()]
            .in_flight
            .is_none());
    }

    #[test]
    fn repeated_generation_changes_stop_with_credential_changed() {
        let state = leaked_state();
        let reads = std::cell::Cell::new(0);
        let result = prepare_summary_request(state, AnalyticsPeriod::Hours24, || {
            reads.set(reads.get() + 1);
            lock_request_state(state).invalidate();
            Ok("test-token".to_string())
        });

        assert!(matches!(result, Ok(PreparedRequest::CredentialChanged)));
        assert_eq!(reads.get(), MAX_CREDENTIAL_GENERATION_ATTEMPTS);
        assert!(
            lock_request_state(state).periods[AnalyticsPeriod::Hours24.index()]
                .in_flight
                .is_none()
        );
    }

    #[test]
    fn invalidating_runtime_clears_all_period_state() {
        let now = Instant::now();
        let mut state = AnalyticsRequestState::default();
        for period in [
            AnalyticsPeriod::Hours24,
            AnalyticsPeriod::Days7,
            AnalyticsPeriod::Days30,
        ] {
            store_sample_cache(&mut state, period, now);
            state.periods[period.index()].cooldown = Some(Cooldown {
                until: now + Duration::from_secs(60),
                retry_at: "2026-07-11T00:01:00Z".to_string(),
                error_code: "rate_limited",
                message: "limited",
            });
            state.periods[period.index()].in_flight = Some(RequestIdentity {
                generation: state.generation,
                request_id: 100 + period.index() as u64,
            });
        }

        state.invalidate();
        for period in &state.periods {
            assert!(period.cache.is_none());
            assert!(period.cooldown.is_none());
            assert!(period.in_flight.is_none());
        }
    }

    #[test]
    fn invalidating_runtime_increments_generation() {
        let mut state = AnalyticsRequestState::default();
        state.generation = u64::MAX;
        state.invalidate();
        assert_eq!(state.generation, 0);
    }

    #[test]
    fn stale_success_cannot_repopulate_after_generation_change() {
        let state = leaked_state();
        let Ok(PreparedRequest::Start { mut lease, .. }) =
            prepare_summary_request(state, AnalyticsPeriod::Hours24, || {
                Ok("test-token".to_string())
            })
        else {
            panic!("request should start");
        };
        lock_request_state(state).invalidate();

        assert!(!lease.finish_success(
            sample_summary(AnalyticsPeriod::Hours24, 10, 8),
            Instant::now(),
        ));
        assert!(
            lock_request_state(state).periods[AnalyticsPeriod::Hours24.index()]
                .cache
                .is_none()
        );
    }

    #[test]
    fn stale_failure_cannot_create_cooldown_after_generation_change() {
        let state = leaked_state();
        let Ok(PreparedRequest::Start { mut lease, .. }) =
            prepare_summary_request(state, AnalyticsPeriod::Days7, || {
                Ok("test-token".to_string())
            })
        else {
            panic!("request should start");
        };
        lock_request_state(state).invalidate();

        assert!(!lease.finish_failure(
            Instant::now(),
            FAILURE_COOLDOWN,
            "2026-07-11T00:00:10Z".to_string(),
            &AnalyticsQueryError::Timeout,
        ));
        assert!(
            lock_request_state(state).periods[AnalyticsPeriod::Days7.index()]
                .cooldown
                .is_none()
        );
    }

    #[test]
    fn dropped_request_lease_releases_in_flight() {
        let state = leaked_state();
        let Ok(PreparedRequest::Start { lease, .. }) =
            prepare_summary_request(state, AnalyticsPeriod::Days30, || {
                Ok("test-token".to_string())
            })
        else {
            panic!("request should start");
        };
        drop(lease);

        assert!(
            lock_request_state(state).periods[AnalyticsPeriod::Days30.index()]
                .in_flight
                .is_none()
        );
    }

    #[test]
    fn old_generation_lease_cannot_clear_new_request() {
        let state = leaked_state();
        let Ok(PreparedRequest::Start { lease: old, .. }) =
            prepare_summary_request(state, AnalyticsPeriod::Hours24, || {
                Ok("old-token".to_string())
            })
        else {
            panic!("old request should start");
        };
        lock_request_state(state).invalidate();
        let Ok(PreparedRequest::Start { lease: new, .. }) =
            prepare_summary_request(state, AnalyticsPeriod::Hours24, || {
                Ok("new-token".to_string())
            })
        else {
            panic!("new request should start");
        };
        let new_identity = new.identity;

        drop(old);
        assert_eq!(
            lock_request_state(state).periods[AnalyticsPeriod::Hours24.index()].in_flight,
            Some(new_identity)
        );
        drop(new);
    }

    #[test]
    fn all_adapter_error_paths_release_in_flight() {
        let errors = [
            AnalyticsQueryError::HttpClientFailed,
            AnalyticsQueryError::InvalidRequest,
            AnalyticsQueryError::Unauthorized,
            AnalyticsQueryError::PlanOrBillingRequired,
            AnalyticsQueryError::Forbidden,
            AnalyticsQueryError::NotFound,
            AnalyticsQueryError::RateLimited {
                retry_after: Duration::from_secs(60),
            },
            AnalyticsQueryError::ServiceUnavailable,
            AnalyticsQueryError::Timeout,
            AnalyticsQueryError::Network,
            AnalyticsQueryError::RequestFailed,
            AnalyticsQueryError::ResponseTooLarge,
            AnalyticsQueryError::InvalidResponse,
            AnalyticsQueryError::ApiError,
        ];

        for error in errors {
            let state = leaked_state();
            let Ok(PreparedRequest::Start { mut lease, .. }) =
                prepare_summary_request(state, AnalyticsPeriod::Hours24, || {
                    Ok("test-token".to_string())
                })
            else {
                panic!("request should start");
            };
            let (_, applied) =
                finish_failed_request(&mut lease, Instant::now(), 1_750_000_000_000, &error);
            assert!(applied);
            assert!(
                lock_request_state(state).periods[AnalyticsPeriod::Hours24.index()]
                    .in_flight
                    .is_none()
            );
        }
    }

    #[test]
    fn general_adapter_failure_creates_ten_second_cooldown() {
        let state = leaked_state();
        let now = Instant::now();
        let Ok(PreparedRequest::Start { mut lease, .. }) =
            prepare_summary_request(state, AnalyticsPeriod::Hours24, || {
                Ok("test-token".to_string())
            })
        else {
            panic!("request should start");
        };
        let (_, applied) = finish_failed_request(
            &mut lease,
            now,
            1_750_000_000_000,
            &AnalyticsQueryError::Timeout,
        );

        assert!(applied);
        assert!(matches!(
            lock_request_state(state).begin(AnalyticsPeriod::Hours24, now + Duration::from_secs(9)),
            BeginRequest::CoolingDown(_)
        ));
        assert!(matches!(
            lock_request_state(state)
                .begin(AnalyticsPeriod::Hours24, now + Duration::from_secs(10)),
            BeginRequest::Start(_)
        ));
    }

    #[test]
    fn parses_numeric_retry_after() {
        let mut headers = HeaderMap::new();
        headers.insert(reqwest::header::RETRY_AFTER, "120".parse().unwrap());
        assert_eq!(
            vercel_adapter::rate_limit_for_test(&headers, 1_750_000_000_000),
            Duration::from_secs(120)
        );
    }

    #[test]
    fn parses_rate_limit_reset_seconds() {
        let now_ms = 1_750_000_000_000;
        let mut headers = HeaderMap::new();
        headers.insert("x-ratelimit-reset", "1750000090".parse().unwrap());
        assert_eq!(
            vercel_adapter::rate_limit_for_test(&headers, now_ms),
            Duration::from_secs(90)
        );
    }

    #[test]
    fn parses_rate_limit_reset_milliseconds() {
        let now_ms = 1_750_000_000_000;
        let mut headers = HeaderMap::new();
        headers.insert("x-ratelimit-reset", "1750000045000".parse().unwrap());
        assert_eq!(
            vercel_adapter::rate_limit_for_test(&headers, now_ms),
            Duration::from_secs(45)
        );
    }

    #[test]
    fn chooses_later_rate_limit_deadline() {
        let now_ms = 1_750_000_000_000;
        let mut headers = HeaderMap::new();
        headers.insert(reqwest::header::RETRY_AFTER, "30".parse().unwrap());
        headers.insert("x-ratelimit-reset", "1750000090000".parse().unwrap());
        assert_eq!(
            vercel_adapter::rate_limit_for_test(&headers, now_ms),
            Duration::from_secs(90)
        );
    }

    #[test]
    fn caps_rate_limit_at_24_hours() {
        let mut headers = HeaderMap::new();
        headers.insert(reqwest::header::RETRY_AFTER, "172800".parse().unwrap());
        assert_eq!(
            vercel_adapter::rate_limit_for_test(&headers, 1_750_000_000_000),
            Duration::from_secs(24 * 60 * 60)
        );
    }

    #[test]
    fn invalid_rate_limit_headers_use_sixty_seconds() {
        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::RETRY_AFTER,
            "not-a-number".parse().unwrap(),
        );
        headers.insert("x-ratelimit-reset", "invalid".parse().unwrap());
        assert_eq!(
            vercel_adapter::rate_limit_for_test(&headers, 1_750_000_000_000),
            Duration::from_secs(60)
        );
    }

    #[test]
    fn rate_limit_is_isolated_per_period() {
        let state = leaked_state();
        let now = Instant::now();
        let identity = start_identity(
            &mut lock_request_state(state),
            AnalyticsPeriod::Hours24,
            now,
        );
        assert!(lock_request_state(state).finish_failure(
            AnalyticsPeriod::Hours24,
            identity,
            now,
            Duration::from_secs(60),
            "2026-07-11T00:01:00Z".to_string(),
            &AnalyticsQueryError::RateLimited {
                retry_after: Duration::from_secs(60),
            },
        ));

        assert!(matches!(
            lock_request_state(state).begin(AnalyticsPeriod::Hours24, now),
            BeginRequest::CoolingDown(_)
        ));
        assert!(matches!(
            lock_request_state(state).begin(AnalyticsPeriod::Days7, now),
            BeginRequest::Start(_)
        ));
    }

    #[test]
    fn connection_test_bypasses_summary_cache() {
        let credential_reads = std::cell::Cell::new(0);
        let fetches = std::cell::Cell::new(0);
        let result = tauri::async_runtime::block_on(test_connection_with(
            || {
                credential_reads.set(credential_reads.get() + 1);
                Ok("test-token".to_string())
            },
            |_, _| {
                fetches.set(fetches.get() + 1);
                async {
                    Ok(vercel_adapter::CountResult {
                        pageviews: 1,
                        visitors: 1,
                    })
                }
            },
        ));

        assert_eq!(result.status, "connected");
        assert_eq!(credential_reads.get(), 1);
        assert_eq!(fetches.get(), 1);
    }

    #[test]
    fn summary_dto_contains_no_secret_or_adapter_values() {
        let dto = AnalyticsSummaryResponse::success(
            sample_summary(AnalyticsPeriod::Days7, 42, 30),
            false,
        );
        let json = serde_json::to_string(&dto).unwrap();

        assert!(!json.contains(vercel_adapter::PROJECT_ID));
        assert!(!json.contains(vercel_adapter::TEAM_ID));
        assert!(!json.contains(vercel_adapter::ENDPOINT));
        assert!(!json.contains("Authorization"));
        assert!(!json.contains("Bearer"));
        assert!(!json.contains("token"));
    }

    #[test]
    fn produces_kst_connection_timestamp() {
        let timestamp = current_kst_timestamp().unwrap();
        assert!(timestamp.ends_with("+09:00"));
        assert_eq!(timestamp.len(), 25);
    }
}
