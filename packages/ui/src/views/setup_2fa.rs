use dioxus::prelude::*;

use crate::api;
use crate::models::{Setup2faPayload, TwoFactorStatus};
use crate::Route;

/// `/settings/2fa` — single-page TOTP enrollment wizard.
///
/// Flow:
/// 1. Mount fetches `fetch_my_2fa_status`. If already enabled, render
///    the already-enabled card and bail out: no accidental re-rotation
///    until a future manage UI lands.
/// 2. Disabled: three section islands stack vertically (Authenticator,
///    Recovery codes, Confirm). Sections 2 and 3 only render after
///    the user clicks "Generate secret" and the server returns the
///    `Setup2faPayload` (secret + otpauth url + QR svg + 10 recovery
///    codes).
/// 3. The user scans the QR (or opens the otpauth link), copies the
///    recovery codes, and types the current 6-digit TOTP. Successful
///    `verify_2fa` redirects to `/settings#account`.
///
/// Compliance-shaped: the page rewards "fastest accurate path" rather
/// than narrative reassurance. Errors render inline; nothing modal.
#[component]
pub fn Setup2fa() -> Element {
    let status = use_resource(|| async { api::fetch_my_2fa_status().await });

    rsx! {
        div { class: "island is-main",
            div { class: "is-main-header",
                h2 { class: "is-main-title", "Two-factor authentication" }
                span { class: "is-main-subtitle",
                    "Add an extra step to sign-in so a leaked password isn't enough."
                }
                div { class: "is-main-actions",
                    Link {
                        to: Route::Settings {},
                        class: "is-btn is-btn--ghost",
                        "← Back to settings"
                    }
                }
            }

            div { class: "is-main-body settings-scroll",
                match &*status.read() {
                    None => rsx! { SetupSkeleton {} },
                    Some(Some(TwoFactorStatus { enabled: true, .. })) => rsx! {
                        AlreadyEnabled {}
                    },
                    _ => rsx! { SetupWizard {} },
                }
            }
        }
    }
}

#[component]
fn AlreadyEnabled() -> Element {
    let nav = use_navigator();
    rsx! {
        section { class: "settings-section-island totp-already",
            p { class: "settings-section-eyebrow", "ALREADY ENABLED" }
            div { class: "settings-section-body",
                p { class: "totp-already-headline",
                    "Two-factor is already on."
                }
                p { class: "totp-already-body",
                    "Disable or rotate it from a future manage page; for now, you're set."
                }
                button {
                    class: "is-btn is-btn--primary",
                    onclick: move |_| { nav.replace(Route::Settings {}); },
                    "Back to settings"
                }
            }
        }
    }
}

#[component]
fn SetupSkeleton() -> Element {
    rsx! {
        for i in 0..3 {
            section {
                key: "{i}",
                class: "settings-section-island",
                span { class: "is-skeleton-line is-skeleton-line--sm" }
                div { class: "settings-section-body",
                    span { class: "is-skeleton-line is-skeleton-line--lg" }
                    span { class: "is-skeleton-line is-skeleton-line--md" }
                }
            }
        }
    }
}

