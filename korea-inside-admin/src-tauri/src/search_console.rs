use keyring_core::{Entry, Error as KeyringError};
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope,
    TokenUrl,
};
use reqwest::{header::CONTENT_TYPE, redirect::Policy, StatusCode};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    io::{self, Read, Write},
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream},
    sync::{Mutex, MutexGuard, OnceLock},
    thread,
    time::{Duration, Instant},
};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use url::form_urlencoded;
use windows_native_keyring_store::{CredPersist, Store};

const CREDENTIAL_SERVICE: &str = "com.getkoreainside.admin.search-console";
const CLIENT_ID_ACCOUNT: &str = "oauth-client-id";
const REFRESH_TOKEN_ACCOUNT: &str = "refresh-token";
#[cfg(test)]
const VERCEL_CREDENTIAL_SERVICE_FOR_TEST: &str = "com.getkoreainside.admin.vercel";

const AUTHORIZATION_ENDPOINT: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const TOKEN_ENDPOINT: &str = "https://oauth2.googleapis.com/token";
const REVOKE_ENDPOINT: &str = "https://oauth2.googleapis.com/revoke";
const SITES_LIST_ENDPOINT: &str = "https://www.googleapis.com/webmasters/v3/sites";
const SEARCH_CONSOLE_SCOPE: &str = "https://www.googleapis.com/auth/webmasters.readonly";
const CALLBACK_PATH: &str = "/search-console/oauth/callback";

const CLIENT_ID_SUFFIX: &str = ".apps.googleusercontent.com";
const MAX_CLIENT_ID_LENGTH: usize = 256;
const AUTHORIZATION_TIMEOUT: Duration = Duration::from_secs(300);
const CALLBACK_ACCEPT_SLEEP: Duration = Duration::from_millis(50);
const CALLBACK_READ_TIMEOUT: Duration = Duration::from_secs(2);
const MAX_CALLBACK_REQUEST_BYTES: usize = 8 * 1024;
const MAX_RESPONSE_BYTES: usize = 64 * 1024;
const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
const ACCESS_TOKEN_REFRESH_MARGIN: Duration = Duration::from_secs(60);

