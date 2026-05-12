use dioxus::prelude::*;

use super::{ContinueEntry, ContinueRail, HighlightEntry, SignUpRail};
use crate::api;
use crate::models::Bookmark;
use crate::state::CURRENT_USER;
use crate::Route;

/// Right-rail for the Discover layout. Branches on auth:
///
/// - **Signed-out**: renders [`SignUpRail`], which carries a soft sign-in
///   invitation plus a curated highlight quote.
/// - **Signed-in**: fetches the user's in-progress bookmarks, reading
///   goal, and recent highlights, and feeds them into [`ContinueRail`].
///   When the user has nothing in flight yet, we still render a single
///   "pick something to begin" island instead of an empty column.
///
/// All data is best-effort: API failures collapse to the empty-state
/// affordances rather than surfacing as an error in the rail.
#[component]
pub fn DiscoverRail() -> Element {
    let signed_in = CURRENT_USER.read().is_some();

    if !signed_in {
        return rsx! { SignUpRail {} };
    }

    let bookmarks =
        use_resource(move || async move { api::fetch_my_bookmarks(Some("reading".into())).await });
    let goal = use_resource(move || async move { api::fetch_my_reading_goal().await });
    let highlights = use_resource(move || async move { api::fetch_my_highlights().await });

    let entries: Vec<ContinueEntry> = bookmarks
        .read()
        .as_ref()
        .and_then(|opt| opt.as_ref())
        .map(|bms| {
            bms.iter()
                .filter(|bm| bm.book.is_some())
                .take(2)
                .map(bookmark_to_entry)
                .collect()
        })
        .unwrap_or_default();

    let goal_value = goal.read().as_ref().and_then(|opt| opt.clone());
    let pace_hint = goal_value.as_ref().map(pace_hint_for);

    let highlight = highlights
        .read()
        .as_ref()
        .and_then(|opt| opt.as_ref())
        .and_then(|hs| hs.first())
        .map(|h| HighlightEntry {
            eyebrow: Some("Recent highlight".into()),
            quote: h.text_snapshot.clone(),
            source: None,
        });

    let has_continue = !entries.is_empty();

    rsx! {
        div { class: "is-rail-col",
            if has_continue {
                ContinueRail {
                    entries,
                    highlight: highlight.clone(),
                    goal: goal_value.clone(),
                    pace_hint,
                }
            } else {
                EmptyContinueCard {}
                if let Some(h) = highlight {
                    super::HighlightCard {
                        eyebrow: h.eyebrow,
                        quote: h.quote,
                        source: h.source,
                    }
                }
                if let Some(g) = goal_value {
                    super::ReadingGoalCard {
                        year: g.year,
                        completed: g.completed,
                        target: g.target,
                        pace_hint: None,
                    }
                } else {
                    EmptyGoalCard {}
                }
            }
        }
    }
}

#[component]
fn EmptyContinueCard() -> Element {
    rsx! {
        section { class: "island is-rail-card is-empty-card",
            p { class: "is-rail-eyebrow", "Continue Reading" }
            p { class: "is-empty-card-headline", "Pick something to begin." }
            p { class: "is-empty-card-body",
                "Tap a cover from the catalog below; we'll remember where you stop."
            }
        }
    }
}

#[component]
fn EmptyGoalCard() -> Element {
    rsx! {
        section { class: "island is-rail-card is-empty-card",
            p { class: "is-rail-eyebrow", "Reading Goal" }
            p { class: "is-empty-card-headline", "Set a 2026 goal." }
            p { class: "is-empty-card-body",
                "Track how many books you read this year, at a pace that feels right."
            }
            Link {
                to: Route::Profile {},
                class: "is-btn is-btn--ghost",
                "Add a goal"
            }
        }
    }
}

fn bookmark_to_entry(bm: &Bookmark) -> ContinueEntry {
    let book = bm.book.as_ref().expect("filtered above");
    let progress_pct = match (bm.progress, book.page_count) {
        (Some(p), Some(pages)) if pages > 0 => (p as f64 / pages as f64 * 100.0).clamp(0.0, 100.0),
        _ => 0.0,
    };
    ContinueEntry {
        title: book.title.clone(),
        chapter: None,
        book_slug: book.slug.clone(),
        progress_pct,
        time_left: None,
        cover_urdu: Some(book.title.clone()),
        cover_mono: book.title.chars().next().map(|c| c.to_string()),
        cover_accent: None,
    }
}

fn pace_hint_for(goal: &crate::models::ReadingGoal) -> String {
    let remaining = (goal.target - goal.completed).max(0);
    if remaining == 0 {
        "Target reached, nice.".to_string()
    } else if goal.progress_pct >= 100.0 {
        "On pace; keep going.".to_string()
    } else {
        format!("{remaining} more to hit your {} target.", goal.year)
    }
}
