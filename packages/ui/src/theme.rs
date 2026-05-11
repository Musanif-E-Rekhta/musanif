//! Theme model and platform-aware persistence.
//!
//! All write paths (theme switcher, settings, platform shells) go through
//! `apply_and_persist` so the DOM attribute and `localStorage` stay in sync.

use dioxus::prelude::*;

const STORAGE_KEY: &str = "musanif-theme";

#[derive(Debug, Clone, Copy, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub enum Theme {
    #[default]
    Parchment,
    Midnight,
    SepiaDark,
    Ink,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Parchment => "parchment",
            Theme::Midnight => "midnight",
            Theme::SepiaDark => "sepia-dark",
            Theme::Ink => "ink",
        }
    }
}

/// Apply a theme to `<html data-theme>` and persist it to localStorage.
pub fn apply_and_persist(theme: &str) {
    let _ = document::eval(&format!(
        "document.documentElement.setAttribute('data-theme', '{theme}'); \
         try {{ localStorage.setItem('{STORAGE_KEY}', '{theme}'); }} catch(e) {{}}"
    ));
}

/// Read the persisted theme from localStorage and apply it to the DOM.
/// Falls back to "parchment" when storage is unavailable or empty.
pub fn load_persisted() {
    let _ = document::eval(&format!(
        "try {{ \
           const t = localStorage.getItem('{STORAGE_KEY}') || 'parchment'; \
           document.documentElement.setAttribute('data-theme', t); \
         }} catch(e) {{}}"
    ));
}
