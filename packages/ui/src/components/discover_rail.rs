use dioxus::prelude::*;

use super::{ContinueEntry, ContinueRail, HighlightEntry};
use crate::api;
use crate::models::Bookmark;

/// Right-rail for the Discover layout. Fetches the signed-in user's
/// in-progress bookmarks + reading goal and feeds them into `ContinueRail`.
///
/// All data is best-effort: when the user is signed out or the API is down,
/// the rail simply renders empty (`ContinueRail` skips empty cards).
#[component]
pub fn DiscoverRail() -> Element {
    let bookmarks = use_resource(move || async move {
        api::fetch_my_bookmarks(Some("reading".into())).await
    });
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

    rsx! {
        ContinueRail {
            entries,
            highlight,
            goal: goal_value,
            pace_hint,
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
        "Target reached — nice.".to_string()
    } else if goal.progress_pct >= 100.0 {
        "On pace — keep going.".to_string()
    } else {
        format!("{remaining} more to hit your {} target.", goal.year)
    }
}
