use dioxus::prelude::*;

#[component]
pub fn Cover(
    title: Option<String>,
    urdu: Option<String>,
    mono: Option<String>,
    big: Option<bool>,
    accent: Option<String>,
) -> Element {
    let style = accent
        .map(|a| format!("background: linear-gradient(160deg, {a} 0%, var(--accent-light) 100%)"));

    rsx! {
        div {
            class: "is-book-cover-art",
            style: style,
            div { class: "is-book-cover-stamp", "{mono.as_deref().unwrap_or(\"م\")}" }
            div {}
            div { class: "is-book-cover-title", "{urdu.as_deref().or(title.as_deref()).unwrap_or_default()}" }
            div { class: "is-book-cover-author",
                if !big.unwrap_or(false) {
                    "{title.as_deref().unwrap_or_default()}"
                }
            }
        }
    }
}