type CommandResult<T> = Result<T, SearchConsoleCommandError>;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchConsoleClientStatus {
    configured: bool,
    authorization_stored: bool,
    connected: bool,
    authentication_in_progress: bool,
    reauthentication_required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_checked_at: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchConsoleActionResult {
    status: &'static str,
    client_status: SearchConsoleClientStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    revoke_attempted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revoke_succeeded: Option<bool>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchConsoleCommandError {
    code: &'static str,
    message: &'static str,
}

impl SearchConsoleCommandError {
    fn new(error: SearchConsoleError) -> Self {
        Self {
            code: error.code(),
            message: error.message(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SearchConsoleError {
    NotConfigured,
    AlreadyInProgress,
    InvalidClientId,
    CredentialStoreFailed,
    CredentialReadFailed,
    CredentialDeleteFailed,
    ListenerBindFailed,
    BrowserOpenFailed,
    AuthorizationDenied,
    AuthorizationFailed,
    CallbackTimeout,
    NetworkTimeout,
    InvalidCallback,
    StateMismatch,
    TokenExchangeFailed,
    MissingRefreshToken,
    ScopeNotGranted,
    ReauthenticationRequired,
    ApiRequestFailed,
    RevokeFailed,
    Internal,
}

impl SearchConsoleError {
    fn code(self) -> &'static str {
        match self {
            Self::NotConfigured => "not_configured",
            Self::AlreadyInProgress => "already_in_progress",
            Self::InvalidClientId => "invalid_client_id",
            Self::CredentialStoreFailed => "credential_store_failed",
            Self::CredentialReadFailed => "credential_read_failed",
            Self::CredentialDeleteFailed => "credential_delete_failed",
            Self::ListenerBindFailed => "listener_bind_failed",
            Self::BrowserOpenFailed => "browser_open_failed",
            Self::AuthorizationDenied => "authorization_denied",
            Self::AuthorizationFailed => "authorization_failed",
            Self::CallbackTimeout => "callback_timeout",
            Self::NetworkTimeout => "network_timeout",
            Self::InvalidCallback => "invalid_callback",
            Self::StateMismatch => "state_mismatch",
            Self::TokenExchangeFailed => "token_exchange_failed",
            Self::MissingRefreshToken => "missing_refresh_token",
            Self::ScopeNotGranted => "scope_not_granted",
            Self::ReauthenticationRequired => "reauthentication_required",
            Self::ApiRequestFailed => "api_request_failed",
            Self::RevokeFailed => "revoke_failed",
            Self::Internal => "internal_error",
        }
    }

    fn message(self) -> &'static str {
        match self {
            Self::NotConfigured => {
                "Search Console OAuth Client ID 또는 연결 토큰이 설정되지 않았습니다."
            }
            Self::AlreadyInProgress => "Search Console 연결 작업이 이미 진행 중입니다.",
            Self::InvalidClientId => "Google Desktop OAuth Client ID 형식을 확인해 주십시오.",
            Self::CredentialStoreFailed => {
                "Windows 자격 증명 관리자에 Search Console 설정을 저장할 수 없습니다."
            }
            Self::CredentialReadFailed => {
                "Windows 자격 증명 관리자에서 Search Console 설정을 읽을 수 없습니다."
            }
            Self::CredentialDeleteFailed => {
                "Windows 자격 증명 관리자에서 Search Console 설정을 삭제할 수 없습니다."
            }
            Self::ListenerBindFailed => "로컬 인증 callback 수신기를 시작할 수 없습니다.",
            Self::BrowserOpenFailed => "기본 브라우저에서 Google 인증 화면을 열 수 없습니다.",
            Self::AuthorizationDenied => "Google 인증이 승인되지 않았습니다.",
            Self::AuthorizationFailed => "Google 인증을 완료할 수 없습니다.",
            Self::CallbackTimeout => "Google 인증 대기 시간이 초과되었습니다.",
            Self::NetworkTimeout => "Google 연결 요청 시간이 초과되었습니다.",
            Self::InvalidCallback => "Google 인증 callback 형식을 확인할 수 없습니다.",
            Self::StateMismatch => "Google 인증 state 검증에 실패했습니다.",
            Self::TokenExchangeFailed => "Google 인증 코드를 토큰으로 교환할 수 없습니다.",
            Self::MissingRefreshToken => "Google 응답에 refresh token이 포함되지 않았습니다.",
            Self::ScopeNotGranted => "Search Console 읽기 전용 권한이 승인되지 않았습니다.",
            Self::ReauthenticationRequired => "Google Search Console 재인증이 필요합니다.",
            Self::ApiRequestFailed => "Google Search Console 연결 시험을 완료할 수 없습니다.",
            Self::RevokeFailed => "Google Search Console 연결 해제 요청을 완료할 수 없습니다.",
            Self::Internal => "Search Console 연결 상태를 처리할 수 없습니다.",
        }
    }
}

impl From<SearchConsoleError> for SearchConsoleCommandError {
    fn from(error: SearchConsoleError) -> Self {
        Self::new(error)
    }
}

#[derive(Default)]
struct SearchConsoleRuntimeState {
    authentication_in_progress: bool,
    refresh_in_progress: bool,
    disconnect_in_progress: bool,
    client_configuration_in_progress: bool,
    connected: bool,
    reauthentication_required: bool,
    last_checked_at: Option<String>,
    access_token: Option<AccessTokenCache>,
}

struct AccessTokenCache {
    token: String,
    expires_at: Instant,
}

#[derive(Clone, Copy)]
enum OperationKind {
    Authentication,
    Refresh,
    Disconnect,
    ClientConfiguration,
}

struct OperationGuard {
    kind: OperationKind,
    active: bool,
}

impl OperationGuard {
    fn begin(kind: OperationKind) -> Result<Self, SearchConsoleError> {
        let mut state = lock_runtime_state()?;
        if state.authentication_in_progress
            || state.refresh_in_progress
            || state.disconnect_in_progress
            || state.client_configuration_in_progress
        {
            return Err(SearchConsoleError::AlreadyInProgress);
        }
        match kind {
            OperationKind::Authentication => state.authentication_in_progress = true,
            OperationKind::Refresh => state.refresh_in_progress = true,
            OperationKind::Disconnect => state.disconnect_in_progress = true,
            OperationKind::ClientConfiguration => state.client_configuration_in_progress = true,
        }
        Ok(Self { kind, active: true })
    }
}

impl Drop for OperationGuard {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        if let Ok(mut state) = runtime_state().lock() {
            match self.kind {
                OperationKind::Authentication => state.authentication_in_progress = false,
                OperationKind::Refresh => state.refresh_in_progress = false,
                OperationKind::Disconnect => state.disconnect_in_progress = false,
                OperationKind::ClientConfiguration => {
                    state.client_configuration_in_progress = false
                }
            }
        }
        self.active = false;
    }
}

struct AuthorizationRequest {
    authorization_url: String,
    redirect_uri: String,
    state: String,
    pkce_verifier: String,
}

#[derive(Debug, Eq, PartialEq)]
enum CallbackOutcome {
    Authorized { code: String },
}

#[derive(Debug)]
struct TokenSet {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: Duration,
}

#[derive(Deserialize)]
struct GoogleTokenResponse {
    access_token: Option<String>,
    expires_in: Option<u64>,
    refresh_token: Option<String>,
    scope: Option<String>,
}

#[derive(Deserialize)]
struct GoogleErrorResponse {
    error: Option<String>,
}

#[derive(Deserialize)]
struct SitesListResponse {
    #[serde(rename = "siteEntry")]
    site_entries: Option<Vec<SiteEntry>>,
}

#[derive(Deserialize)]
struct SiteEntry {
    #[serde(rename = "siteUrl")]
    _site_url: String,
    #[serde(rename = "permissionLevel")]
    _permission_level: String,
}

#[tauri::command]
pub fn get_search_console_client_status() -> CommandResult<SearchConsoleClientStatus> {
    client_status().map_err(Into::into)
}

#[tauri::command]
pub fn save_search_console_client_id(
    client_id: String,
) -> CommandResult<SearchConsoleClientStatus> {
    let client_id = validate_client_id(&client_id).map_err(SearchConsoleCommandError::from)?;
    let _guard = OperationGuard::begin(OperationKind::ClientConfiguration)
        .map_err(SearchConsoleCommandError::from)?;
    save_search_console_client_id_with(
        client_id,
        read_stored_client_id,
        |value| save_credential(CLIENT_ID_ACCOUNT, value),
        || delete_credential_if_present(REFRESH_TOKEN_ACCOUNT),
        || clear_runtime_connection_state(false),
        client_status,
    )
    .map_err(Into::into)
}

#[tauri::command]
pub fn delete_search_console_client_id() -> CommandResult<SearchConsoleClientStatus> {
    let _guard = OperationGuard::begin(OperationKind::ClientConfiguration)
        .map_err(SearchConsoleCommandError::from)?;
    delete_search_console_client_id_with(
        || delete_credential_if_present(REFRESH_TOKEN_ACCOUNT),
        || clear_runtime_connection_state(false),
        || delete_credential_if_present(CLIENT_ID_ACCOUNT),
        client_status,
    )
    .map_err(Into::into)
}

#[tauri::command]
pub async fn start_search_console_oauth() -> CommandResult<SearchConsoleActionResult> {
    let _guard = OperationGuard::begin(OperationKind::Authentication)
        .map_err(SearchConsoleCommandError::from)?;
    let client_id = read_credential(CLIENT_ID_ACCOUNT).map_err(|error| match error {
        KeyringError::NoEntry => SearchConsoleCommandError::from(SearchConsoleError::NotConfigured),
        _ => SearchConsoleCommandError::from(SearchConsoleError::CredentialReadFailed),
    })?;
    let client_id = validate_client_id(&client_id).map_err(SearchConsoleCommandError::from)?;

    let listener = bind_callback_listener().map_err(SearchConsoleCommandError::from)?;
    let port = listener
        .local_addr()
        .map_err(|_| SearchConsoleCommandError::from(SearchConsoleError::ListenerBindFailed))?
        .port();
    let request =
        build_authorization_request(&client_id, port).map_err(SearchConsoleCommandError::from)?;
    let expected_state = request.state.clone();

    if webbrowser::open(&request.authorization_url).is_err() {
        return Err(SearchConsoleCommandError::from(
            SearchConsoleError::BrowserOpenFailed,
        ));
    }

    let callback = tauri::async_runtime::spawn_blocking(move || {
        wait_for_callback(listener, expected_state, AUTHORIZATION_TIMEOUT)
    })
    .await
    .map_err(|_| SearchConsoleCommandError::from(SearchConsoleError::Internal))?
    .map_err(SearchConsoleCommandError::from)?;

    let CallbackOutcome::Authorized { code } = callback;

    let tokens = exchange_authorization_code(
        &client_id,
        &request.redirect_uri,
        &code,
        &request.pkce_verifier,
    )
    .await
    .map_err(SearchConsoleCommandError::from)?;
    let refresh_token = tokens
        .refresh_token
        .ok_or(SearchConsoleError::MissingRefreshToken)
        .map_err(SearchConsoleCommandError::from)?;

    fetch_sites_list(&tokens.access_token)
        .await
        .map_err(SearchConsoleCommandError::from)?;
    save_credential(REFRESH_TOKEN_ACCOUNT, &refresh_token)
        .map_err(SearchConsoleCommandError::from)?;
    store_access_token(tokens.access_token, tokens.expires_in)?;
    set_last_checked_now(false)?;

    Ok(SearchConsoleActionResult {
        status: "connected",
        client_status: client_status().map_err(SearchConsoleCommandError::from)?,
        revoke_attempted: None,
        revoke_succeeded: None,
    })
}

#[tauri::command]
pub async fn disconnect_search_console() -> CommandResult<SearchConsoleActionResult> {
    let _guard = OperationGuard::begin(OperationKind::Disconnect)
        .map_err(SearchConsoleCommandError::from)?;
    let refresh_token = match read_credential(REFRESH_TOKEN_ACCOUNT) {
        Ok(token) => Some(token),
        Err(KeyringError::NoEntry) => None,
        Err(_) => {
            return Err(SearchConsoleCommandError::from(
                SearchConsoleError::CredentialReadFailed,
            ));
        }
    };

    let mut revoke_attempted = false;
    let mut revoke_succeeded = None;
    if let Some(token) = refresh_token.as_deref() {
        revoke_attempted = true;
        revoke_succeeded = Some(revoke_token(token).await.is_ok());
    }

    delete_credential_if_present(REFRESH_TOKEN_ACCOUNT).map_err(SearchConsoleCommandError::from)?;
    clear_runtime_connection_state(false)?;

    Ok(SearchConsoleActionResult {
        status: if revoke_attempted && revoke_succeeded == Some(false) {
            "disconnected_revoke_failed"
        } else {
            "disconnected"
        },
        client_status: client_status().map_err(SearchConsoleCommandError::from)?,
        revoke_attempted: Some(revoke_attempted),
        revoke_succeeded,
    })
}

#[tauri::command]
pub async fn test_search_console_connection() -> CommandResult<SearchConsoleClientStatus> {
    let _guard =
        OperationGuard::begin(OperationKind::Refresh).map_err(SearchConsoleCommandError::from)?;
    let client_id = read_credential(CLIENT_ID_ACCOUNT).map_err(|error| match error {
        KeyringError::NoEntry => SearchConsoleCommandError::from(SearchConsoleError::NotConfigured),
        _ => SearchConsoleCommandError::from(SearchConsoleError::CredentialReadFailed),
    })?;
    validate_client_id(&client_id).map_err(SearchConsoleCommandError::from)?;
    read_credential(REFRESH_TOKEN_ACCOUNT).map_err(|error| match error {
        KeyringError::NoEntry => SearchConsoleCommandError::from(SearchConsoleError::NotConfigured),
        _ => SearchConsoleCommandError::from(SearchConsoleError::CredentialReadFailed),
    })?;

    let access_token = match refresh_access_token().await {
        Ok(token) => token,
        Err(error) => {
            if !matches!(
                error,
                SearchConsoleError::ReauthenticationRequired
                    | SearchConsoleError::CredentialDeleteFailed
            ) {
                mark_connection_failed(false).map_err(SearchConsoleCommandError::from)?;
            }
            return Err(SearchConsoleCommandError::from(error));
        }
    };

    if let Err(error) = fetch_sites_list(&access_token).await {
        mark_connection_failed(false).map_err(SearchConsoleCommandError::from)?;
        return Err(SearchConsoleCommandError::from(error));
    }

    set_last_checked_now(false).map_err(SearchConsoleCommandError::from)?;
    client_status().map_err(Into::into)
}

async fn refresh_access_token() -> Result<String, SearchConsoleError> {
    if let Some(token) = cached_access_token()? {
        return Ok(token);
    }

    if let Some(token) = cached_access_token()? {
        return Ok(token);
    }

    let client_id = read_credential(CLIENT_ID_ACCOUNT).map_err(|error| match error {
        KeyringError::NoEntry => SearchConsoleError::NotConfigured,
        _ => SearchConsoleError::CredentialReadFailed,
    })?;
    let client_id = validate_client_id(&client_id)?;
    let refresh_token = read_credential(REFRESH_TOKEN_ACCOUNT).map_err(|error| match error {
        KeyringError::NoEntry => SearchConsoleError::NotConfigured,
        _ => SearchConsoleError::CredentialReadFailed,
    })?;

    let tokens = refresh_access_token_with(&client_id, &refresh_token).await?;
    if let Some(new_refresh_token) = tokens.refresh_token.as_deref() {
        save_credential(REFRESH_TOKEN_ACCOUNT, new_refresh_token)?;
    }
    store_access_token(tokens.access_token.clone(), tokens.expires_in)?;
    Ok(tokens.access_token)
}

async fn refresh_access_token_with(
    client_id: &str,
    refresh_token: &str,
) -> Result<TokenSet, SearchConsoleError> {
    let client = secure_http_client()?;
    let body = form_request_body(&[
        ("grant_type", "refresh_token"),
        ("client_id", client_id),
        ("refresh_token", refresh_token),
    ]);
    let response = client
        .post(TOKEN_ENDPOINT)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .map_err(map_request_error)?;

    let status = response.status();
    let body = read_limited_body(response).await?;
    if status != StatusCode::OK {
        if token_error_is_invalid_grant(&body) {
            mark_reauthentication_required()?;
            delete_credential_if_present(REFRESH_TOKEN_ACCOUNT)?;
            return Err(SearchConsoleError::ReauthenticationRequired);
        }
        return Err(SearchConsoleError::TokenExchangeFailed);
    }
    parse_refresh_token_response(&body)
}

async fn exchange_authorization_code(
    client_id: &str,
    redirect_uri: &str,
    code: &str,
    pkce_verifier: &str,
) -> Result<TokenSet, SearchConsoleError> {
    let client = secure_http_client()?;
    let body = form_request_body(&[
        ("grant_type", "authorization_code"),
        ("client_id", client_id),
        ("code", code),
        ("redirect_uri", redirect_uri),
        ("code_verifier", pkce_verifier),
    ]);
    let response = client
        .post(TOKEN_ENDPOINT)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .map_err(map_request_error)?;

    let status = response.status();
    let body = read_limited_body(response).await?;
    if status != StatusCode::OK {
        return Err(SearchConsoleError::TokenExchangeFailed);
    }
    parse_initial_token_response(&body)
}

async fn fetch_sites_list(access_token: &str) -> Result<(), SearchConsoleError> {
    let client = secure_http_client()?;
    let response = client
        .get(SITES_LIST_ENDPOINT)
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(map_request_error)?;

    if response.status() != StatusCode::OK {
        return Err(SearchConsoleError::ApiRequestFailed);
    }
    let body = read_limited_body(response).await?;
    parse_sites_list_response(&body)?;
    Ok(())
}

async fn revoke_token(refresh_token: &str) -> Result<(), SearchConsoleError> {
    let client = secure_http_client()?;
    let body = form_request_body(&[("token", refresh_token)]);
    let response = client
        .post(REVOKE_ENDPOINT)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .map_err(map_request_error)?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(SearchConsoleError::RevokeFailed)
    }
}

fn bind_callback_listener() -> Result<TcpListener, SearchConsoleError> {
    let address = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0);
    let listener =
        TcpListener::bind(address).map_err(|_| SearchConsoleError::ListenerBindFailed)?;
    listener
        .set_nonblocking(true)
        .map_err(|_| SearchConsoleError::ListenerBindFailed)?;
    Ok(listener)
}

fn build_authorization_request(
    client_id: &str,
    port: u16,
) -> Result<AuthorizationRequest, SearchConsoleError> {
    let redirect_uri = format!("http://127.0.0.1:{port}{CALLBACK_PATH}");
    let auth_url = AuthUrl::new(AUTHORIZATION_ENDPOINT.to_string())
        .map_err(|_| SearchConsoleError::Internal)?;
    let token_url =
        TokenUrl::new(TOKEN_ENDPOINT.to_string()).map_err(|_| SearchConsoleError::Internal)?;
    let redirect_url =
        RedirectUrl::new(redirect_uri.clone()).map_err(|_| SearchConsoleError::Internal)?;
    let client = BasicClient::new(
        ClientId::new(client_id.to_string()),
        None,
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(redirect_url);
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (authorization_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(SEARCH_CONSOLE_SCOPE.to_string()))
        .set_pkce_challenge(pkce_challenge)
        .add_extra_param("access_type", "offline")
        .add_extra_param("prompt", "consent")
        .url();

    Ok(AuthorizationRequest {
        authorization_url: authorization_url.to_string(),
        redirect_uri,
        state: csrf_token.secret().to_string(),
        pkce_verifier: pkce_verifier.secret().to_string(),
    })
}

fn wait_for_callback(
    listener: TcpListener,
    expected_state: String,
    timeout: Duration,
) -> Result<CallbackOutcome, SearchConsoleError> {
    let deadline = Instant::now() + timeout;
    loop {
        if Instant::now() >= deadline {
            return Err(SearchConsoleError::CallbackTimeout);
        }
        match listener.accept() {
            Ok((mut stream, peer)) => {
                if !is_loopback_peer(peer) {
                    let _ = write_callback_response(&mut stream, false);
                    return Err(SearchConsoleError::InvalidCallback);
                }
                let request = match read_limited_http_request(&mut stream) {
                    Ok(request) => request,
                    Err(_) => {
                        let _ = write_callback_response(&mut stream, false);
                        return Err(SearchConsoleError::InvalidCallback);
                    }
                };
                let parsed = parse_callback_request(&request, &expected_state);
                let _ = write_callback_response(&mut stream, parsed.is_ok());
                return parsed;
            }
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(CALLBACK_ACCEPT_SLEEP);
            }
            Err(_) => return Err(SearchConsoleError::InvalidCallback),
        }
    }
}

fn read_limited_http_request(stream: &mut TcpStream) -> io::Result<String> {
    stream.set_read_timeout(Some(CALLBACK_READ_TIMEOUT))?;
    let mut buffer = Vec::new();
    let mut chunk = [0_u8; 512];
    loop {
        let read = stream.read(&mut chunk)?;
        if read == 0 {
            break;
        }
        if buffer.len().saturating_add(read) > MAX_CALLBACK_REQUEST_BYTES {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "callback request too large",
            ));
        }
        buffer.extend_from_slice(&chunk[..read]);
        if buffer.windows(4).any(|window| window == b"\r\n\r\n") {
            break;
        }
    }
    String::from_utf8(buffer)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "callback is not utf-8"))
}

