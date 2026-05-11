use dioxus::prelude::*;

use crate::components::{AuthField, AuthShell};
use crate::{api, state::CURRENT_USER, Route};

#[component]
pub fn Signup() -> Element {
    let name = use_signal(String::new);
    let email = use_signal(String::new);
    let password = use_signal(String::new);
    let confirm = use_signal(String::new);
    let nav = use_navigator();

    rsx! {
        AuthShell {
            tagline: "Create your account",
            footer_prompt: "Already have an account?",
            footer_route: Route::Login {},
            footer_link: "Sign in",

            form {
                class: "login-form",
                onsubmit: move |e| {
                    e.prevent_default();
                    if password.cloned() != confirm.cloned() {
                        return;
                    }
                    let username = name.cloned();
                    let email_val = email.cloned();
                    let password_val = password.cloned();
                    spawn(async move {
                        if let Some(payload) = api::register(username, email_val, password_val).await {
                            api::set_auth_token(Some(payload.token));
                            api::set_refresh_token(payload.refresh_token);
                            *CURRENT_USER.write() = Some(payload.user);
                            nav.push(Route::Home {});
                        }
                    });
                },

                AuthField {
                    id: "name",
                    label: "Full Name",
                    input_type: "text",
                    placeholder: "Your name",
                    value: name,
                }
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
                AuthField {
                    id: "confirm",
                    label: "Confirm Password",
                    input_type: "password",
                    placeholder: "••••••••",
                    value: confirm,
                }

                button { class: "login-btn", r#type: "submit", "Create account" }
            }
        }
    }
}
