use dioxus::prelude::*;

use crate::api;
use crate::models::Setup2faPayload;

/// Two-factor enrollment screen.
///
/// Three-step flow:
/// 1. Click "Generate" → calls `setup_2fa` mutation → server returns a
///    plaintext secret + `otpauth_url` + 10 plaintext recovery codes.
///    Client stashes the recovery codes locally so it can pass them back
///    to `verify_2fa` once the user confirms a TOTP code.
/// 2. User scans the `otpauth_url` (rendered as a clickable link until a
///    QR-code crate lands in this package) and types a 6-digit code.
/// 3. Click "Confirm" → calls `verify_2fa(code, plaintext_recovery_codes)`
///    → server hashes the recovery codes and flips `totp_enabled_at`.
///    Recovery codes are then displayed one final time for the user to
///    record; the next call will not return them.
#[component]
pub fn Setup2fa() -> Element {
    let mut payload = use_signal(|| None::<Setup2faPayload>);
    let mut code = use_signal(String::new);
    let status = use_signal(|| "".to_string());

    rsx! {
        div { class: "island is-main",
            h1 { class: "is-main-title", "Set up two-factor authentication" }

            match payload.cloned() {
                None => rsx! {
                    p { class: "is-main-blurb",
                        "Generate a fresh secret. You'll see a QR-code URL to scan with an "
                        "authenticator app, plus ten recovery codes. Keep the codes — they "
                        "let you sign in if you lose your device."
                    }
                    button {
                        class: "login-btn",
                        onclick: move |_| {
                            let mut payload = payload;
                            spawn(async move {
                                if let Some(p) = api::setup_2fa().await {
                                    payload.set(Some(p));
                                }
                            });
                        },
                        "Generate secret"
                    }
                },
                Some(p) => {
                    let secret = p.secret.clone();
                    let url = p.otpauth_url.clone();
                    let codes = p.recovery_codes.clone();
                    let codes_for_verify = codes.clone();
                    rsx! {
                        section { class: "totp-section",
                            h2 { "1. Scan in your authenticator" }
                            p { class: "is-main-blurb",
                                "Scan the QR code in your authenticator app, or open the "
                                "link below on the device with your authenticator installed."
                            }
                            a { href: "{url}", target: "_blank", class: "totp-otpauth-link", "{url}" }
                            details {
                                summary { "Or enter the secret manually" }
                                code { class: "totp-secret", "{secret}" }
                            }
                        }
                        section { class: "totp-section",
                            h2 { "2. Save your recovery codes" }
                            p { class: "is-main-blurb",
                                "Store these somewhere safe. Each one can be used once if "
                                "you lose access to your authenticator."
                            }
                            ul { class: "totp-recovery-codes",
                                for c in codes.iter() {
                                    li { code { "{c}" } }
                                }
                            }
                        }
                        section { class: "totp-section",
                            h2 { "3. Confirm a code" }
                            input {
                                r#type: "text",
                                placeholder: "123456",
                                value: "{code}",
                                oninput: move |e| code.set(e.value()),
                            }
                            button {
                                class: "login-btn",
                                onclick: move |_| {
                                    let code_val = code.cloned();
                                    let codes = codes_for_verify.clone();
                                    let mut status = status;
                                    spawn(async move {
                                        if api::verify_2fa(code_val, codes).await {
                                            status.set("Two-factor authentication enabled.".into());
                                        } else {
                                            status.set("That code didn't match. Try again.".into());
                                        }
                                    });
                                },
                                "Confirm"
                            }
                            if !status.read().is_empty() {
                                p { class: "totp-status", "{status}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