fn parse_callback_request(
    request: &str,
    expected_state: &str,
) -> Result<CallbackOutcome, SearchConsoleError> {
    if request.len() > MAX_CALLBACK_REQUEST_BYTES {
        return Err(SearchConsoleError::InvalidCallback);
    }
    let first_line = request
        .lines()
        .next()
        .ok_or(SearchConsoleError::InvalidCallback)?;
    let mut parts = first_line.split_whitespace();
    let method = parts.next().ok_or(SearchConsoleError::InvalidCallback)?;
    let target = parts.next().ok_or(SearchConsoleError::InvalidCallback)?;
    let version = parts.next().ok_or(SearchConsoleError::InvalidCallback)?;
    if parts.next().is_some()
        || method != "GET"
        || !matches!(version, "HTTP/1.1" | "HTTP/1.0")
        || target.contains('#')
        || target.contains("://")
        || !target.starts_with('/')
    {
        return Err(SearchConsoleError::InvalidCallback);
    }

    let (path, query) = match target.split_once('?') {
        Some((path, query)) => (path, query),
        None => (target, ""),
    };
    if path != CALLBACK_PATH || query.is_empty() {
        return Err(SearchConsoleError::InvalidCallback);
    }

    let params = parse_query(query)?;
    let state = params
        .get("state")
        .map(String::as_str)
        .filter(|value| !value.is_empty())
        .ok_or(SearchConsoleError::InvalidCallback)?;
    if !constant_time_eq(state, expected_state) {
        return Err(SearchConsoleError::StateMismatch);
    }

    let code = params.get("code").map(String::as_str);
    let error = params.get("error").map(String::as_str);
    match (code, error) {
        (Some(_), Some(_)) | (None, None) => Err(SearchConsoleError::InvalidCallback),
        (None, Some("")) => Err(SearchConsoleError::InvalidCallback),
        (None, Some("access_denied")) => Err(SearchConsoleError::AuthorizationDenied),
        (None, Some(_)) => Err(SearchConsoleError::AuthorizationFailed),
        (Some(""), None) => Err(SearchConsoleError::InvalidCallback),
        (Some(code), None) => Ok(CallbackOutcome::Authorized {
            code: code.to_string(),
        }),
    }
}

