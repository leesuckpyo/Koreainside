use keyring_core::{Entry, Error as KeyringError};
use oauth2::{
    basic::{BasicClient, BasicErrorResponse, BasicErrorResponseType},
    AuthType, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, HttpRequest,
    HttpResponse, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RequestTokenError, Scope,
    TokenResponse, TokenUrl,
};
use reqwest::{header::CONTENT_TYPE, redirect::Policy, StatusCode};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt,
    fs::File,
    io::{self, Read, Write},
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream},
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, MutexGuard, OnceLock,
    },
    thread,
    time::{Duration, Instant},
};
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use url::form_urlencoded;
use windows_native_keyring_store::{CredPersist, Store};

const CREDENTIAL_SERVICE: &str = "com.getkoreainside.admin.search-console";
const CLIENT_ID_ACCOUNT: &str = "oauth-client-id";
const CLIENT_SECRET_ACCOUNT: &str = "oauth-client-secret";
const REFRESH_TOKEN_ACCOUNT: &str = "refresh-token";
const OAUTH_CREDENTIAL_UPDATE_LOCK: &str =
    "__KOREA_INSIDE_SEARCH_CONSOLE_OAUTH_UPDATE_INCOMPLETE__";
#[cfg(test)]
const VERCEL_CREDENTIAL_SERVICE_FOR_TEST: &str = "com.getkoreainside.admin.vercel";

const AUTHORIZATION_ENDPOINT: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const TOKEN_ENDPOINT: &str = "https://oauth2.googleapis.com/token";
const REVOKE_ENDPOINT: &str = "https://oauth2.googleapis.com/revoke";
const SITES_LIST_ENDPOINT: &str = "https://www.googleapis.com/webmasters/v3/sites";
const SEARCH_CONSOLE_SCOPE: &str = "https://www.googleapis.com/auth/webmasters.readonly";
const CALLBACK_PATH: &str = "/";
const GOOGLE_ISSUER: &str = "https://accounts.google.com";

const CLIENT_ID_SUFFIX: &str = ".apps.googleusercontent.com";
const MAX_CLIENT_ID_LENGTH: usize = 256;
const MAX_OAUTH_JSON_BYTES: usize = 64 * 1024;
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
    client_secret_stored: bool,
    authorization_stored: bool,
    connected: bool,
    authentication_in_progress: bool,
    reauthentication_required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_checked_at: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchConsoleOAuthImportResult {
    status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id_changed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret_stored: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reauthentication_required: Option<bool>,
}

impl SearchConsoleOAuthImportResult {
    fn cancelled() -> Self {
        Self {
            status: "cancelled",
            client_id_changed: None,
            client_secret_stored: None,
            reauthentication_required: None,
        }
    }

    fn imported(client_id_changed: bool, reauthentication_required: bool) -> Self {
        Self {
            status: "imported",
            client_id_changed: Some(client_id_changed),
            client_secret_stored: Some(true),
            reauthentication_required: Some(reauthentication_required),
        }
    }
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
    ClientSecretNotConfigured,
    OAuthJsonSelectionFailed,
    OAuthJsonReadFailed,
    OAuthJsonTooLarge,
    InvalidOAuthJson,
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
    TokenInvalidGrant,
    TokenInvalidClient,
    TokenInvalidRequest,
    TokenUnauthorizedClient,
    TokenRedirectUriMismatch,
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
            Self::ClientSecretNotConfigured => "client_secret_not_configured",
            Self::OAuthJsonSelectionFailed => "oauth_json_selection_failed",
            Self::OAuthJsonReadFailed => "oauth_json_read_failed",
            Self::OAuthJsonTooLarge => "oauth_json_too_large",
            Self::InvalidOAuthJson => "invalid_oauth_json",
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
            Self::TokenInvalidGrant => "token_invalid_grant",
            Self::TokenInvalidClient => "token_invalid_client",
            Self::TokenInvalidRequest => "token_invalid_request",
            Self::TokenUnauthorizedClient => "token_unauthorized_client",
            Self::TokenRedirectUriMismatch => "token_redirect_uri_mismatch",
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
            Self::ClientSecretNotConfigured => {
                "Google OAuth Client Secret이 저장되지 않았습니다. OAuth JSON을 가져와 Client 설정을 완료하십시오."
            }
            Self::OAuthJsonSelectionFailed => "Google OAuth JSON 파일을 선택할 수 없습니다.",
            Self::OAuthJsonReadFailed => "선택한 Google OAuth JSON 파일을 읽을 수 없습니다.",
            Self::OAuthJsonTooLarge => "Google OAuth JSON 파일 크기가 허용 범위를 초과했습니다.",
            Self::InvalidOAuthJson => "Google Desktop OAuth JSON 형식이 올바르지 않습니다.",
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
            Self::TokenInvalidGrant => "Google 인증 코드를 사용할 수 없습니다.",
            Self::TokenInvalidClient => "Google OAuth Client 설정을 확인해야 합니다.",
            Self::TokenInvalidRequest => "Google 인증정보 요청 형식이 올바르지 않습니다.",
            Self::TokenUnauthorizedClient => {
                "이 Google OAuth Client에서는 해당 인증방식을 사용할 수 없습니다."
            }
            Self::TokenRedirectUriMismatch => "Google 인증의 되돌아오기 주소가 일치하지 않습니다.",
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

#[derive(Clone)]
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

#[derive(Debug)]
enum OAuthHttpClientError {
    InvalidRequest,
    RequestFailed,
    NetworkTimeout,
    InvalidResponse,
}

impl fmt::Display for OAuthHttpClientError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidRequest => "invalid oauth http request",
            Self::RequestFailed => "oauth http request failed",
            Self::NetworkTimeout => "oauth http request timed out",
            Self::InvalidResponse => "invalid oauth http response",
        };
        formatter.write_str(message)
    }
}

impl Error for OAuthHttpClientError {}

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
struct GoogleOAuthClientJson {
    installed: Option<GoogleInstalledOAuthClient>,
    web: Option<serde_json::Value>,
}

#[derive(Deserialize)]
struct GoogleInstalledOAuthClient {
    client_id: Option<String>,
    client_secret: Option<String>,
}

