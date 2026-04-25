use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons::LdSearch, Icon};

use crate::{api, components::Cover, models::Book, Route};

#[component]
pub fn Home() -> Element {
    let books = use_resource(move || async move { api::fetch_books(None, None).await });

    rsx! {
        div { class: "island is-main",
            div { class: "is-main-header",
                h2 { class: "is-main-title", "Discover" }
                span { class: "is-main-subtitle", "Urdu literature, curated" }
                div { class: "is-main-actions",
                    div { class: "is-search",
                        Icon { icon: LdSearch, width: 14, height: 14, class: "is-search-icon" }
                        input { placeholder: "Search books, authors, ghazals…" }
                    }
                }
            }

            div { class: "is-main-body",


                // Recently Added header
                div { style: "display: flex; align-items: baseline; justify-content: space-between; margin-bottom: 14px",
                    h3 { style: "font-size: 14px; font-weight: 700; margin: 0; letter-spacing: -0.005em", "Recently Added" }
                    button { class: "is-btn is-btn--ghost", "View all" }
                }

                match &*books.read() {
                    None => rsx! { div { class: "state-loading", "Loading books…" } },
                    Some(None) => rsx! {
                        div { class: "state-error",
                            p { "Could not reach the server." }
                            p { class: "state-error-hint",
                                "Make sure the backend is running on "
                                code { "localhost:9678" }
                            }
                        }
                    },
                    Some(Some(books)) if books.is_empty() => rsx! {
                        div { class: "state-empty", "No books found." }
                    },
                    Some(Some(books)) => rsx! {
                        div { class: "is-grid",
                            for book in books {
                                BookCard { key: "{book.id}", book: book.clone() }
                            }
                        }
                    },
                }
            }
        }
    }
}

#[component]
fn BookCard(book: Book) -> Element {
    let author_names = book
        .authors
        .as_deref()
        .unwrap_or(&[])
        .iter()
        .map(|a| a.author.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");

    let first_char = book.title.chars().next().unwrap_or('م').to_string();

    rsx! {
        Link {
            class: "is-book",
            to: Route::BookDetail { slug: book.slug.clone() },

            div { class: "is-book-cover",
                Cover {
                    title: Some(book.title.clone()),
                    urdu: Some(book.title.clone()),
                    mono: Some(first_char),
                }
            }

            p { class: "is-book-meta", "{book.title}" }

            if !author_names.is_empty() {
                p { class: "is-book-author", "{author_names}" }
            }

            if let Some(rating) = book.avg_rating {
                span { class: "is-rating", "★ {rating:.1}" }
            }
        }
    }
}