fn parse_query(query: &str) -> Result<HashMap<String, String>, SearchConsoleError> {
    if query.len() > MAX_CALLBACK_REQUEST_BYTES {
        return Err(SearchConsoleError::InvalidCallback);
    }
    let mut params = HashMap::new();
    for pair in query.split('&') {
        if pair.is_empty() {
            continue;
        }
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        percent_decode(key)?;
        percent_decode(value)?;
    }
    for (key, value) in form_urlencoded::parse(query.as_bytes()) {
        let key = key.into_owned();
        if params.contains_key(&key) {
            return Err(SearchConsoleError::InvalidCallback);
        }
        params.insert(key, value.into_owned());
    }
    Ok(params)
}

fn constant_time_eq(left: &str, right: &str) -> bool {
    let left = left.as_bytes();
    let right = right.as_bytes();
    let mut diff = left.len() ^ right.len();
    let max_len = left.len().max(right.len());
    for index in 0..max_len {
        let left_byte = left.get(index).copied().unwrap_or(0);
        let right_byte = right.get(index).copied().unwrap_or(0);
        diff |= usize::from(left_byte ^ right_byte);
    }
    diff == 0
}

fn percent_decode(value: &str) -> Result<String, SearchConsoleError> {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut cursor = 0;
    while cursor < bytes.len() {
        match bytes[cursor] {
            b'+' => {
                decoded.push(b' ');
                cursor += 1;
            }
            b'%' => {
                if cursor + 2 >= bytes.len() {
                    return Err(SearchConsoleError::InvalidCallback);
                }
                let hex = &value[cursor + 1..cursor + 3];
                let byte =
                    u8::from_str_radix(hex, 16).map_err(|_| SearchConsoleError::InvalidCallback)?;
                decoded.push(byte);
                cursor += 3;
            }
            byte => {
                decoded.push(byte);
                cursor += 1;
            }
        }
    }
    String::from_utf8(decoded).map_err(|_| SearchConsoleError::InvalidCallback)
}

fn write_callback_response(stream: &mut TcpStream, success: bool) -> io::Result<()> {
    let body = if success {
        "<!doctype html><meta charset=\"utf-8\"><title>Korea Inside Admin</title><p>Korea Inside Admin 연결이 완료되었습니다.</p><p>이 창을 닫고 관리자 앱으로 돌아가세요.</p>"
    } else {
        "<!doctype html><meta charset=\"utf-8\"><title>Korea Inside Admin</title><p>인증을 완료하지 못했습니다.</p><p>관리자 앱으로 돌아가 다시 시도하세요.</p>"
    };
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nCache-Control: no-store\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );
    stream.write_all(response.as_bytes())?;
    stream.flush()
}

fn parse_initial_token_response(body: &[u8]) -> Result<TokenSet, SearchConsoleError> {
    let tokens = parse_token_response(body)?;
    if tokens.refresh_token.as_deref().is_none_or(str::is_empty) {
        return Err(SearchConsoleError::MissingRefreshToken);
    }
    if !scope_matches_requested(tokens.scope.as_deref()) {
        return Err(SearchConsoleError::ScopeNotGranted);
    }
    Ok(token_set(tokens))
}

fn parse_refresh_token_response(body: &[u8]) -> Result<TokenSet, SearchConsoleError> {
    let tokens = parse_token_response(body)?;
    if !scope_matches_requested(tokens.scope.as_deref()) {
        return Err(SearchConsoleError::ScopeNotGranted);
    }
    Ok(token_set(tokens))
}

fn parse_token_response(body: &[u8]) -> Result<GoogleTokenResponse, SearchConsoleError> {
    if body.len() > MAX_RESPONSE_BYTES {
        return Err(SearchConsoleError::TokenExchangeFailed);
    }
    let tokens: GoogleTokenResponse =
        serde_json::from_slice(body).map_err(|_| SearchConsoleError::TokenExchangeFailed)?;
    if tokens.access_token.as_deref().is_none_or(str::is_empty) {
        return Err(SearchConsoleError::TokenExchangeFailed);
    }
    Ok(tokens)
}

fn token_set(tokens: GoogleTokenResponse) -> TokenSet {
    TokenSet {
        access_token: tokens.access_token.unwrap_or_default(),
        refresh_token: tokens.refresh_token,
        expires_in: Duration::from_secs(tokens.expires_in.unwrap_or(3600).max(1)),
    }
}

fn token_error_is_invalid_grant(body: &[u8]) -> bool {
    serde_json::from_slice::<GoogleErrorResponse>(body)
        .ok()
        .and_then(|error| error.error)
        .is_some_and(|error| error == "invalid_grant")
}

fn parse_sites_list_response(body: &[u8]) -> Result<(), SearchConsoleError> {
    if body.len() > MAX_RESPONSE_BYTES {
        return Err(SearchConsoleError::ApiRequestFailed);
    }
    let response: SitesListResponse =
        serde_json::from_slice(body).map_err(|_| SearchConsoleError::ApiRequestFailed)?;
    response
        .site_entries
        .ok_or(SearchConsoleError::ApiRequestFailed)
        .map(|_| ())
}

fn scope_matches_requested(scope: Option<&str>) -> bool {
    let Some(scope) = scope else {
        return true;
    };
    let mut values = HashSet::new();
    let mut count = 0;
    for value in scope.split_whitespace() {
        count += 1;
        if value != SEARCH_CONSOLE_SCOPE || !values.insert(value) {
            return false;
        }
    }
    count == 1 && values.contains(SEARCH_CONSOLE_SCOPE)
}

fn secure_http_client() -> Result<reqwest::Client, SearchConsoleError> {
    reqwest::Client::builder()
        .https_only(true)
        .redirect(Policy::none())
        .connect_timeout(CONNECT_TIMEOUT)
        .timeout(REQUEST_TIMEOUT)
        .no_gzip()
        .no_brotli()
        .retry(reqwest::retry::never())
        .build()
        .map_err(|_| SearchConsoleError::Internal)
}

async fn read_limited_body(mut response: reqwest::Response) -> Result<Vec<u8>, SearchConsoleError> {
    let mut body = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| SearchConsoleError::ApiRequestFailed)?
    {
        if body.len().saturating_add(chunk.len()) > MAX_RESPONSE_BYTES {
            return Err(SearchConsoleError::ApiRequestFailed);
        }
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}

fn map_request_error(error: reqwest::Error) -> SearchConsoleError {
    if error.is_timeout() {
        SearchConsoleError::NetworkTimeout
    } else {
        SearchConsoleError::ApiRequestFailed
    }
}

fn form_request_body(params: &[(&str, &str)]) -> String {
    params
        .iter()
        .map(|(key, value)| {
            format!(
                "{}={}",
                form_encode_component(key),
                form_encode_component(value)
            )
        })
        .collect::<Vec<_>>()
        .join("&")
}

fn form_encode_component(value: &str) -> String {
    let mut encoded = String::new();
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
            encoded.push(char::from(byte));
        } else {
            encoded.push_str(&format!("%{byte:02X}"));
        }
    }
    encoded
}

fn validate_client_id(client_id: &str) -> Result<String, SearchConsoleError> {
    let trimmed = client_id.trim();
    if trimmed.is_empty() || trimmed.len() > MAX_CLIENT_ID_LENGTH {
        return Err(SearchConsoleError::InvalidClientId);
    }
    if trimmed
        .chars()
        .any(|value| value.is_whitespace() || value.is_control())
    {
        return Err(SearchConsoleError::InvalidClientId);
    }
    if trimmed.contains("://")
        || trimmed.contains('/')
        || trimmed.contains('\\')
        || trimmed.contains('?')
        || trimmed.contains('#')
    {
        return Err(SearchConsoleError::InvalidClientId);
    }
    if !trimmed.ends_with(CLIENT_ID_SUFFIX) {
        return Err(SearchConsoleError::InvalidClientId);
    }
    Ok(trimmed.to_string())
}