#[component]
fn SetupWizard() -> Element {
    let payload = use_signal(|| None::<Setup2faPayload>);
    let gen_error = use_signal(|| None::<&'static str>);

    rsx! {
        SectionGenerate { payload, gen_error }
        if payload.read().is_some() {
            SectionRecoveryCodes { payload }
            SectionConfirm { payload }
        }
    }
}

#[component]
fn SectionGenerate(
    payload: Signal<Option<Setup2faPayload>>,
    gen_error: Signal<Option<&'static str>>,
) -> Element {
    let mut payload = payload;
    let mut gen_error = gen_error;
    let mut generating = use_signal(|| false);

    let snapshot = payload.read().clone();

    let generate = move |_| {
        gen_error.set(None);
        generating.set(true);
        spawn(async move {
            match api::setup_2fa().await {
                Some(p) => payload.set(Some(p)),
                None => gen_error.set(Some(
                    "Could not start setup. Refresh and try again.",
                )),
            }
            generating.set(false);
        });
    };

    rsx! {
        section { class: "settings-section-island",
            p { class: "settings-section-eyebrow", "1 · AUTHENTICATOR" }
            div { class: "settings-section-body",
                match snapshot {
                    None => rsx! {
                        p { class: "totp-step-body",
                            "Generate a fresh secret, then scan it with your authenticator app."
                        }
                        if let Some(msg) = *gen_error.read() {
                            p { class: "settings-inline-error", "{msg}" }
                        }
                        button {
                            class: "is-btn is-btn--primary",
                            disabled: *generating.read(),
                            onclick: generate,
                            if *generating.read() { "Generating" } else { "Generate secret" }
                        }
                    },
                    Some(p) => rsx! {
                        p { class: "totp-step-body",
                            "Scan the QR code in your authenticator, or open the link if you're already on your phone."
                        }
                        div { class: "totp-qr-row",
                            div {
                                class: "totp-qr",
                                dangerous_inner_html: "{p.qr_svg}",
                            }
                            div { class: "totp-qr-side",
                                a {
                                    href: "{p.otpauth_url}",
                                    target: "_blank",
                                    rel: "noopener",
                                    class: "is-btn is-btn--block",
                                    "Open in authenticator"
                                }
                                details { class: "totp-manual",
                                    summary { "Or enter the secret manually" }
                                    code { class: "totp-secret", "{p.secret}" }
                                }
                            }
                        }
                    },
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum CopyState {
    Idle,
    Copied,
}

#[component]
fn SectionRecoveryCodes(payload: Signal<Option<Setup2faPayload>>) -> Element {
    let mut copy_state = use_signal(|| CopyState::Idle);
    let codes = payload
        .read()
        .as_ref()
        .map(|p| p.recovery_codes.clone())
        .unwrap_or_default();
    let codes_for_copy = codes.clone();

    let copy_all = move |_| {
        let joined = codes_for_copy.join("\n");
        let mut e = document::eval(
            r#"
            const text = await dioxus.recv();
            try { await navigator.clipboard.writeText(text); } catch (e) {}
            await new Promise(r => setTimeout(r, 1200));
            return null;
            "#,
        );
        let _ = e.send(joined);
        copy_state.set(CopyState::Copied);
        spawn(async move {
            let _ = e.recv::<serde_json::Value>().await;
            copy_state.set(CopyState::Idle);
        });
    };

    rsx! {
        section { class: "settings-section-island",
            p { class: "settings-section-eyebrow", "2 · RECOVERY CODES" }
            div { class: "settings-section-body",
                p { class: "totp-step-body",
                    "Keep these. Each one signs you in once if you lose your authenticator. They won't appear again."
                }
                ul { class: "totp-codes",
                    for code in codes.iter() {
                        li { key: "{code}", class: "totp-code", "{code}" }
                    }
                }
                button {
                    class: "is-btn",
                    onclick: copy_all,
                    if *copy_state.read() == CopyState::Copied { "Copied" } else { "Copy all" }
                }
            }
        }
    }
}

#[component]
fn SectionConfirm(payload: Signal<Option<Setup2faPayload>>) -> Element {
    let nav = use_navigator();
    let mut code = use_signal(String::new);
    let mut error = use_signal(|| None::<&'static str>);
    let mut confirming = use_signal(|| false);

    let codes = payload
        .read()
        .as_ref()
        .map(|p| p.recovery_codes.clone())
        .unwrap_or_default();

    let submit = move |e: Event<FormData>| {
        e.prevent_default();
        let raw = code.cloned();
        if raw.len() != 6 || !raw.chars().all(|c| c.is_ascii_digit()) {
            error.set(Some("Enter the six-digit code from your authenticator."));
            return;
        }
        error.set(None);
        confirming.set(true);
        let codes = codes.clone();
        spawn(async move {
            if api::verify_2fa(raw, codes).await {
                nav.replace(Route::Settings {});
            } else {
                error.set(Some("That code didn't match. Try again."));
                confirming.set(false);
            }
        });
    };

    rsx! {
        section { class: "settings-section-island",
            p { class: "settings-section-eyebrow", "3 · CONFIRM" }
            div { class: "settings-section-body",
                p { class: "totp-step-body",
                    "Enter the six-digit code your authenticator shows now."
                }
                form {
                    class: "totp-confirm-form",
                    onsubmit: submit,

                    input {
                        class: "totp-code-input",
                        r#type: "text",
                        inputmode: "numeric",
                        pattern: "[0-9]{6}",
                        maxlength: 6,
                        autocomplete: "one-time-code",
                        placeholder: "000000",
                        value: "{code}",
                        oninput: move |e| {
                            let digits: String = e.value().chars().filter(|c| c.is_ascii_digit()).take(6).collect();
                            code.set(digits);
                        },
                    }

                    if let Some(msg) = *error.read() {
                        p { class: "settings-inline-error", "{msg}" }
                    }

                    button {
                        class: "is-btn is-btn--primary",
                        r#type: "submit",
                        disabled: *confirming.read(),
                        if *confirming.read() { "Confirming" } else { "Confirm" }
                    }
                }
            }
        }
    }
}
