use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons::LdSearch, Icon};

use crate::{api, models::Author, Route};

#[component]
pub fn MobileAuthors() -> Element {
    let authors = use_resource(move || async move { api::fetch_authors(None, None, None).await });
    let mut query = use_signal(String::new);

    rsx! {
        div { class: "is-mob-header",
            div {
                p { class: "is-mob-greet", "Voices of Urdu" }
                h1 { class: "is-mob-title", "Authors" }
            }
        }

        div { class: "is-mob-search",
            Icon { icon: LdSearch, width: 16, height: 16 }
            input {
                placeholder: "Search authors…",
                value: "{query}",
                oninput: move |e| query.set(e.value()),
            }
        }

        match &*authors.read() {
            None => rsx! { div { class: "state-loading", "Loading authors…" } },
            Some(None) => rsx! { div { class: "state-error", "Could not load authors." } },
            Some(Some(list)) => {
                let q = query.read().to_lowercase();
                let q = q.trim();
                let filtered: Vec<&Author> = if q.is_empty() {
                    list.iter().collect()
                } else {
                    list.iter().filter(|a| a.name.to_lowercase().contains(q)).collect()
                };

                if filtered.is_empty() {
                    rsx! { div { class: "state-empty", "No authors matched \"{q}\"." } }
                } else {
                    rsx! {
                        div { class: "is-mob-list",
                            for a in filtered {
                                AuthorRow { key: "{a.id}", author: a.clone() }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AuthorRow(author: Author) -> Element {
    let glyph = author.name.chars().next().unwrap_or('ا').to_string();
    rsx! {
        Link {
            class: "is-mob-row",
            to: Route::AuthorDetail { slug: author.slug.clone() },
            div { class: "author-avatar", "{glyph}" }
            div { class: "is-mob-row-info",
                p { class: "is-mob-row-title", "{author.name}" }
                p { class: "is-mob-row-meta", "{author.followers} followers" }
            }
            span { style: "color: var(--text-muted)", "›" }
        }
    }
}
