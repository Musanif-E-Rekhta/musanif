use dioxus::prelude::*;

use crate::{
    api,
    components::{PageHeader, Tab, TabStrip},
    Route,
};

#[component]
pub fn Shelf() -> Element {
    if cfg!(feature = "mobile") {
        return rsx! { crate::views::mobile::library::MobileLibrary {} };
    }

    let active_status = use_signal(|| "reading".to_string());

    let bookmarks = use_resource(move || {
        let status = active_status();
        async move { api::fetch_my_bookmarks(Some(status)).await }
    });

    // Build a count map across statuses so the tab labels can show counts.
    let counts = use_resource(move || async move {
        api::fetch_my_bookmarks(None)
            .await
            .map(|all| {
                let mut c = ShelfCounts::default();
                for bm in &all {
                    match bm.status.as_str() {
                        "reading" => c.reading += 1,
                        "readlater" => c.want += 1,
                        "completed" => c.finished += 1,
                        "dropped" => c.dropped += 1,
                        _ => {}
                    }
                }
                c
            })
            .unwrap_or_default()
    });

    let shelf_counts = counts.read().as_ref().cloned().unwrap_or_default();

    let tabs = vec![
        Tab::new("reading", "Reading").with_count(shelf_counts.reading),
        Tab::new("readlater", "Want to Read").with_count(shelf_counts.want),
        Tab::new("completed", "Finished").with_count(shelf_counts.finished),
        Tab::new("dropped", "Dropped").with_count(shelf_counts.dropped),
    ];

    rsx! {
        div { class: "island is-main",
            PageHeader {
                title: "My Shelf".to_string(),
                subtitle: "Your saved books".to_string(),
                actions: rsx! {
                    Link {
                        to: Route::Home {},
                        class: "is-btn",
                        "Discover books"
                    }
                },
            }

            div { class: "is-main-body",
                TabStrip { tabs, active: active_status }

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
                        div { class: "row-list",
                            for bm in bms {
                                if let Some(book) = &bm.book {
                                    Link {
                                        key: "{bm.id}",
                                        class: "row-card",
                                        to: Route::BookDetail { slug: book.slug.clone() },
                                        div { class: "row-card-cover",
                                            div { class: "is-book-cover-art",
                                                div { class: "is-book-cover-stamp", "{book.title.chars().next().unwrap_or('م')}" }
                                                div {}
                                                div { class: "is-book-cover-title", "{book.title}" }
                                            }
                                        }
                                        div { class: "row-card-body",
                                            p { class: "row-card-title", "{book.title}" }
                                            p { class: "row-card-meta",
                                                {
                                                    book.authors.as_ref()
                                                        .map(|authors| authors.iter().map(|a| a.author.name.as_str()).collect::<Vec<_>>().join(", "))
                                                        .unwrap_or_default()
                                                }
                                            }
                                            if let Some(progress) = bm.progress {
                                                div { class: "is-progress",
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

#[derive(Default, Clone)]
struct ShelfCounts {
    reading: u32,
    want: u32,
    finished: u32,
    dropped: u32,
}
