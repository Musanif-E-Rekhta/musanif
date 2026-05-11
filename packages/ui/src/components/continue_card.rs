use dioxus::prelude::*;

use super::Cover;
use crate::Route;

/// One row inside a "Continue Reading" rail — small cover, title, current
/// chapter line, and a thin progress bar with `% · time left` underneath.
#[component]
pub fn ContinueCard(
    title: String,
    chapter: Option<String>,
    book_slug: String,
    progress_pct: f64,
    time_left: Option<String>,
    cover_urdu: Option<String>,
    cover_mono: Option<String>,
    cover_accent: Option<String>,
) -> Element {
    let pct = progress_pct.clamp(0.0, 100.0);
    rsx! {
        Link {
            class: "is-continue",
            to: Route::BookDetail { slug: book_slug },
            div {
                class: "is-continue-cover",
                style: cover_accent.as_ref().map(|a| format!("background: {a}")),
                Cover {
                    urdu: cover_urdu,
                    mono: cover_mono,
                    accent: cover_accent.clone(),
                }
            }
            div { class: "is-continue-info",
                p { class: "is-continue-title", "{title}" }
                if let Some(c) = chapter.as_deref().filter(|s| !s.is_empty()) {
                    p { class: "is-continue-chapter", "{c}" }
                }
                div { class: "is-progress",
                    div { class: "is-progress-fill", style: "width: {pct:.0}%" }
                }
                div { class: "is-progress-label",
                    span { "{pct:.0}%" }
                    if let Some(t) = time_left.as_deref().filter(|s| !s.is_empty()) {
                        span { "{t}" }
                    }
                }
            }
        }
    }
}
