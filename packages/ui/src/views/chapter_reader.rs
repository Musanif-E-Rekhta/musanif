use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{LdArrowLeft, LdArrowRight, LdBookmark, LdMoon, LdEllipsis, LdType},
    Icon,
};
use pulldown_cmark::{html, Options, Parser};

use crate::{api, models::RecordReadingSessionInput, Route};

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
    if cfg!(feature = "mobile") {
        return rsx! {
            crate::views::mobile::reader::MobileReader {
                book_slug: book_slug.clone(),
                chapter_slug: chapter_slug.clone(),
            }
        };
    }

    let book_slug = use_memo(move || book_slug.clone());
    let chapter_slug = use_memo(move || chapter_slug.clone());

    let chapter = use_resource(move || {
        let bs = book_slug();
        let cs = chapter_slug();
        async move { api::fetch_chapter(bs, cs).await }
    });

    // Track when the user opened this chapter; on unmount fire a
    // record_reading_session mutation so the backend can stitch together
    // an activity stream. Sessions <1 min are dropped as noise.
    let started_at: Signal<DateTime<Utc>> = use_signal(Utc::now);
    use_drop(move || {
        let started = *started_at.peek();
        let bs = book_slug();
        let cs = chapter_slug();
        let now = Utc::now();
        let mins = ((now - started).num_seconds() / 60) as i32;
        if mins < 1 {
            return;
        }
        spawn(async move {
            let _ = api::record_reading_session(RecordReadingSessionInput {
                book_slug: bs,
                chapter_slug: Some(cs),
                started_at: started.to_rfc3339(),
                ended_at: Some(now.to_rfc3339()),
                duration_mins: Some(mins),
                page_start: None,
                page_end: None,
                device: None,
            })
            .await;
        });
    });

    rsx! {
        match &*chapter.read() {
            None => rsx! { div { class: "island is-main", div { class: "state-loading", "Loading chapter…" } } },
            Some(None) => rsx! {
                div { class: "island is-main",
                    div { class: "state-error",
                        p { "Chapter not found." }
                        Link { to: Route::Home {}, class: "btn-link", "← Back to home" }
                    }
                }
            },
            Some(Some(ch)) => {
                let html_content = to_html(&ch.content, &ch.content_format);
                let book_slug_nav = ch.book.as_ref().map(|b| b.slug.clone()).unwrap_or_default();

                rsx! {
                    div { class: "island is-main",
                        // ── reader topbar ──────────────────────────────────────
                        div { class: "is-main-header",
                            Link {
                                class: "is-btn is-btn--ghost",
                                to: Route::BookDetail { slug: book_slug_nav.clone() },
                                Icon { icon: LdArrowLeft, width: 16, height: 16, class: "is-nav-item-icon" }
                                if let Some(book) = &ch.book {
                                    " {book.title}"
                                }
                            }
                            span { class: "is-main-subtitle",
                                "Chapter {ch.number}"
                                if let Some(mins) = ch.reading_time_mins {
                                    " · {mins} min read"
                                }
                            }
                            div { class: "is-main-actions",
                                button { class: "is-icon-btn", aria_label: "Type",
                                    Icon { icon: LdType, width: 14, height: 14 }
                                }
                                button { class: "is-icon-btn", aria_label: "Theme",
                                    Icon { icon: LdMoon, width: 14, height: 14 }
                                }
                                button { class: "is-icon-btn", aria_label: "Bookmark",
                                    Icon { icon: LdBookmark, width: 14, height: 14 }
                                }
                                button { class: "is-icon-btn", aria_label: "More",
                                    Icon { icon: LdEllipsis, width: 14, height: 14 }
                                }
                            }
                        }

                        div { class: "is-main-body is-main-body--reader",
                            // ── chapter header ─────────────────────────────────────
                            div { class: "is-reader-meta",
                                span { "Chapter {ch.number}" }
                                if let Some(mins) = ch.reading_time_mins {
                                    span { "{mins} min left" }
                                }
                            }

                            h1 { class: "is-reader-h1",
                                if let Some(title) = &ch.title {
                                    "{title}"
                                } else {
                                    "Chapter {ch.number}"
                                }
                            }

                            if let Some(summary) = &ch.summary {
                                p { class: "is-reader-lede", "{summary}" }
                            }

                            hr { class: "is-reader-rule" }

                            // ── content ────────────────────────────────────────────
                            div {
                                class: "is-reader-body",
                                dangerous_inner_html: "{html_content}",
                            }

                            hr { class: "is-reader-rule" }

                            // ── navigation (bottom) ────────────────────────────────
                            ChapterNav {
                                book_slug: book_slug_nav.clone(),
                                prev: ch.prev_chapter.clone(),
                                next: ch.next_chapter.clone(),
                            }
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
        nav { class: "reader-nav",
            div {
                if let Some(prev) = &prev {
                    Link {
                        to: Route::ChapterReader {
                            book_slug: book_slug.clone(),
                            chapter_slug: prev.slug.clone(),
                        },
                        class: "is-btn",
                        Icon { icon: LdArrowLeft, width: 16, height: 16, class: "is-nav-item-icon" }
                        if let Some(title) = &prev.title {
                            "{title}"
                        } else {
                            "Chapter {prev.number}"
                        }
                    }
                }
            }
            div {
                if let Some(next) = &next {
                    Link {
                        to: Route::ChapterReader {
                            book_slug: book_slug.clone(),
                            chapter_slug: next.slug.clone(),
                        },
                        class: "is-btn is-btn--primary",
                        if let Some(title) = &next.title {
                            "{title}"
                        } else {
                            "Chapter {next.number}"
                        }
                        Icon { icon: LdArrowRight, width: 16, height: 16, class: "is-nav-item-icon" }
                    }
                }
            }
        }
    }
}
