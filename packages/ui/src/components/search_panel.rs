use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons::LdSearch, Icon};

use crate::api;
use crate::models::{Author, Book};
use crate::state::SEARCH_OPEN;
use crate::Route;

/// Discover's inline expanding search panel. Renders nothing when
/// `SEARCH_OPEN` is false. When opened, it takes over the top of the
/// main island with an autofocused input and live Books + Authors
/// results. ESC closes; click on any result closes and navigates.
///
/// The panel uses the "Committed" color treatment from DESIGN.md: a
/// saturated primary-soft surround so the open state is visually
/// decisive. The grid behind it is dimmed by `.is-discover-content--dimmed`
/// applied at the view level.
#[component]
pub fn SearchPanel() -> Element {
    if !*SEARCH_OPEN.read() {
        return rsx! { Fragment {} };
    }

    let query = use_signal(String::new);

    let books = use_resource(move || async move {
        let q = query.read().trim().to_string();
        if q.is_empty() {
            return None;
        }
        api::fetch_books(Some(q), None, Some(6), None).await
    });

    let authors = use_resource(move || async move {
        let q = query.read().trim().to_string();
        if q.is_empty() {
            return None;
        }
        api::fetch_authors(Some(q), Some(6), None).await
    });

    rsx! {
        div {
            class: "is-search-panel",
            role: "search",
            "aria-label": "Search the catalog",

            SearchPanelInput { query }
            SearchPanelResults { query, books, authors }
        }
    }
}

#[component]
fn SearchPanelInput(query: Signal<String>) -> Element {
    let mut q = query;
    rsx! {
        div { class: "is-search-panel-input",
            Icon {
                icon: LdSearch,
                width: 18,
                height: 18,
                class: "is-search-panel-icon",
            }
            input {
                r#type: "search",
                placeholder: "Search books, poets, or ghazals",
                autofocus: true,
                value: "{q}",
                "aria-label": "Search the catalog",
                oninput: move |e| q.set(e.value()),
                onkeydown: move |e| {
                    if e.key() == Key::Escape {
                        *SEARCH_OPEN.write() = false;
                    }
                },
            }
            button {
                class: "is-search-panel-close",
                "aria-label": "Close search",
                onclick: move |_| *SEARCH_OPEN.write() = false,
                "Esc"
            }
        }
    }
}

#[component]
fn SearchPanelResults(
    query: Signal<String>,
    books: Resource<Option<Vec<Book>>>,
    authors: Resource<Option<Vec<Author>>>,
) -> Element {
    let q = query.read().trim().to_string();

    if q.is_empty() {
        return rsx! {
            div { class: "is-search-panel-body",
                div { class: "is-search-panel-hint",
                    p { class: "is-search-panel-hint-eyebrow", "TRY A POET" }
                    div { class: "is-search-panel-suggestions",
                        SearchSuggestion { query, label: "Ghalib" }
                        SearchSuggestion { query, label: "Iqbal" }
                        SearchSuggestion { query, label: "Faiz" }
                        SearchSuggestion { query, label: "Mir Taqi Mir" }
                        SearchSuggestion { query, label: "Parveen Shakir" }
                    }
                }
            }
        };
    }

    let books_view = match &*books.read() {
        None => rsx! { SearchSectionSkeleton { title: "Books" } },
        Some(None) => rsx! { Fragment {} },
        Some(Some(items)) if items.is_empty() => rsx! { Fragment {} },
        Some(Some(items)) => rsx! { BookResults { items: items.clone() } },
    };

    let authors_view = match &*authors.read() {
        None => rsx! { SearchSectionSkeleton { title: "Authors" } },
        Some(None) => rsx! { Fragment {} },
        Some(Some(items)) if items.is_empty() => rsx! { Fragment {} },
        Some(Some(items)) => rsx! { AuthorResults { items: items.clone() } },
    };

    let books_loaded = matches!(&*books.read(), Some(Some(items)) if !items.is_empty());
    let authors_loaded = matches!(&*authors.read(), Some(Some(items)) if !items.is_empty());
    let any_loaded = books_loaded || authors_loaded;
    let any_done = matches!(&*books.read(), Some(_)) && matches!(&*authors.read(), Some(_));

    rsx! {
        div { class: "is-search-panel-body",
            {books_view}
            {authors_view}
            if any_done && !any_loaded {
                p { class: "is-search-panel-empty",
                    "No matches for \u{201C}{q}\u{201D} in the corpus."
                    span { class: "is-search-panel-empty-hint",
                        "Try without diacritics, or search in English."
                    }
                }
            }
        }
    }
}

#[component]
fn SearchSuggestion(query: Signal<String>, label: String) -> Element {
    let label_clone = label.clone();
    let mut q = query;
    rsx! {
        button {
            class: "is-search-panel-suggestion",
            onclick: move |_| q.set(label_clone.clone()),
            "{label}"
        }
    }
}

#[component]
fn BookResults(items: Vec<Book>) -> Element {
    rsx! {
        section { class: "is-search-section",
            h3 { class: "is-search-section-title", "Books" }
            ul { class: "is-search-section-list",
                for book in items.iter().take(3) {
                    li { key: "{book.id}",
                        Link {
                            class: "is-search-result",
                            to: Route::BookDetail { slug: book.slug.clone() },
                            onclick: move |_| *SEARCH_OPEN.write() = false,
                            span {
                                class: "is-search-result-mark",
                                "{book.title.chars().next().unwrap_or('م')}"
                            }
                            span { class: "is-search-result-body",
                                span { class: "is-search-result-title", "{book.title}" }
                                if let Some(summary) = book.summary.as_deref().or(book.description.as_deref()) {
                                    span { class: "is-search-result-meta",
                                        "{summary.chars().take(80).collect::<String>()}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AuthorResults(items: Vec<Author>) -> Element {
    rsx! {
        section { class: "is-search-section",
            h3 { class: "is-search-section-title", "Poets and authors" }
            ul { class: "is-search-section-list",
                for author in items.iter().take(3) {
                    li { key: "{author.id}",
                        Link {
                            class: "is-search-result",
                            to: Route::AuthorDetail { slug: author.slug.clone() },
                            onclick: move |_| *SEARCH_OPEN.write() = false,
                            span {
                                class: "is-search-result-mark is-search-result-mark--author",
                                "{author.name.chars().next().unwrap_or('ا')}"
                            }
                            span { class: "is-search-result-body",
                                span { class: "is-search-result-title", "{author.name}" }
                                if let Some(bio) = author.bio.as_deref() {
                                    span { class: "is-search-result-meta",
                                        "{bio.chars().take(80).collect::<String>()}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SearchSectionSkeleton(title: &'static str) -> Element {
    rsx! {
        section { class: "is-search-section",
            h3 { class: "is-search-section-title", "{title}" }
            ul { class: "is-search-section-list",
                for i in 0..2 {
                    li { key: "{i}", class: "is-search-result is-search-result--skeleton",
                        span { class: "is-search-result-mark is-skeleton-block" }
                        span { class: "is-search-result-body",
                            span { class: "is-skeleton-line is-skeleton-line--lg" }
                            span { class: "is-skeleton-line is-skeleton-line--sm" }
                        }
                    }
                }
            }
        }
    }
}
