//! Bearer + refresh-token storage.
//!
//! - **Web (wasm32):** `localStorage` keyed under `musanif:auth_token` /
//!   `musanif:refresh_token`. Survives reloads; cleared on `clear_all_tokens`.
//! - **Native (desktop, iOS, Android):** the [`keyring`] crate, which
//!   wraps the OS-level secret store: Linux Secret Service / KWallet,
//!   macOS Keychain, Windows Credential Manager, iOS Keychain, Android
//!   AccountManager-backed storage. Tokens persist across app restarts.
//!
//! An in-memory cache sits in front of either backend so a hot path
//! reads don't repeatedly hit `localStorage` or the OS keystore.
//!
//! If the native keystore is unavailable — locked, missing service, etc. —
//! the impl quietly falls back to in-memory only. The user can still sign
//! in; they just lose the persistence guarantee until the keystore comes
//! back. We log at `debug` rather than `error` to avoid spamming logs on
//! systems where there's just no Secret Service running.
//!
//! [`keyring`]: https://github.com/hwchen/keyring-rs

use std::sync::{Mutex, OnceLock};

const ACCESS_KEY: &str = "musanif:auth_token";
const REFRESH_KEY: &str = "musanif:refresh_token";
#[cfg(not(target_arch = "wasm32"))]
const KEYRING_SERVICE: &str = "musanif";

/// Storage abstraction. Platform impls vary; the public API never does.
pub trait TokenStore: Send + Sync {
    fn load(&self, key: &str) -> Option<String>;
    fn save(&self, key: &str, token: Option<String>);
}

// ── Web (wasm32): localStorage ─────────────────────────────────────────────

#[cfg(target_arch = "wasm32")]
struct LocalStorageStore;

#[cfg(target_arch = "wasm32")]
impl TokenStore for LocalStorageStore {
    fn load(&self, key: &str) -> Option<String> {
        web_sys::window()
            .and_then(|w| w.local_storage().ok().flatten())
            .and_then(|s| s.get_item(key).ok().flatten())
    }

    fn save(&self, key: &str, token: Option<String>) {
        let Some(window) = web_sys::window() else { return };
        let Some(storage) = window.local_storage().ok().flatten() else { return };
        match token {
            Some(t) => {
                let _ = storage.set_item(key, &t);
            }
            None => {
                let _ = storage.remove_item(key);
            }
        }
    }
}

// ── Native (desktop, iOS, Android): OS keyring ─────────────────────────────

#[cfg(not(target_arch = "wasm32"))]
struct KeyringStore;

#[cfg(not(target_arch = "wasm32"))]
impl TokenStore for KeyringStore {
    fn load(&self, key: &str) -> Option<String> {
        match keyring::Entry::new(KEYRING_SERVICE, key) {
            Ok(entry) => match entry.get_password() {
                Ok(p) => Some(p),
                Err(keyring::Error::NoEntry) => None,
                Err(e) => {
                    tracing::debug!(%key, error = %e, "keyring read failed");
                    None
                }
            },
            Err(e) => {
                tracing::debug!(%key, error = %e, "keyring open failed");
                None
            }
        }
    }

    fn save(&self, key: &str, token: Option<String>) {
        let Ok(entry) = keyring::Entry::new(KEYRING_SERVICE, key) else {
            return;
        };
        match token {
            Some(t) => {
                if let Err(e) = entry.set_password(&t) {
                    tracing::debug!(%key, error = %e, "keyring write failed");
                }
            }
            None => match entry.delete_credential() {
                Ok(_) | Err(keyring::Error::NoEntry) => {}
                Err(e) => {
                    tracing::debug!(%key, error = %e, "keyring delete failed");
                }
            },
        }
    }
}

// ── Public free-function API ───────────────────────────────────────────────

#[cfg(target_arch = "wasm32")]
fn store() -> &'static dyn TokenStore {
    static S: LocalStorageStore = LocalStorageStore;
    &S
}

#[cfg(not(target_arch = "wasm32"))]
fn store() -> &'static dyn TokenStore {
    static S: KeyringStore = KeyringStore;
    &S
}

fn cache_for(key: &str) -> &'static Mutex<Option<String>> {
    static ACCESS: OnceLock<Mutex<Option<String>>> = OnceLock::new();
    static REFRESH: OnceLock<Mutex<Option<String>>> = OnceLock::new();
    if key == REFRESH_KEY {
        REFRESH.get_or_init(|| Mutex::new(None))
    } else {
        ACCESS.get_or_init(|| Mutex::new(None))
    }
}

fn get(key: &str) -> Option<String> {
    if let Ok(g) = cache_for(key).lock() {
        if let Some(t) = g.clone() {
            return Some(t);
        }
    }
    let loaded = store().load(key);
    if let Ok(mut g) = cache_for(key).lock() {
        *g = loaded.clone();
    }
    loaded
}

fn set(key: &str, token: Option<String>) {
    if let Ok(mut g) = cache_for(key).lock() {
        *g = token.clone();
    }
    store().save(key, token);
}

/// Read the current bearer (access) token.
pub fn get_auth_token() -> Option<String> {
    get(ACCESS_KEY)
}

/// Persist (or clear) the bearer (access) token.
pub fn set_auth_token(token: Option<String>) {
    set(ACCESS_KEY, token);
}

/// Read the current refresh token.
pub fn get_refresh_token() -> Option<String> {
    get(REFRESH_KEY)
}

/// Persist (or clear) the refresh token.
pub fn set_refresh_token(token: Option<String>) {
    set(REFRESH_KEY, token);
}

/// Clear both access and refresh tokens. Used on logout and on a
/// terminal refresh failure.
pub fn clear_all_tokens() {
    set_auth_token(None);
    set_refresh_token(None);
}
