use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons::LdSettings, Icon};

use crate::{api, state::CURRENT_USER, Route};

#[component]
pub fn MobileProfile() -> Element {
    let goal = use_resource(move || async move { api::fetch_my_reading_goal().await });
    let bookmarks =
        use_resource(move || async move { api::fetch_my_bookmarks(Some("completed".into())).await });
    let highlights = use_resource(move || async move { api::fetch_my_highlights().await });

    let current_user = CURRENT_USER.read();
    let username = current_user
        .as_ref()
        .map(|u| u.username.clone())
        .unwrap_or_else(|| "Guest".to_string());

    let finished = bookmarks.read().as_ref().and_then(|o| o.clone()).map(|v| v.len()).unwrap_or(0);
    let highlights_count = highlights.read().as_ref().and_then(|o| o.clone()).map(|v| v.len()).unwrap_or(0);
    let goal_value = goal.read().as_ref().and_then(|o| o.clone());

    rsx! {
        div { class: "is-mob-header",
            div {
                p { class: "is-mob-greet", "Profile" }
                h1 { class: "is-mob-title", "{username}" }
            }
            Link {
                to: Route::Settings {},
                class: "is-icon-btn",
                style: "width: 36px; height: 36px; text-decoration: none",
                Icon { icon: LdSettings, width: 16, height: 16 }
            }
        }

        // Avatar card
        div {
            class: "island is-rail-card",
            style: "margin: 0 4px 14px; text-align: center; padding: 22px",
            div {
                style: "width: 84px; height: 84px; border-radius: 50%; background: var(--primary); color: var(--bg-card); display: flex; align-items: center; justify-content: center; font-size: 32px; font-weight: 700; margin: 0 auto 12px",
                "{username.chars().next().unwrap_or('?').to_ascii_uppercase()}"
            }
            p { style: "margin: 0 0 4px; font-size: 16px; font-weight: 700", "{username}" }
            if let Some(user) = current_user.as_ref() {
                p { style: "margin: 0; font-size: 12px; color: var(--text-muted)", "{user.email}" }
            } else {
                p { style: "margin: 0; font-size: 12px; color: var(--text-muted)",
                    "Sign in to sync your progress."
                }
            }
        }

        // Stats grid
        div { class: "is-mob-stats",
            StatTile {
                value: finished.to_string(),
                label: "Books finished".to_string(),
            }
            StatTile {
                value: goal_value.as_ref().map(|g| g.completed.to_string()).unwrap_or_else(|| "0".to_string()),
                label: format!("In {}", goal_value.as_ref().map(|g| g.year.to_string()).unwrap_or_else(|| "year".to_string())),
            }
            StatTile {
                value: highlights_count.to_string(),
                label: "Highlights".to_string(),
            }
            StatTile { value: "0".to_string(), label: "Day streak".to_string() }
        }

        // Reading goal
        if let Some(g) = goal_value {
            ReadingGoalIsland { year: g.year, completed: g.completed, target: g.target }
        }

        // Recent highlights
        div { class: "is-mob-section",
            span { "Recent highlights" }
            span { class: "is-mob-section-link", "All" }
        }
        match &*highlights.read() {
            Some(Some(hs)) if !hs.is_empty() => rsx! {
                div { class: "island is-rail-card", style: "margin: 0 4px 14px",
                    p { class: "is-quote", "\u{201C}{hs[0].text_snapshot}\u{201D}" }
                    if let Some(note) = &hs[0].note {
                        div { class: "is-quote-source", "{note}" }
                    }
                }
            },
            _ => rsx! {
                div { class: "state-empty", style: "padding: 18px 4px; text-align: left",
                    "No highlights yet."
                }
            },
        }
    }
}

#[component]
fn StatTile(value: String, label: String) -> Element {
    rsx! {
        div { class: "is-mob-stat",
            p { class: "is-mob-stat-value", "{value}" }
            p { class: "is-mob-stat-label", "{label}" }
        }
    }
}

#[component]
fn ReadingGoalIsland(year: i32, completed: i32, target: i32) -> Element {
    let pct = if target > 0 {
        (completed as f64 / target as f64 * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };
    rsx! {
        div { class: "island is-rail-card", style: "margin: 0 4px 14px",
            div { class: "is-rail-eyebrow", "{year} Goal" }
            p { style: "margin: 8px 0 4px; font-family: var(--font-serif, 'Crimson Pro', Georgia, serif)",
                span { style: "font-size: 26px; font-weight: 700", "{completed}" }
                span { style: "font-size: 13px; color: var(--text-muted)", " / {target} books" }
            }
            div { class: "is-progress", style: "height: 5px; margin-top: 8px",
                div { class: "is-progress-fill", style: "width: {pct:.0}%" }
            }
        }
    }
}
