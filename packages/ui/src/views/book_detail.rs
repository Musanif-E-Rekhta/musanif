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
                div { class: "book-detail",
                    // ── hero ──────────────────────────────────────────────────
                    div { class: "book-hero",
                        div { class: "book-cover",
                            if let Some(url) = &book.cover_url {
                                img { src: "{url}", alt: "{book.title}" }
                            } else {
                                div { class: "book-cover-placeholder",
                                    span { "{book.title}" }
                                }
                            }
                        }

                        div { class: "book-info",
                            h1 { class: "book-info-title", "{book.title}" }

                            if let Some(authors) = &book.authors {
                                div { class: "book-info-authors",
                                    for ba in authors {
                                        Link {
                                            class: "author-link",
                                            to: Route::AuthorDetail { slug: ba.author.slug.clone() },
                                            "{ba.author.name}"
                                        }
                                        if ba.role != "author" {
                                            span { class: "author-role", " ({ba.role})" }
                                        }
                                    }
                                }
                            }

                            div { class: "book-info-meta",
                                if let Some(rating) = book.avg_rating {
                                    div { class: "meta-item",
                                        span { class: "meta-label", "Rating" }
                                        span { class: "meta-value rating-stars",
                                            "★ {rating:.1}"
                                            span { class: "meta-sub", " ({book.review_count} reviews)" }
                                        }
                                    }
                                }
                                div { class: "meta-item",
                                    span { class: "meta-label", "Chapters" }
                                    span { class: "meta-value", "{book.chapter_count}" }
                                }
                                if let Some(pages) = book.page_count {
                                    div { class: "meta-item",
                                        span { class: "meta-label", "Pages" }
                                        span { class: "meta-value", "{pages}" }
                                    }
                                }
                                div { class: "meta-item",
                                    span { class: "meta-label", "Language" }
                                    span { class: "meta-value", "{book.language.to_uppercase()}" }
                                }
                            }

                            if let Some(cats) = &book.categories {
                                if !cats.is_empty() {
                                    div { class: "book-tags",
                                        for cat in cats {
                                            span { class: "tag tag-category", "{cat.name}" }
                                        }
                                    }
                                }
                            }

                            if let Some(tags) = &book.tags {
                                if !tags.is_empty() {
                                    div { class: "book-tags",
                                        for tag in tags {
                                            span { class: "tag", "#{tag.name}" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // ── description ───────────────────────────────────────────
                    if let Some(desc) = &book.description {
                        section { class: "book-section",
                            h2 { class: "section-title", "About" }
                            p { class: "book-description", "{desc}" }
                        }
                    }

                    // ── table of contents ─────────────────────────────────────
                    section { class: "book-section",
                        h2 { class: "section-title", "Chapters" }
                        match &*chapters.read() {
                            None => rsx! { div { class: "state-loading", "Loading chapters…" } },
                            Some(None) => rsx! {
                                p { class: "state-empty", "No chapters yet." }
                            },
                            Some(Some(chs)) if chs.is_empty() => rsx! {
                                p { class: "state-empty", "No chapters yet." }
                            },
                            Some(Some(chs)) => rsx! {
                                ol { class: "toc",
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
    let nav = use_navigator();
    let ch_slug = chapter.slug.clone();
    let bk_slug = book_slug.clone();

    rsx! {
        li {
            class: "toc-row",
            onclick: move |_| {
                nav.push(Route::ChapterReader {
                    book_slug: bk_slug.clone(),
                    chapter_slug: ch_slug.clone(),
                });
            },

            span { class: "toc-number", "{chapter.number}" }

            span { class: "toc-title",
                if let Some(title) = &chapter.title {
                    "{title}"
                } else {
                    "Chapter {chapter.number}"
                }
            }

            div { class: "toc-meta",
                if let Some(mins) = chapter.reading_time_mins {
                    span { class: "toc-time", "{mins} min" }
                }
                if let Some(rating) = chapter.avg_rating {
                    span { class: "toc-rating", "★ {rating:.1}" }
                }
            }
        }
    }
}
