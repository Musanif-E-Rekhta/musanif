//! Toast notifications.
//!
//! A single global queue (`TOASTS`) feeds the [`ToastViewport`]
//! component, which mounts once near the root of every shell and
//! renders a floating stack in the bottom-right corner.
//!
//! Toasts are user-dismissible only — no auto-fade timer. That keeps
//! the cross-platform story simple (no wasm/native timer split) and
//! matches what the design tokens favour: explicit, calm UI.
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
}

#[derive(Clone, Debug, PartialEq)]
pub struct Toast {
    pub id: u64,
    pub kind: ToastKind,
    pub title: String,
    pub message: String,
}

/// Global queue. Push via [`push`] / [`push_error`]; the
/// [`ToastViewport`] reads it and renders.
pub static TOASTS: GlobalSignal<Vec<Toast>> = Signal::global(Vec::new);

fn next_id() -> u64 {
    static SEQ: AtomicU64 = AtomicU64::new(1);
    SEQ.fetch_add(1, Ordering::Relaxed)
}

pub fn push(kind: ToastKind, title: impl Into<String>, message: impl Into<String>) {
    let toast = Toast {
        id: next_id(),
        kind,
        title: title.into(),
        message: message.into(),
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
