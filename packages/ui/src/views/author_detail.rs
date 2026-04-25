use dioxus::prelude::*;

use crate::{api, models::Book, Route};

#[component]
pub fn AuthorDetail(slug: String) -> Element {
    let slug = use_memo(move || slug.clone());

    let author = use_resource(move || {
        let s = slug();
        async move { api::fetch_author(s).await }
    });

    let books = use_resource(move || {
        let s = slug();
        async move { api::fetch_author_books(s).await }
    });

    rsx! {
        match &*author.read() {
            None => rsx! { div { class: "island is-main", div { class: "state-loading", "Loading…" } } },
            Some(None) => rsx! { div { class: "island is-main", div { class: "state-error", "Author not found." } } },
            Some(Some(author)) => rsx! {
                div { class: "island is-main",
                    div { class: "is-main-header",
                        h2 { class: "is-main-title", "{author.name}" }
                        span { class: "is-main-subtitle", "{author.followers} followers" }
                    }

                    div { class: "is-main-body",
                        div {
                            style: "display: flex; gap: 14px; padding: 14px; \
                                    background: var(--bg-color); border-radius: 12px; \
                                    align-items: center; margin-bottom: 24px",
                            div {
                                style: "width: 48px; height: 48px; border-radius: 50%; \
                                        background: var(--accent-light); color: var(--primary); \
                                        display: flex; align-items: center; justify-content: center; \
                                        font-family: var(--font-urdu); fontSize: 20px; fontWeight: 700; flex-shrink: 0",
                                "{author.name.chars().next().unwrap_or(' ')}"
                            }
                            div {
                                p { style: "font-size: 14px; font-weight: 700; margin: 0 0 2px", "{author.name}" }
                                if let Some(bio) = &author.bio {
                                    p { style: "font-size: 11px; color: var(--text-muted); margin: 0", "{bio}" }
                                }
                            }
                        }

                        div { class: "is-grid",
                            match &*books.read() {
                                None => rsx! { div { class: "state-loading", "Loading books…" } },
                                Some(None) => rsx! {
                                    p { class: "state-empty", "No books found." }
                                },
                                Some(Some(bks)) if bks.is_empty() => rsx! {
                                    p { class: "state-empty", "No books found." }
                                },
                                Some(Some(bks)) => rsx! {
                                    for book in bks {
                                        AuthorBookCard { key: "{book.id}", book: book.clone() }
                                    }
                                },
                            }
                        }
                    }
                }
            },
        }
    }
}

#[component]
fn AuthorBookCard(book: Book) -> Element {
    rsx! {
        Link {
            class: "is-book",
            to: Route::BookDetail { slug: book.slug.clone() },

            div { class: "is-book-cover",
                div { class: "is-book-cover-art",
                    div { class: "is-book-cover-stamp", "{book.title.chars().next().unwrap_or(' ')}" }
                    div {}
                    div { class: "is-book-cover-title", "{book.title}" }
                }
            }
            p { class: "is-book-meta", "{book.title}" }
            div { class: "is-book-meta",
                if let Some(rating) = book.avg_rating {
                    span { class: "is-rating", "★ {rating:.1}" }
                }
                span { "{book.chapter_count} ch" }
            }
        }
    }
}
