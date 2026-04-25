use dioxus::prelude::*;

use crate::{api, Route};

const SHELF_CSS: Asset = asset!("/assets/styling/shelf.css");

const STATUSES: &[(&str, &str)] = &[
    ("reading", "Reading"),
    ("readlater", "Want to Read"),
    ("completed", "Completed"),
    ("dropped", "Dropped"),
];

#[component]
fn ShelfHeader() -> Element {
    if cfg!(feature = "mobile") {
        rsx! {
            div { class: "is-mob-header",
                div {
                    p { class: "is-mob-greet", "My Collection" }
                    h1 { class: "is-mob-title", "Shelf" }
                }
            }
        }
    } else {
        rsx! {
            div { class: "is-main-header",
                h2 { class: "is-main-title", "My Shelf" }
                span { class: "is-main-subtitle", "12 books · 3 in progress" }
            }
        }
    }
}

#[component]
pub fn Shelf() -> Element {
    let mut active_status = use_signal(|| "reading".to_string());
    let nav = use_navigator();

    let bookmarks = use_resource(move || {
        let status = active_status();
        async move { api::fetch_my_bookmarks(Some(status)).await }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: SHELF_CSS }

        div { class: "is-main",
            ShelfHeader {}

            div { class: "is-main-body",
                // ── status tabs ───────────────────────────────────────────────────
                div { 
                    style: "display: flex; gap: 4px; margin-bottom: 18px; border-bottom: 1px solid var(--border-light)",
                    for (value, label) in STATUSES {
                        button {
                            style: format_args!(
                                "padding: 10px 14px; border: none; background: none; \
                                 font-family: inherit; font-size: 13px; font-weight: 600; \
                                 cursor: pointer; color: {}; \
                                 border-bottom: 2px solid {}; \
                                 margin-bottom: -1px",
                                if active_status() == *value { "var(--primary)" } else { "var(--text-muted)" },
                                if active_status() == *value { "var(--primary)" } else { "transparent" }
                            ),
                            onclick: {
                                let value = value.to_string();
                                move |_| active_status.set(value.clone())
                            },
                            "{label}"
                        }
                    }
                }

                // ── book list ─────────────────────────────────────────────────────
                match &*bookmarks.read() {
                    None => rsx! { div { class: "state-loading", "Loading…" } },
                    Some(None) => rsx! {
                        div { class: "state-error",
                            "Could not load bookmarks. Make sure you are signed in."
                        }
                    },
                    Some(Some(bms)) if bms.is_empty() => rsx! {
                        div { class: "state-empty",
                            p { "Nothing here yet." }
                            Link { to: Route::Home {}, class: "btn-link", "Discover books →" }
                        }
                    },
                    Some(Some(bms)) => rsx! {
                        div { style: "display: flex; flex-direction: column; gap: 12px",
                            for bm in bms {
                                div {
                                    style: "display: flex; gap: 16px; padding: 14px; \
                                            background: var(--bg-color); border-radius: 12px; \
                                            align-items: center; cursor: pointer",
                                    key: "{bm.id}",
                                    onclick: {
                                        let slug = bm.book.as_ref().map(|b| b.slug.clone()).unwrap_or_default();
                                        move |_| { if !slug.is_empty() { nav.push(Route::BookDetail { slug: slug.clone() }); } }
                                    },
                                    if let Some(book) = &bm.book {
                                        div { class: "is-continue-cover", style: "width: 64px",
                                            div { class: "is-book-cover-art",
                                                div { class: "is-book-cover-stamp", "{book.title.chars().next().unwrap_or(' ')}" }
                                                div {}
                                                div { class: "is-book-cover-title", "{book.title}" }
                                            }
                                        }
                                        div { style: "flex: 1",
                                            p { style: "font-size: 14px; font-weight: 700; margin: 0 0 2px", "{book.title}" }
                                            p { style: "font-size: 12px; color: var(--text-muted); margin: 0 0 8px",
                                                {
                                                    book.authors.as_ref()
                                                        .map(|authors| authors.iter().map(|a| a.author.name.as_str()).collect::<Vec<_>>().join(", "))
                                                        .unwrap_or_default()
                                                }
                                            }
                                            if let Some(progress) = bm.progress {
                                                div { class: "is-progress", style: "height: 4px",
                                                    div {
                                                        class: "is-progress-fill",
                                                        style: if let Some(pages) = book.page_count {
                                                            format!("width: {}%", (progress as f64 / pages as f64 * 100.0).min(100.0))
                                                        } else {
                                                            "width: 0%".to_string()
                                                        },
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        p { style: "font-size: 14px; font-weight: 700; margin: 0", "Book #{bm.book_id}" }
                                    }
                                }
                            }
                        }
                    },
                }
            }
        }
    }
}
