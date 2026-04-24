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
pub fn Shelf() -> Element {
    let mut active_status = use_signal(|| "reading".to_string());

    let bookmarks = use_resource(move || {
        let status = active_status();
        async move { api::fetch_my_bookmarks(Some(status)).await }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: SHELF_CSS }

        div { class: "shelf",
            header { class: "shelf-header",
                h1 { "My Shelf" }
            }

            // ── status tabs ───────────────────────────────────────────────────
            div { class: "shelf-tabs",
                for (value, label) in STATUSES {
                    button {
                        class: if active_status() == *value { "shelf-tab shelf-tab--active" } else { "shelf-tab" },
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
                    div { class: "shelf-list",
                        for bm in bms {
                            div { class: "shelf-item", key: "{bm.id}",
                                if let Some(book) = &bm.book {
                                    div { class: "shelf-item-cover",
                                        if let Some(url) = &book.cover_url {
                                            img { src: "{url}", alt: "{book.title}" }
                                        } else {
                                            div { class: "shelf-cover-placeholder",
                                                span { "{book.title}" }
                                            }
                                        }
                                    }
                                    div { class: "shelf-item-info",
                                        h3 {
                                            class: "shelf-item-title",
                                            Link {
                                                to: Route::BookDetail { slug: book.slug.clone() },
                                                "{book.title}"
                                            }
                                        }
                                        if let Some(progress) = bm.progress {
                                            div { class: "shelf-progress",
                                                div { class: "shelf-progress-bar",
                                                    div {
                                                        class: "shelf-progress-fill",
                                                        style: if let Some(pages) = book.page_count {
                                                            format!("width: {}%", (progress as f64 / pages as f64 * 100.0).min(100.0))
                                                        } else {
                                                            "width: 0%".to_string()
                                                        },
                                                    }
                                                }
                                                span { class: "shelf-progress-label",
                                                    "p. {progress}"
                                                    if let Some(pages) = book.page_count {
                                                        " / {pages}"
                                                    }
                                                }
                                            }
                                        }
                                        if let Some(notes) = &bm.notes {
                                            p { class: "shelf-notes", "{notes}" }
                                        }
                                    }
                                } else {
                                    p { class: "shelf-item-title", "Book #{bm.book_id}" }
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}
