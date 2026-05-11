use dioxus::prelude::*;

use crate::Route;

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const LOGIN_CSS: Asset = asset!("/assets/styling/login.css");

/// Shared chrome for /login and /signup: stylesheets, brand, tagline, footer link.
/// Children render the `<form>` inside the card.
#[component]
pub fn AuthShell(
    tagline: &'static str,
    footer_prompt: &'static str,
    footer_route: Route,
    footer_link: &'static str,
    children: Element,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: LOGIN_CSS }

        div { class: "login-page",
            div { class: "login-card",
                div { class: "login-header",
                    h1 { class: "login-brand", "مصنف" }
                    p { class: "login-tagline", "{tagline}" }
                }

                {children}

                p { class: "login-footer",
                    "{footer_prompt} "
                    Link { to: footer_route, class: "login-footer-link", "{footer_link}" }
                }
            }
        }
    }
}

/// Labeled text input bound to a `Signal<String>` for the auth forms.
#[component]
pub fn AuthField(
    id: &'static str,
    label: &'static str,
    input_type: &'static str,
    placeholder: &'static str,
    value: Signal<String>,
) -> Element {
    let mut value = value;
    rsx! {
        div { class: "form-group",
            label { class: "form-label", r#for: "{id}", "{label}" }
            input {
                id: "{id}",
                class: "form-input",
                r#type: "{input_type}",
                placeholder: "{placeholder}",
                value: "{value}",
                oninput: move |e| value.set(e.value()),
            }
        }
    }
}
