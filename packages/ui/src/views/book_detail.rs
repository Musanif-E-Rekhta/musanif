use dioxus::prelude::*;

use crate::{api, models::ChapterSummary, Route};

const BOOK_CSS: Asset = asset!("/assets/styling/book.css");

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
        document::Link { rel: "stylesheet", href: BOOK_CSS }

        match &*book.read() {
            None => rsx! { div { class: "state-loading", "Loading…" } },
            Some(None) => rsx! { div { class: "state-error", "Book not found." } },
            Some(Some(book)) => rsx! {
                div { class: "is-main",
                    div { class: "is-main-header",
                        h2 { class: "is-main-title", "{book.title}" }
                    }

                    div { class: "is-main-body",
                        div { class: "is-detail",
                            div { class: "is-detail-cover",
                                div { class: "is-book-cover-art",
                                    div { class: "is-book-cover-stamp", "{book.title.chars().next().unwrap_or(' ')}" }
                                    div {}
                                    div { class: "is-book-cover-title", "{book.title}" }
                                }
                            }

                            div {
                                h1 { class: "is-detail-h1", "{book.title}" }
                                p { class: "is-detail-author",
                                    if let Some(authors) = &book.authors {
                                        for ba in authors {
                                            Link {
                                                to: Route::AuthorDetail { slug: ba.author.slug.clone() },
                                                "{ba.author.name}"
                                            }
                                            if ba.role != "author" {
                                                " ({ba.role})"
                                            }
                                            " "
                                        }
                                    }
                                }

                                div { class: "is-detail-stats",
                                    div {
                                        div { class: "is-detail-stat-label", "Rating" }
                                        div { class: "is-detail-stat-value", 
                                            if let Some(rating) = book.avg_rating {
                                                "{rating:.1} ★"
                                            } else {
                                                "N/A"
                                            }
                                        }
                                    }
                                    div {
                                        div { class: "is-detail-stat-label", "Chapters" }
                                        div { class: "is-detail-stat-value", "{book.chapter_count}" }
                                    }
                                    div {
                                        div { class: "is-detail-stat-label", "Pages" }
                                        div { class: "is-detail-stat-value", 
                                            if let Some(pages) = book.page_count {
                                                "{pages}"
                                            } else {
                                                "N/A"
                                            }
                                        }
                                    }
                                }

                                if let Some(desc) = &book.description {
                                    p { class: "is-detail-blurb", "{desc}" }
                                }

                                div { class: "is-detail-actions",
                                    button { class: "is-btn is-btn--primary", "Read Now" }
                                    button { class: "is-btn", "Add to Shelf" }
                                }
                            }
                        }

                        div { class: "is-toc-title", "Table of Contents" }
                        match &*chapters.read() {
                            None => rsx! { div { class: "state-loading", "Loading chapters…" } },
                            Some(None) => rsx! {
                                p { class: "state-empty", "No chapters yet." }
                            },
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
            style: "text-decoration: none; color: inherit",

            span { class: "is-toc-num", "{chapter.number}" }

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
