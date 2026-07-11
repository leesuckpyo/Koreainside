use crate::credentials::{connection_result, read_vercel_access_token, VercelConnectionStatus};
use reqwest::{redirect::Policy, StatusCode};
use serde::Deserialize;
use std::time::Duration;
use time::{format_description, OffsetDateTime, UtcOffset};

const VERCEL_ANALYTICS_ENDPOINT: &str =
    "https://api.vercel.com/v1/query/web-analytics/visits/count";
const VERCEL_PROJECT_ID: &str = "prj_E6IPJCgBEvh3J7Ga8AMNXeq3Gb4M";
const VERCEL_TEAM_ID: &str = "team_C5MoIigwWP5u47OShZDSyz4Z";
const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_RESPONSE_BYTES: usize = 64 * 1024;

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

#[tauri::command]
pub async fn test_vercel_analytics_connection() -> VercelConnectionStatus {
    let token = match read_vercel_access_token() {
        Ok(token) => token,
        Err(keyring_core::Error::NoEntry) => {
            return connection_result(
                "not_configured",
                false,
                None,
                Some("TOKEN_NOT_CONFIGURED"),
                Some("저장된 Vercel Access Token이 없습니다."),
            );
        }
        Err(_) => {
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
    let client = match reqwest::Client::builder()
        .user_agent(concat!("Korea-Inside-Admin/", env!("CARGO_PKG_VERSION")))
        .https_only(true)
        .redirect(Policy::none())
        .referer(false)
        .connect_timeout(CONNECT_TIMEOUT)
        .timeout(REQUEST_TIMEOUT)
        .retry(reqwest::retry::never())
        .build()
    {
        Ok(client) => client,
        Err(_) => {
            return connection_result(
                "error",
                true,
                checked_at,
                Some("HTTP_CLIENT_FAILED"),
                Some("보안 HTTP 연결을 준비할 수 없습니다."),
            );
        }
    };

    let now = OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000;
    let since = now - 86_400_000;
    let response = client
        .get(VERCEL_ANALYTICS_ENDPOINT)
        .bearer_auth(token)
        .query(&[
            ("projectId", VERCEL_PROJECT_ID.to_string()),
            ("teamId", VERCEL_TEAM_ID.to_string()),
            ("since", since.to_string()),
            ("until", now.to_string()),
        ])
        .send()
        .await;

    match response {
        Ok(response) if response.status() == StatusCode::OK => {
            let body = match response.bytes().await {
                Ok(body) if body.len() <= MAX_RESPONSE_BYTES => body,
                _ => return invalid_response(checked_at),
            };
            if validate_success_response(&body) {
                connection_result("connected", true, checked_at, None, None)
            } else {
                invalid_response(checked_at)
            }
        }
        Ok(response) => status_from_http(response.status(), checked_at),
        Err(error) if error.is_timeout() => connection_result(
            "error",
            true,
            checked_at,
            Some("REQUEST_TIMEOUT"),
            Some("Vercel 연결 시간이 초과되었습니다."),
        ),
        Err(error) if error.is_connect() => connection_result(
            "error",
            true,
            checked_at,
            Some("NETWORK_ERROR"),
            Some("Vercel API에 연결할 수 없습니다."),
        ),
        Err(_) => connection_result(
            "error",
            true,
            checked_at,
            Some("REQUEST_FAILED"),
            Some("Vercel 연결 요청을 완료할 수 없습니다."),
        ),
    }
}

fn status_from_http(status: StatusCode, checked_at: Option<String>) -> VercelConnectionStatus {
    match status {
        StatusCode::BAD_REQUEST => connection_result(
            "error",
            true,
            checked_at,
            Some("invalid_request"),
            Some("Vercel Analytics 연결 요청이 올바르지 않습니다."),
        ),
        StatusCode::UNAUTHORIZED => connection_result(
            "error",
            true,
            checked_at,
            Some("UNAUTHORIZED"),
            Some("Vercel 자격 증명이 유효하지 않습니다."),
        ),
        StatusCode::PAYMENT_REQUIRED => connection_result(
            "error",
            true,
            checked_at,
            Some("plan_or_billing_required"),
            Some("Vercel 플랜 또는 결제 상태를 확인해 주십시오."),
        ),
        StatusCode::FORBIDDEN => connection_result(
            "error",
            true,
            checked_at,
            Some("FORBIDDEN"),
            Some("Vercel Analytics를 읽을 권한이 없습니다."),
        ),
        StatusCode::NOT_FOUND => connection_result(
            "error",
            true,
            checked_at,
            Some("PROJECT_NOT_FOUND"),
            Some("Vercel 프로젝트 또는 Analytics 데이터를 확인할 수 없습니다."),
        ),
        StatusCode::TOO_MANY_REQUESTS => connection_result(
            "rate_limited",
            true,
            checked_at,
            Some("RATE_LIMITED"),
            Some("Vercel 요청 제한 상태입니다. 잠시 후 다시 확인해 주십시오."),
        ),
        _ if status.is_server_error() => connection_result(
            "error",
            true,
            checked_at,
            Some("VERCEL_SERVICE_ERROR"),
            Some("Vercel 서비스가 요청을 처리하지 못했습니다."),
        ),
        _ => connection_result(
            "error",
            true,
            checked_at,
            Some("API_ERROR"),
            Some("Vercel Analytics 연결을 확인할 수 없습니다."),
        ),
    }
}

fn validate_success_response(body: &[u8]) -> bool {
    serde_json::from_slice::<VercelCountResponse>(body)
        .map(|response| {
            let _ = (
                response.version,
                response.data.pageviews,
                response.data.visitors,
            );
            true
        })
        .unwrap_or(false)
}

fn invalid_response(checked_at: Option<String>) -> VercelConnectionStatus {
    connection_result(
        "error",
        true,
        checked_at,
        Some("invalid_response"),
        Some("Vercel Analytics 응답 형식을 확인할 수 없습니다."),
    )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_supported_http_statuses_without_response_details() {
        assert_eq!(
            status_from_http(StatusCode::BAD_REQUEST, None).error_code,
            Some("invalid_request")
        );
        assert_eq!(
            status_from_http(StatusCode::UNAUTHORIZED, None).error_code,
            Some("UNAUTHORIZED")
        );
        assert_eq!(
            status_from_http(StatusCode::PAYMENT_REQUIRED, None).error_code,
            Some("plan_or_billing_required")
        );
        assert_eq!(
            status_from_http(StatusCode::FORBIDDEN, None).error_code,
            Some("FORBIDDEN")
        );
        assert_eq!(
            status_from_http(StatusCode::NOT_FOUND, None).error_code,
            Some("PROJECT_NOT_FOUND")
        );
        assert_eq!(
            status_from_http(StatusCode::TOO_MANY_REQUESTS, None).status,
            "rate_limited"
        );
    }

    #[test]
    fn produces_kst_check_timestamp() {
        let timestamp = current_kst_timestamp().unwrap();
        assert!(timestamp.ends_with("+09:00"));
        assert_eq!(timestamp.len(), 25);
    }

    #[test]
    fn validates_minimum_count_response_and_allows_unknown_fields() {
        let response = br#"{
            "version": 1,
            "data": {
                "pageviews": 42,
                "visitors": 30,
                "futureField": "allowed"
            },
            "anotherFutureField": true
        }"#;
        assert!(validate_success_response(response));
    }

    #[test]
    fn rejects_missing_or_invalid_count_response_fields() {
        assert!(!validate_success_response(
            br#"{"version":1,"data":{"pageviews":42}}"#
        ));
        assert!(!validate_success_response(
            br#"{"version":"1","data":{"pageviews":42,"visitors":30}}"#
        ));
        assert!(!validate_success_response(b"not-json"));
    }
}
