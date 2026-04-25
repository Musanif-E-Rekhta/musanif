use dioxus::prelude::*;

use crate::Route;

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const LOGIN_CSS: Asset = asset!("/assets/styling/login.css");

#[component]
pub fn Signup() -> Element {
    let mut name = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut confirm = use_signal(String::new);
    let nav = use_navigator();

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: LOGIN_CSS }

        div { class: "login-page",
            div { class: "login-card",

                div { class: "login-header",
                    h1 { class: "login-brand", "مصنف" }
                    p { class: "login-tagline", "Create your account" }
                }

                form {
                    class: "login-form",
                    onsubmit: move |e| {
                        e.prevent_default();
                        let name_val = name.cloned();
                        let email_val = email.cloned();
                        let password_val = password.cloned();
                        let confirm_val = confirm.cloned();

                        if password_val != confirm_val {
                            // Basic validation for UI
                            return;
                        }

                        spawn(async move {
                            if let Some(payload) = crate::api::register(crate::models::RegisterInput {
                                username: name_val.clone(),
                                email: email_val.clone(),
                                password: password_val,
                            }).await {
                                crate::api::set_auth_token(Some(payload.token));
                                *crate::CURRENT_USER.write() = Some(payload.user);
                                nav.push(Route::Home {});
                            }
                        });
                    },

                    div { class: "form-group",
                        label { class: "form-label", r#for: "name", "Full Name" }
                        input {
                            id: "name",
                            class: "form-input",
                            r#type: "text",
                            placeholder: "Your name",
                            value: "{name}",
                            oninput: move |e| name.set(e.value()),
                        }
                    }

                    div { class: "form-group",
                        label { class: "form-label", r#for: "email", "Email" }
                        input {
                            id: "email",
                            class: "form-input",
                            r#type: "email",
                            placeholder: "you@example.com",
                            value: "{email}",
                            oninput: move |e| email.set(e.value()),
                        }
                    }

                    div { class: "form-group",
                        label { class: "form-label", r#for: "password", "Password" }
                        input {
                            id: "password",
                            class: "form-input",
                            r#type: "password",
                            placeholder: "••••••••",
                            value: "{password}",
                            oninput: move |e| password.set(e.value()),
                        }
                    }

                    div { class: "form-group",
                        label { class: "form-label", r#for: "confirm", "Confirm Password" }
                        input {
                            id: "confirm",
                            class: "form-input",
                            r#type: "password",
                            placeholder: "••••••••",
                            value: "{confirm}",
                            oninput: move |e| confirm.set(e.value()),
                        }
                    }

                    button { class: "login-btn", r#type: "submit", "Create account" }
                }

                p { class: "login-footer",
                    "Already have an account? "
                    Link { to: Route::Login {}, class: "login-footer-link", "Sign in" }
                }
            }
        }
    }
}
