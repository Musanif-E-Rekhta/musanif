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
        async move { api::fetch_books_by_author(s, None, None).await }
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
                        div { class: "author-hero",
                            div { class: "author-hero-avatar",
                                "{author.name.chars().next().unwrap_or(' ')}"
                            }
                            div {
                                p { class: "author-hero-name", "{author.name}" }
                                if let Some(bio) = &author.bio {
                                    p { class: "author-hero-bio", "{bio}" }
                                }
                            }
                        }

                        div { class: "section-head",
                            h3 { class: "section-head-title", "Works" }
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
                span { class: "is-book-author", "{book.chapter_count} ch" }
            }
        }
    }
}
