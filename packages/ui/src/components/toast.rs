//! Toast notifications.
//!
//! A single global queue (`TOASTS`) feeds the [`ToastViewport`]
//! component, which mounts once near the root of every shell and
//! renders a floating stack in the bottom-right corner.
//!
//! Toasts auto-dismiss after a kind-dependent delay (errors linger
//! longest) and stay user-dismissible via the close button. The
//! per-target timer uses `gloo-timers` on wasm and `tokio::time` on
//! native — both crates are already pulled in transitively for the
//! WebSocket transport, so this adds no fresh runtime baggage.
//!
//! ```ignore
//! if let Err(e) = api::register(input).await {
//!     toast::push_error(&e);
//! }
//! ```

use std::sync::atomic::{AtomicU64, Ordering};

use dioxus::prelude::*;

use crate::error::Error;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ToastKind {
    Error,
    Warning,
    Info,
    Success,
}

impl ToastKind {
    fn modifier(self) -> &'static str {
        match self {
            ToastKind::Error => "toast--error",
            ToastKind::Warning => "toast--warning",
            ToastKind::Info => "toast--info",
            ToastKind::Success => "toast--success",
        }
    }

    /// Single-character glyph rendered in the round badge on the left.
    fn glyph(self) -> &'static str {
        match self {
            ToastKind::Error => "!",
            ToastKind::Warning => "!",
            ToastKind::Info => "i",
            ToastKind::Success => "✓",
        }
    }

    /// Default auto-dismiss delay. Errors and warnings linger so the
    /// reader has time to actually parse them; success/info land lighter.
    fn default_dismiss_ms(self) -> u64 {
        match self {
            ToastKind::Error => 8000,
            ToastKind::Warning => 6500,
            ToastKind::Info => 5000,
            ToastKind::Success => 3500,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Toast {
    pub id: u64,
    pub kind: ToastKind,
    pub title: String,
    pub message: String,
    /// `Some(ms)` auto-dismisses after the delay; `None` is sticky and
    /// requires an explicit close.
    pub auto_dismiss_ms: Option<u64>,
}

/// Global queue. Push via [`push`] / [`push_error`]; the
/// [`ToastViewport`] reads it and renders.
pub static TOASTS: GlobalSignal<Vec<Toast>> = Signal::global(Vec::new);

fn next_id() -> u64 {
    static SEQ: AtomicU64 = AtomicU64::new(1);
    SEQ.fetch_add(1, Ordering::Relaxed)
}

pub fn push(kind: ToastKind, title: impl Into<String>, message: impl Into<String>) {
    push_with(kind, title, message, Some(kind.default_dismiss_ms()));
}

/// Push with an explicit auto-dismiss override. Pass `None` for a
/// sticky toast that only the close button can clear.
pub fn push_with(
    kind: ToastKind,
    title: impl Into<String>,
    message: impl Into<String>,
    auto_dismiss_ms: Option<u64>,
) {
    let toast = Toast {
        id: next_id(),
        kind,
        title: title.into(),
        message: message.into(),
        auto_dismiss_ms,
    };
    TOASTS.write().push(toast);
}

/// Surface a `ui::Error` to the user. Logs it (via `Error::log`) and
/// queues a toast with a kind-appropriate title.
pub fn push_error(err: &Error) {
    err.log();
    let title = match err {
        Error::Unauthorized { .. } => "Sign in required",
        Error::Forbidden { .. } => "Not allowed",
        Error::NotFound { .. } => "Not found",
        Error::Conflict { .. } => "Already exists",
        Error::BadRequest { .. } => "Invalid request",
        Error::Upstream { .. } => "Connection issue",
        Error::Internal { .. } => "Something went wrong",
    };
    push(ToastKind::Error, title, err.message());
}

pub fn dismiss(id: u64) {
    TOASTS.write().retain(|t| t.id != id);
}

pub fn clear_all() {
    TOASTS.write().clear();
}

async fn sleep_ms(ms: u64) {
    #[cfg(target_arch = "wasm32")]
    {
        gloo_timers::future::TimeoutFuture::new(ms as u32).await;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
    }
}

/// Renders the live toast stack. Mount once at the root of each shell.
#[component]
pub fn ToastViewport() -> Element {
    let toasts = TOASTS.read().clone();
    if toasts.is_empty() {
        return rsx! {};
    }
    rsx! {
        div {
            class: "toast-stack",
            role: "region",
            aria_live: "polite",
            for toast in toasts {
                ToastView { key: "{toast.id}", toast }
            }
        }
    }
}

#[component]
fn ToastView(toast: Toast) -> Element {
    let id = toast.id;
    let kind_class = toast.kind.modifier();
    let glyph = toast.kind.glyph();
    let auto_dismiss_ms = toast.auto_dismiss_ms;

    // `use_future` runs once per mount; the `key={toast.id}` on the
    // parent guarantees a fresh mount per toast, so the timer captures
    // this specific id. Manual dismiss unmounts the component and
    // cancels the future — no double-remove, no zombie timer.
    use_future(move || async move {
        if let Some(ms) = auto_dismiss_ms {
            sleep_ms(ms).await;
            dismiss(id);
        }
    });

    rsx! {
        div {
            class: "toast {kind_class}",
            role: "alert",
            div { class: "toast-glyph", "{glyph}" }
            div { class: "toast-body",
                div { class: "toast-title", "{toast.title}" }
                div { class: "toast-message", "{toast.message}" }
            }
            button {
                class: "is-btn is-btn--ghost toast-dismiss",
                onclick: move |_| dismiss(id),
                aria_label: "Dismiss notification",
                "✕"
            }
        }
    }
}
