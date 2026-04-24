use dioxus::prelude::*;

use crate::{api, models::Book, Route};

const HOME_CSS: Asset = asset!("/assets/styling/home.css");

#[component]
pub fn Home() -> Element {
    let mut input = use_signal(String::new);
    let mut search_term = use_signal(String::new);

    let books = use_resource(move || {
        let q = search_term();
        async move { api::fetch_books(if q.is_empty() { None } else { Some(q) }, None).await }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }

        div { class: "home",
            header { class: "home-header",
                h1 { class: "home-title", "مصنف" }
                p { class: "home-subtitle", "Discover and read great Urdu literature" }

                div { class: "search-bar",
                    input {
                        class: "search-input",
                        r#type: "text",
                        placeholder: "Search books or authors…",
                        value: "{input}",
                        oninput: move |e| input.set(e.value()),
                        onkeydown: move |e| {
                            if e.key() == Key::Enter {
                                search_term.set(input());
                            }
                        },
                    }
                    button {
                        class: "search-btn",
                        onclick: move |_| search_term.set(input()),
                        "Search"
                    }
                }
            }

            match &*books.read() {
                None => rsx! {
                    div { class: "state-loading", "Loading books…" }
                },
                Some(None) => rsx! {
                    div { class: "state-error",
                        p { "Could not reach the server." }
                        p { class: "state-error-hint", "Make sure the backend is running on " code { "localhost:3000" } }
                    }
                },
                Some(Some(books)) if books.is_empty() => rsx! {
                    div { class: "state-empty", "No books found." }
                },
                Some(Some(books)) => rsx! {
                    div { class: "book-grid",
                        for book in books {
                            BookCard { key: "{book.id}", book: book.clone() }
                        }
                    }
                },
            }
        }
    }
}

#[component]
fn BookCard(book: Book) -> Element {
    let slug = book.slug.clone();
    let nav = use_navigator();

    let author_names = book
        .authors
        .as_deref()
        .unwrap_or(&[])
        .iter()
        .map(|a| a.author.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");

    rsx! {
        div {
            class: "book-card",
            onclick: move |_| { nav.push(Route::BookDetail { slug: slug.clone() }); },

            div { class: "book-card-cover",
                if let Some(url) = &book.cover_url {
                    img { src: "{url}", alt: "{book.title}" }
                } else {
                    div { class: "book-card-cover-placeholder",
                        span { "{book.title}" }
                    }
                }
            }

            div { class: "book-card-body",
                h3 { class: "book-card-title", "{book.title}" }

                if !author_names.is_empty() {
                    p { class: "book-card-authors", "{author_names}" }
                }

                div { class: "book-card-meta",
                    if let Some(rating) = book.avg_rating {
                        span { class: "badge badge-rating", "★ {rating:.1}" }
                    }
                    span { class: "badge", "{book.chapter_count} chapters" }
                    span { class: "badge", "{book.language.to_uppercase()}" }
                }

                if let Some(summary) = &book.summary {
                    p { class: "book-card-summary", "{summary}" }
                }
            }
        }
    }
}
