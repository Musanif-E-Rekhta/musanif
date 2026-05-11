use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{LdArrowLeft, LdArrowRight, LdType},
    Icon,
};
use pulldown_cmark::{html, Options, Parser};

use crate::{api, Route};

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
pub fn MobileReader(book_slug: String, chapter_slug: String) -> Element {
    let book_slug = use_memo(move || book_slug.clone());
    let chapter_slug = use_memo(move || chapter_slug.clone());

    let chapter = use_resource(move || {
        let bs = book_slug();
        let cs = chapter_slug();
        async move { api::fetch_chapter(bs, cs).await }
    });

    rsx! {
        match &*chapter.read() {
            None => rsx! { div { class: "state-loading", "Loading chapter…" } },
            Some(None) => rsx! {
                div { class: "state-error",
                    p { "Chapter not found." }
                    Link { to: Route::Home {}, class: "btn-link", "← Back to home" }
                }
            },
            Some(Some(ch)) => {
                let html_content = to_html(&ch.content, &ch.content_format);
                let book_slug_nav = ch.book.as_ref().map(|b| b.slug.clone()).unwrap_or_default();
                let book_title = ch.book.as_ref().map(|b| b.title.clone()).unwrap_or_default();

                rsx! {
                    div { class: "is-mob-reader-chrome",
                        Link {
                            class: "is-mob-reader-back",
                            to: Route::BookDetail { slug: book_slug_nav.clone() },
                            Icon { icon: LdArrowLeft, width: 16, height: 16 }
                            "{book_title}"
                        }
                        button { class: "is-icon-btn", style: "width: 36px; height: 36px",
                            Icon { icon: LdType, width: 16, height: 16 }
                        }
                    }

                    div { style: "text-align: center",
                        span { class: "is-mob-reader-pill",
                            "Ch. {ch.number}"
                            if let Some(mins) = ch.reading_time_mins {
                                " · {mins} min"
                            }
                        }
                    }

                    div { class: "island is-mob-reader-card",
                        div { class: "is-reader-meta",
                            span { "Chapter {ch.number}" }
                            if let Some(mins) = ch.reading_time_mins {
                                span { "·" }
                                span { "{mins} min" }
                            }
                        }
                        h1 { class: "is-reader-h1",
                            if let Some(t) = &ch.title { "{t}" } else { "Chapter {ch.number}" }
                        }
                        if let Some(s) = &ch.summary {
                            p { class: "is-reader-lede", "{s}" }
                        }
                        hr { class: "is-reader-rule" }
                        div {
                            class: "is-reader-body",
                            dangerous_inner_html: "{html_content}",
                        }
                    }

                    div { class: "is-mob-action-bar",
                        if let Some(prev) = &ch.prev_chapter {
                            Link {
                                to: Route::ChapterReader {
                                    book_slug: book_slug_nav.clone(),
                                    chapter_slug: prev.slug.clone(),
                                },
                                class: "is-btn",
                                Icon { icon: LdArrowLeft, width: 14, height: 14 }
                                "Prev"
                            }
                        } else {
                            span { class: "is-btn", style: "opacity: 0.4; pointer-events: none",
                                Icon { icon: LdArrowLeft, width: 14, height: 14 }
                                "Prev"
                            }
                        }
                        if let Some(next) = &ch.next_chapter {
                            Link {
                                to: Route::ChapterReader {
                                    book_slug: book_slug_nav.clone(),
                                    chapter_slug: next.slug.clone(),
                                },
                                class: "is-btn is-btn--primary",
                                "Next"
                                Icon { icon: LdArrowRight, width: 14, height: 14 }
                            }
                        } else {
                            span { class: "is-btn is-btn--primary", style: "opacity: 0.4; pointer-events: none",
                                "Next"
                                Icon { icon: LdArrowRight, width: 14, height: 14 }
                            }
                        }
                    }
                }
            },
        }
    }
}
