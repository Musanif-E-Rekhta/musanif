use dioxus::prelude::*;

use crate::{api, models::Bookmark, Route};

const STATUSES: &[(&str, &str)] = &[
    ("reading", "Reading"),
    ("readlater", "Want"),
    ("completed", "Finished"),
    ("dropped", "Dropped"),
];

#[component]
pub fn MobileLibrary() -> Element {
    let mut active = use_signal(|| "reading".to_string());
    let bookmarks = use_resource(move || {
        let s = active();
        async move { api::fetch_my_bookmarks(Some(s)).await }
    });
    let counts =
        use_resource(move || async move { api::fetch_my_bookmarks(None).await.unwrap_or_default() });

    let count_of = |status: &str| -> usize {
        counts
            .read()
            .as_ref()
            .map(|all| all.iter().filter(|b| b.status == status).count())
            .unwrap_or(0)
    };

    rsx! {
        div { class: "is-mob-header",
            div {
                p { class: "is-mob-greet", "{count_of(\"reading\") + count_of(\"completed\")} books" }
                h1 { class: "is-mob-title", "My Shelf" }
            }
            button { class: "is-icon-btn", style: "width: 36px; height: 36px", "+" }
        }

        div {
            style: "display: flex; gap: 6px; padding: 0 14px 12px; overflow-x: auto",
            for (id, label) in STATUSES {
                button {
                    key: "{id}",
                    class: if *active.read() == *id { "is-chip is-chip--primary" } else { "is-chip" },
                    style: "border: none; cursor: pointer; font-family: inherit",
                    onclick: {
                        let id = id.to_string();
                        move |_| active.set(id.clone())
                    },
                    "{label} · {count_of(id)}"
                }
            }
        }

        match &*bookmarks.read() {
            None => rsx! { div { class: "state-loading", "Loading…" } },
            Some(None) => rsx! {
                div { class: "state-error", "Could not load bookmarks." }
            },
            Some(Some(bms)) if bms.is_empty() => rsx! {
                div { class: "state-empty",
                    p { "Nothing here yet." }
                    Link { to: Route::Home {}, class: "btn-link", "Discover books →" }
                }
            },
            Some(Some(bms)) => rsx! {
                div { class: "is-mob-list",
                    for bm in bms {
                        if bm.book.is_some() {
                            BookRow { key: "{bm.id}", bookmark: bm.clone() }
                        }
                    }
                }
            },
        }
    }
}

#[component]
fn BookRow(bookmark: Bookmark) -> Element {
    let book = bookmark.book.as_ref().expect("filtered above");
    let pct = match (bookmark.progress, book.page_count) {
        (Some(p), Some(pages)) if pages > 0 => {
            (p as f64 / pages as f64 * 100.0).clamp(0.0, 100.0)
        }
        _ => 0.0,
    };
    let author = book
        .authors
        .as_ref()
        .and_then(|a| a.first())
        .map(|x| x.author.name.clone())
        .unwrap_or_default();

    rsx! {
        Link {
            class: "is-mob-row",
            to: Route::BookDetail { slug: book.slug.clone() },
            div { class: "is-mob-row-cover",
                div { class: "is-book-cover-art",
                    div { class: "is-book-cover-stamp",
                        "{book.title.chars().next().unwrap_or('م')}"
                    }
                    div {}
                    div { class: "is-book-cover-title", "{book.title}" }
                }
            }
            div { class: "is-mob-row-info",
                p { class: "is-mob-row-title", "{book.title}" }
                p { class: "is-mob-row-meta", "{author}" }
                div { class: "is-progress is-mob-row-progress",
                    div { class: "is-progress-fill", style: "width: {pct:.0}%" }
                }
            }
        }
    }
}
