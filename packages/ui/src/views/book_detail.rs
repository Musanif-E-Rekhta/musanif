use dioxus::prelude::*;

use crate::{api, components::Cover, models::ChapterSummary, Route};

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
                    div { class: "is-main-header",
                        button { class: "is-btn is-btn--ghost",
                            onclick: move |_| {},
                            "← Discover"
                        }
                        span { class: "is-main-subtitle", "Poetry · Classical" }
                        div { class: "is-main-actions",
                            button { class: "is-btn", "Add to Shelf" }
                            button { class: "is-btn", "Share" }
                        }
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
                                div { style: "display: flex; flex-direction: column; gap: 6px; margin-top: 14px",
                                    button { class: "is-btn is-btn--primary", style: "justify-content: center",
                                        "Start reading"
                                    }
                                    button { class: "is-btn", style: "justify-content: center",
                                        "Add to shelf"
                                    }
                                }
                            }

                            // Right column: info + TOC
                            div {
                                span { class: "is-chip", "Urdu" }
                                if let Some(categories) = &book.categories {
                                    for cat in categories.iter().take(2) {
                                        span { class: "is-chip", style: "margin-left: 4px", "{cat.name}" }
                                    }
                                }

                                h1 { class: "is-detail-h1", style: "margin-top: 10px", "{book.title}" }

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

                                div { class: "is-detail-stats",
                                    div {
                                        div { class: "is-detail-stat-label", "Rating" }
                                        div { class: "is-detail-stat-value",
                                            if let Some(r) = book.avg_rating {
                                                "★ {r:.1}"
                                            } else {
                                                "N/A"
                                            }
                                        }
                                    }
                                    div {
                                        div { class: "is-detail-stat-label", "Chapters" }
                                        div { class: "is-detail-stat-value", "{book.chapter_count}" }
                                    }
                                    if let Some(pages) = book.page_count {
                                        div {
                                            div { class: "is-detail-stat-label", "Pages" }
                                            div { class: "is-detail-stat-value", "{pages}" }
                                        }
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
                                    div { style: "display: flex; gap: 6px; flex-wrap: wrap; margin-bottom: 8px",
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
