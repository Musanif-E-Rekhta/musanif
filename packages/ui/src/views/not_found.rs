use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "not-found",
            h1 { class: "not-found-code", "404" }
            p { class: "not-found-msg", "Page not found: /{route.join(\"/\")}" }
            Link { to: Route::Home {}, class: "btn-link", "← Back to home" }
        }
    }
}
