use dioxus::prelude::*;

use crate::Route;

const LOGIN_CSS: Asset = asset!("/assets/styling/login.css");

#[component]
pub fn Signup() -> Element {
    let mut name = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut confirm = use_signal(String::new);
    let nav = use_navigator();

    rsx! {
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
                        nav.push(Route::Home {});
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
