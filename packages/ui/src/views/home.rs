use dioxus::prelude::*;

use crate::{api, models::Book, Route};

const HOME_CSS: Asset = asset!("/assets/styling/home.css");

#[component]
fn HomeHeader() -> Element {
    if cfg!(feature = "mobile") {
        rsx! {
            div { class: "is-mob-header",
                div {
                    p { class: "is-mob-greet", "Explore Urdu Literature" }
                    h1 { class: "is-mob-title", "Discover" }
                }
                Link {
                    class: "is-icon-btn",
                    to: Route::Settings {},
                    svg { width: "20", height: "20", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24",
                        circle { cx: "12", cy: "12", r: "3" }
                        path { d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" }
                    }
                }
            }
        }
    } else {
        rsx! {
            div { class: "is-main-header",
                h2 { class: "is-main-title", "Discover" }
                span { class: "is-main-subtitle", "Explore the world of Urdu literature" }
            }
        }
    }
}

#[component]
fn HomeContinue() -> Element {
    if cfg!(feature = "mobile") {
        rsx! { div {} }
    } else {
        rsx! { ContinueRail {} }
    }
}

#[component]
pub fn Home() -> Element {
    let _input = use_signal(String::new);
    let search_term = use_signal(String::new);

    let books = use_resource(move || {
        let q = search_term();
        async move { api::fetch_books(if q.is_empty() { None } else { Some(q) }, None).await }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }

        div { class: "is-main",
            HomeHeader {}

            div { class: "is-main-body",
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
                        div { class: "is-grid",
                            for book in books {
                                BookCard { key: "{book.id}", book: book.clone() }
                            }
                        }
                    },
                }
            }
        }
        HomeContinue {}
    }
}

#[component]
fn ContinueRail() -> Element {
    rsx! {
        aside { class: "island is-rail-card",
            div { class: "is-rail-eyebrow", "Continue Reading" }
            div { class: "is-continue",
                div { class: "is-continue-cover",
                    div { class: "is-book-cover-art",
                        div { class: "is-book-cover-stamp", "بانگ" }
                        div {}
                        div { class: "is-book-cover-title", "ا" }
                    }
                }
                div { class: "is-continue-info",
                    div { class: "is-continue-title", "Bang-e-Dara" }
                    div { class: "is-continue-chapter", "Allama Iqbal · Ch.47" }
                    div { class: "is-progress",
                        div { class: "is-progress-fill", style: "width: 68%" }
                    }
                    div { class: "is-progress-label",
                        span { "68%" }
                        span { "14 min left" }
                    }
                }
            }

            div { class: "is-divider" }

            div { class: "is-rail-eyebrow", "Daily Quote" }
            p { class: "is-quote", "“Hazaaron khwahishen aisi ke har khwahish pe dam nikle...”" }
            div { class: "is-quote-source", "— Mirza Ghalib" }
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

            if !author_names.is_empty() {
                p { class: "is-book-author", "{author_names}" }
            }

            div { class: "is-book-meta",
                if let Some(rating) = book.avg_rating {
                    span { class: "is-rating", "★ {rating:.1}" }
                }
                span { "{book.chapter_count} ch" }
            }
        }
    }
}
