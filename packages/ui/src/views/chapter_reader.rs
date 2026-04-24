use dioxus::prelude::*;
use pulldown_cmark::{html, Options, Parser};

use crate::{api, Route};

const READER_CSS: Asset = asset!("/assets/styling/reader.css");

fn to_html(content: &str, format: &str) -> String {
    match format {
        "markdown" => {
            let parser = Parser::new_ext(content, Options::all());
            let mut out = String::new();
            html::push_html(&mut out, parser);
            out
        }
        "html" => content.to_string(),
        _ => format!("<pre class=\"plaintext\">{content}</pre>"),
    }
}

#[component]
pub fn ChapterReader(book_slug: String, chapter_slug: String) -> Element {
    let book_slug = use_memo(move || book_slug.clone());
    let chapter_slug = use_memo(move || chapter_slug.clone());

    let chapter = use_resource(move || {
        let bs = book_slug();
        let cs = chapter_slug();
        async move { api::fetch_chapter(bs, cs).await }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: READER_CSS }

        match &*chapter.read() {
            None => rsx! { div { class: "state-loading reader-loading", "Loading chapter…" } },
            Some(None) => rsx! {
                div { class: "state-error",
                    p { "Chapter not found." }
                    Link { to: Route::Home {}, class: "btn-link", "← Back to home" }
                }
            },
            Some(Some(ch)) => {
                let html_content = to_html(&ch.content, &ch.content_format);
                let book_slug_nav = ch.book.as_ref().map(|b| b.slug.clone()).unwrap_or_default();

                rsx! {
                    div { class: "reader",
                        // ── reader topbar ──────────────────────────────────────
                        div { class: "reader-topbar",
                            Link {
                                class: "reader-back",
                                to: Route::BookDetail { slug: book_slug_nav.clone() },
                                "←"
                                if let Some(book) = &ch.book {
                                    " {book.title}"
                                }
                            }
                            div { class: "reader-topbar-meta",
                                if let Some(mins) = ch.reading_time_mins {
                                    span { class: "reader-time", "{mins} min read" }
                                }
                                if let Some(rating) = ch.avg_rating {
                                    span { class: "reader-rating", "★ {rating:.1}" }
                                }
                            }
                        }

                        // ── chapter header ─────────────────────────────────────
                        header { class: "reader-header",
                            p { class: "reader-chapter-num", "Chapter {ch.number}" }
                            h1 { class: "reader-title",
                                if let Some(title) = &ch.title {
                                    "{title}"
                                } else {
                                    "Chapter {ch.number}"
                                }
                            }
                            if let Some(summary) = &ch.summary {
                                p { class: "reader-summary", "{summary}" }
                            }
                        }

                        // ── navigation (top) ───────────────────────────────────
                        ChapterNav {
                            book_slug: book_slug_nav.clone(),
                            prev: ch.prev_chapter.clone(),
                            next: ch.next_chapter.clone(),
                        }

                        // ── content ────────────────────────────────────────────
                        article {
                            class: "reader-content",
                            dangerous_inner_html: "{html_content}",
                        }

                        // ── navigation (bottom) ────────────────────────────────
                        ChapterNav {
                            book_slug: book_slug_nav.clone(),
                            prev: ch.prev_chapter.clone(),
                            next: ch.next_chapter.clone(),
                        }
                    }
                }
            },
        }
    }
}

#[component]
fn ChapterNav(
    book_slug: String,
    prev: Option<crate::models::ChapterNav>,
    next: Option<crate::models::ChapterNav>,
) -> Element {
    rsx! {
        nav { class: "chapter-nav",
            div { class: "chapter-nav-prev",
                if let Some(prev) = &prev {
                    Link {
                        to: Route::ChapterReader {
                            book_slug: book_slug.clone(),
                            chapter_slug: prev.slug.clone(),
                        },
                        class: "chapter-nav-btn",
                        "← "
                        if let Some(title) = &prev.title {
                            "{title}"
                        } else {
                            "Chapter {prev.number}"
                        }
                    }
                }
            }
            div { class: "chapter-nav-next",
                if let Some(next) = &next {
                    Link {
                        to: Route::ChapterReader {
                            book_slug: book_slug.clone(),
                            chapter_slug: next.slug.clone(),
                        },
                        class: "chapter-nav-btn",
                        if let Some(title) = &next.title {
                            "{title}"
                        } else {
                            "Chapter {next.number}"
                        }
                        " →"
                    }
                }
            }
        }
    }
}
