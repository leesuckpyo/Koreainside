use keyring_core::{Entry, Error as KeyringError};
use serde::Serialize;
use std::{collections::HashMap, sync::OnceLock};
use windows_native_keyring_store::{CredPersist, Store};

pub(crate) const CREDENTIAL_SERVICE: &str = "com.getkoreainside.admin.vercel";
pub(crate) const CREDENTIAL_ACCOUNT: &str = "access-token";
const MAX_TOKEN_LENGTH: usize = 512;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VercelConnectionStatus {
    pub(crate) status: &'static str,
    pub(crate) token_stored: bool,
    pub(crate) last_checked_at: Option<String>,
    pub(crate) error_code: Option<&'static str>,
    pub(crate) message: Option<&'static str>,
}

#[tauri::command]
pub fn save_vercel_access_token(token: String) -> VercelConnectionStatus {
    let token = match validate_token(token) {
        Ok(token) => token,
        Err((code, message)) => return status("error", false, code, message),
    };

    with_runtime_invalidation(crate::analytics::invalidate_analytics_runtime_state, || {
        save_validated_token(token)
    })
}

fn save_validated_token(token: String) -> VercelConnectionStatus {
    let entry = match credential_entry() {
        Ok(entry) => entry,
        Err(_) => {
            return status(
                "error",
                false,
                "CREDENTIAL_SAVE_FAILED",
                "Windows 자격 증명 관리자에 토큰을 저장할 수 없습니다.",
            );
        }
    };

    if entry.set_password(&token).is_err() {
        return status(
            "error",
            false,
            "CREDENTIAL_SAVE_FAILED",
            "Windows 자격 증명 관리자에 토큰을 저장할 수 없습니다.",
        );
    }

    match credential_persistence_is_local(&entry) {
        Ok(true) => status("credential_stored", true, "", ""),
        Ok(false) | Err(_) => {
            let _ = entry.delete_credential();
            status(
                "error",
                false,
                "CREDENTIAL_PERSISTENCE_FAILED",
                "자격 증명을 이 PC에만 안전하게 저장할 수 없습니다.",
            )
        }
    }
}

#[tauri::command]
pub fn get_vercel_connection_status() -> VercelConnectionStatus {
    match read_vercel_access_token() {
        Ok(_) => status("credential_stored", true, "", ""),
        Err(KeyringError::NoEntry) => status("not_configured", false, "", ""),
        Err(_) => status(
            "error",
            false,
            "CREDENTIAL_READ_FAILED",
            "Windows 자격 증명 관리자에서 연결 상태를 확인할 수 없습니다.",
        ),
    }
}

#[tauri::command]
pub fn delete_vercel_access_token() -> VercelConnectionStatus {
    with_runtime_invalidation(
        crate::analytics::invalidate_analytics_runtime_state,
        delete_stored_token,
    )
}

fn delete_stored_token() -> VercelConnectionStatus {
    let entry = match credential_entry() {
        Ok(entry) => entry,
        Err(_) => {
            return status(
                "error",
                false,
                "CREDENTIAL_ACCESS_FAILED",
                "Windows 자격 증명 관리자에 접근할 수 없습니다.",
            );
        }
    };

    match entry.delete_credential() {
        Ok(()) | Err(KeyringError::NoEntry) => status("not_configured", false, "", ""),
        Err(_) => status(
            "error",
            true,
            "CREDENTIAL_DELETE_FAILED",
            "저장된 Vercel 자격 증명을 삭제할 수 없습니다.",
        ),
    }
}

fn with_runtime_invalidation<T, I, O>(mut invalidate: I, operation: O) -> T
where
    I: FnMut(),
    O: FnOnce() -> T,
{
    invalidate();
    let result = operation();
    invalidate();
    result
}

pub(crate) fn read_vercel_access_token() -> Result<String, KeyringError> {
    credential_entry()?.get_password()
}

pub(crate) fn connection_result(
    status_value: &'static str,
    token_stored: bool,
    last_checked_at: Option<String>,
    error_code: Option<&'static str>,
    message: Option<&'static str>,
) -> VercelConnectionStatus {
    VercelConnectionStatus {
        status: status_value,
        token_stored,
        last_checked_at,
        error_code,
        message,
    }
}

fn credential_entry() -> Result<Entry, KeyringError> {
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
    Entry::new_with_modifiers(CREDENTIAL_SERVICE, CREDENTIAL_ACCOUNT, &modifiers)
}

fn credential_persistence_is_local(entry: &Entry) -> Result<bool, KeyringError> {
    let attributes = entry.get_attributes()?;
    Ok(attributes
        .get("persistence")
        .is_some_and(|value| value.eq_ignore_ascii_case(&CredPersist::Local.to_string())))
}

fn validate_token(token: String) -> Result<String, (&'static str, &'static str)> {
    let trimmed = token.trim();
    if trimmed.is_empty() {
        return Err(("TOKEN_REQUIRED", "Access Token을 입력해 주십시오."));
    }
    if trimmed.len() > MAX_TOKEN_LENGTH {
        return Err((
            "TOKEN_TOO_LONG",
            "Access Token이 허용된 최대 길이를 초과합니다.",
        ));
    }
    if trimmed.chars().any(char::is_whitespace) {
        return Err(("TOKEN_INVALID", "Access Token 형식을 확인해 주십시오."));
    }
    Ok(trimmed.to_string())
}

fn status(
    status_value: &'static str,
    token_stored: bool,
    error_code: &'static str,
    message: &'static str,
) -> VercelConnectionStatus {
    connection_result(
        status_value,
        token_stored,
        None,
        (!error_code.is_empty()).then_some(error_code),
        (!message.is_empty()).then_some(message),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn rejects_empty_and_whitespace_tokens() {
        assert_eq!(
            validate_token(String::new()).unwrap_err().0,
            "TOKEN_REQUIRED"
        );
        assert_eq!(
            validate_token("   \r\n".to_string()).unwrap_err().0,
            "TOKEN_REQUIRED"
        );
    }

    #[test]
    fn rejects_oversized_and_internal_whitespace_tokens() {
        assert_eq!(
            validate_token("x".repeat(MAX_TOKEN_LENGTH + 1))
                .unwrap_err()
                .0,
            "TOKEN_TOO_LONG"
        );
        assert_eq!(
            validate_token("token value".to_string()).unwrap_err().0,
            "TOKEN_INVALID"
        );
    }

    #[test]
    fn trims_a_valid_token() {
        assert_eq!(
            validate_token("  valid-token  ".to_string()).unwrap(),
            "valid-token"
        );
    }

    #[test]
    fn uses_local_non_roaming_persistence_value() {
        assert_eq!(CredPersist::Local.to_string(), "Local");
    }

    #[test]
    fn saving_token_path_invalidates_runtime_state() {
        let invalidations = Cell::new(0);
        let result =
            with_runtime_invalidation(|| invalidations.set(invalidations.get() + 1), || "saved");

        assert_eq!(result, "saved");
        assert_eq!(invalidations.get(), 2);
    }

    #[test]
    fn deleting_token_path_invalidates_runtime_state() {
        let invalidations = Cell::new(0);
        let result =
            with_runtime_invalidation(|| invalidations.set(invalidations.get() + 1), || "deleted");

        assert_eq!(result, "deleted");
        assert_eq!(invalidations.get(), 2);
    }
}
