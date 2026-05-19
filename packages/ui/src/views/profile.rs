use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{LdLogIn, LdLogOut, LdSettings, LdShare2, LdUserPlus},
    Icon,
};

use crate::components::{PageHeader, ReadingGoalCard, StatGrid, StatTile};
use crate::{api, Route, CURRENT_USER};

#[component]
pub fn Profile() -> Element {
    if cfg!(feature = "mobile") {
        return rsx! { crate::views::mobile::profile::MobileProfile {} };
    }

    let goal = use_resource(move || async move { api::fetch_my_reading_goal().await });
    let bookmarks =
        use_resource(move || async move { api::fetch_my_bookmarks(Some("completed".into())).await });
    let highlights = use_resource(move || async move { api::fetch_my_highlights().await });
    let stats = use_resource(move || async move { api::fetch_my_stats().await });

    let current_user = CURRENT_USER.read();
    let is_authenticated = current_user.is_some();

    let bms = bookmarks.read().as_ref().and_then(|o| o.clone()).unwrap_or_default();
    let hls = highlights.read().as_ref().and_then(|o| o.clone()).unwrap_or_default();
    let goal_value = goal.read().as_ref().and_then(|o| o.clone());
    let stats_value = stats.read().as_ref().and_then(|o| o.clone());

    let finished_count = stats_value
        .as_ref()
        .map(|s| s.books_completed as usize)
        .unwrap_or_else(|| bms.len());
    let highlights_count = stats_value
        .as_ref()
        .map(|s| s.highlights_count as usize)
        .unwrap_or_else(|| hls.len());
    let day_streak = stats_value.as_ref().map(|s| s.day_streak).unwrap_or(0);

    rsx! {
        div { class: "island is-main",
            PageHeader {
                title: "Profile".to_string(),
                subtitle: "Reading life".to_string(),
                actions: rsx! {
                    if is_authenticated {
                        button { class: "is-btn",
                            Icon { icon: LdShare2, width: 14, height: 14 }
                            "Share"
                        }
                        Link {
                            to: Route::Settings {},
                            class: "is-btn",
                            Icon { icon: LdSettings, width: 14, height: 14 }
                            "Account settings"
                        }
                        button {
                            class: "is-btn",
                            onclick: move |_| {
                                let refresh = api::get_refresh_token();
                                *CURRENT_USER.write() = None;
                                api::clear_all_tokens();
                                if let Some(refresh) = refresh {
                                    spawn(async move {
                                        let _ = api::logout_user(refresh).await;
                                    });
                                }
                            },
                            Icon { icon: LdLogOut, width: 14, height: 14 }
                            "Sign out"
                        }
                    } else {
                        Link {
                            to: Route::Login {},
                            class: "is-btn is-btn--primary",
                            Icon { icon: LdLogIn, width: 14, height: 14 }
                            "Sign In"
                        }
                        Link {
                            to: Route::Signup {},
                            class: "is-btn",
                            Icon { icon: LdUserPlus, width: 14, height: 14 }
                            "Sign Up"
                        }
                    }
                },
            }

            div { class: "is-main-body",
                div { class: "profile-hero",
                    if let Some(user) = current_user.as_ref() {
                        h3 { class: "profile-hero-name", "{user.username}" }
                        p { class: "profile-hero-meta", "{user.email}" }
                    } else {
                        h3 { class: "profile-hero-name", "Guest" }
                        p { class: "profile-hero-meta",
                            "Sign in to sync your progress and highlights."
                        }
                    }
                }

                StatGrid {
                    StatTile {
                        value: finished_count.to_string(),
                        label: "Books finished".to_string(),
                    }
                    StatTile {
                        value: goal_value.as_ref().map(|g| g.completed.to_string()).unwrap_or_else(|| "0".to_string()),
                        label: format!("In {}", goal_value.as_ref().map(|g| g.year.to_string()).unwrap_or_else(|| "this year".into())),
                    }
                    StatTile {
                        value: highlights_count.to_string(),
                        label: "Highlights".to_string(),
                    }
                    StatTile {
                        value: day_streak.to_string(),
                        label: "Day streak".to_string(),
                    }
                }

                if let Some(g) = goal_value.clone() {
                    div { style: "margin-bottom: 24px",
                        ReadingGoalCard {
                            year: g.year,
                            completed: g.completed,
                            target: g.target,
                            pace_hint: pace_hint(&g),
                        }
                    }
                }

                div { class: "profile-cols",
                    div {
                        h4 { class: "profile-col-head", "Recently Finished" }
                        if bms.is_empty() {
                            p { class: "state-empty", style: "padding: 18px 0; text-align: left",
                                "No finished books yet."
                            }
                        } else {
                            div { class: "profile-recent",
                                for bm in bms.iter().take(6) {
                                    if let Some(book) = &bm.book {
                                        Link {
                                            key: "{bm.id}",
                                            to: Route::BookDetail { slug: book.slug.clone() },
                                            class: "profile-recent-row",
                                            div { class: "is-continue-cover",
                                                div { class: "is-book-cover-art",
                                                    div { class: "is-book-cover-stamp",
                                                        "{book.title.chars().next().unwrap_or('م')}"
                                                    }
                                                    div {}
                                                    div { class: "is-book-cover-title", "{book.title}" }
                                                }
                                            }
                                            div { style: "flex: 1; min-width: 0",
                                                p { class: "row-card-title", "{book.title}" }
                                                p { class: "row-card-meta",
                                                    {
                                                        book.authors.as_ref()
                                                            .map(|a| a.iter().map(|x| x.author.name.as_str()).collect::<Vec<_>>().join(", "))
                                                            .unwrap_or_default()
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div {
                        h4 { class: "profile-col-head", "Recent Highlights" }
                        if hls.is_empty() {
                            p { class: "state-empty", style: "padding: 18px 0; text-align: left",
                                "No highlights yet."
                            }
                        } else {
                            div { style: "display: flex; flex-direction: column; gap: 12px",
                                for h in hls.iter().take(5) {
                                    div { key: "{h.id}", class: "profile-quote",
                                        p { class: "is-quote", "\u{201C}{h.text_snapshot}\u{201D}" }
                                        if let Some(note) = &h.note {
                                            p { class: "is-quote-source", "{note}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn pace_hint(goal: &crate::models::ReadingGoal) -> Option<String> {
    let remaining = (goal.target - goal.completed).max(0);
    if remaining == 0 {
        Some("Target reached, nice.".to_string())
    } else {
        Some(format!("{remaining} to go to hit your {} target.", goal.year))
    }
}
