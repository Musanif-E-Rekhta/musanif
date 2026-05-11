use dioxus::prelude::*;

use crate::{
    api,
    components::{Cover, PageHeader, StatGrid, StatTile},
    models::ChapterSummary,
    Route,
};

#[component]
pub fn BookDetail(slug: String) -> Element {
    let slug = use_memo(move || slug.clone());

    let book = use_resource(move || {
        let s = slug();
        async move { api::fetch_book(s).await }
    });

    let chapters = use_resource(move || {
        let s = slug();
        async move { api::fetch_chapters(s).await }
    });

    rsx! {
        match &*book.read() {
            None => rsx! {
                div { class: "island is-main",
                    div { class: "state-loading", "Loading…" }
                }
            },
            Some(None) => rsx! {
                div { class: "island is-main",
                    div { class: "state-error", "Book not found." }
                }
            },
            Some(Some(book)) => rsx! {
                div { class: "island is-main",
                    PageHeader {
                        title_slot: rsx! {
                            Link {
                                to: Route::Home {},
                                class: "is-btn is-btn--ghost",
                                "← Discover"
                            }
                        },
                        subtitle: "Poetry · Classical".to_string(),
                        actions: rsx! {
                            button { class: "is-btn", "Add to Shelf" }
                            button { class: "is-btn", "Share" }
                        },
                    }

                    div { class: "is-main-body",
                        div { class: "is-detail",
                            // Left column: cover + actions
                            div {
                                div { class: "is-detail-cover",
                                    Cover {
                                        urdu: Some(book.title.clone()),
                                        mono: Some(book.title.chars().next().unwrap_or('م').to_string()),
                                        big: true,
                                    }
                                }
                                div { class: "is-detail-actions",
                                    button { class: "is-btn is-btn--primary is-btn--block",
                                        "Start reading"
                                    }
                                    button { class: "is-btn is-btn--block",
                                        "Add to shelf"
                                    }
                                }
                            }

                            // Right column: info + TOC
                            div {
                                div { class: "is-detail-chips",
                                    span { class: "is-chip", "Urdu" }
                                    if let Some(categories) = &book.categories {
                                        for cat in categories.iter().take(2) {
                                            span { class: "is-chip", "{cat.name}" }
                                        }
                                    }
                                }

                                h1 { class: "is-detail-h1", "{book.title}" }

                                p { class: "is-detail-author",
                                    if let Some(authors) = &book.authors {
                                        for (i, ba) in authors.iter().enumerate() {
                                            if i > 0 { ", " }
                                            Link {
                                                to: Route::AuthorDetail { slug: ba.author.slug.clone() },
                                                class: "is-link",
                                                "{ba.author.name}"
                                            }
                                        }
                                    } else {
                                        "Unknown Author"
                                    }
                                }

                                StatGrid {
                                    StatTile {
                                        value: book.avg_rating
                                            .map(|r| format!("★ {r:.1}"))
                                            .unwrap_or_else(|| "—".to_string()),
                                        label: "Rating".to_string(),
                                    }
                                    StatTile {
                                        value: book.chapter_count.to_string(),
                                        label: "Chapters".to_string(),
                                    }
                                    StatTile {
                                        value: book.page_count
                                            .map(|p| p.to_string())
                                            .unwrap_or_else(|| "—".to_string()),
                                        label: "Pages".to_string(),
                                    }
                                    StatTile {
                                        value: book.review_count.to_string(),
                                        label: "Reviews".to_string(),
                                    }
                                }

                                p { class: "is-detail-blurb",
                                    if let Some(desc) = &book.description {
                                        "{desc}"
                                    } else {
                                        "No description available for this work."
                                    }
                                }

                                if let Some(tags) = &book.tags {
                                    div { class: "is-detail-tags",
                                        for tag in tags.iter().take(5) {
                                            span { class: "is-chip", "{tag.name}" }
                                        }
                                    }
                                }

                                div { class: "is-toc-title", "Table of Contents" }

                                match &*chapters.read() {
                                    None => rsx! { div { class: "state-loading", "Loading chapters…" } },
                                    Some(None) => rsx! { p { class: "state-empty", "No chapters yet." } },
                                    Some(Some(chs)) if chs.is_empty() => rsx! {
                                        p { class: "state-empty", "No chapters yet." }
                                    },
                                    Some(Some(chs)) => rsx! {
                                        div {
                                            for ch in chs {
                                                TocRow {
                                                    key: "{ch.id}",
                                                    chapter: ch.clone(),
                                                    book_slug: book.slug.clone(),
                                                }
                                            }
                                        }
                                    },
                                }
                            }
                        }
                    }
                }
            },
        }
    }
}

#[component]
fn TocRow(chapter: ChapterSummary, book_slug: String) -> Element {
    rsx! {
        Link {
            class: "is-toc-row",
            to: Route::ChapterReader {
                book_slug: book_slug.clone(),
                chapter_slug: chapter.slug.clone(),
            },
            span { class: "is-toc-num", "{chapter.number:02}" }
            span { class: "is-toc-name",
                if let Some(title) = &chapter.title {
                    "{title}"
                } else {
                    "Chapter {chapter.number}"
                }
            }
            if let Some(mins) = chapter.reading_time_mins {
                span { class: "is-toc-time", "{mins} min" }
            }
        }
    }
}
