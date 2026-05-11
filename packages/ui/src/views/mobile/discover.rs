use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons::LdSearch, Icon};

use crate::{api, components::Cover, models::Book, state::CURRENT_USER, Route};

#[component]
pub fn MobileDiscover() -> Element {
    let books = use_resource(move || async move { api::fetch_books(None, None, None, None).await });
    let bookmarks =
        use_resource(move || async move { api::fetch_my_bookmarks(Some("reading".into())).await });

    let username = CURRENT_USER
        .read()
        .as_ref()
        .map(|u| u.username.clone())
        .unwrap_or_else(|| "reader".to_string());

    rsx! {
        div { class: "is-mob-header",
            div {
                p { class: "is-mob-greet", "Good evening, {username}" }
                h1 { class: "is-mob-title", "Find something to read" }
            }
            div { class: "is-mob-avatar",
                "{username.chars().next().unwrap_or('?').to_ascii_uppercase()}"
            }
        }

        div { class: "is-mob-search",
            Icon { icon: LdSearch, width: 16, height: 16 }
            input { placeholder: "Books, authors, ghazals…" }
        }

        ContinueIsland { bookmarks }

        div { class: "is-mob-section",
            span { "Recommended" }
            span { class: "is-mob-section-link", "All" }
        }

        match &*books.read() {
            None => rsx! { div { class: "state-loading", "Loading…" } },
            Some(None) => rsx! { div { class: "state-error", "Could not load books." } },
            Some(Some(list)) if list.is_empty() => rsx! {
                div { class: "state-empty", "No books yet." }
            },
            Some(Some(list)) => rsx! {
                div { class: "is-mob-shelf",
                    for book in list.iter().take(8) {
                        ShelfCard { key: "{book.id}", book: book.clone() }
                    }
                }
            },
        }

        div { class: "is-mob-section",
            span { "Highlights" }
            span { class: "is-mob-section-link", "All" }
        }
        div { class: "island is-rail-card", style: "margin: 0 4px 14px",
            p { class: "is-quote",
                "\u{201C}The night is long; even the moon is tired of waiting. Yet we sit, with the wine cooling, and pretend we are not.\u{201D}"
            }
            div { class: "is-quote-source", "Diwan-e-Ghalib · Ghazal 47" }
        }
    }
}

#[component]
fn ContinueIsland(bookmarks: Resource<Option<Vec<crate::models::Bookmark>>>) -> Element {
    let bms = bookmarks.read();
    let entries: Vec<&crate::models::Bookmark> = bms
        .as_ref()
        .and_then(|opt| opt.as_ref())
        .map(|v| v.iter().filter(|b| b.book.is_some()).take(2).collect())
        .unwrap_or_default();

    if entries.is_empty() {
        return rsx! { Fragment {} };
    }

    rsx! {
        div { class: "island is-rail-card", style: "margin: 0 4px 14px",
            div { class: "is-rail-eyebrow", "Continue Reading" }
            for (i, bm) in entries.iter().enumerate() {
                ContinueRow { key: "{bm.id}", divider: i > 0, bookmark: (*bm).clone() }
            }
        }
    }
}

#[component]
fn ContinueRow(divider: bool, bookmark: crate::models::Bookmark) -> Element {
    let book = bookmark.book.as_ref().expect("filtered above");
    let pct = match (bookmark.progress, book.page_count) {
        (Some(p), Some(pages)) if pages > 0 => {
            (p as f64 / pages as f64 * 100.0).clamp(0.0, 100.0)
        }
        _ => 0.0,
    };
    let mono = book.title.chars().next().map(|c| c.to_string());
    let slug = book.slug.clone();

    rsx! {
        if divider {
            hr { class: "is-divider" }
        }
        Link {
            class: "is-continue",
            to: Route::BookDetail { slug },
            div { class: "is-continue-cover",
                Cover { urdu: Some(book.title.clone()), mono }
            }
            div { class: "is-continue-info",
                p { class: "is-continue-title", "{book.title}" }
                p { class: "is-continue-chapter",
                    {
                        book.authors.as_ref()
                            .map(|a| a.iter().map(|x| x.author.name.as_str()).collect::<Vec<_>>().join(", "))
                            .unwrap_or_default()
                    }
                }
                div { class: "is-progress",
                    div { class: "is-progress-fill", style: "width: {pct:.0}%" }
                }
                div { class: "is-progress-label",
                    span { "{pct:.0}%" }
                    span {}
                }
            }
        }
    }
}

#[component]
fn ShelfCard(book: Book) -> Element {
    let mono = book.title.chars().next().map(|c| c.to_string());
    let author = book
        .authors
        .as_ref()
        .and_then(|a| a.first())
        .map(|x| x.author.name.clone())
        .unwrap_or_default();
    rsx! {
        Link {
            class: "is-mob-shelf-card",
            to: Route::BookDetail { slug: book.slug.clone() },
            div { class: "is-book-cover",
                Cover { urdu: Some(book.title.clone()), mono }
            }
            p { class: "is-book-meta", style: "margin-top: 8px", "{book.title}" }
            p { class: "is-book-author", "{author}" }
        }
    }
}
