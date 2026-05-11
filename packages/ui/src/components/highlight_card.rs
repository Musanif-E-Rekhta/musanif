use dioxus::prelude::*;

/// Right-rail card displaying a saved highlight quote and its source.
#[component]
pub fn HighlightCard(
    eyebrow: Option<String>,
    quote: String,
    source: Option<String>,
) -> Element {
    rsx! {
        div { class: "island is-rail-card",
            if let Some(e) = eyebrow.as_deref().filter(|s| !s.is_empty()) {
                div { class: "is-rail-eyebrow", "{e}" }
            }
            p { class: "is-quote", "\u{201C}{quote}\u{201D}" }
            if let Some(s) = source.as_deref().filter(|s| !s.is_empty()) {
                p { class: "is-quote-source", "{s}" }
            }
        }
    }
}