#[derive(Debug, Eq, PartialEq)]
struct ImportedOAuthClient {
    client_id: String,
    client_secret: String,
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
pub async fn import_search_console_oauth_json(
    app: AppHandle,
) -> CommandResult<SearchConsoleOAuthImportResult> {
    let dialog_app = app.clone();
    let selected = tauri::async_runtime::spawn_blocking(move || {
        dialog_app
            .dialog()
            .file()
            .add_filter("JSON", &["json"])
            .blocking_pick_file()
    })
    .await
    .map_err(|_| SearchConsoleCommandError::from(SearchConsoleError::OAuthJsonSelectionFailed))?;

    let Some(selected) = selected else {
        return Ok(SearchConsoleOAuthImportResult::cancelled());
    };
    let path = selected.into_path().map_err(|_| {
        SearchConsoleCommandError::from(SearchConsoleError::OAuthJsonSelectionFailed)
    })?;
    let oauth_client = tauri::async_runtime::spawn_blocking(move || read_oauth_client_json(&path))
        .await
        .map_err(|_| SearchConsoleCommandError::from(SearchConsoleError::OAuthJsonReadFailed))?
        .map_err(SearchConsoleCommandError::from)?;

    let _guard = OperationGuard::begin(OperationKind::ClientConfiguration)
        .map_err(SearchConsoleCommandError::from)?;
    let client_id_changed =
        import_oauth_client_credentials(&oauth_client).map_err(SearchConsoleCommandError::from)?;
    let reauthentication_required = if client_id_changed {
        true
    } else {
        client_status()
            .map_err(SearchConsoleCommandError::from)?
            .reauthentication_required
    };

    Ok(SearchConsoleOAuthImportResult::imported(
        client_id_changed,
        reauthentication_required,
    ))
}

#[tauri::command]
pub fn save_search_console_client_id(
    client_id: String,
) -> CommandResult<SearchConsoleClientStatus> {
    let client_id = validate_client_id(&client_id).map_err(SearchConsoleCommandError::from)?;
    let _guard = OperationGuard::begin(OperationKind::ClientConfiguration)
        .map_err(SearchConsoleCommandError::from)?;
    let mut credential_backend = WindowsSearchConsoleCredentialBackend;
    let mut runtime_backend = LiveSearchConsoleRuntimeBackend;
    save_search_console_client_id_with(
        &mut credential_backend,
        &mut runtime_backend,
        client_id,
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
        || delete_credential_if_present(CLIENT_SECRET_ACCOUNT),
        || delete_credential_if_present(CLIENT_ID_ACCOUNT),
        || clear_runtime_connection_state(false),
        client_status,
    )
    .map_err(Into::into)
}

#[tauri::command]
pub async fn start_search_console_oauth() -> CommandResult<SearchConsoleActionResult> {
    let _guard = OperationGuard::begin(OperationKind::Authentication)
        .map_err(SearchConsoleCommandError::from)?;
    let client_id = read_required_client_id().map_err(SearchConsoleCommandError::from)?;
    let client_secret = read_required_client_secret().map_err(SearchConsoleCommandError::from)?;

    let listener = bind_callback_listener().map_err(SearchConsoleCommandError::from)?;
    let port = listener
        .local_addr()
        .map_err(|_| SearchConsoleCommandError::from(SearchConsoleError::ListenerBindFailed))?
        .port();
    let request =
        build_authorization_request(&client_id, port).map_err(SearchConsoleCommandError::from)?;
    let expected_state = request.state.clone();

    let callback_cancelled = Arc::new(AtomicBool::new(false));
    let callback_cancel_signal = Arc::clone(&callback_cancelled);
    let callback_handle = tauri::async_runtime::spawn_blocking(move || {
        wait_for_callback(
            listener,
            expected_state,
            AUTHORIZATION_TIMEOUT,
            callback_cancel_signal,
        )
    });

    if webbrowser::open(&request.authorization_url).is_err() {
        callback_cancelled.store(true, Ordering::SeqCst);
        let _ = callback_handle.await;
        return Err(SearchConsoleCommandError::from(
            SearchConsoleError::BrowserOpenFailed,
        ));
    }

    let callback = callback_handle
        .await
        .map_err(|_| SearchConsoleCommandError::from(SearchConsoleError::Internal))?
        .map_err(SearchConsoleCommandError::from)?;

    let CallbackOutcome::Authorized { code } = callback;

    let tokens = exchange_authorization_code(
        &client_id,
        &client_secret,
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
    read_required_client_id().map_err(SearchConsoleCommandError::from)?;
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

    let client_id = read_required_client_id()?;
    let client_secret = read_required_client_secret()?;
    let refresh_token = read_credential(REFRESH_TOKEN_ACCOUNT).map_err(|error| match error {
        KeyringError::NoEntry => SearchConsoleError::NotConfigured,
        _ => SearchConsoleError::CredentialReadFailed,
    })?;

    let tokens = refresh_access_token_with(&client_id, &client_secret, &refresh_token).await?;
    if let Some(new_refresh_token) = tokens.refresh_token.as_deref() {
        save_credential(REFRESH_TOKEN_ACCOUNT, new_refresh_token)?;
    }
    store_access_token(tokens.access_token.clone(), tokens.expires_in)?;
    Ok(tokens.access_token)
}

async fn refresh_access_token_with(
    client_id: &str,
    client_secret: &str,
    refresh_token: &str,
) -> Result<TokenSet, SearchConsoleError> {
    refresh_access_token_with_token_endpoint(
        client_id,
        client_secret,
        refresh_token,
        TOKEN_ENDPOINT,
    )
    .await
}

async fn refresh_access_token_with_token_endpoint(
    client_id: &str,
    client_secret: &str,
    refresh_token: &str,
    token_endpoint: &str,
) -> Result<TokenSet, SearchConsoleError> {
    let client = secure_http_client()?;
    refresh_access_token_with_http_client(
        client,
        client_id,
        client_secret,
        refresh_token,
        token_endpoint,
    )
    .await
}

async fn refresh_access_token_with_http_client(
    client: reqwest::Client,
    client_id: &str,
    client_secret: &str,
    refresh_token: &str,
    token_endpoint: &str,
) -> Result<TokenSet, SearchConsoleError> {
    let body = urlencoded_body(&[
        ("grant_type", "refresh_token"),
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("refresh_token", refresh_token),
    ]);
    let response = client
        .post(token_endpoint)
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
    client_secret: &str,
    redirect_uri: &str,
    code: &str,
    pkce_verifier: &str,
) -> Result<TokenSet, SearchConsoleError> {
    let http_client = secure_http_client()?;
    request_oauth_authorization_code(
        client_id,
        client_secret,
        redirect_uri,
        code,
        pkce_verifier,
        move |request| oauth_http_client(http_client, request),
    )
    .await
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
    let body = urlencoded_body(&[("token", refresh_token)]);
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
    let redirect_url = RedirectUrl::new(format!("http://127.0.0.1:{port}"))
        .map_err(|_| SearchConsoleError::Internal)?;
    let redirect_uri = redirect_url.as_str().to_string();
    let client = build_oauth_client(client_id, &redirect_uri)?;
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

fn build_oauth_client(
    client_id: &str,
    redirect_uri: &str,
) -> Result<BasicClient, SearchConsoleError> {
    build_oauth_client_with_token_endpoint(client_id, redirect_uri, TOKEN_ENDPOINT)
}

fn build_oauth_client_with_token_endpoint(
    client_id: &str,
    redirect_uri: &str,
    token_endpoint: &str,
) -> Result<BasicClient, SearchConsoleError> {
    let auth_url = AuthUrl::new(AUTHORIZATION_ENDPOINT.to_string())
        .map_err(|_| SearchConsoleError::Internal)?;
    let token_url =
        TokenUrl::new(token_endpoint.to_string()).map_err(|_| SearchConsoleError::Internal)?;
    let redirect_url =
        RedirectUrl::new(redirect_uri.to_string()).map_err(|_| SearchConsoleError::Internal)?;
    Ok(BasicClient::new(
        ClientId::new(client_id.to_string()),
        None,
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(redirect_url))
}

fn build_oauth_token_client_with_token_endpoint(
    client_id: &str,
    client_secret: &str,
    redirect_uri: &str,
    token_endpoint: &str,
) -> Result<BasicClient, SearchConsoleError> {
    let auth_url = AuthUrl::new(AUTHORIZATION_ENDPOINT.to_string())
        .map_err(|_| SearchConsoleError::Internal)?;
    let token_url =
        TokenUrl::new(token_endpoint.to_string()).map_err(|_| SearchConsoleError::Internal)?;
    let redirect_url =
        RedirectUrl::new(redirect_uri.to_string()).map_err(|_| SearchConsoleError::Internal)?;
    Ok(BasicClient::new(
        ClientId::new(client_id.to_string()),
        Some(ClientSecret::new(client_secret.to_string())),
        auth_url,
        Some(token_url),
    )
    .set_auth_type(AuthType::RequestBody)
    .set_redirect_uri(redirect_url))
}

fn wait_for_callback(
    listener: TcpListener,
    expected_state: String,
    timeout: Duration,
    cancelled: Arc<AtomicBool>,
) -> Result<CallbackOutcome, SearchConsoleError> {
    let deadline = Instant::now() + timeout;
    loop {
        if cancelled.load(Ordering::SeqCst) {
            return Err(SearchConsoleError::Internal);
        }
        if Instant::now() >= deadline {
            return Err(SearchConsoleError::CallbackTimeout);
        }
        match listener.accept() {
            Ok((mut stream, peer)) => {
                if !is_loopback_peer(peer) {
                    let _ = write_callback_response(&mut stream, false);
                    continue;
                }
                let request = match read_limited_http_request(&mut stream) {
                    Ok(request) => request,
                    Err(_) => {
                        let _ = write_callback_response(&mut stream, false);
                        continue;
                    }
                };
                let parsed = parse_callback_request(&request, &expected_state);
                let _ = write_callback_response(&mut stream, parsed.is_ok());
                match parsed {
                    Ok(outcome) => return Ok(outcome),
                    Err(SearchConsoleError::InvalidCallback) => continue,
                    Err(error) => return Err(error),
                }
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
    let absolute_form_target = target.starts_with("http://") || target.starts_with("https://");
    if parts.next().is_some()
        || method != "GET"
        || !matches!(version, "HTTP/1.1" | "HTTP/1.0")
        || target.contains('#')
        || absolute_form_target
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
    validate_callback_issuer(&params)?;

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

fn validate_callback_issuer(params: &HashMap<String, String>) -> Result<(), SearchConsoleError> {
    if let Some(issuer) = params.get("iss") {
        if issuer.is_empty() || issuer != GOOGLE_ISSUER {
            return Err(SearchConsoleError::InvalidCallback);
        }
    }
    Ok(())
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
    let body = callback_response_body(success);
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nCache-Control: no-store\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );
    stream.write_all(response.as_bytes())?;
    stream.flush()
}

fn callback_response_body(success: bool) -> &'static str {
    if success {
        "<!doctype html><meta charset=\"utf-8\"><title>Korea Inside Admin</title><p>Google 인증 응답을 받았습니다.</p><p>관리자 앱에서 연결 결과를 확인하세요.</p>"
    } else {
        "<!doctype html><meta charset=\"utf-8\"><title>Korea Inside Admin</title><p>인증을 완료하지 못했습니다.</p><p>관리자 앱으로 돌아가 다시 시도하세요.</p>"
    }
}

#[cfg(test)]
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

#[cfg(test)]
fn map_authorization_token_error(body: &[u8]) -> SearchConsoleError {
    match serde_json::from_slice::<GoogleErrorResponse>(body)
        .ok()
        .and_then(|error| error.error)
        .as_deref()
    {
        Some("invalid_grant") => SearchConsoleError::TokenInvalidGrant,
        Some("invalid_client") => SearchConsoleError::TokenInvalidClient,
        Some("invalid_request") => SearchConsoleError::TokenInvalidRequest,
        Some("unauthorized_client") => SearchConsoleError::TokenUnauthorizedClient,
        Some("redirect_uri_mismatch") => SearchConsoleError::TokenRedirectUriMismatch,
        _ => SearchConsoleError::TokenExchangeFailed,
    }
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

async fn request_oauth_authorization_code<C, F>(
    client_id: &str,
    client_secret: &str,
    redirect_uri: &str,
    code: &str,
    pkce_verifier: &str,
    http_client: C,
) -> Result<TokenSet, SearchConsoleError>
where
    C: FnOnce(HttpRequest) -> F,
    F: std::future::Future<Output = Result<HttpResponse, OAuthHttpClientError>>,
{
    request_oauth_authorization_code_with_token_endpoint(
        client_id,
        client_secret,
        redirect_uri,
        code,
        pkce_verifier,
        TOKEN_ENDPOINT,
        http_client,
    )
    .await
}

async fn request_oauth_authorization_code_with_token_endpoint<C, F>(
    client_id: &str,
    client_secret: &str,
    redirect_uri: &str,
    code: &str,
    pkce_verifier: &str,
    token_endpoint: &str,
    http_client: C,
) -> Result<TokenSet, SearchConsoleError>
where
    C: FnOnce(HttpRequest) -> F,
    F: std::future::Future<Output = Result<HttpResponse, OAuthHttpClientError>>,
{
    let client = build_oauth_token_client_with_token_endpoint(
        client_id,
        client_secret,
        redirect_uri,
        token_endpoint,
    )?;
    let token_result = client
        .exchange_code(AuthorizationCode::new(code.to_string()))
        .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier.to_string()))
        .request_async(http_client)
        .await
        .map_err(map_oauth_token_error)?;
    token_set_from_oauth_response(token_result)
}

async fn oauth_http_client(
    client: reqwest::Client,
    request: HttpRequest,
) -> Result<HttpResponse, OAuthHttpClientError> {
    let method = reqwest::Method::from_bytes(request.method.as_str().as_bytes())
        .map_err(|_| OAuthHttpClientError::InvalidRequest)?;
    let mut builder = client.request(method, request.url.as_str());
    for (name, value) in request.headers.iter() {
        let header_name = reqwest::header::HeaderName::from_bytes(name.as_str().as_bytes())
            .map_err(|_| OAuthHttpClientError::InvalidRequest)?;
        let header_value = reqwest::header::HeaderValue::from_bytes(value.as_bytes())
            .map_err(|_| OAuthHttpClientError::InvalidRequest)?;
        builder = builder.header(header_name, header_value);
    }
    let response = builder.body(request.body).send().await.map_err(|error| {
        if error.is_timeout() {
            OAuthHttpClientError::NetworkTimeout
        } else {
            OAuthHttpClientError::RequestFailed
        }
    })?;
    let status_code = oauth2::http::StatusCode::from_u16(response.status().as_u16())
        .map_err(|_| OAuthHttpClientError::InvalidResponse)?;
    let mut headers = oauth2::http::HeaderMap::new();
    for (name, value) in response.headers().iter() {
        let header_name = oauth2::http::header::HeaderName::from_bytes(name.as_str().as_bytes())
            .map_err(|_| OAuthHttpClientError::InvalidResponse)?;
        let header_value = oauth2::http::HeaderValue::from_bytes(value.as_bytes())
            .map_err(|_| OAuthHttpClientError::InvalidResponse)?;
        headers.append(header_name, header_value);
    }
    let body = read_limited_oauth_body(response).await?;
    Ok(HttpResponse {
        status_code,
        headers,
        body,
    })
}

async fn read_limited_oauth_body(
    mut response: reqwest::Response,
) -> Result<Vec<u8>, OAuthHttpClientError> {
    let mut body = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| OAuthHttpClientError::InvalidResponse)?
    {
        if body.len().saturating_add(chunk.len()) > MAX_RESPONSE_BYTES {
            return Err(OAuthHttpClientError::InvalidResponse);
        }
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}

fn token_set_from_oauth_response(
    response: oauth2::basic::BasicTokenResponse,
) -> Result<TokenSet, SearchConsoleError> {
    if !scope_matches_requested_from_oauth(response.scopes()) {
        return Err(SearchConsoleError::ScopeNotGranted);
    }
    let expires_in = response.expires_in().unwrap_or(Duration::from_secs(3600));
    Ok(TokenSet {
        access_token: response.access_token().secret().to_string(),
        refresh_token: response
            .refresh_token()
            .map(|token| token.secret().to_string()),
        expires_in: expires_in.max(Duration::from_secs(1)),
    })
}

fn scope_matches_requested_from_oauth(scopes: Option<&Vec<Scope>>) -> bool {
    let Some(scopes) = scopes else {
        return true;
    };
    scopes.len() == 1 && scopes[0].as_ref() == SEARCH_CONSOLE_SCOPE
}

fn map_oauth_token_error(
    error: RequestTokenError<OAuthHttpClientError, BasicErrorResponse>,
) -> SearchConsoleError {
    match error {
        RequestTokenError::ServerResponse(response) => map_oauth_server_error(response.error()),
        RequestTokenError::Request(OAuthHttpClientError::NetworkTimeout) => {
            SearchConsoleError::NetworkTimeout
        }
        RequestTokenError::Request(_)
        | RequestTokenError::Parse(_, _)
        | RequestTokenError::Other(_) => SearchConsoleError::TokenExchangeFailed,
    }
}

fn map_oauth_server_error(error: &BasicErrorResponseType) -> SearchConsoleError {
    match error {
        BasicErrorResponseType::InvalidGrant => SearchConsoleError::TokenInvalidGrant,
        BasicErrorResponseType::InvalidClient => SearchConsoleError::TokenInvalidClient,
        BasicErrorResponseType::InvalidRequest => SearchConsoleError::TokenInvalidRequest,
        BasicErrorResponseType::UnauthorizedClient => SearchConsoleError::TokenUnauthorizedClient,
        BasicErrorResponseType::Extension(value) if value == "redirect_uri_mismatch" => {
            SearchConsoleError::TokenRedirectUriMismatch
        }
        _ => SearchConsoleError::TokenExchangeFailed,
    }
}

fn urlencoded_body(params: &[(&str, &str)]) -> String {
    let mut serializer = form_urlencoded::Serializer::new(String::new());
    for (key, value) in params {
        serializer.append_pair(key, value);
    }
    serializer.finish()
}

fn read_oauth_client_json(path: &Path) -> Result<ImportedOAuthClient, SearchConsoleError> {
    if path
        .extension()
        .and_then(|extension| extension.to_str())
        .is_none_or(|extension| !extension.eq_ignore_ascii_case("json"))
    {
        return Err(SearchConsoleError::InvalidOAuthJson);
    }
    let file = File::open(path).map_err(|_| SearchConsoleError::OAuthJsonReadFailed)?;
    let mut bytes = Vec::new();
    file.take((MAX_OAUTH_JSON_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| SearchConsoleError::OAuthJsonReadFailed)?;
    parse_oauth_client_json(&bytes)
}

fn parse_oauth_client_json(bytes: &[u8]) -> Result<ImportedOAuthClient, SearchConsoleError> {
    if bytes.len() > MAX_OAUTH_JSON_BYTES {
        return Err(SearchConsoleError::OAuthJsonTooLarge);
    }
    let parsed = serde_json::from_slice::<GoogleOAuthClientJson>(bytes)
        .map_err(|_| SearchConsoleError::InvalidOAuthJson)?;
    if parsed.web.is_some() {
        return Err(SearchConsoleError::InvalidOAuthJson);
    }
    let installed = parsed
        .installed
        .ok_or(SearchConsoleError::InvalidOAuthJson)?;
    let client_id = installed
        .client_id
        .ok_or(SearchConsoleError::InvalidOAuthJson)?;
    let client_id =
        validate_client_id(&client_id).map_err(|_| SearchConsoleError::InvalidOAuthJson)?;
    let client_secret = installed
        .client_secret
        .ok_or(SearchConsoleError::InvalidOAuthJson)?;
    let client_secret = client_secret.trim();
    if client_secret.is_empty() {
        return Err(SearchConsoleError::InvalidOAuthJson);
    }

    Ok(ImportedOAuthClient {
        client_id,
        client_secret: client_secret.to_string(),
    })
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

fn usable_stored_client_id(client_id: &str) -> Option<String> {
    if client_id == OAUTH_CREDENTIAL_UPDATE_LOCK {
        return None;
    }
    validate_client_id(client_id).ok()
}

trait SearchConsoleCredentialBackend {
    fn read_optional(&mut self, account: &str) -> Result<Option<String>, SearchConsoleError>;
    fn save(&mut self, account: &str, value: &str) -> Result<(), SearchConsoleError>;
    fn delete(&mut self, account: &str) -> Result<(), SearchConsoleError>;
}

struct WindowsSearchConsoleCredentialBackend;

impl SearchConsoleCredentialBackend for WindowsSearchConsoleCredentialBackend {
    fn read_optional(&mut self, account: &str) -> Result<Option<String>, SearchConsoleError> {
        read_optional_credential(account)
    }

    fn save(&mut self, account: &str, value: &str) -> Result<(), SearchConsoleError> {
        save_credential(account, value)
    }

    fn delete(&mut self, account: &str) -> Result<(), SearchConsoleError> {
        delete_credential_if_present(account)
    }
}

fn read_required_client_id_with<S: SearchConsoleCredentialBackend>(
    backend: &mut S,
) -> Result<String, SearchConsoleError> {
    let client_id = backend
        .read_optional(CLIENT_ID_ACCOUNT)?
        .ok_or(SearchConsoleError::NotConfigured)?;
    if client_id == OAUTH_CREDENTIAL_UPDATE_LOCK {
        return Err(SearchConsoleError::NotConfigured);
    }
    validate_client_id(&client_id)
}

fn read_required_client_id() -> Result<String, SearchConsoleError> {
    let mut backend = WindowsSearchConsoleCredentialBackend;
    read_required_client_id_with(&mut backend)
}

struct OAuthCredentialSnapshot {
    client_id: Option<String>,
    client_secret: Option<String>,
    refresh_token: Option<String>,
}

#[derive(Clone)]
struct RuntimeConnectionSnapshot {
    connected: bool,
    reauthentication_required: bool,
    last_checked_at: Option<String>,
    access_token: Option<AccessTokenCache>,
}

trait SearchConsoleRuntimeBackend {
    fn snapshot(&mut self) -> Result<RuntimeConnectionSnapshot, SearchConsoleError>;
    fn clear(&mut self, reauthentication_required: bool) -> Result<(), SearchConsoleError>;
    fn restore(&mut self, snapshot: RuntimeConnectionSnapshot) -> Result<(), SearchConsoleError>;
}

struct LiveSearchConsoleRuntimeBackend;

impl SearchConsoleRuntimeBackend for LiveSearchConsoleRuntimeBackend {
    fn snapshot(&mut self) -> Result<RuntimeConnectionSnapshot, SearchConsoleError> {
        snapshot_runtime_connection_state()
    }

    fn clear(&mut self, reauthentication_required: bool) -> Result<(), SearchConsoleError> {
        clear_runtime_connection_state(reauthentication_required)
    }

    fn restore(&mut self, snapshot: RuntimeConnectionSnapshot) -> Result<(), SearchConsoleError> {
        restore_runtime_connection_state(snapshot)
    }
}

fn import_oauth_client_credentials(
    oauth_client: &ImportedOAuthClient,
) -> Result<bool, SearchConsoleError> {
    let mut backend = WindowsSearchConsoleCredentialBackend;
    let mut runtime_backend = LiveSearchConsoleRuntimeBackend;
    import_oauth_client_credentials_with(&mut backend, &mut runtime_backend, oauth_client)
}

fn import_oauth_client_credentials_with<S, R>(
    backend: &mut S,
    runtime_backend: &mut R,
    oauth_client: &ImportedOAuthClient,
) -> Result<bool, SearchConsoleError>
where
    S: SearchConsoleCredentialBackend,
    R: SearchConsoleRuntimeBackend,
{
    let client_id = backend.read_optional(CLIENT_ID_ACCOUNT)?;
    let client_secret = backend.read_optional(CLIENT_SECRET_ACCOUNT)?;
    let same_client_id = client_id
        .as_deref()
        .and_then(usable_stored_client_id)
        .as_deref()
        == Some(oauth_client.client_id.as_str());

    if same_client_id {
        if let Err(error) = backend.save(CLIENT_SECRET_ACCOUNT, &oauth_client.client_secret) {
            if restore_credential_entry(backend, CLIENT_SECRET_ACCOUNT, client_secret.as_deref())
                .is_err()
            {
                delete_oauth_credential_fragments_best_effort(backend);
            }
            return Err(error);
        }
        return Ok(false);
    }

    let snapshot = OAuthCredentialSnapshot {
        client_id,
        client_secret,
        refresh_token: backend.read_optional(REFRESH_TOKEN_ACCOUNT)?,
    };
    let runtime_snapshot = runtime_backend.snapshot()?;
    backend.save(CLIENT_ID_ACCOUNT, OAUTH_CREDENTIAL_UPDATE_LOCK)?;
    let update_result = (|| {
        backend.save(CLIENT_SECRET_ACCOUNT, &oauth_client.client_secret)?;
        backend.delete(REFRESH_TOKEN_ACCOUNT)?;
        runtime_backend.clear(true)?;
        backend.save(CLIENT_ID_ACCOUNT, &oauth_client.client_id)?;
        Ok(())
    })();

    if let Err(error) = update_result {
        let _ = rollback_oauth_transaction(backend, runtime_backend, &snapshot, runtime_snapshot);
        return Err(error);
    }

    Ok(true)
}

fn restore_oauth_transaction_snapshot<S, R>(
    backend: &mut S,
    runtime_backend: &mut R,
    snapshot: &OAuthCredentialSnapshot,
    runtime_snapshot: RuntimeConnectionSnapshot,
) -> Result<(), SearchConsoleError>
where
    S: SearchConsoleCredentialBackend,
    R: SearchConsoleRuntimeBackend,
{
    let mut first_error = None;
    for (account, value) in [
        (CLIENT_SECRET_ACCOUNT, snapshot.client_secret.as_deref()),
        (REFRESH_TOKEN_ACCOUNT, snapshot.refresh_token.as_deref()),
    ] {
        if let Err(error) = restore_credential_entry(backend, account, value) {
            first_error.get_or_insert(error);
        }
    }
    if let Err(error) = runtime_backend.restore(runtime_snapshot) {
        first_error.get_or_insert(error);
    }
    if let Some(error) = first_error {
        return Err(error);
    }
    restore_credential_entry(backend, CLIENT_ID_ACCOUNT, snapshot.client_id.as_deref())
}

fn restore_credential_entry<S: SearchConsoleCredentialBackend>(
    backend: &mut S,
    account: &str,
    value: Option<&str>,
) -> Result<(), SearchConsoleError> {
    match value {
        Some(value) => backend.save(account, value),
        None => backend.delete(account),
    }
}

fn delete_oauth_credential_fragments_best_effort<S: SearchConsoleCredentialBackend>(
    backend: &mut S,
) {
    for _ in 0..2 {
        let _ = backend.delete(REFRESH_TOKEN_ACCOUNT);
        let _ = backend.delete(CLIENT_SECRET_ACCOUNT);
    }
}

fn rollback_oauth_transaction<S, R>(
    backend: &mut S,
    runtime_backend: &mut R,
    snapshot: &OAuthCredentialSnapshot,
    runtime_snapshot: RuntimeConnectionSnapshot,
) -> Result<(), SearchConsoleError>
where
    S: SearchConsoleCredentialBackend,
    R: SearchConsoleRuntimeBackend,
{
    let result =
        restore_oauth_transaction_snapshot(backend, runtime_backend, snapshot, runtime_snapshot);
    if result.is_err() {
        delete_oauth_credential_fragments_best_effort(backend);
        let _ = runtime_backend.clear(true);
    }
    result
}

fn save_search_console_client_id_with<S, R, T>(
    credential_backend: &mut S,
    runtime_backend: &mut R,
    client_id: String,
    mut status: T,
) -> Result<SearchConsoleClientStatus, SearchConsoleError>
where
    S: SearchConsoleCredentialBackend,
    R: SearchConsoleRuntimeBackend,
    T: FnMut() -> Result<SearchConsoleClientStatus, SearchConsoleError>,
{
    let client_id = validate_client_id(&client_id)?;
    let stored_client_id = credential_backend.read_optional(CLIENT_ID_ACCOUNT)?;
    let same_client_id = stored_client_id
        .as_deref()
        .and_then(usable_stored_client_id)
        .as_deref()
        == Some(client_id.as_str());
    if same_client_id {
        return status();
    }

    let credential_snapshot = OAuthCredentialSnapshot {
        client_id: stored_client_id,
        client_secret: credential_backend.read_optional(CLIENT_SECRET_ACCOUNT)?,
        refresh_token: credential_backend.read_optional(REFRESH_TOKEN_ACCOUNT)?,
    };
    let runtime_snapshot = runtime_backend.snapshot()?;
    credential_backend.save(CLIENT_ID_ACCOUNT, OAUTH_CREDENTIAL_UPDATE_LOCK)?;
    let update_result = (|| {
        credential_backend.delete(REFRESH_TOKEN_ACCOUNT)?;
        credential_backend.delete(CLIENT_SECRET_ACCOUNT)?;
        runtime_backend.clear(true)?;
        credential_backend.save(CLIENT_ID_ACCOUNT, &client_id)?;
        Ok(())
    })();

    if let Err(error) = update_result {
        let _ = rollback_oauth_transaction(
            credential_backend,
            runtime_backend,
            &credential_snapshot,
            runtime_snapshot,
        );
        return Err(error);
    }

    Ok(client_id_change_completed_status())
}

fn client_id_change_completed_status() -> SearchConsoleClientStatus {
    SearchConsoleClientStatus {
        configured: true,
        client_secret_stored: false,
        authorization_stored: false,
        connected: false,
        authentication_in_progress: false,
        reauthentication_required: true,
        last_checked_at: None,
    }
}

fn delete_search_console_client_id_with<D, S, R, C, T>(
    mut delete_refresh_token: D,
    mut delete_client_secret: S,
    mut delete_client_id: R,
    mut clear_runtime: C,
    mut status: T,
) -> Result<SearchConsoleClientStatus, SearchConsoleError>
where
    D: FnMut() -> Result<(), SearchConsoleError>,
    S: FnMut() -> Result<(), SearchConsoleError>,
    R: FnMut() -> Result<(), SearchConsoleError>,
    C: FnMut() -> Result<(), SearchConsoleError>,
    T: FnMut() -> Result<SearchConsoleClientStatus, SearchConsoleError>,
{
    let mut first_error = None;
    for result in [
        delete_refresh_token(),
        delete_client_secret(),
        delete_client_id(),
        clear_runtime(),
    ] {
        if let Err(error) = result {
            first_error.get_or_insert(error);
        }
    }
    if let Some(error) = first_error {
        return Err(error);
    }
    status()
}

fn client_status() -> Result<SearchConsoleClientStatus, SearchConsoleError> {
    let configured = read_stored_client_id()?.is_some();
    let client_secret_stored = credential_exists(CLIENT_SECRET_ACCOUNT)?;
    let authorization_stored = credential_exists(REFRESH_TOKEN_ACCOUNT)?;
    client_status_from_parts(configured, client_secret_stored, authorization_stored)
}

fn client_status_from_parts(
    configured: bool,
    client_secret_stored: bool,
    authorization_stored: bool,
) -> Result<SearchConsoleClientStatus, SearchConsoleError> {
    let state = lock_runtime_state()?;
    Ok(SearchConsoleClientStatus {
        configured,
        client_secret_stored,
        authorization_stored,
        connected: state.connected
            && configured
            && client_secret_stored
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

fn snapshot_runtime_connection_state() -> Result<RuntimeConnectionSnapshot, SearchConsoleError> {
    let state = lock_runtime_state()?;
    Ok(RuntimeConnectionSnapshot {
        connected: state.connected,
        reauthentication_required: state.reauthentication_required,
        last_checked_at: state.last_checked_at.clone(),
        access_token: state.access_token.clone(),
    })
}

fn restore_runtime_connection_state(
    snapshot: RuntimeConnectionSnapshot,
) -> Result<(), SearchConsoleError> {
    let mut state = lock_runtime_state()?;
    state.connected = snapshot.connected;
    state.reauthentication_required = snapshot.reauthentication_required;
    state.last_checked_at = snapshot.last_checked_at;
    state.access_token = snapshot.access_token;
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

fn read_optional_credential(account: &str) -> Result<Option<String>, SearchConsoleError> {
    match read_credential(account) {
        Ok(value) => Ok(Some(value)),
        Err(KeyringError::NoEntry) => Ok(None),
        Err(_) => Err(SearchConsoleError::CredentialReadFailed),
    }
}

fn read_stored_client_id() -> Result<Option<String>, SearchConsoleError> {
    Ok(read_optional_credential(CLIENT_ID_ACCOUNT)?
        .as_deref()
        .and_then(usable_stored_client_id))
}

fn read_required_client_secret() -> Result<String, SearchConsoleError> {
    require_client_secret(read_optional_credential(CLIENT_SECRET_ACCOUNT)?)
}

fn require_client_secret(value: Option<String>) -> Result<String, SearchConsoleError> {
    value
        .filter(|value| !value.trim().is_empty())
        .ok_or(SearchConsoleError::ClientSecretNotConfigured)
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

    #[derive(Debug)]
    struct MockTokenRequestMetadata {
        method: String,
        path: String,
        headers: HashMap<String, String>,
        body: String,
    }

    #[derive(Default)]
    struct FakeCredentialBackend {
        values: HashMap<String, String>,
        operations: Vec<String>,
        attempts: HashMap<String, usize>,
        failures: HashSet<String>,
        persistent_failures: HashMap<String, usize>,
        shared_operations: Option<Rc<RefCell<Vec<String>>>>,
    }

    impl FakeCredentialBackend {
        fn fail_save_on_attempt(&mut self, account: &str, attempt: usize) {
            self.failures.insert(format!("save:{account}:{attempt}"));
        }

        fn fail_delete_on_attempt(&mut self, account: &str, attempt: usize) {
            self.failures.insert(format!("delete:{account}:{attempt}"));
        }

        fn fail_save_from_attempt(&mut self, account: &str, attempt: usize) {
            self.persistent_failures
                .insert(format!("save:{account}"), attempt);
        }

        fn fail_delete_from_attempt(&mut self, account: &str, attempt: usize) {
            self.persistent_failures
                .insert(format!("delete:{account}"), attempt);
        }

        fn record_operations_with(&mut self, operations: Rc<RefCell<Vec<String>>>) {
            self.shared_operations = Some(operations);
        }

        fn attempt_count(&self, operation: &str, account: &str) -> usize {
            self.attempts
                .get(&format!("{operation}:{account}"))
                .copied()
                .unwrap_or_default()
        }

        fn should_fail(&mut self, operation: &str, account: &str) -> bool {
            let operation_key = format!("{operation}:{account}");
            let attempt = {
                let attempt = self.attempts.entry(operation_key.clone()).or_default();
                *attempt += 1;
                *attempt
            };
            self.operations.push(operation_key.clone());
            if let Some(operations) = &self.shared_operations {
                operations.borrow_mut().push(operation_key.clone());
            }
            let one_shot_failure = self.failures.remove(&format!("{operation_key}:{attempt}"));
            let persistent_failure = self
                .persistent_failures
                .get(&operation_key)
                .is_some_and(|first_failure| attempt >= *first_failure);
            one_shot_failure || persistent_failure
        }
    }

    impl SearchConsoleCredentialBackend for FakeCredentialBackend {
        fn read_optional(&mut self, account: &str) -> Result<Option<String>, SearchConsoleError> {
            Ok(self.values.get(account).cloned())
        }

        fn save(&mut self, account: &str, value: &str) -> Result<(), SearchConsoleError> {
            if self.should_fail("save", account) {
                return Err(SearchConsoleError::CredentialStoreFailed);
            }
            self.values.insert(account.to_string(), value.to_string());
            Ok(())
        }

        fn delete(&mut self, account: &str) -> Result<(), SearchConsoleError> {
            if self.should_fail("delete", account) {
                return Err(SearchConsoleError::CredentialDeleteFailed);
            }
            self.values.remove(account);
            Ok(())
        }
    }

    struct FakeRuntimeBackend {
        state: RuntimeConnectionSnapshot,
        snapshot_calls: usize,
        clear_calls: usize,
        restore_calls: usize,
        fail_clear_attempts: HashSet<usize>,
        fail_restore_attempts: HashSet<usize>,
        shared_operations: Option<Rc<RefCell<Vec<String>>>>,
    }

    impl FakeRuntimeBackend {
        fn connected() -> Self {
            Self {
                state: RuntimeConnectionSnapshot {
                    connected: true,
                    reauthentication_required: false,
                    last_checked_at: Some("2026-07-14T00:00:00Z".to_string()),
                    access_token: Some(AccessTokenCache {
                        token: ["fixture", "access", "token"].join("-"),
                        expires_at: Instant::now() + Duration::from_secs(3600),
                    }),
                },
                snapshot_calls: 0,
                clear_calls: 0,
                restore_calls: 0,
                fail_clear_attempts: HashSet::new(),
                fail_restore_attempts: HashSet::new(),
                shared_operations: None,
            }
        }

        fn restarted() -> Self {
            Self {
                state: RuntimeConnectionSnapshot {
                    connected: false,
                    reauthentication_required: false,
                    last_checked_at: None,
                    access_token: None,
                },
                snapshot_calls: 0,
                clear_calls: 0,
                restore_calls: 0,
                fail_clear_attempts: HashSet::new(),
                fail_restore_attempts: HashSet::new(),
                shared_operations: None,
            }
        }

        fn record_operations_with(&mut self, operations: Rc<RefCell<Vec<String>>>) {
            self.shared_operations = Some(operations);
        }

        fn fail_clear_on_attempt(&mut self, attempt: usize) {
            self.fail_clear_attempts.insert(attempt);
        }

        fn fail_restore_on_attempt(&mut self, attempt: usize) {
            self.fail_restore_attempts.insert(attempt);
        }
    }

    impl SearchConsoleRuntimeBackend for FakeRuntimeBackend {
        fn snapshot(&mut self) -> Result<RuntimeConnectionSnapshot, SearchConsoleError> {
            self.snapshot_calls += 1;
            if let Some(operations) = &self.shared_operations {
                operations.borrow_mut().push("snapshot:runtime".to_string());
            }
            Ok(self.state.clone())
        }

        fn clear(&mut self, reauthentication_required: bool) -> Result<(), SearchConsoleError> {
            self.clear_calls += 1;
            if let Some(operations) = &self.shared_operations {
                operations.borrow_mut().push("clear:runtime".to_string());
            }
            if self.fail_clear_attempts.remove(&self.clear_calls) {
                return Err(SearchConsoleError::Internal);
            }
            self.state.access_token = None;
            self.state.connected = false;
            self.state.reauthentication_required = reauthentication_required;
            self.state.last_checked_at = None;
            Ok(())
        }

        fn restore(
            &mut self,
            snapshot: RuntimeConnectionSnapshot,
        ) -> Result<(), SearchConsoleError> {
            self.restore_calls += 1;
            if let Some(operations) = &self.shared_operations {
                operations.borrow_mut().push("restore:runtime".to_string());
            }
            if self.fail_restore_attempts.remove(&self.restore_calls) {
                return Err(SearchConsoleError::Internal);
            }
            self.state = snapshot;
            Ok(())
        }
    }

    fn import_oauth_client_credentials_for_test(
        backend: &mut FakeCredentialBackend,
        oauth_client: &ImportedOAuthClient,
    ) -> Result<bool, SearchConsoleError> {
        let mut runtime = FakeRuntimeBackend::connected();
        import_oauth_client_credentials_with(backend, &mut runtime, oauth_client)
    }

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
    fn parses_installed_desktop_oauth_json_and_trims_outer_whitespace() {
        let client_secret = test_client_secret();
        let bytes = desktop_oauth_json_fixture(
            &format!("  {VALID_CLIENT_ID}\r\n"),
            &format!("  {client_secret}\r\n"),
        );

        let parsed = parse_oauth_client_json(&bytes).unwrap();

        assert_eq!(parsed.client_id, VALID_CLIENT_ID);
        assert_eq!(parsed.client_secret, client_secret);
    }

    #[test]
    fn rejects_web_oauth_json() {
        let bytes = serde_json::to_vec(&serde_json::json!({
            "web": {
                "client_id": VALID_CLIENT_ID,
                "client_secret": test_client_secret()
            }
        }))
        .unwrap();
        assert_eq!(
            parse_oauth_client_json(&bytes).unwrap_err(),
            SearchConsoleError::InvalidOAuthJson
        );
    }

    #[test]
    fn rejects_oauth_json_with_missing_client_id() {
        let bytes = serde_json::to_vec(&serde_json::json!({
            "installed": { "client_secret": test_client_secret() }
        }))
        .unwrap();
        assert_eq!(
            parse_oauth_client_json(&bytes).unwrap_err(),
            SearchConsoleError::InvalidOAuthJson
        );
    }

    #[test]
    fn rejects_oauth_json_with_missing_or_empty_client_secret() {
        for installed in [
            serde_json::json!({ "client_id": VALID_CLIENT_ID }),
            serde_json::json!({ "client_id": VALID_CLIENT_ID, "client_secret": "   " }),
        ] {
            let bytes = serde_json::to_vec(&serde_json::json!({ "installed": installed })).unwrap();
            assert_eq!(
                parse_oauth_client_json(&bytes).unwrap_err(),
                SearchConsoleError::InvalidOAuthJson
            );
        }
    }

    #[test]
    fn rejects_oauth_json_with_invalid_client_id() {
        let bytes =
            desktop_oauth_json_fixture("not-a-desktop-client.example.com", &test_client_secret());
        assert_eq!(
            parse_oauth_client_json(&bytes).unwrap_err(),
            SearchConsoleError::InvalidOAuthJson
        );
    }

    #[test]
    fn rejects_oversized_oauth_json_before_parsing() {
        let bytes = vec![b' '; MAX_OAUTH_JSON_BYTES + 1];
        assert_eq!(
            parse_oauth_client_json(&bytes).unwrap_err(),
            SearchConsoleError::OAuthJsonTooLarge
        );
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
        assert_eq!(request.redirect_uri, "http://127.0.0.1:49152");
        assert!(!request
            .redirect_uri
            .contains("/search-console/oauth/callback"));
        assert!(request
            .authorization_url
            .contains("redirect_uri=http%3A%2F%2F127.0.0.1%3A49152"));
    }

    #[test]
    fn authorization_code_token_request_uses_post_endpoint_and_form_content_type() {
        let request = build_authorization_request(VALID_CLIENT_ID, 49152).unwrap();
        let token_request = authorization_code_token_request_for_test(
            VALID_CLIENT_ID,
            &request.redirect_uri,
            "test-code",
            &request.pkce_verifier,
        );

        assert_eq!(token_request.method, oauth2::http::Method::POST);
        assert_eq!(token_request.url.as_str(), TOKEN_ENDPOINT);
        assert_eq!(
            token_request
                .headers
                .get(oauth2::http::header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok()),
            Some("application/x-www-form-urlencoded")
        );
    }

    #[test]
    fn authorization_code_token_form_contains_required_fields_once() {
        let request = build_authorization_request(VALID_CLIENT_ID, 49152).unwrap();
        let token_request = authorization_code_token_request_for_test(
            VALID_CLIENT_ID,
            &request.redirect_uri,
            "test-code",
            &request.pkce_verifier,
        );
        let body = request_body_string_for_test(&token_request);
        let params = parse_form_body_for_test(&body);
        let keys = parse_form_keys_for_test(&body);

        assert_eq!(params.len(), 6);
        assert_eq!(keys.len(), 6);
        assert_eq!(keys.iter().collect::<HashSet<_>>().len(), 6);
        assert_eq!(
            params.get("client_id").map(String::as_str),
            Some(VALID_CLIENT_ID)
        );
        assert_eq!(
            params.get("client_secret").map(String::as_str),
            Some(test_client_secret().as_str())
        );
        assert_eq!(params.get("code").map(String::as_str), Some("test-code"));
        assert_eq!(
            params.get("code_verifier").map(String::as_str),
            Some(request.pkce_verifier.as_str())
        );
        assert_eq!(
            params.get("grant_type").map(String::as_str),
            Some("authorization_code")
        );
        assert_eq!(
            params.get("redirect_uri").map(String::as_str),
            Some(request.redirect_uri.as_str())
        );
    }

    #[test]
    fn authorization_code_token_form_has_no_empty_fields_and_includes_client_secret() {
        let request = build_authorization_request(VALID_CLIENT_ID, 49152).unwrap();
        let token_request = authorization_code_token_request_for_test(
            VALID_CLIENT_ID,
            &request.redirect_uri,
            "test-code",
            &request.pkce_verifier,
        );
        let body = request_body_string_for_test(&token_request);
        let params = parse_form_body_for_test(&body);

        assert!(params.contains_key("client_secret"));
        assert!(params.values().all(|value| !value.is_empty()));
    }

    #[test]
    fn missing_or_empty_stored_client_secret_is_rejected_before_token_request() {
        for value in [None, Some(String::new()), Some("   ".to_string())] {
            assert_eq!(
                require_client_secret(value).unwrap_err(),
                SearchConsoleError::ClientSecretNotConfigured
            );
        }
        let error = SearchConsoleCommandError::from(SearchConsoleError::ClientSecretNotConfigured);
        assert_eq!(error.code, "client_secret_not_configured");
        assert_eq!(
            error.message,
            "Google OAuth Client Secret이 저장되지 않았습니다. OAuth JSON을 가져와 Client 설정을 완료하십시오."
        );
    }

    #[test]
    fn non_empty_stored_client_secret_is_preserved_for_token_request() {
        let client_secret = test_client_secret();
        assert_eq!(
            require_client_secret(Some(client_secret.clone())).unwrap(),
            client_secret
        );
    }

    #[test]
    fn authorization_and_token_redirect_uri_match_exactly() {
        let request = build_authorization_request(VALID_CLIENT_ID, 49152).unwrap();
        let token_request = authorization_code_token_request_for_test(
            VALID_CLIENT_ID,
            &request.redirect_uri,
            "test-code",
            &request.pkce_verifier,
        );
        let body = request_body_string_for_test(&token_request);
        let params = parse_form_body_for_test(&body);

        assert_eq!(
            params.get("redirect_uri").map(String::as_str),
            Some(request.redirect_uri.as_str())
        );
    }

    #[test]
    fn authorization_code_token_form_preserves_redirect_structure() {
        let request = build_authorization_request(VALID_CLIENT_ID, 49152).unwrap();
        let token_request = authorization_code_token_request_for_test(
            VALID_CLIENT_ID,
            &request.redirect_uri,
            "test-code",
            &request.pkce_verifier,
        );
        let body = request_body_string_for_test(&token_request);
        let params = parse_form_body_for_test(&body);
        let redirect_uri = params.get("redirect_uri").unwrap();
        let redirect_url = url::Url::parse(redirect_uri).unwrap();

        assert_eq!(redirect_url.scheme(), "http");
        assert_eq!(redirect_url.host_str(), Some("127.0.0.1"));
        assert_eq!(redirect_url.port(), Some(49152));
        assert_eq!(redirect_url.path(), CALLBACK_PATH);
        assert!(redirect_url.query().is_none());
        assert!(redirect_url.fragment().is_none());
    }

    #[test]
    fn authorization_code_token_form_preserves_pkce_verifier() {
        let request = build_authorization_request(VALID_CLIENT_ID, 49152).unwrap();
        let token_request = authorization_code_token_request_for_test(
            VALID_CLIENT_ID,
            &request.redirect_uri,
            "test-code",
            &request.pkce_verifier,
        );
        let body = request_body_string_for_test(&token_request);
        let params = parse_form_body_for_test(&body);

        assert_eq!(
            params.get("code_verifier").map(String::as_str),
            Some(request.pkce_verifier.as_str())
        );
    }

    #[test]
    fn authorization_code_token_form_encodes_code_once() {
        let request = build_authorization_request(VALID_CLIENT_ID, 49152).unwrap();
        let decoded_code = "code/with+reserved%value";
        let token_request = authorization_code_token_request_for_test(
            VALID_CLIENT_ID,
            &request.redirect_uri,
            decoded_code,
            &request.pkce_verifier,
        );
        let body = request_body_string_for_test(&token_request);
        let params = parse_form_body_for_test(&body);

        assert_eq!(params.get("code").map(String::as_str), Some(decoded_code));
        assert!(body.contains("code=code%2Fwith%2Breserved%25value"));
        assert!(!body.contains(decoded_code));
    }

    #[test]
    fn authorization_code_plus_is_not_decoded_as_space() {
        let request = build_authorization_request(VALID_CLIENT_ID, 49152).unwrap();
        let decoded_code = "code+plus";
        let token_request = authorization_code_token_request_for_test(
            VALID_CLIENT_ID,
            &request.redirect_uri,
            decoded_code,
            &request.pkce_verifier,
        );
        let body = request_body_string_for_test(&token_request);
        let params = parse_form_body_for_test(&body);

        assert_eq!(params.get("code").map(String::as_str), Some("code+plus"));
        assert_ne!(params.get("code").map(String::as_str), Some("code plus"));
    }

    #[test]
    fn oauth_http_client_adapter_sends_expected_authorization_code_request_to_mock_endpoint() {
        let request = build_authorization_request(VALID_CLIENT_ID, 49152).unwrap();
        let expected_code = "fixture-code+with/reserved%chars";
        let expected_verifier = request.pkce_verifier.clone();
        let expected_client_secret = test_client_secret();
        let (token_endpoint, server) = spawn_mock_token_endpoint();
        let client = reqwest::Client::builder()
            .redirect(Policy::none())
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();

        let result =
            tauri::async_runtime::block_on(request_oauth_authorization_code_with_token_endpoint(
                VALID_CLIENT_ID,
                &expected_client_secret,
                &request.redirect_uri,
                expected_code,
                &expected_verifier,
                &token_endpoint,
                move |token_request| oauth_http_client(client, token_request),
            ));

        assert!(result.is_ok());
        let metadata = server.join().unwrap();
        let body = metadata.body;
        let params = parse_form_body_for_test(&body);
        let keys = parse_form_keys_for_test(&body);

        assert_eq!(metadata.method, "POST");
        assert_eq!(metadata.path, "/token");
        assert_eq!(
            metadata.headers.get("content-type").map(String::as_str),
            Some("application/x-www-form-urlencoded")
        );
        assert!(!body.is_empty());
        assert_eq!(params.len(), 6);
        assert_eq!(keys.len(), 6);
        for key in [
            "client_id",
            "client_secret",
            "code",
            "code_verifier",
            "grant_type",
            "redirect_uri",
        ] {
            assert_eq!(
                keys.iter()
                    .filter(|candidate| candidate.as_str() == key)
                    .count(),
                1
            );
            assert!(params.get(key).is_some_and(|value| !value.is_empty()));
        }
        assert_eq!(
            params.get("client_id").map(String::as_str),
            Some(VALID_CLIENT_ID)
        );
        assert_eq!(
            params.get("client_secret").map(String::as_str),
            Some(expected_client_secret.as_str())
        );
        assert_eq!(params.get("code").map(String::as_str), Some(expected_code));
        assert_eq!(
            params.get("code_verifier").map(String::as_str),
            Some(expected_verifier.as_str())
        );
        assert_eq!(
            params.get("grant_type").map(String::as_str),
            Some("authorization_code")
        );
        assert_eq!(
            params.get("redirect_uri").map(String::as_str),
            Some(request.redirect_uri.as_str())
        );
        assert!(!metadata.headers.contains_key("authorization"));
        assert!(!body.contains("%253A%252F%252F"));
    }

    #[test]
    fn refresh_token_form_includes_client_secret_and_existing_fields_once() {
        let expected_client_secret = test_client_secret();
        let expected_refresh_token = ["fixture", "refresh", "token"].join("-");
        let (token_endpoint, server) = spawn_mock_token_endpoint();
        let client = reqwest::Client::builder()
            .redirect(Policy::none())
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();

        let result = tauri::async_runtime::block_on(refresh_access_token_with_http_client(
            client,
            VALID_CLIENT_ID,
            &expected_client_secret,
            &expected_refresh_token,
            &token_endpoint,
        ));

        assert!(result.is_ok());
        let metadata = server.join().unwrap();
        let params = parse_form_body_for_test(&metadata.body);
        let keys = parse_form_keys_for_test(&metadata.body);
        assert_eq!(metadata.method, "POST");
        assert_eq!(metadata.path, "/token");
        assert_eq!(params.len(), 4);
        assert_eq!(keys.len(), 4);
        for key in ["client_id", "client_secret", "refresh_token", "grant_type"] {
            assert_eq!(
                keys.iter()
                    .filter(|candidate| candidate.as_str() == key)
                    .count(),
                1
            );
        }
        assert_eq!(
            params.get("client_id").map(String::as_str),
            Some(VALID_CLIENT_ID)
        );
        assert_eq!(
            params.get("client_secret").map(String::as_str),
            Some(expected_client_secret.as_str())
        );
        assert_eq!(
            params.get("refresh_token").map(String::as_str),
            Some(expected_refresh_token.as_str())
        );
        assert_eq!(
            params.get("grant_type").map(String::as_str),
            Some("refresh_token")
        );
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
    fn parses_successful_callback_with_google_issuer() {
        let request = format!(
            "GET {CALLBACK_PATH}?state=state123&iss=https://accounts.google.com&code=abc123 HTTP/1.1\r\n\r\n"
        );
        assert_eq!(
            parse_callback_request(&request, "state123").unwrap(),
            CallbackOutcome::Authorized {
                code: "abc123".to_string()
            }
        );
    }

    #[test]
    fn issuer_query_value_is_not_mistaken_for_absolute_form() {
        let request = format!(
            "GET {CALLBACK_PATH}?state=state123&iss=https://accounts.google.com&code=abc123 HTTP/1.1\r\n\r\n"
        );
        assert!(parse_callback_request(&request, "state123").is_ok());
    }

    #[test]
    fn rejects_https_absolute_form_request_target() {
        let request = format!(
            "GET https://127.0.0.1{CALLBACK_PATH}?state=state123&iss=https://accounts.google.com&code=abc123 HTTP/1.1\r\n\r\n"
        );
        assert_eq!(
            parse_callback_request(&request, "state123").unwrap_err(),
            SearchConsoleError::InvalidCallback
        );
    }

    #[test]
    fn rejects_duplicate_issuer_parameter() {
        let request = format!(
            "GET {CALLBACK_PATH}?state=state123&iss=https://accounts.google.com&iss=https://accounts.google.com&code=abc123 HTTP/1.1\r\n\r\n"
        );
        assert_eq!(
            parse_callback_request(&request, "state123").unwrap_err(),
            SearchConsoleError::InvalidCallback
        );
    }

    #[test]
    fn rejects_empty_issuer_parameter() {
        let request =
            format!("GET {CALLBACK_PATH}?state=state123&iss=&code=abc123 HTTP/1.1\r\n\r\n");
        assert_eq!(
            parse_callback_request(&request, "state123").unwrap_err(),
            SearchConsoleError::InvalidCallback
        );
    }

    #[test]
    fn rejects_wrong_issuer_parameter() {
        let request = format!(
            "GET {CALLBACK_PATH}?state=state123&iss=https://example.invalid&code=abc123 HTTP/1.1\r\n\r\n"
        );
        assert_eq!(
            parse_callback_request(&request, "state123").unwrap_err(),
            SearchConsoleError::InvalidCallback
        );
    }

    #[test]
    fn parses_successful_callback_without_issuer() {
        let request = format!("GET {CALLBACK_PATH}?state=state123&code=abc123 HTTP/1.1\r\n\r\n");
        assert_eq!(
            parse_callback_request(&request, "state123").unwrap(),
            CallbackOutcome::Authorized {
                code: "abc123".to_string()
            }
        );
    }

    #[test]
    fn parses_callback_with_parameters_in_any_order() {
        for request in [
            format!(
                "GET {CALLBACK_PATH}?state=state123&iss=https://accounts.google.com&code=abc123 HTTP/1.1\r\n\r\n"
            ),
            format!(
                "GET {CALLBACK_PATH}?code=abc123&state=state123&iss=https://accounts.google.com HTTP/1.1\r\n\r\n"
            ),
            format!(
                "GET {CALLBACK_PATH}?iss=https://accounts.google.com&code=abc123&state=state123 HTTP/1.1\r\n\r\n"
            ),
        ] {
            assert_eq!(
                parse_callback_request(&request, "state123").unwrap(),
                CallbackOutcome::Authorized {
                    code: "abc123".to_string()
                }
            );
        }
    }

    #[test]
    fn issuer_callback_errors_do_not_include_raw_values() {
        let request = format!(
            "GET {CALLBACK_PATH}?state=secret-state&iss=https://example.invalid&code=secret-code HTTP/1.1\r\n\r\n"
        );
        let error = parse_callback_request(&request, "secret-state").unwrap_err();
        assert_eq!(error, SearchConsoleError::InvalidCallback);

        let error_json = serde_json::to_string(&SearchConsoleCommandError::from(error)).unwrap();
        assert!(!error_json.contains("secret-state"));
        assert!(!error_json.contains("secret-code"));
        assert!(!error_json.contains("example.invalid"));
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
    fn rejects_legacy_long_callback_path() {
        let request =
            "GET /search-console/oauth/callback?code=abc123&state=state123 HTTP/1.1\r\n\r\n";
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
    fn loopback_listener_accepts_callback_and_returns_http_200() {
        let listener = bind_callback_listener().unwrap();
        let port = listener.local_addr().unwrap().port();
        let handle = thread::spawn(move || {
            wait_for_callback(
                listener,
                "state123".to_string(),
                Duration::from_secs(2),
                callback_cancel_flag(),
            )
        });

        let response = send_callback_request(port, successful_callback_request("state123"));
        assert!(response.starts_with("HTTP/1.1 200 OK"));
        assert!(response.contains("Korea Inside Admin"));

        assert_eq!(
            handle.join().unwrap().unwrap(),
            CallbackOutcome::Authorized {
                code: "dummy-code".to_string()
            }
        );
    }

    #[test]
    fn callback_success_html_does_not_claim_connection_complete() {
        let body = callback_response_body(true);

        assert!(body.contains("Google 인증 응답을 받았습니다."));
        assert!(body.contains("관리자 앱에서 연결 결과를 확인하세요."));
        assert!(!body.contains("연결이 완료되었습니다."));
    }

    #[test]
    fn listener_remains_alive_until_callback_waiter_is_awaited() {
        let listener = bind_callback_listener().unwrap();
        let port = listener.local_addr().unwrap().port();
        let handle = thread::spawn(move || {
            wait_for_callback(
                listener,
                "state123".to_string(),
                Duration::from_secs(2),
                callback_cancel_flag(),
            )
        });

        let response = send_callback_request(port, successful_callback_request("state123"));

        assert!(response.starts_with("HTTP/1.1 200 OK"));
        assert!(handle.join().unwrap().is_ok());
    }

    #[test]
    fn listener_ignores_invalid_loopback_request_until_valid_callback() {
        let listener = bind_callback_listener().unwrap();
        let port = listener.local_addr().unwrap().port();
        let handle = thread::spawn(move || {
            wait_for_callback(
                listener,
                "state123".to_string(),
                Duration::from_secs(2),
                callback_cancel_flag(),
            )
        });

        let invalid_response = send_callback_request(
            port,
            "GET /favicon.ico HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n".to_string(),
        );
        assert!(invalid_response.starts_with("HTTP/1.1 200 OK"));
        let valid_response = send_callback_request(port, successful_callback_request("state123"));
        assert!(valid_response.starts_with("HTTP/1.1 200 OK"));

        assert_eq!(
            handle.join().unwrap().unwrap(),
            CallbackOutcome::Authorized {
                code: "dummy-code".to_string()
            }
        );
    }

    #[test]
    fn callback_waiter_times_out_and_closes_listener() {
        let listener = bind_callback_listener().unwrap();
        let port = listener.local_addr().unwrap().port();

        assert_eq!(
            wait_for_callback(
                listener,
                "state123".to_string(),
                Duration::from_millis(20),
                callback_cancel_flag(),
            )
            .unwrap_err(),
            SearchConsoleError::CallbackTimeout
        );
        assert!(TcpStream::connect((Ipv4Addr::LOCALHOST, port)).is_err());
    }

    #[test]
    fn browser_open_failure_cancel_closes_listener() {
        let listener = bind_callback_listener().unwrap();
        let port = listener.local_addr().unwrap().port();
        let cancel = callback_cancel_flag();
        let waiter_cancel = Arc::clone(&cancel);
        let handle = thread::spawn(move || {
            wait_for_callback(
                listener,
                "state123".to_string(),
                Duration::from_secs(2),
                waiter_cancel,
            )
        });

        cancel.store(true, Ordering::SeqCst);

        assert_eq!(
            handle.join().unwrap().unwrap_err(),
            SearchConsoleError::Internal
        );
        assert!(TcpStream::connect((Ipv4Addr::LOCALHOST, port)).is_err());
    }

    #[test]
    fn callback_success_releases_authentication_guard() {
        let _lock = runtime_test_lock();
        reset_runtime_state_for_test();
        let result: Result<(), SearchConsoleError> = (|| {
            let _guard = OperationGuard::begin(OperationKind::Authentication)?;
            let listener = bind_callback_listener()?;
            let port = listener.local_addr().unwrap().port();
            let handle = thread::spawn(move || {
                wait_for_callback(
                    listener,
                    "state123".to_string(),
                    Duration::from_secs(2),
                    callback_cancel_flag(),
                )
            });
            let response = send_callback_request(port, successful_callback_request("state123"));
            assert!(response.is_empty() || response.starts_with("HTTP/1.1 200 OK"));
            handle.join().unwrap()?;
            Ok(())
        })();

        assert!(result.is_ok());
        assert!(OperationGuard::begin(OperationKind::ClientConfiguration).is_ok());
        reset_runtime_state_for_test();
    }

    #[test]
    fn callback_error_releases_authentication_guard() {
        let _lock = runtime_test_lock();
        reset_runtime_state_for_test();
        let result: Result<(), SearchConsoleError> = (|| {
            let _guard = OperationGuard::begin(OperationKind::Authentication)?;
            let listener = bind_callback_listener()?;
            let port = listener.local_addr().unwrap().port();
            let handle = thread::spawn(move || {
                wait_for_callback(
                    listener,
                    "state123".to_string(),
                    Duration::from_secs(2),
                    callback_cancel_flag(),
                )
            });
            let response = send_callback_request(port, successful_callback_request("wrong-state"));
            assert!(response.is_empty() || response.starts_with("HTTP/1.1 200 OK"));
            assert_eq!(
                handle.join().unwrap().unwrap_err(),
                SearchConsoleError::StateMismatch
            );
            Err(SearchConsoleError::StateMismatch)
        })();

        assert_eq!(result.err(), Some(SearchConsoleError::StateMismatch));
        assert!(OperationGuard::begin(OperationKind::ClientConfiguration).is_ok());
        reset_runtime_state_for_test();
    }

    #[test]
    fn command_initial_error_releases_authentication_guard() {
        let _lock = runtime_test_lock();
        reset_runtime_state_for_test();
        let result: Result<(), SearchConsoleError> = (|| {
            let _guard = OperationGuard::begin(OperationKind::Authentication)?;
            Err(SearchConsoleError::NotConfigured)
        })();

        assert_eq!(result.err(), Some(SearchConsoleError::NotConfigured));
        assert!(OperationGuard::begin(OperationKind::ClientConfiguration).is_ok());
        reset_runtime_state_for_test();
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
            client_secret_stored: true,
            authorization_stored: true,
            connected: true,
            authentication_in_progress: false,
            reauthentication_required: false,
            last_checked_at: Some("2026-07-13T00:00:00Z".to_string()),
        };
        let status_json = serde_json::to_string(&status).unwrap();
        assert!(!status_json.contains("clientId"));
        assert!(status_json.contains("\"clientSecretStored\":true"));
        assert!(!status_json.contains(&test_client_secret()));
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
    fn maps_authorization_token_error_codes_safely() {
        for (body, expected) in [
            (
                br#"{"error":"invalid_grant","error_description":"hidden"}"#.as_slice(),
                SearchConsoleError::TokenInvalidGrant,
            ),
            (
                br#"{"error":"invalid_client","error_description":"hidden"}"#.as_slice(),
                SearchConsoleError::TokenInvalidClient,
            ),
            (
                br#"{"error":"invalid_request","error_description":"hidden"}"#.as_slice(),
                SearchConsoleError::TokenInvalidRequest,
            ),
            (
                br#"{"error":"unauthorized_client","error_description":"hidden"}"#.as_slice(),
                SearchConsoleError::TokenUnauthorizedClient,
            ),
            (
                br#"{"error":"redirect_uri_mismatch","error_description":"hidden"}"#.as_slice(),
                SearchConsoleError::TokenRedirectUriMismatch,
            ),
        ] {
            assert_eq!(map_authorization_token_error(body), expected);
        }
    }

    #[test]
    fn unknown_authorization_token_error_uses_general_mapping() {
        for body in [
            br#"{"error":"temporarily_unavailable","error_description":"hidden"}"#.as_slice(),
            br#"{"error_description":"hidden"}"#.as_slice(),
            b"not-json".as_slice(),
        ] {
            assert_eq!(
                map_authorization_token_error(body),
                SearchConsoleError::TokenExchangeFailed
            );
        }
    }

    #[test]
    fn token_error_mapping_does_not_expose_description_or_body() {
        let error = map_authorization_token_error(
            br#"{"error":"invalid_grant","error_description":"secret-description"}"#,
        );
        let error_json = serde_json::to_string(&SearchConsoleCommandError::from(error)).unwrap();

        assert_eq!(
            SearchConsoleCommandError::from(error).code,
            "token_invalid_grant"
        );
        assert!(!error_json.contains("secret-description"));
        assert!(!error_json.contains("error_description"));
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
        assert_ne!(CLIENT_ID_ACCOUNT, CLIENT_SECRET_ACCOUNT);
        assert_ne!(CLIENT_SECRET_ACCOUNT, REFRESH_TOKEN_ACCOUNT);
    }

    #[test]
    fn locked_client_id_is_not_reported_as_configured() {
        let mut backend = credential_backend_for_client(OAUTH_CREDENTIAL_UPDATE_LOCK);
        let runtime = FakeRuntimeBackend::restarted();

        assert_eq!(
            validate_client_id(OAUTH_CREDENTIAL_UPDATE_LOCK).unwrap_err(),
            SearchConsoleError::InvalidClientId
        );
        let status = client_status_from_fake_state(&backend, &runtime);
        assert!(!status.configured);
        assert!(status.client_secret_stored);
        assert!(status.authorization_stored);
        assert!(!status.connected);
        assert!(!serde_json::to_string(&status)
            .unwrap()
            .contains(OAUTH_CREDENTIAL_UPDATE_LOCK));
        assert_oauth_entry_points_blocked(&mut backend);
    }

    #[test]
    fn locked_client_id_can_be_recovered_by_manual_save() {
        let mut backend = credential_backend_for_client(OAUTH_CREDENTIAL_UPDATE_LOCK);
        let mut runtime = FakeRuntimeBackend::restarted();

        save_search_console_client_id_with(
            &mut backend,
            &mut runtime,
            VALID_CLIENT_ID.to_string(),
            dummy_status,
        )
        .unwrap();

        assert_eq!(
            backend.values.get(CLIENT_ID_ACCOUNT).map(String::as_str),
            Some(VALID_CLIENT_ID)
        );
        assert!(!backend.values.contains_key(CLIENT_SECRET_ACCOUNT));
        assert!(!backend.values.contains_key(REFRESH_TOKEN_ACCOUNT));
    }

    #[test]
    fn locked_client_id_can_be_recovered_by_oauth_import() {
        let mut backend = credential_backend_for_client(OAUTH_CREDENTIAL_UPDATE_LOCK);
        let mut runtime = FakeRuntimeBackend::restarted();
        let oauth_client = imported_oauth_client_fixture();

        assert!(
            import_oauth_client_credentials_with(&mut backend, &mut runtime, &oauth_client)
                .unwrap()
        );

        assert_eq!(
            backend.values.get(CLIENT_ID_ACCOUNT),
            Some(&oauth_client.client_id)
        );
        assert_eq!(
            backend.values.get(CLIENT_SECRET_ACCOUNT),
            Some(&oauth_client.client_secret)
        );
        assert!(!backend.values.contains_key(REFRESH_TOKEN_ACCOUNT));
    }

    #[test]
    fn same_client_id_import_updates_secret_and_preserves_refresh_token() {
        let refresh_token = ["fixture", "refresh", "token"].join("-");
        let mut backend = FakeCredentialBackend::default();
        backend
            .values
            .insert(CLIENT_ID_ACCOUNT.to_string(), VALID_CLIENT_ID.to_string());
        backend.values.insert(
            CLIENT_SECRET_ACCOUNT.to_string(),
            ["old", "fixture", "secret"].join("-"),
        );
        backend
            .values
            .insert(REFRESH_TOKEN_ACCOUNT.to_string(), refresh_token.clone());
        let oauth_client = ImportedOAuthClient {
            client_id: VALID_CLIENT_ID.to_string(),
            client_secret: test_client_secret(),
        };

        assert!(!import_oauth_client_credentials_for_test(&mut backend, &oauth_client).unwrap());
        assert_eq!(
            backend.values.get(REFRESH_TOKEN_ACCOUNT),
            Some(&refresh_token)
        );
        assert_eq!(
            backend.values.get(CLIENT_SECRET_ACCOUNT),
            Some(&oauth_client.client_secret)
        );
        assert!(!backend
            .operations
            .contains(&format!("delete:{REFRESH_TOKEN_ACCOUNT}")));
    }

    #[test]
    fn different_client_id_import_replaces_pair_and_deletes_refresh_token() {
        let mut backend = FakeCredentialBackend::default();
        let operations = Rc::new(RefCell::new(Vec::new()));
        backend.record_operations_with(Rc::clone(&operations));
        let mut runtime = FakeRuntimeBackend::connected();
        runtime.record_operations_with(Rc::clone(&operations));
        backend.values.insert(
            CLIENT_ID_ACCOUNT.to_string(),
            "other-client.apps.googleusercontent.com".to_string(),
        );
        backend.values.insert(
            CLIENT_SECRET_ACCOUNT.to_string(),
            ["old", "fixture", "secret"].join("-"),
        );
        backend.values.insert(
            REFRESH_TOKEN_ACCOUNT.to_string(),
            ["fixture", "refresh", "token"].join("-"),
        );
        let oauth_client = ImportedOAuthClient {
            client_id: VALID_CLIENT_ID.to_string(),
            client_secret: test_client_secret(),
        };

        assert!(
            import_oauth_client_credentials_with(&mut backend, &mut runtime, &oauth_client)
                .unwrap()
        );
        assert_eq!(
            operations.borrow().clone(),
            vec![
                "snapshot:runtime".to_string(),
                format!("save:{CLIENT_ID_ACCOUNT}"),
                format!("save:{CLIENT_SECRET_ACCOUNT}"),
                format!("delete:{REFRESH_TOKEN_ACCOUNT}"),
                "clear:runtime".to_string(),
                format!("save:{CLIENT_ID_ACCOUNT}"),
            ]
        );
        assert_eq!(
            backend.values.get(CLIENT_ID_ACCOUNT),
            Some(&oauth_client.client_id)
        );
        assert_eq!(
            backend.values.get(CLIENT_SECRET_ACCOUNT),
            Some(&oauth_client.client_secret)
        );
        assert!(!backend.values.contains_key(REFRESH_TOKEN_ACCOUNT));
    }

    #[test]
    fn failed_refresh_token_delete_import_restores_original_credentials() {
        let mut backend = original_credential_backend();
        backend.fail_delete_on_attempt(REFRESH_TOKEN_ACCOUNT, 1);
        let oauth_client = imported_oauth_client_fixture();

        assert_eq!(
            import_oauth_client_credentials_for_test(&mut backend, &oauth_client).unwrap_err(),
            SearchConsoleError::CredentialDeleteFailed
        );
        assert_original_credentials(&backend);
    }

    #[test]
    fn failed_client_id_save_import_restores_original_credentials() {
        let mut backend = original_credential_backend();
        backend.fail_save_on_attempt(CLIENT_ID_ACCOUNT, 2);

        assert_eq!(
            import_oauth_client_credentials_for_test(
                &mut backend,
                &imported_oauth_client_fixture(),
            )
            .unwrap_err(),
            SearchConsoleError::CredentialStoreFailed
        );
        assert_original_credentials(&backend);
    }

    #[test]
    fn failed_update_lock_save_import_preserves_credentials_and_runtime() {
        let mut backend = original_credential_backend();
        backend.fail_save_on_attempt(CLIENT_ID_ACCOUNT, 1);
        let mut runtime = FakeRuntimeBackend::connected();
        let original_runtime = runtime.state.clone();

        assert_eq!(
            import_oauth_client_credentials_with(
                &mut backend,
                &mut runtime,
                &imported_oauth_client_fixture(),
            )
            .unwrap_err(),
            SearchConsoleError::CredentialStoreFailed
        );

        assert_original_credentials(&backend);
        assert_runtime_matches(&runtime.state, &original_runtime);
        assert_eq!(backend.operations, [format!("save:{CLIENT_ID_ACCOUNT}")]);
        assert_eq!(runtime.snapshot_calls, 1);
        assert_eq!(runtime.clear_calls, 0);
        assert_eq!(runtime.restore_calls, 0);
    }

    #[test]
    fn failed_client_secret_save_import_restores_original_credentials() {
        let mut backend = original_credential_backend();
        backend.fail_save_on_attempt(CLIENT_SECRET_ACCOUNT, 1);

        assert_eq!(
            import_oauth_client_credentials_for_test(
                &mut backend,
                &imported_oauth_client_fixture(),
            )
            .unwrap_err(),
            SearchConsoleError::CredentialStoreFailed
        );
        assert_original_credentials(&backend);
    }

    #[test]
    fn failed_client_id_restore_after_import_failure_cleans_credentials() {
        let mut backend = original_credential_backend();
        backend.fail_save_on_attempt(CLIENT_SECRET_ACCOUNT, 1);
        backend.fail_save_on_attempt(CLIENT_ID_ACCOUNT, 2);

        assert!(import_oauth_client_credentials_for_test(
            &mut backend,
            &imported_oauth_client_fixture()
        )
        .is_err());
        assert_only_update_lock_is_stored(&backend);
    }

    #[test]
    fn failed_client_secret_restore_after_import_failure_cleans_credentials() {
        let mut backend = original_credential_backend();
        backend.fail_delete_on_attempt(REFRESH_TOKEN_ACCOUNT, 1);
        backend.fail_save_on_attempt(CLIENT_SECRET_ACCOUNT, 2);

        assert!(import_oauth_client_credentials_for_test(
            &mut backend,
            &imported_oauth_client_fixture()
        )
        .is_err());
        assert_only_update_lock_is_stored(&backend);
    }

    #[test]
    fn failed_refresh_token_restore_after_import_failure_cleans_credentials() {
        let mut backend = original_credential_backend();
        backend.fail_save_on_attempt(CLIENT_SECRET_ACCOUNT, 1);
        backend.fail_save_on_attempt(REFRESH_TOKEN_ACCOUNT, 1);

        assert!(import_oauth_client_credentials_for_test(
            &mut backend,
            &imported_oauth_client_fixture()
        )
        .is_err());
        assert_only_update_lock_is_stored(&backend);
    }

    #[test]
    fn import_cleanup_retries_every_one_shot_delete_failure_combination() {
        for failed_accounts in [
            vec![REFRESH_TOKEN_ACCOUNT],
            vec![CLIENT_SECRET_ACCOUNT],
            vec![REFRESH_TOKEN_ACCOUNT, CLIENT_SECRET_ACCOUNT],
        ] {
            let mut backend = original_credential_backend();
            backend.fail_save_on_attempt(CLIENT_SECRET_ACCOUNT, 1);
            backend.fail_save_on_attempt(CLIENT_ID_ACCOUNT, 2);
            for account in failed_accounts {
                backend.fail_delete_on_attempt(account, 1);
            }

            assert!(import_oauth_client_credentials_for_test(
                &mut backend,
                &imported_oauth_client_fixture()
            )
            .is_err());
            assert_only_update_lock_is_stored(&backend);
        }
    }

    #[test]
    fn failed_import_rollback_and_persistent_cleanup_remains_locked_after_restart() {
        let mut backend = original_credential_backend();
        backend.fail_delete_from_attempt(REFRESH_TOKEN_ACCOUNT, 1);
        backend.fail_save_from_attempt(CLIENT_SECRET_ACCOUNT, 2);
        backend.fail_delete_from_attempt(CLIENT_SECRET_ACCOUNT, 1);
        let mut runtime = FakeRuntimeBackend::connected();
        let error = import_oauth_client_credentials_with(
            &mut backend,
            &mut runtime,
            &imported_oauth_client_fixture(),
        )
        .unwrap_err();

        assert_eq!(error, SearchConsoleError::CredentialDeleteFailed);
        assert_update_lock_is_stored(&backend);
        assert!(backend.values.contains_key(CLIENT_SECRET_ACCOUNT));
        assert!(backend.values.contains_key(REFRESH_TOKEN_ACCOUNT));
        assert!(backend.attempt_count("delete", REFRESH_TOKEN_ACCOUNT) >= 3);
        assert!(backend.attempt_count("delete", CLIENT_SECRET_ACCOUNT) >= 2);

        drop(runtime);
        let restarted_runtime = FakeRuntimeBackend::restarted();
        let status = client_status_from_fake_state(&backend, &restarted_runtime);
        assert!(!status.configured);
        assert!(!status.connected);
        assert_oauth_entry_points_blocked(&mut backend);

        let serialized = serde_json::to_string(&SearchConsoleCommandError::from(error)).unwrap();
        for sensitive_fixture in [
            original_client_id(),
            original_client_secret(),
            original_refresh_token(),
            test_client_secret(),
        ] {
            assert!(!serialized.contains(&sensitive_fixture));
        }
    }

    #[test]
    fn persistent_cleanup_failure_blocks_configured_status() {
        let mut backend = credential_backend_for_client(OAUTH_CREDENTIAL_UPDATE_LOCK);
        backend.fail_delete_from_attempt(CLIENT_SECRET_ACCOUNT, 1);
        backend.fail_delete_from_attempt(REFRESH_TOKEN_ACCOUNT, 1);

        delete_oauth_credential_fragments_best_effort(&mut backend);

        assert_update_lock_is_stored(&backend);
        assert_eq!(backend.attempt_count("delete", CLIENT_SECRET_ACCOUNT), 2);
        assert_eq!(backend.attempt_count("delete", REFRESH_TOKEN_ACCOUNT), 2);
        let restarted_runtime = FakeRuntimeBackend::restarted();
        let status = client_status_from_fake_state(&backend, &restarted_runtime);
        assert!(!status.configured);
        assert!(!status.connected);
    }

    #[test]
    fn oauth_import_result_exposes_only_non_sensitive_state() {
        let json =
            serde_json::to_value(SearchConsoleOAuthImportResult::imported(false, false)).unwrap();
        let object = json.as_object().unwrap();

        assert_eq!(
            object.keys().cloned().collect::<HashSet<_>>(),
            HashSet::from([
                "status".to_string(),
                "clientIdChanged".to_string(),
                "clientSecretStored".to_string(),
                "reauthenticationRequired".to_string(),
            ])
        );
        assert_eq!(
            object.get("clientSecretStored"),
            Some(&serde_json::json!(true))
        );
        assert!(!json.to_string().contains(&test_client_secret()));
    }

    #[test]
    fn same_manual_client_id_preserves_credentials_and_runtime_state() {
        let mut backend = credential_backend_for_client(VALID_CLIENT_ID);
        let mut runtime = FakeRuntimeBackend::connected();
        let original_runtime = runtime.state.clone();

        save_search_console_client_id_with(
            &mut backend,
            &mut runtime,
            VALID_CLIENT_ID.to_string(),
            dummy_status,
        )
        .unwrap();
        assert_credentials_for_client(&backend, VALID_CLIENT_ID);
        assert!(backend.operations.is_empty());
        assert_eq!(runtime.snapshot_calls, 0);
        assert_eq!(runtime.clear_calls, 0);
        assert_eq!(runtime.restore_calls, 0);
        assert_runtime_matches(&runtime.state, &original_runtime);
    }

    #[test]
    fn invalid_manual_client_id_does_not_modify_credentials_or_runtime() {
        let mut backend = original_credential_backend();
        let mut runtime = FakeRuntimeBackend::connected();
        let original_runtime = runtime.state.clone();

        assert_eq!(
            save_search_console_client_id_with(
                &mut backend,
                &mut runtime,
                "invalid-client-id".to_string(),
                dummy_status,
            )
            .err()
            .unwrap(),
            SearchConsoleError::InvalidClientId
        );
        assert_original_credentials(&backend);
        assert!(backend.operations.is_empty());
        assert_eq!(runtime.snapshot_calls, 0);
        assert_eq!(runtime.clear_calls, 0);
        assert_eq!(runtime.restore_calls, 0);
        assert_runtime_matches(&runtime.state, &original_runtime);
    }

    #[test]
    fn different_manual_client_id_succeeds_only_after_old_credentials_are_removed() {
        let mut backend = original_credential_backend();
        let operations = Rc::new(RefCell::new(Vec::new()));
        backend.record_operations_with(Rc::clone(&operations));
        let mut runtime = FakeRuntimeBackend::connected();
        runtime.record_operations_with(Rc::clone(&operations));

        save_search_console_client_id_with(
            &mut backend,
            &mut runtime,
            VALID_CLIENT_ID.to_string(),
            dummy_status,
        )
        .unwrap();

        assert_eq!(
            operations.borrow().clone(),
            vec![
                "snapshot:runtime".to_string(),
                format!("save:{CLIENT_ID_ACCOUNT}"),
                format!("delete:{REFRESH_TOKEN_ACCOUNT}"),
                format!("delete:{CLIENT_SECRET_ACCOUNT}"),
                "clear:runtime".to_string(),
                format!("save:{CLIENT_ID_ACCOUNT}"),
            ]
        );
        assert_eq!(
            backend.values.get(CLIENT_ID_ACCOUNT).map(String::as_str),
            Some(VALID_CLIENT_ID)
        );
        assert!(!backend.values.contains_key(CLIENT_SECRET_ACCOUNT));
        assert!(!backend.values.contains_key(REFRESH_TOKEN_ACCOUNT));
        assert_runtime_fail_closed(&runtime.state);
        assert_eq!(runtime.snapshot_calls, 1);
        assert_eq!(runtime.clear_calls, 1);
        assert_eq!(runtime.restore_calls, 0);
    }

    #[test]
    fn failed_manual_refresh_token_delete_restores_credentials_and_runtime() {
        let mut backend = original_credential_backend();
        backend.fail_delete_on_attempt(REFRESH_TOKEN_ACCOUNT, 1);
        let mut runtime = FakeRuntimeBackend::connected();
        let original_runtime = runtime.state.clone();

        let result = save_search_console_client_id_with(
            &mut backend,
            &mut runtime,
            VALID_CLIENT_ID.to_string(),
            dummy_status,
        );
        assert_eq!(
            result.err(),
            Some(SearchConsoleError::CredentialDeleteFailed)
        );
        assert_original_credentials(&backend);
        assert_runtime_matches(&runtime.state, &original_runtime);
        assert_eq!(runtime.restore_calls, 1);
    }

    #[test]
    fn failed_manual_client_secret_delete_restores_credentials_and_runtime() {
        let mut backend = original_credential_backend();
        backend.fail_delete_on_attempt(CLIENT_SECRET_ACCOUNT, 1);
        let mut runtime = FakeRuntimeBackend::connected();
        let original_runtime = runtime.state.clone();

        let result = save_search_console_client_id_with(
            &mut backend,
            &mut runtime,
            VALID_CLIENT_ID.to_string(),
            dummy_status,
        );
        assert_eq!(
            result.err(),
            Some(SearchConsoleError::CredentialDeleteFailed)
        );
        assert_original_credentials(&backend);
        assert_runtime_matches(&runtime.state, &original_runtime);
        assert_eq!(runtime.restore_calls, 1);
    }

    #[test]
    fn failed_manual_client_id_save_restores_credentials_and_runtime() {
        let mut backend = original_credential_backend();
        backend.fail_save_on_attempt(CLIENT_ID_ACCOUNT, 2);
        let mut runtime = FakeRuntimeBackend::connected();
        let original_runtime = runtime.state.clone();

        let result = save_search_console_client_id_with(
            &mut backend,
            &mut runtime,
            VALID_CLIENT_ID.to_string(),
            dummy_status,
        );
        assert_eq!(
            result.err(),
            Some(SearchConsoleError::CredentialStoreFailed)
        );
        assert_original_credentials(&backend);
        assert_runtime_matches(&runtime.state, &original_runtime);
        assert_eq!(runtime.restore_calls, 1);
    }

    #[test]
    fn failed_manual_update_lock_save_preserves_credentials_and_runtime() {
        let mut backend = original_credential_backend();
        backend.fail_save_on_attempt(CLIENT_ID_ACCOUNT, 1);
        let mut runtime = FakeRuntimeBackend::connected();
        let original_runtime = runtime.state.clone();

        assert_eq!(
            save_search_console_client_id_with(
                &mut backend,
                &mut runtime,
                VALID_CLIENT_ID.to_string(),
                dummy_status,
            )
            .err()
            .unwrap(),
            SearchConsoleError::CredentialStoreFailed
        );

        assert_original_credentials(&backend);
        assert_runtime_matches(&runtime.state, &original_runtime);
        assert_eq!(backend.operations, [format!("save:{CLIENT_ID_ACCOUNT}")]);
        assert_eq!(runtime.snapshot_calls, 1);
        assert_eq!(runtime.clear_calls, 0);
        assert_eq!(runtime.restore_calls, 0);
    }

    #[test]
    fn failed_manual_runtime_clear_restores_credentials_and_runtime() {
        let mut backend = original_credential_backend();
        let mut runtime = FakeRuntimeBackend::connected();
        runtime.fail_clear_on_attempt(1);
        let original_runtime = runtime.state.clone();

        let result = save_search_console_client_id_with(
            &mut backend,
            &mut runtime,
            VALID_CLIENT_ID.to_string(),
            dummy_status,
        );
        assert_eq!(result.err(), Some(SearchConsoleError::Internal));
        assert_original_credentials(&backend);
        assert_runtime_matches(&runtime.state, &original_runtime);
        assert_eq!(runtime.restore_calls, 1);
    }

    #[test]
    fn failed_manual_rollback_returns_error_and_enters_fail_closed_state() {
        let mut backend = original_credential_backend();
        backend.fail_delete_on_attempt(CLIENT_SECRET_ACCOUNT, 1);
        backend.fail_save_on_attempt(REFRESH_TOKEN_ACCOUNT, 1);
        let mut runtime = FakeRuntimeBackend::connected();

        let error = save_search_console_client_id_with(
            &mut backend,
            &mut runtime,
            VALID_CLIENT_ID.to_string(),
            dummy_status,
        )
        .err()
        .unwrap();

        assert_eq!(error, SearchConsoleError::CredentialDeleteFailed);
        assert_only_update_lock_is_stored(&backend);
        assert_runtime_fail_closed(&runtime.state);
        assert_eq!(runtime.restore_calls, 1);
        assert_eq!(runtime.clear_calls, 1);
        let serialized = serde_json::to_string(&SearchConsoleCommandError::from(error)).unwrap();
        for sensitive_fixture in [
            original_client_id(),
            original_client_secret(),
            original_refresh_token(),
        ] {
            assert!(!serialized.contains(&sensitive_fixture));
        }
    }

    #[test]
    fn failed_manual_runtime_restore_cleans_credentials_and_runtime() {
        let mut backend = original_credential_backend();
        backend.fail_delete_on_attempt(REFRESH_TOKEN_ACCOUNT, 1);
        let mut runtime = FakeRuntimeBackend::connected();
        runtime.fail_restore_on_attempt(1);

        assert!(save_search_console_client_id_with(
            &mut backend,
            &mut runtime,
            VALID_CLIENT_ID.to_string(),
            dummy_status,
        )
        .is_err());
        assert_only_update_lock_is_stored(&backend);
        assert_runtime_fail_closed(&runtime.state);
        assert_eq!(runtime.restore_calls, 1);
        assert_eq!(runtime.clear_calls, 1);
    }

    #[test]
    fn failed_manual_rollback_and_persistent_cleanup_remains_locked_after_restart() {
        let mut backend = original_credential_backend();
        backend.fail_delete_from_attempt(CLIENT_SECRET_ACCOUNT, 1);
        backend.fail_save_from_attempt(REFRESH_TOKEN_ACCOUNT, 1);
        backend.fail_delete_from_attempt(REFRESH_TOKEN_ACCOUNT, 2);
        let mut runtime = FakeRuntimeBackend::connected();
        let error = save_search_console_client_id_with(
            &mut backend,
            &mut runtime,
            VALID_CLIENT_ID.to_string(),
            dummy_status,
        )
        .err()
        .unwrap();

        assert_eq!(error, SearchConsoleError::CredentialDeleteFailed);
        assert_update_lock_is_stored(&backend);
        assert!(backend.values.contains_key(CLIENT_SECRET_ACCOUNT));
        assert!(backend.attempt_count("delete", CLIENT_SECRET_ACCOUNT) >= 3);
        assert!(backend.attempt_count("delete", REFRESH_TOKEN_ACCOUNT) >= 3);

        drop(runtime);
        let restarted_runtime = FakeRuntimeBackend::restarted();
        let status = client_status_from_fake_state(&backend, &restarted_runtime);
        assert!(!status.configured);
        assert!(!status.connected);
        assert_oauth_entry_points_blocked(&mut backend);

        let serialized = serde_json::to_string(&SearchConsoleCommandError::from(error)).unwrap();
        for sensitive_fixture in [
            original_client_id(),
            original_client_secret(),
            original_refresh_token(),
        ] {
            assert!(!serialized.contains(&sensitive_fixture));
        }
    }

    #[test]
    fn persistent_failure_blocks_oauth_request_after_restart() {
        let mut backend = credential_backend_for_client(OAUTH_CREDENTIAL_UPDATE_LOCK);
        let restarted_runtime = FakeRuntimeBackend::restarted();
        let status = client_status_from_fake_state(&backend, &restarted_runtime);

        assert!(!status.configured);
        assert!(!status.connected);
        assert_oauth_entry_points_blocked(&mut backend);
    }

    #[test]
    fn client_id_delete_removes_refresh_secret_and_client_id() {
        let events = Rc::new(RefCell::new(Vec::new()));
        let delete_refresh_events = Rc::clone(&events);
        let delete_secret_events = Rc::clone(&events);
        let delete_client_events = Rc::clone(&events);
        let clear_events = Rc::clone(&events);
        delete_search_console_client_id_with(
            || {
                delete_refresh_events.borrow_mut().push("delete_refresh");
                Ok(())
            },
            || {
                delete_secret_events
                    .borrow_mut()
                    .push("delete_client_secret");
                Ok(())
            },
            || {
                delete_client_events.borrow_mut().push("delete_client_id");
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
            [
                "delete_refresh",
                "delete_client_secret",
                "delete_client_id",
                "clear_runtime"
            ]
        );
    }

    #[test]
    fn failed_refresh_delete_still_attempts_secret_and_client_id_delete() {
        let events = Rc::new(RefCell::new(Vec::new()));
        let delete_secret_events = Rc::clone(&events);
        let delete_client_events = Rc::clone(&events);
        let clear_events = Rc::clone(&events);
        let result = delete_search_console_client_id_with(
            || Err(SearchConsoleError::CredentialDeleteFailed),
            || {
                delete_secret_events
                    .borrow_mut()
                    .push("delete_client_secret");
                Ok(())
            },
            || {
                delete_client_events.borrow_mut().push("delete_client_id");
                Ok(())
            },
            || {
                clear_events.borrow_mut().push("clear_runtime");
                Ok(())
            },
            dummy_status,
        );
        assert_eq!(
            result.err(),
            Some(SearchConsoleError::CredentialDeleteFailed)
        );
        assert_eq!(
            events.borrow().as_slice(),
            ["delete_client_secret", "delete_client_id", "clear_runtime"]
        );
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
        let status = client_status_from_parts(true, true, true).unwrap();
        assert!(status.connected);
        assert!(status.last_checked_at.is_some());
        reset_runtime_state_for_test();
    }

    #[test]
    fn stored_authorization_alone_is_not_connected_after_app_start() {
        let _lock = runtime_test_lock();
        reset_runtime_state_for_test();
        let status = client_status_from_parts(true, true, true).unwrap();
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
            client_secret_stored: true,
            authorization_stored: true,
            connected: false,
            authentication_in_progress: false,
            reauthentication_required: false,
            last_checked_at: None,
        })
    }

    fn callback_cancel_flag() -> Arc<AtomicBool> {
        Arc::new(AtomicBool::new(false))
    }

    fn successful_callback_request(state: &str) -> String {
        format!(
            "GET {CALLBACK_PATH}?code=dummy-code&state={state} HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n"
        )
    }

    fn send_callback_request(port: u16, request: String) -> String {
        let mut stream = TcpStream::connect((Ipv4Addr::LOCALHOST, port)).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        stream.write_all(request.as_bytes()).unwrap();
        let _ = stream.shutdown(std::net::Shutdown::Write);
        let mut response = String::new();
        if let Err(error) = stream.read_to_string(&mut response) {
            if error.raw_os_error() != Some(10054) {
                panic!("failed to read callback response: {error}");
            }
        }
        response
    }

    fn parse_form_body_for_test(body: &str) -> HashMap<String, String> {
        form_urlencoded::parse(body.as_bytes())
            .map(|(key, value)| (key.into_owned(), value.into_owned()))
            .collect()
    }

    fn parse_form_keys_for_test(body: &str) -> Vec<String> {
        form_urlencoded::parse(body.as_bytes())
            .map(|(key, _)| key.into_owned())
            .collect()
    }

    fn authorization_code_token_request_for_test(
        client_id: &str,
        redirect_uri: &str,
        code: &str,
        pkce_verifier: &str,
    ) -> HttpRequest {
        let client_secret = test_client_secret();
        let captured = Rc::new(RefCell::new(None));
        let captured_request = Rc::clone(&captured);
        let result = tauri::async_runtime::block_on(request_oauth_authorization_code(
            client_id,
            &client_secret,
            redirect_uri,
            code,
            pkce_verifier,
            move |request| {
                *captured_request.borrow_mut() = Some(request);
                async {
                    Ok(HttpResponse {
                        status_code: oauth2::http::StatusCode::OK,
                        headers: oauth2::http::HeaderMap::new(),
                        body: br#"{"access_token":"access","token_type":"Bearer","refresh_token":"refresh","expires_in":3600,"scope":"https://www.googleapis.com/auth/webmasters.readonly"}"#.to_vec(),
                    })
                }
            },
        ));
        assert!(result.is_ok());
        let request = captured.borrow_mut().take().unwrap();
        request
    }

    fn original_client_id() -> String {
        "other-client.apps.googleusercontent.com".to_string()
    }

    fn original_client_secret() -> String {
        ["original", "fixture", "secret"].join("-")
    }

    fn original_refresh_token() -> String {
        ["original", "fixture", "refresh"].join("-")
    }

    fn credential_backend_for_client(client_id: &str) -> FakeCredentialBackend {
        let mut backend = FakeCredentialBackend::default();
        backend
            .values
            .insert(CLIENT_ID_ACCOUNT.to_string(), client_id.to_string());
        backend
            .values
            .insert(CLIENT_SECRET_ACCOUNT.to_string(), original_client_secret());
        backend
            .values
            .insert(REFRESH_TOKEN_ACCOUNT.to_string(), original_refresh_token());
        backend
    }

    fn original_credential_backend() -> FakeCredentialBackend {
        credential_backend_for_client(&original_client_id())
    }

    fn imported_oauth_client_fixture() -> ImportedOAuthClient {
        ImportedOAuthClient {
            client_id: VALID_CLIENT_ID.to_string(),
            client_secret: test_client_secret(),
        }
    }

    fn assert_credentials_for_client(backend: &FakeCredentialBackend, client_id: &str) {
        assert!(backend
            .values
            .get(CLIENT_ID_ACCOUNT)
            .is_some_and(|value| value == client_id));
        assert!(backend
            .values
            .get(CLIENT_SECRET_ACCOUNT)
            .is_some_and(|value| value == &original_client_secret()));
        assert!(backend
            .values
            .get(REFRESH_TOKEN_ACCOUNT)
            .is_some_and(|value| value == &original_refresh_token()));
    }

    fn assert_original_credentials(backend: &FakeCredentialBackend) {
        assert_credentials_for_client(backend, &original_client_id());
    }

    fn assert_only_update_lock_is_stored(backend: &FakeCredentialBackend) {
        assert_eq!(
            backend.values.get(CLIENT_ID_ACCOUNT).map(String::as_str),
            Some(OAUTH_CREDENTIAL_UPDATE_LOCK)
        );
        assert!(usable_stored_client_id(OAUTH_CREDENTIAL_UPDATE_LOCK).is_none());
        assert!(!backend.values.contains_key(CLIENT_SECRET_ACCOUNT));
        assert!(!backend.values.contains_key(REFRESH_TOKEN_ACCOUNT));
        assert_eq!(backend.values.len(), 1);
    }

    fn assert_update_lock_is_stored(backend: &FakeCredentialBackend) {
        assert_eq!(
            backend.values.get(CLIENT_ID_ACCOUNT).map(String::as_str),
            Some(OAUTH_CREDENTIAL_UPDATE_LOCK)
        );
        assert!(usable_stored_client_id(OAUTH_CREDENTIAL_UPDATE_LOCK).is_none());
    }

    fn client_status_from_fake_state(
        backend: &FakeCredentialBackend,
        runtime: &FakeRuntimeBackend,
    ) -> SearchConsoleClientStatus {
        let configured = backend
            .values
            .get(CLIENT_ID_ACCOUNT)
            .and_then(|value| usable_stored_client_id(value))
            .is_some();
        let client_secret_stored = backend.values.contains_key(CLIENT_SECRET_ACCOUNT);
        let authorization_stored = backend.values.contains_key(REFRESH_TOKEN_ACCOUNT);
        SearchConsoleClientStatus {
            configured,
            client_secret_stored,
            authorization_stored,
            connected: runtime.state.connected
                && configured
                && client_secret_stored
                && authorization_stored
                && !runtime.state.reauthentication_required,
            authentication_in_progress: false,
            reauthentication_required: runtime.state.reauthentication_required,
            last_checked_at: runtime.state.last_checked_at.clone(),
        }
    }

    fn assert_oauth_entry_points_blocked(backend: &mut FakeCredentialBackend) {
        let mut request_count = 0;
        for _ in 0..2 {
            let result = read_required_client_id_with(backend).and_then(|_| {
                request_count += 1;
                Ok(())
            });
            assert_eq!(result.unwrap_err(), SearchConsoleError::NotConfigured);
        }
        assert_eq!(request_count, 0);
    }

    fn assert_runtime_matches(
        actual: &RuntimeConnectionSnapshot,
        expected: &RuntimeConnectionSnapshot,
    ) {
        assert_eq!(actual.connected, expected.connected);
        assert_eq!(
            actual.reauthentication_required,
            expected.reauthentication_required
        );
        assert_eq!(actual.last_checked_at, expected.last_checked_at);
        assert!(actual
            .access_token
            .as_ref()
            .zip(expected.access_token.as_ref())
            .is_some_and(|(actual, expected)| {
                actual.token == expected.token && actual.expires_at == expected.expires_at
            }));
    }

    fn assert_runtime_fail_closed(runtime: &RuntimeConnectionSnapshot) {
        assert!(!runtime.connected);
        assert!(runtime.reauthentication_required);
        assert!(runtime.last_checked_at.is_none());
        assert!(runtime.access_token.is_none());
    }

    fn test_client_secret() -> String {
        ["fixture", "client", "secret"].join("-")
    }

    fn desktop_oauth_json_fixture(client_id: &str, client_secret: &str) -> Vec<u8> {
        serde_json::to_vec(&serde_json::json!({
            "installed": {
                "client_id": client_id,
                "client_secret": client_secret,
                "auth_uri": AUTHORIZATION_ENDPOINT,
                "token_uri": TOKEN_ENDPOINT,
                "redirect_uris": ["http://localhost"]
            }
        }))
        .unwrap()
    }

    fn request_body_string_for_test(request: &HttpRequest) -> String {
        String::from_utf8(request.body.clone()).unwrap()
    }

    fn spawn_mock_token_endpoint() -> (String, thread::JoinHandle<MockTokenRequestMetadata>) {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        listener.set_nonblocking(true).unwrap();
        let port = listener.local_addr().unwrap().port();
        let endpoint = format!("http://127.0.0.1:{port}/token");
        let handle = thread::spawn(move || {
            let deadline = Instant::now() + Duration::from_secs(5);
            let (mut stream, _) = loop {
                match listener.accept() {
                    Ok(connection) => break connection,
                    Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                        assert!(Instant::now() < deadline, "mock token server timed out");
                        thread::sleep(Duration::from_millis(10));
                    }
                    Err(error) => panic!("mock token server failed: {error}"),
                }
            };
            let raw_request = read_mock_http_request(&mut stream);
            let metadata = parse_mock_token_request(&raw_request);
            let body = br#"{"access_token":"fixture-access","token_type":"Bearer","refresh_token":"fixture-refresh","expires_in":3600,"scope":"https://www.googleapis.com/auth/webmasters.readonly"}"#;
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            );
            stream.write_all(response.as_bytes()).unwrap();
            stream.write_all(body).unwrap();
            stream.flush().unwrap();
            metadata
        });
        (endpoint, handle)
    }

    fn read_mock_http_request(stream: &mut TcpStream) -> String {
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        let mut buffer = Vec::new();
        let mut chunk = [0_u8; 512];
        let header_end = loop {
            let read = stream.read(&mut chunk).unwrap();
            assert!(read > 0, "mock token request closed before headers");
            buffer.extend_from_slice(&chunk[..read]);
            if let Some(header_end) = http_header_end(&buffer) {
                break header_end;
            }
        };
        let header_text = String::from_utf8(buffer[..header_end].to_vec()).unwrap();
        let content_length = header_text
            .lines()
            .find_map(|line| {
                let (name, value) = line.split_once(':')?;
                name.eq_ignore_ascii_case("content-length")
                    .then(|| value.trim().parse::<usize>().unwrap())
            })
            .unwrap_or(0);
        while buffer.len() < header_end + content_length {
            let read = stream.read(&mut chunk).unwrap();
            assert!(read > 0, "mock token request closed before body");
            buffer.extend_from_slice(&chunk[..read]);
        }
        String::from_utf8(buffer).unwrap()
    }

    fn http_header_end(buffer: &[u8]) -> Option<usize> {
        buffer
            .windows(4)
            .position(|window| window == b"\r\n\r\n")
            .map(|index| index + 4)
    }

    fn parse_mock_token_request(raw_request: &str) -> MockTokenRequestMetadata {
        let (headers, body) = raw_request.split_once("\r\n\r\n").unwrap();
        let mut lines = headers.lines();
        let request_line = lines.next().unwrap();
        let mut request_parts = request_line.split_whitespace();
        let method = request_parts.next().unwrap().to_string();
        let target = request_parts.next().unwrap();
        let url = url::Url::parse(&format!("http://127.0.0.1{target}")).unwrap();
        let headers = lines
            .filter_map(|line| {
                let (name, value) = line.split_once(':')?;
                Some((name.to_ascii_lowercase(), value.trim().to_string()))
            })
            .collect();

        MockTokenRequestMetadata {
            method,
            path: url.path().to_string(),
            headers,
            body: body.to_string(),
        }
    }

    fn runtime_test_lock() -> TestMutexGuard<'static, ()> {
        static LOCK: TestOnceLock<TestMutex<()>> = TestOnceLock::new();
        LOCK.get_or_init(|| TestMutex::new(()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
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
