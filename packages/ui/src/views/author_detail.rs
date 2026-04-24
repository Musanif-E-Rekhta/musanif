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
            None => rsx! { div { class: "state-loading", "Loading…" } },
            Some(None) => rsx! { div { class: "state-error", "Author not found." } },
            Some(Some(author)) => rsx! {
                div { class: "author-detail",
                    div { class: "author-hero",
                        if let Some(avatar) = &author.avatar_url {
                            img { class: "author-avatar", src: "{avatar}", alt: "{author.name}" }
                        } else {
                            div { class: "author-avatar-placeholder",
                                span { "{author.name.chars().next().unwrap_or('?')}" }
                            }
                        }

                        div { class: "author-info",
                            h1 { class: "author-name", "{author.name}" }
                            div { class: "author-meta",
                                span { class: "badge", "{author.followers} followers" }
                                if let Some(site) = &author.website {
                                    a { class: "author-website", href: "{site}", target: "_blank", "Website ↗" }
                                }
                            }
                            if let Some(bio) = &author.bio {
                                p { class: "author-bio", "{bio}" }
                            }
                        }
                    }

                    section { class: "book-section",
                        h2 { class: "section-title", "Books" }
                        match &*books.read() {
                            None => rsx! { div { class: "state-loading", "Loading books…" } },
                            Some(None) => rsx! {
                                p { class: "state-empty", "No books found." }
                            },
                            Some(Some(bks)) if bks.is_empty() => rsx! {
                                p { class: "state-empty", "No books found." }
                            },
                            Some(Some(bks)) => rsx! {
                                div { class: "book-grid book-grid--compact",
                                    for book in bks {
                                        AuthorBookCard { key: "{book.id}", book: book.clone() }
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
fn AuthorBookCard(book: Book) -> Element {
    rsx! {
        Link {
            class: "author-book-card",
            to: Route::BookDetail { slug: book.slug.clone() },

            div { class: "book-card-cover",
                if let Some(url) = &book.cover_url {
                    img { src: "{url}", alt: "{book.title}" }
                } else {
                    div { class: "book-card-cover-placeholder",
                        span { "{book.title}" }
                    }
                }
            }
            div { class: "author-book-card-body",
                p { class: "author-book-title", "{book.title}" }
                if let Some(rating) = book.avg_rating {
                    span { class: "badge badge-rating", "★ {rating:.1}" }
                }
            }
        }
    }
}
