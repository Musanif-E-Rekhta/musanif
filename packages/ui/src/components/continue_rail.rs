use dioxus::prelude::*;

use super::{ContinueCard, HighlightCard, ReadingGoalCard};
use crate::models::ReadingGoal;

#[component]
fn ContinueRow(divider: bool, entry: ContinueEntry) -> Element {
    rsx! {
        if divider {
            hr { class: "is-divider" }
        }
        ContinueCard {
            title: entry.title,
            chapter: entry.chapter,
            book_slug: entry.book_slug,
            progress_pct: entry.progress_pct,
            time_left: entry.time_left,
            cover_urdu: entry.cover_urdu,
            cover_mono: entry.cover_mono,
            cover_accent: entry.cover_accent,
        }
    }
}

/// One book row supplied to `ContinueRail`. Owned strings so the rail can
/// rebuild without holding model references.
#[derive(Clone, PartialEq)]
pub struct ContinueEntry {
    pub title: String,
    pub chapter: Option<String>,
    pub book_slug: String,
    pub progress_pct: f64,
    pub time_left: Option<String>,
    pub cover_urdu: Option<String>,
    pub cover_mono: Option<String>,
    pub cover_accent: Option<String>,
}

/// One highlight quote row supplied to `ContinueRail`.
#[derive(Clone, PartialEq)]
pub struct HighlightEntry {
    pub eyebrow: Option<String>,
    pub quote: String,
    pub source: Option<String>,
}

/// Right-rail card stack on Discover/Profile: continue-reading + highlight +
/// reading-goal, each rendered only when data is provided.
#[component]
pub fn ContinueRail(
    entries: Vec<ContinueEntry>,
    highlight: Option<HighlightEntry>,
    goal: Option<ReadingGoal>,
    pace_hint: Option<String>,
) -> Element {
    rsx! {
        div { class: "is-rail-col",
            if !entries.is_empty() {
                div { class: "island is-rail-card",
                    div { class: "is-rail-eyebrow", "Continue Reading" }
                    for (i, entry) in entries.iter().enumerate() {
                        ContinueRow {
                            key: "{entry.book_slug}",
                            divider: i > 0,
                            entry: entry.clone(),
                        }
                    }
                }
            }

            if let Some(h) = highlight {
                HighlightCard {
                    eyebrow: h.eyebrow,
                    quote: h.quote,
                    source: h.source,
                }
            }

            if let Some(g) = goal {
                ReadingGoalCard {
                    year: g.year,
                    completed: g.completed,
                    target: g.target,
                    pace_hint,
                }
            }
        }
    }
}
