use dioxus::prelude::*;

use super::Cover;

/// Discover-page hero: large featured book card with cover, eyebrow tag,
/// title, blurb, and a slot for action buttons.
#[component]
pub fn FeatureCard(
    eyebrow: Option<String>,
    title: String,
    blurb: Option<String>,
    cover_urdu: Option<String>,
    cover_mono: Option<String>,
    cover_accent: Option<String>,
    actions: Option<Element>,
) -> Element {
    rsx! {
        div { class: "is-feature",
            div { class: "is-feature-cover",
                Cover {
                    urdu: cover_urdu,
                    mono: cover_mono,
                    big: true,
                    accent: cover_accent,
                }
            }
            div {
                if let Some(e) = eyebrow.as_deref().filter(|s| !s.is_empty()) {
                    div { class: "is-feature-eyebrow", "{e}" }
                }
                h3 { class: "is-feature-title", "{title}" }
                if let Some(b) = blurb.as_deref().filter(|s| !s.is_empty()) {
                    p { class: "is-feature-blurb", "{b}" }
                }
                if let Some(a) = actions {
                    div { class: "is-feature-actions", {a} }
                }
            }
        }
    }
}
