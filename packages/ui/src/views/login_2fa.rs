use dioxus::prelude::*;

use crate::components::{AuthField, AuthShell};
use crate::{api, state::{CURRENT_2FA_CHALLENGE, CURRENT_USER}, Route};

/// Two-factor login challenge screen. Reached when `Login` receives
/// `requires_2fa = true`; reads the challenge token from
/// [`CURRENT_2FA_CHALLENGE`] and exchanges it + a TOTP/recovery code
/// for a real session.
#[component]
pub fn Login2fa() -> Element {
    let code = use_signal(String::new);
    let error = use_signal(|| None::<&'static str>);
    let nav = use_navigator();

    rsx! {
        AuthShell {
            tagline: "Enter your authenticator code",
            footer_prompt: "Lost your device?",
            footer_route: Route::Login {},
            footer_link: "Use a recovery code or sign in again",

            form {
                class: "login-form",
                onsubmit: move |e| {
                    e.prevent_default();
                    let challenge = CURRENT_2FA_CHALLENGE.read().clone();
                    let Some(challenge) = challenge else {
                        let mut err = error;
                        err.set(Some("Challenge expired. Please sign in again."));
                        return;
                    };
                    let code_val = code.cloned();
                    let mut err = error;
                    spawn(async move {
                        match api::login_2fa_complete(challenge, code_val).await {
                            Some(payload) if !payload.requires_2fa => {
                                api::set_auth_token(Some(payload.token));
                                api::set_refresh_token(payload.refresh_token);
                                *CURRENT_USER.write() = Some(payload.user);
                                *CURRENT_2FA_CHALLENGE.write() = None;
                                nav.push(Route::Home {});
                            }
                            _ => {
                                err.set(Some("Invalid code. Try again."));
                            }
                        }
                    });
                },

                AuthField {
                    id: "code",
                    label: "Verification code",
                    input_type: "text",
                    placeholder: "123 456 — or recovery code",
                    value: code,
                }

                if let Some(msg) = *error.read() {
                    div { class: "auth-error", "{msg}" }
                }

                button { class: "login-btn", r#type: "submit", "Verify" }
            }
        }
    }
}