fn save_search_console_client_id_with<R, S, D, C, T>(
    client_id: String,
    mut read_existing_client_id: R,
    mut save_client_id: S,
    mut delete_refresh_token: D,
    mut clear_runtime: C,
    mut status: T,
) -> Result<SearchConsoleClientStatus, SearchConsoleError>
where
    R: FnMut() -> Result<Option<String>, SearchConsoleError>,
    S: FnMut(&str) -> Result<(), SearchConsoleError>,
    D: FnMut() -> Result<(), SearchConsoleError>,
    C: FnMut() -> Result<(), SearchConsoleError>,
    T: FnMut() -> Result<SearchConsoleClientStatus, SearchConsoleError>,
{
    if read_existing_client_id()?.as_deref() == Some(client_id.as_str()) {
        return status();
    }
    delete_refresh_token()?;
    clear_runtime()?;
    save_client_id(&client_id)?;
    status()
}

fn delete_search_console_client_id_with<D, C, R, T>(
    mut delete_refresh_token: D,
    mut clear_runtime: C,
    mut delete_client_id: R,
    mut status: T,
) -> Result<SearchConsoleClientStatus, SearchConsoleError>
where
    D: FnMut() -> Result<(), SearchConsoleError>,
    C: FnMut() -> Result<(), SearchConsoleError>,
    R: FnMut() -> Result<(), SearchConsoleError>,
    T: FnMut() -> Result<SearchConsoleClientStatus, SearchConsoleError>,
{
    delete_refresh_token()?;
    clear_runtime()?;
    delete_client_id()?;
    status()
}

fn client_status() -> Result<SearchConsoleClientStatus, SearchConsoleError> {
    let configured = read_stored_client_id()?.is_some();
    let authorization_stored = credential_exists(REFRESH_TOKEN_ACCOUNT)?;
    client_status_from_parts(configured, authorization_stored)
}

fn client_status_from_parts(
    configured: bool,
    authorization_stored: bool,
) -> Result<SearchConsoleClientStatus, SearchConsoleError> {
    let state = lock_runtime_state()?;
    Ok(SearchConsoleClientStatus {
        configured,
        authorization_stored,
        connected: state.connected
            && configured
            && authorization_stored
            && !state.reauthentication_required,
        authentication_in_progress: state.authentication_in_progress,
        reauthentication_required: state.reauthentication_required,
        last_checked_at: state.last_checked_at.clone(),
    })
}

fn cached_access_token() -> Result<Option<String>, SearchConsoleError> {
    let state = lock_runtime_state()?;
    Ok(state.access_token.as_ref().and_then(|cache| {
        let refresh_at = cache
            .expires_at
            .checked_sub(ACCESS_TOKEN_REFRESH_MARGIN)
            .unwrap_or(cache.expires_at);
        (Instant::now() < refresh_at).then(|| cache.token.clone())
    }))
}

fn store_access_token(token: String, expires_in: Duration) -> Result<(), SearchConsoleError> {
    let expires_at = Instant::now() + expires_in;
    let mut state = lock_runtime_state()?;
    state.access_token = Some(AccessTokenCache { token, expires_at });
    state.reauthentication_required = false;
    Ok(())
}

fn set_last_checked_now(reauthentication_required: bool) -> Result<(), SearchConsoleError> {
    let mut state = lock_runtime_state()?;
    state.last_checked_at = Some(current_utc_timestamp()?);
    state.reauthentication_required = reauthentication_required;
    state.connected = !reauthentication_required;
    Ok(())
}

fn clear_runtime_connection_state(
    reauthentication_required: bool,
) -> Result<(), SearchConsoleError> {
    let mut state = lock_runtime_state()?;
    state.access_token = None;
    state.connected = false;
    state.reauthentication_required = reauthentication_required;
    state.last_checked_at = None;
    Ok(())
}

fn mark_connection_failed(reauthentication_required: bool) -> Result<(), SearchConsoleError> {
    let mut state = lock_runtime_state()?;
    state.access_token = None;
    state.connected = false;
    state.reauthentication_required = reauthentication_required;
    Ok(())
}

fn mark_reauthentication_required() -> Result<(), SearchConsoleError> {
    mark_connection_failed(true)
}

fn runtime_state() -> &'static Mutex<SearchConsoleRuntimeState> {
    static STATE: OnceLock<Mutex<SearchConsoleRuntimeState>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(SearchConsoleRuntimeState::default()))
}

fn lock_runtime_state() -> Result<MutexGuard<'static, SearchConsoleRuntimeState>, SearchConsoleError>
{
    runtime_state()
        .lock()
        .map_err(|_| SearchConsoleError::Internal)
}

fn credential_entry(account: &str) -> Result<Entry, KeyringError> {
    static STORE_READY: OnceLock<bool> = OnceLock::new();
    let ready = STORE_READY.get_or_init(|| match Store::new() {
        Ok(store) => {
            keyring_core::set_default_store(store);
            true
        }
        Err(_) => false,
    });
    if !ready {
        return Err(KeyringError::NoDefaultStore);
    }
    let persistence = CredPersist::Local.to_string();
    let modifiers = HashMap::from([("persistence", persistence.as_str())]);
    Entry::new_with_modifiers(CREDENTIAL_SERVICE, account, &modifiers)
}

fn read_credential(account: &str) -> Result<String, KeyringError> {
    credential_entry(account)?.get_password()
}

fn read_stored_client_id() -> Result<Option<String>, SearchConsoleError> {
    match read_credential(CLIENT_ID_ACCOUNT) {
        Ok(client_id) => Ok(validate_client_id(&client_id).ok()),
        Err(KeyringError::NoEntry) => Ok(None),
        Err(_) => Err(SearchConsoleError::CredentialReadFailed),
    }
}

fn save_credential(account: &str, value: &str) -> Result<(), SearchConsoleError> {
    let entry = credential_entry(account).map_err(|_| SearchConsoleError::CredentialStoreFailed)?;
    entry
        .set_password(value)
        .map_err(|_| SearchConsoleError::CredentialStoreFailed)?;
    match credential_persistence_is_local(&entry) {
        Ok(true) => Ok(()),
        Ok(false) | Err(_) => {
            let _ = entry.delete_credential();
            Err(SearchConsoleError::CredentialStoreFailed)
        }
    }
}

fn delete_credential_if_present(account: &str) -> Result<(), SearchConsoleError> {
    let entry =
        credential_entry(account).map_err(|_| SearchConsoleError::CredentialDeleteFailed)?;
    match entry.delete_credential() {
        Ok(()) | Err(KeyringError::NoEntry) => Ok(()),
        Err(_) => Err(SearchConsoleError::CredentialDeleteFailed),
    }
}

fn credential_exists(account: &str) -> Result<bool, SearchConsoleError> {
    match read_credential(account) {
        Ok(_) => Ok(true),
        Err(KeyringError::NoEntry) => Ok(false),
        Err(_) => Err(SearchConsoleError::CredentialReadFailed),
    }
}

fn credential_persistence_is_local(entry: &Entry) -> Result<bool, KeyringError> {
    let attributes = entry.get_attributes()?;
    Ok(attributes
        .get("persistence")
        .is_some_and(|value| value.eq_ignore_ascii_case(&CredPersist::Local.to_string())))
}

fn current_utc_timestamp() -> Result<String, SearchConsoleError> {
    OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .map_err(|_| SearchConsoleError::Internal)
}

