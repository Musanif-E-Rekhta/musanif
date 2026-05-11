use dioxus::prelude::*;

use crate::{
    api,
    components::{Cover, FeatureCard, PageHeader, SearchInput},
    models::{Book, FeaturedBook},
    Route,
};

#[component]
pub fn Home() -> Element {
    if cfg!(feature = "mobile") {
        return rsx! { crate::views::mobile::discover::MobileDiscover {} };
    }

    let query = use_signal(String::new);
    let books = use_resource(move || async move {
        let q = query.read().trim().to_string();
        let q = if q.is_empty() { None } else { Some(q) };
        api::fetch_books(q, None, None, None).await
    });
    let featured = use_resource(move || async move { api::fetch_featured().await });

    rsx! {
        div { class: "island is-main",
            PageHeader {
                title: "Discover".to_string(),
                subtitle: "Urdu literature, curated".to_string(),
                actions: rsx! {
                    SearchInput {
                        placeholder: "Search books, authors, ghazals…".to_string(),
                        value: query,
                        show_kbd: true,
                    }
                },
            }

            div { class: "is-main-body",
                FeaturedSlot { featured: featured.clone() }

                div { class: "section-head",
                    h3 { class: "section-head-title", "Recently Added" }
                    button { class: "is-btn is-btn--ghost", "View all" }
                }

                BookGrid { books, query }
            }
        }
    }
}

#[component]
fn FeaturedSlot(featured: Resource<Option<FeaturedBook>>) -> Element {
    let pick = featured
        .read()
        .as_ref()
        .and_then(|opt| opt.as_ref())
        .cloned();

    let Some(pick) = pick else {
        return rsx! { Fragment {} };
    };

    let book = pick.book;
    let mono = book.title.chars().next().map(|c| c.to_string());
    let blurb = pick
        .blurb
        .or_else(|| book.description.clone())
        .or_else(|| book.summary.clone())
        .unwrap_or_else(|| "A new arrangement, freshly annotated.".to_string());
    let eyebrow = pick.eyebrow.unwrap_or_else(|| "Editor's Pick".to_string());
    let title = pick.headline.unwrap_or_else(|| book.title.clone());
    let slug = book.slug.clone();

    rsx! {
        FeatureCard {
            eyebrow,
            title,
            blurb,
            cover_urdu: Some(book.title.clone()),
            cover_mono: mono,
            actions: rsx! {
                Link {
                    to: Route::BookDetail { slug: slug.clone() },
                    class: "is-btn is-btn--primary",
                    "Start reading"
                }
                Link {
                    to: Route::BookDetail { slug },
                    class: "is-btn",
                    "Details"
                }
            },
        }
    }
}

#[component]
fn BookGrid(books: Resource<Option<Vec<Book>>>, query: Signal<String>) -> Element {
    rsx! {
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
            Some(Some(books)) => {
                let q = query.read().to_lowercase();
                let q = q.trim();
                let filtered: Vec<&Book> = if q.is_empty() {
                    books.iter().collect()
                } else {
                    books
                        .iter()
                        .filter(|b| {
                            b.title.to_lowercase().contains(q)
                                || b.authors
                                    .as_deref()
                                    .unwrap_or(&[])
                                    .iter()
                                    .any(|a| a.author.name.to_lowercase().contains(q))
                        })
                        .collect()
                };

                if filtered.is_empty() {
                    rsx! { div { class: "state-empty", "Nothing matched \"{q}\"." } }
                } else {
                    rsx! {
                        div { class: "is-grid",
                            for book in filtered {
                                BookCard { key: "{book.id}", book: book.clone() }
                            }
                        }
                    }
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
