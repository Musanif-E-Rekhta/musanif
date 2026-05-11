//! Client-side persistence for user preferences that don't have a
//! backend mutation yet (display, reading, library toggles).
//!
//! `use_persisted_*` returns a `Signal<T>` that:
//! 1. Starts at the supplied default,
//! 2. Hydrates from `localStorage` on mount (async — flips the UI once
//!    the read resolves), and
//! 3. Writes any subsequent change back to `localStorage`.
//!
//! All I/O goes through `document::eval` so the same code runs on web
//! (real `localStorage`), desktop, and mobile shells. When the backend
//! grows an `update_preferences` mutation we can swap the bodies for a
//! server round-trip — the public surface is deliberately key/value-shaped.

use dioxus::prelude::*;

const PREFIX: &str = "musanif-pref-";

fn full_key(name: &str) -> String {
    format!("{PREFIX}{name}")
}

/// Hand-rolled JSON-string escape for `eval` interpolation. We only
/// produce the value as a string literal so we just need to escape `\`
/// and `'`.
fn escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('\'', "\\'")
}

fn write_string(name: &str, value: &str) {
    let _ = document::eval(&format!(
        "try {{ localStorage.setItem('{}', '{}'); }} catch(e) {{}}",
        full_key(name),
        escape(value),
    ));
}

async fn read_string(name: &str) -> Option<String> {
    let mut handle = document::eval(&format!(
        "return localStorage.getItem('{}')",
        full_key(name),
    ));
    match handle.recv::<serde_json::Value>().await.ok()? {
        serde_json::Value::String(s) => Some(s),
        _ => None,
    }
}

/// String-valued preference. Hydrates async from localStorage and
/// persists subsequent edits.
pub fn use_persisted_string(name: &'static str, default: &str) -> Signal<String> {
    let mut signal = use_signal(|| default.to_string());
    let mut hydrated = use_signal(|| false);

    use_future(move || async move {
        if let Some(stored) = read_string(name).await {
            signal.set(stored);
        }
        hydrated.set(true);
    });

    use_effect(move || {
        if !*hydrated.read() {
            return;
        }
        let v = signal.read().clone();
        write_string(name, &v);
    });

    signal
}

/// Boolean preference. Stored as the literal strings `"true"` /
/// `"false"`.
pub fn use_persisted_bool(name: &'static str, default: bool) -> Signal<bool> {
    let mut signal = use_signal(|| default);
    let mut hydrated = use_signal(|| false);

    use_future(move || async move {
        if let Some(stored) = read_string(name).await {
            signal.set(stored == "true");
        }
        hydrated.set(true);
    });

    use_effect(move || {
        if !*hydrated.read() {
            return;
        }
        let v = *signal.read();
        write_string(name, if v { "true" } else { "false" });
    });

    signal
}

/// Unsigned-integer preference (font size, line height ×100, etc.).
/// Falls back to the default when storage holds a non-parseable string.
pub fn use_persisted_u32(name: &'static str, default: u32) -> Signal<u32> {
    let mut signal = use_signal(|| default);
    let mut hydrated = use_signal(|| false);

    use_future(move || async move {
        if let Some(stored) = read_string(name).await {
            if let Ok(v) = stored.parse::<u32>() {
                signal.set(v);
            }
        }
        hydrated.set(true);
    });

    use_effect(move || {
        if !*hydrated.read() {
            return;
        }
        let v = *signal.read();
        write_string(name, &v.to_string());
    });

    signal
}
