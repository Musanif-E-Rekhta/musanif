use dioxus::prelude::*;

use crate::components::{AuthField, AuthShell};
use crate::{api, state::{CURRENT_2FA_CHALLENGE, CURRENT_USER}, Route};

#[component]
pub fn Login() -> Element {
    let email = use_signal(String::new);
    let password = use_signal(String::new);
    let nav = use_navigator();

    rsx! {
        AuthShell {
            tagline: "Sign in to your library",
            footer_prompt: "Don't have an account?",
            footer_route: Route::Signup {},
            footer_link: "Sign up",

            form {
                class: "login-form",
                onsubmit: move |e| {
                    e.prevent_default();
                    let email_val = email.cloned();
                    let password_val = password.cloned();
                    spawn(async move {
                        if let Some(payload) = api::login(email_val, password_val).await {
                            if payload.requires_2fa {
                                *CURRENT_2FA_CHALLENGE.write() = payload.challenge;
                                nav.push(Route::Login2fa {});
                            } else {
                                api::set_auth_token(Some(payload.token));
                                api::set_refresh_token(payload.refresh_token);
                                *CURRENT_USER.write() = Some(payload.user);
                                nav.push(Route::Home {});
                            }
                        }
                    });
                },

                AuthField {
                    id: "email",
                    label: "Email",
                    input_type: "email",
                    placeholder: "you@example.com",
                    value: email,
                }
                AuthField {
                    id: "password",
                    label: "Password",
                    input_type: "password",
                    placeholder: "••••••••",
                    value: password,
                }

                button { class: "login-btn", r#type: "submit", "Sign in" }
            }
        }
    }
}
