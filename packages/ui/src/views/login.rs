use dioxus::prelude::*;

use crate::Route;

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const LOGIN_CSS: Asset = asset!("/assets/styling/login.css");

#[component]
pub fn Login() -> Element {
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let nav = use_navigator();

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: LOGIN_CSS }

        div { class: "login-page",
            div { class: "login-card",

                div { class: "login-header",
                    h1 { class: "login-brand", "مصنف" }
                    p { class: "login-tagline", "Sign in to your library" }
                }

                form {
                    class: "login-form",
                    onsubmit: move |e| {
                        e.prevent_default();
                        let email_val = email.cloned();
                        let password_val = password.cloned();
                        spawn(async move {
                            if let Some(payload) = crate::api::login(crate::models::LoginInput {
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

                    button { class: "login-btn", r#type: "submit", "Sign in" }
                }

                p { class: "login-footer",
                    "Don't have an account? "
                    Link { to: Route::Signup {}, class: "login-footer-link", "Sign up" }
                }
            }
        }
    }
}