fn is_loopback_peer(peer: SocketAddr) -> bool {
    matches!(peer, SocketAddr::V4(address) if address.ip().is_loopback())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        cell::RefCell,
        rc::Rc,
        sync::{Mutex as TestMutex, MutexGuard as TestMutexGuard, OnceLock as TestOnceLock},
    };

    const VALID_CLIENT_ID: &str = "1234567890-testdesktop.apps.googleusercontent.com";

    #[test]
    fn accepts_valid_client_id() {
        assert_eq!(
            validate_client_id(VALID_CLIENT_ID).unwrap(),
            VALID_CLIENT_ID
        );
    }

    #[test]
    fn trims_client_id_before_validation() {
        assert_eq!(
            validate_client_id(&format!("  {VALID_CLIENT_ID}\r\n")).unwrap(),
            VALID_CLIENT_ID
        );
    }

    #[test]
    fn rejects_empty_client_id() {
        assert_eq!(
            validate_client_id("   ").unwrap_err(),
            SearchConsoleError::InvalidClientId
        );
    }

    #[test]
    fn rejects_wrong_client_id_suffix() {
        assert_eq!(
            validate_client_id("1234567890-testdesktop.example.com").unwrap_err(),
            SearchConsoleError::InvalidClientId
        );
    }

    #[test]
    fn rejects_url_shaped_client_id() {
        assert_eq!(
            validate_client_id(&format!("https://{VALID_CLIENT_ID}")).unwrap_err(),
            SearchConsoleError::InvalidClientId
        );
    }

    #[test]
    fn rejects_whitespace_and_control_characters() {
        for value in [
            format!("abc def{CLIENT_ID_SUFFIX}"),
            format!("abc\tdef{CLIENT_ID_SUFFIX}"),
            format!("abc\u{7}def{CLIENT_ID_SUFFIX}"),
        ] {
            assert_eq!(
                validate_client_id(&value).unwrap_err(),
                SearchConsoleError::InvalidClientId
            );
        }
    }

    #[test]
    fn rejects_overlong_client_id() {
        let value = format!("{}{}", "a".repeat(MAX_CLIENT_ID_LENGTH), CLIENT_ID_SUFFIX);
        assert_eq!(
            validate_client_id(&value).unwrap_err(),
            SearchConsoleError::InvalidClientId
        );
    }

    #[test]
    fn rejects_slash_query_and_fragment() {
        for value in [
            format!("abc/def{CLIENT_ID_SUFFIX}"),
            format!("abcdef{CLIENT_ID_SUFFIX}?x=1"),
            format!("abcdef{CLIENT_ID_SUFFIX}#frag"),
        ] {
            assert_eq!(
                validate_client_id(&value).unwrap_err(),
                SearchConsoleError::InvalidClientId
            );
        }
    }

    #[test]
    fn authorization_url_uses_fixed_endpoint() {
        let request = build_authorization_request(VALID_CLIENT_ID, 49152).unwrap();
        assert!(request
            .authorization_url
            .starts_with(AUTHORIZATION_ENDPOINT));
        assert!(request.authorization_url.contains("response_type=code"));
    }

    #[test]
    fn authorization_url_uses_readonly_scope_only() {
        let request = build_authorization_request(VALID_CLIENT_ID, 49152).unwrap();
        assert!(request
            .authorization_url
            .contains("scope=https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fwebmasters.readonly"));
        assert!(!request.authorization_url.contains("auth%2Fwebmasters&"));
    }

    #[test]
    fn pkce_s256_is_generated() {
        let request = build_authorization_request(VALID_CLIENT_ID, 49152).unwrap();
        assert!(request.authorization_url.contains("code_challenge="));
        assert!(request
            .authorization_url
            .contains("code_challenge_method=S256"));
        assert!(request.pkce_verifier.len() >= 43);
    }

    #[test]
    fn state_differs_for_each_request() {
        let first = build_authorization_request(VALID_CLIENT_ID, 49152).unwrap();
        let second = build_authorization_request(VALID_CLIENT_ID, 49153).unwrap();
        assert_ne!(first.state, second.state);
        assert_ne!(first.pkce_verifier, second.pkce_verifier);
    }

    #[test]
    fn redirect_uri_uses_ipv4_loopback() {
        let request = build_authorization_request(VALID_CLIENT_ID, 49152).unwrap();
        assert_eq!(
            request.redirect_uri,
            format!("http://127.0.0.1:49152{CALLBACK_PATH}")
        );
        assert!(request
            .authorization_url
            .contains("redirect_uri=http%3A%2F%2F127.0.0.1%3A49152"));
    }

    #[test]
    fn parses_successful_callback_code() {
        let request = format!("GET {CALLBACK_PATH}?code=abc123&state=state123 HTTP/1.1\r\n\r\n");
        assert_eq!(
            parse_callback_request(&request, "state123").unwrap(),
            CallbackOutcome::Authorized {
                code: "abc123".to_string()
            }
        );
    }

    #[test]
    fn parses_access_denied_callback() {
        let request =
            format!("GET {CALLBACK_PATH}?error=access_denied&state=state123 HTTP/1.1\r\n\r\n");
        assert_eq!(
            parse_callback_request(&request, "state123").unwrap_err(),
            SearchConsoleError::AuthorizationDenied
        );
    }

    #[test]
    fn rejects_state_mismatch_before_code_use() {
        let request = format!("GET {CALLBACK_PATH}?code=abc123&state=wrong HTTP/1.1\r\n\r\n");
        assert_eq!(
            parse_callback_request(&request, "state123").unwrap_err(),
            SearchConsoleError::StateMismatch
        );
    }

    #[test]
    fn rejects_wrong_callback_path() {
        let request = "GET /wrong/path?code=abc123&state=state123 HTTP/1.1\r\n\r\n";
        assert_eq!(
            parse_callback_request(request, "state123").unwrap_err(),
            SearchConsoleError::InvalidCallback
        );
    }

    #[test]
    fn rejects_oversized_callback_input() {
        let request = "G".repeat(MAX_CALLBACK_REQUEST_BYTES + 1);
        assert_eq!(
            parse_callback_request(&request, "state123").unwrap_err(),
            SearchConsoleError::InvalidCallback
        );
    }

    #[test]
    fn rejects_duplicate_state_code_and_error_parameters() {
        for request in [
            format!("GET {CALLBACK_PATH}?code=abc123&state=state123&state=state123 HTTP/1.1\r\n\r\n"),
            format!("GET {CALLBACK_PATH}?code=abc123&code=def456&state=state123 HTTP/1.1\r\n\r\n"),
            format!("GET {CALLBACK_PATH}?error=access_denied&error=server_error&state=state123 HTTP/1.1\r\n\r\n"),
        ] {
            assert_eq!(
                parse_callback_request(&request, "state123").unwrap_err(),
                SearchConsoleError::InvalidCallback
            );
        }
    }

    #[test]
    fn rejects_encoded_duplicate_query_key() {
        let request = format!(
            "GET {CALLBACK_PATH}?code=abc123&state=state123&st%61te=state123 HTTP/1.1\r\n\r\n"
        );
        assert_eq!(
            parse_callback_request(&request, "state123").unwrap_err(),
            SearchConsoleError::InvalidCallback
        );
    }

    #[test]
    fn rejects_malformed_percent_encoding() {
        let request = format!("GET {CALLBACK_PATH}?code=%ZZ&state=state123 HTTP/1.1\r\n\r\n");
        assert_eq!(
            parse_callback_request(&request, "state123").unwrap_err(),
            SearchConsoleError::InvalidCallback
        );
    }

    #[test]
    fn rejects_code_and_error_exclusivity_violations() {
        for request in [
            format!("GET {CALLBACK_PATH}?code=abc123&error=access_denied&state=state123 HTTP/1.1\r\n\r\n"),
            format!("GET {CALLBACK_PATH}?state=state123 HTTP/1.1\r\n\r\n"),
            format!("GET {CALLBACK_PATH}?code=&state=state123 HTTP/1.1\r\n\r\n"),
            format!("GET {CALLBACK_PATH}?error=&state=state123 HTTP/1.1\r\n\r\n"),
        ] {
            assert_eq!(
                parse_callback_request(&request, "state123").unwrap_err(),
                SearchConsoleError::InvalidCallback
            );
        }
    }

    #[test]
    fn unknown_oauth_error_maps_to_authorization_failed() {
        let request =
            format!("GET {CALLBACK_PATH}?error=server_error&state=state123 HTTP/1.1\r\n\r\n");
        assert_eq!(
            parse_callback_request(&request, "state123").unwrap_err(),
            SearchConsoleError::AuthorizationFailed
        );
    }

    #[test]
    fn rejects_error_callback_state_mismatch_before_error_handling() {
        let request =
            format!("GET {CALLBACK_PATH}?error=access_denied&state=wrong HTTP/1.1\r\n\r\n");
        assert_eq!(
            parse_callback_request(&request, "state123").unwrap_err(),
            SearchConsoleError::StateMismatch
        );
    }

    #[test]
    fn rejects_non_get_method_and_bad_http_version() {
        for request in [
            format!("POST {CALLBACK_PATH}?code=abc123&state=state123 HTTP/1.1\r\n\r\n"),
            format!("GET {CALLBACK_PATH}?code=abc123&state=state123 HTTP/2.0\r\n\r\n"),
            format!("GET {CALLBACK_PATH}?code=abc123&state=state123#frag HTTP/1.1\r\n\r\n"),
            format!(
                "GET http://127.0.0.1{CALLBACK_PATH}?code=abc123&state=state123 HTTP/1.1\r\n\r\n"
            ),
        ] {
            assert_eq!(
                parse_callback_request(&request, "state123").unwrap_err(),
                SearchConsoleError::InvalidCallback
            );
        }
    }

    #[test]
    fn callback_errors_do_not_include_query_code_or_state_values() {
        let error_json = serde_json::to_string(&SearchConsoleCommandError::from(
            SearchConsoleError::StateMismatch,
        ))
        .unwrap();
        assert!(!error_json.contains("abc123"));
        assert!(!error_json.contains("state123"));
        assert!(!error_json.contains("access_denied"));
    }

    #[test]
    fn rejects_initial_token_response_without_refresh_token() {
        let body = br#"{"access_token":"access","expires_in":3600,"scope":"https://www.googleapis.com/auth/webmasters.readonly"}"#;
        assert_eq!(
            parse_initial_token_response(body).unwrap_err(),
            SearchConsoleError::MissingRefreshToken
        );
    }

    #[test]
    fn accepts_initial_token_response_without_scope() {
        let body = br#"{"access_token":"access","refresh_token":"refresh","expires_in":3600}"#;
        assert!(parse_initial_token_response(body).is_ok());
    }

    #[test]
    fn accepts_exact_readonly_scope() {
        let body = br#"{"access_token":"access","refresh_token":"refresh","expires_in":3600,"scope":"https://www.googleapis.com/auth/webmasters.readonly"}"#;
        assert!(parse_initial_token_response(body).is_ok());
    }

    #[test]
    fn rejects_token_response_without_readonly_scope() {
        let body = br#"{"access_token":"access","refresh_token":"refresh","expires_in":3600,"scope":"https://www.googleapis.com/auth/webmasters"}"#;
        assert_eq!(
            parse_initial_token_response(body).unwrap_err(),
            SearchConsoleError::ScopeNotGranted
        );
    }

    #[test]
    fn rejects_extra_empty_and_duplicate_scopes() {
        for body in [
            br#"{"access_token":"access","refresh_token":"refresh","expires_in":3600,"scope":"https://www.googleapis.com/auth/webmasters.readonly https://www.googleapis.com/auth/calendar.readonly"}"#.as_slice(),
            br#"{"access_token":"access","refresh_token":"refresh","expires_in":3600,"scope":""}"#.as_slice(),
            br#"{"access_token":"access","refresh_token":"refresh","expires_in":3600,"scope":"https://www.googleapis.com/auth/webmasters.readonly https://www.googleapis.com/auth/webmasters.readonly"}"#.as_slice(),
        ] {
            assert_eq!(
                parse_initial_token_response(body).unwrap_err(),
                SearchConsoleError::ScopeNotGranted
            );
        }
    }

    #[test]
    fn parses_sites_list_json() {
        let body = br#"{"siteEntry":[{"siteUrl":"sc-domain:getkoreainside.com","permissionLevel":"siteOwner"}]}"#;
        assert!(parse_sites_list_response(body).is_ok());
    }

    #[test]
    fn rejects_invalid_sites_list_json() {
        for body in [
            br#"{"siteEntry":[{"siteUrl":"sc-domain:getkoreainside.com","permissionLevel":1}]}"#
                .as_slice(),
            br#"{"notSiteEntry":[]}"#.as_slice(),
            b"not-json".as_slice(),
        ] {
            assert_eq!(
                parse_sites_list_response(body).unwrap_err(),
                SearchConsoleError::ApiRequestFailed
            );
        }
    }

    #[test]
    fn token_and_error_dtos_do_not_expose_secret_words() {
        let status = SearchConsoleClientStatus {
            configured: true,
            authorization_stored: true,
            connected: true,
            authentication_in_progress: false,
            reauthentication_required: false,
            last_checked_at: Some("2026-07-13T00:00:00Z".to_string()),
        };
        let status_json = serde_json::to_string(&status).unwrap();
        assert!(!status_json.contains("clientId"));
        assert!(!status_json.contains("refresh"));
        assert!(!status_json.contains("access"));
        assert!(!status_json.contains("code"));
        assert!(!status_json.contains("state"));
        assert!(!status_json.contains("verifier"));

        let error_json = serde_json::to_string(&SearchConsoleCommandError::from(
            SearchConsoleError::Internal,
        ))
        .unwrap();
        assert!(!error_json.contains(VALID_CLIENT_ID));
        assert!(!error_json.contains("Bearer"));
    }

    #[test]
    fn in_flight_duplicate_operations_are_blocked() {
        let mut state = SearchConsoleRuntimeState::default();
        assert!(begin_operation_for_test(&mut state, OperationKind::Authentication).is_ok());
        assert_eq!(
            begin_operation_for_test(&mut state, OperationKind::Authentication).unwrap_err(),
            SearchConsoleError::AlreadyInProgress
        );
        assert_eq!(
            begin_operation_for_test(&mut state, OperationKind::Disconnect).unwrap_err(),
            SearchConsoleError::AlreadyInProgress
        );
    }

    #[test]
    fn mutating_operations_are_exclusive_with_client_configuration() {
        let mut state = SearchConsoleRuntimeState::default();
        assert!(begin_operation_for_test(&mut state, OperationKind::Authentication).is_ok());
        assert_eq!(
            begin_operation_for_test(&mut state, OperationKind::ClientConfiguration).unwrap_err(),
            SearchConsoleError::AlreadyInProgress
        );

        let mut state = SearchConsoleRuntimeState::default();
        assert!(begin_operation_for_test(&mut state, OperationKind::ClientConfiguration).is_ok());
        assert_eq!(
            begin_operation_for_test(&mut state, OperationKind::Authentication).unwrap_err(),
            SearchConsoleError::AlreadyInProgress
        );

        let mut state = SearchConsoleRuntimeState::default();
        assert!(begin_operation_for_test(&mut state, OperationKind::Disconnect).is_ok());
        assert_eq!(
            begin_operation_for_test(&mut state, OperationKind::ClientConfiguration).unwrap_err(),
            SearchConsoleError::AlreadyInProgress
        );
    }

    #[test]
    fn operation_guard_releases_after_error_and_drop() {
        let _lock = runtime_test_lock();
        reset_runtime_state_for_test();
        let result: Result<(), SearchConsoleError> = (|| {
            let _guard = OperationGuard::begin(OperationKind::ClientConfiguration)?;
            Err(SearchConsoleError::Internal)
        })();
        assert_eq!(result.err(), Some(SearchConsoleError::Internal));
        let guard = OperationGuard::begin(OperationKind::Authentication).unwrap();
        drop(guard);
        assert!(OperationGuard::begin(OperationKind::Disconnect).is_ok());
        reset_runtime_state_for_test();
    }

    #[test]
    fn timeout_maps_to_callback_timeout() {
        assert_eq!(
            SearchConsoleError::CallbackTimeout.code(),
            "callback_timeout"
        );
    }

    #[test]
    fn invalid_grant_maps_to_reauthentication_required() {
        assert!(token_error_is_invalid_grant(
            br#"{"error":"invalid_grant"}"#
        ));
        assert_eq!(
            SearchConsoleError::ReauthenticationRequired.code(),
            "reauthentication_required"
        );
    }

    #[test]
    fn search_console_and_vercel_credential_services_are_separate() {
        assert_ne!(CREDENTIAL_SERVICE, VERCEL_CREDENTIAL_SERVICE_FOR_TEST);
        assert_ne!(CLIENT_ID_ACCOUNT, REFRESH_TOKEN_ACCOUNT);
    }

    #[test]
    fn same_client_id_save_does_not_delete_refresh_token() {
        let events = Rc::new(RefCell::new(Vec::new()));
        let delete_events = Rc::clone(&events);
        let save_events = Rc::clone(&events);
        let clear_events = Rc::clone(&events);
        save_search_console_client_id_with(
            VALID_CLIENT_ID.to_string(),
            || Ok(Some(VALID_CLIENT_ID.to_string())),
            |_| {
                save_events.borrow_mut().push("save_client_id");
                Ok(())
            },
            || {
                delete_events.borrow_mut().push("delete_refresh");
                Ok(())
            },
            || {
                clear_events.borrow_mut().push("clear_runtime");
                Ok(())
            },
            dummy_status,
        )
        .unwrap();
        assert!(events.borrow().is_empty());
    }

    #[test]
    fn different_client_id_deletes_refresh_before_saving_client_id() {
        let events = Rc::new(RefCell::new(Vec::new()));
        let delete_events = Rc::clone(&events);
        let save_events = Rc::clone(&events);
        let clear_events = Rc::clone(&events);
        save_search_console_client_id_with(
            VALID_CLIENT_ID.to_string(),
            || Ok(Some("other-client.apps.googleusercontent.com".to_string())),
            |_| {
                save_events.borrow_mut().push("save_client_id");
                Ok(())
            },
            || {
                delete_events.borrow_mut().push("delete_refresh");
                Ok(())
            },
            || {
                clear_events.borrow_mut().push("clear_runtime");
                Ok(())
            },
            dummy_status,
        )
        .unwrap();
        assert_eq!(
            events.borrow().as_slice(),
            ["delete_refresh", "clear_runtime", "save_client_id"]
        );
    }

    #[test]
    fn failed_refresh_delete_prevents_new_client_id_save() {
        let events = Rc::new(RefCell::new(Vec::new()));
        let save_events = Rc::clone(&events);
        let result = save_search_console_client_id_with(
            VALID_CLIENT_ID.to_string(),
            || Ok(Some("other-client.apps.googleusercontent.com".to_string())),
            |_| {
                save_events.borrow_mut().push("save_client_id");
                Ok(())
            },
            || Err(SearchConsoleError::CredentialDeleteFailed),
            || Ok(()),
            dummy_status,
        );
        assert_eq!(
            result.err(),
            Some(SearchConsoleError::CredentialDeleteFailed)
        );
        assert!(events.borrow().is_empty());
    }

    #[test]
    fn client_id_delete_removes_refresh_before_client_id() {
        let events = Rc::new(RefCell::new(Vec::new()));
        let delete_refresh_events = Rc::clone(&events);
        let clear_events = Rc::clone(&events);
        let delete_client_events = Rc::clone(&events);
        delete_search_console_client_id_with(
            || {
                delete_refresh_events.borrow_mut().push("delete_refresh");
                Ok(())
            },
            || {
                clear_events.borrow_mut().push("clear_runtime");
                Ok(())
            },
            || {
                delete_client_events.borrow_mut().push("delete_client_id");
                Ok(())
            },
            dummy_status,
        )
        .unwrap();
        assert_eq!(
            events.borrow().as_slice(),
            ["delete_refresh", "clear_runtime", "delete_client_id"]
        );
    }

    #[test]
    fn failed_refresh_delete_prevents_client_id_delete() {
        let events = Rc::new(RefCell::new(Vec::new()));
        let delete_client_events = Rc::clone(&events);
        let result = delete_search_console_client_id_with(
            || Err(SearchConsoleError::CredentialDeleteFailed),
            || Ok(()),
            || {
                delete_client_events.borrow_mut().push("delete_client_id");
                Ok(())
            },
            dummy_status,
        );
        assert_eq!(
            result.err(),
            Some(SearchConsoleError::CredentialDeleteFailed)
        );
        assert!(events.borrow().is_empty());
    }

    #[test]
    fn cached_access_token_is_reused_until_refresh_margin() {
        let _lock = runtime_test_lock();
        reset_runtime_state_for_test();
        {
            let mut state = runtime_state().lock().unwrap();
            state.access_token = Some(AccessTokenCache {
                token: "cached-token".to_string(),
                expires_at: Instant::now() + ACCESS_TOKEN_REFRESH_MARGIN + Duration::from_secs(30),
            });
        }
        assert_eq!(
            cached_access_token().unwrap(),
            Some("cached-token".to_string())
        );
        reset_runtime_state_for_test();
    }

    #[test]
    fn cache_expiring_inside_margin_requires_refresh() {
        let _lock = runtime_test_lock();
        reset_runtime_state_for_test();
        {
            let mut state = runtime_state().lock().unwrap();
            state.access_token = Some(AccessTokenCache {
                token: "cached-token".to_string(),
                expires_at: Instant::now() + Duration::from_secs(30),
            });
        }
        assert_eq!(cached_access_token().unwrap(), None);
        reset_runtime_state_for_test();
    }

    #[test]
    fn invalid_grant_marks_reauthentication_required() {
        let _lock = runtime_test_lock();
        reset_runtime_state_for_test();
        {
            let mut state = runtime_state().lock().unwrap();
            state.connected = true;
            state.access_token = Some(AccessTokenCache {
                token: "cached-token".to_string(),
                expires_at: Instant::now() + Duration::from_secs(3600),
            });
        }
        mark_reauthentication_required().unwrap();
        let state = runtime_state().lock().unwrap();
        assert!(!state.connected);
        assert!(state.reauthentication_required);
        assert!(state.access_token.is_none());
    }

    #[test]
    fn general_network_failure_does_not_mark_reauthentication_required() {
        let _lock = runtime_test_lock();
        reset_runtime_state_for_test();
        {
            let mut state = runtime_state().lock().unwrap();
            state.connected = true;
            state.reauthentication_required = false;
        }
        mark_connection_failed(false).unwrap();
        let state = runtime_state().lock().unwrap();
        assert!(!state.connected);
        assert!(!state.reauthentication_required);
    }

    #[test]
    fn successful_sites_list_updates_connected_and_last_checked_at() {
        let _lock = runtime_test_lock();
        reset_runtime_state_for_test();
        set_last_checked_now(false).unwrap();
        let status = client_status_from_parts(true, true).unwrap();
        assert!(status.connected);
        assert!(status.last_checked_at.is_some());
        reset_runtime_state_for_test();
    }

    #[test]
    fn stored_authorization_alone_is_not_connected_after_app_start() {
        let _lock = runtime_test_lock();
        reset_runtime_state_for_test();
        let status = client_status_from_parts(true, true).unwrap();
        assert!(status.configured);
        assert!(status.authorization_stored);
        assert!(!status.connected);
    }

    #[test]
    fn fixed_endpoints_are_google_and_readonly() {
        assert_eq!(
            AUTHORIZATION_ENDPOINT,
            "https://accounts.google.com/o/oauth2/v2/auth"
        );
        assert_eq!(TOKEN_ENDPOINT, "https://oauth2.googleapis.com/token");
        assert_eq!(REVOKE_ENDPOINT, "https://oauth2.googleapis.com/revoke");
        assert_eq!(
            SITES_LIST_ENDPOINT,
            "https://www.googleapis.com/webmasters/v3/sites"
        );
        assert_eq!(
            SEARCH_CONSOLE_SCOPE,
            "https://www.googleapis.com/auth/webmasters.readonly"
        );
    }

    fn dummy_status() -> Result<SearchConsoleClientStatus, SearchConsoleError> {
        Ok(SearchConsoleClientStatus {
            configured: true,
            authorization_stored: true,
            connected: false,
            authentication_in_progress: false,
            reauthentication_required: false,
            last_checked_at: None,
        })
    }

    fn runtime_test_lock() -> TestMutexGuard<'static, ()> {
        static LOCK: TestOnceLock<TestMutex<()>> = TestOnceLock::new();
        LOCK.get_or_init(|| TestMutex::new(())).lock().unwrap()
    }

    fn reset_runtime_state_for_test() {
        *runtime_state().lock().unwrap() = SearchConsoleRuntimeState::default();
    }

    fn begin_operation_for_test(
        state: &mut SearchConsoleRuntimeState,
        kind: OperationKind,
    ) -> Result<(), SearchConsoleError> {
        if state.authentication_in_progress
            || state.refresh_in_progress
            || state.disconnect_in_progress
            || state.client_configuration_in_progress
        {
            return Err(SearchConsoleError::AlreadyInProgress);
        }
        match kind {
            OperationKind::Authentication => state.authentication_in_progress = true,
            OperationKind::Refresh => state.refresh_in_progress = true,
            OperationKind::Disconnect => state.disconnect_in_progress = true,
            OperationKind::ClientConfiguration => state.client_configuration_in_progress = true,
        }
        Ok(())
    }
}
