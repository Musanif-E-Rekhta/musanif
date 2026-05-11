use dioxus::prelude::*;

use crate::{
    api,
    components::{PageHeader, SearchInput},
    models::Author,
    Route,
};

#[component]
pub fn Authors() -> Element {
    if cfg!(feature = "mobile") {
        return rsx! { crate::views::mobile::authors::MobileAuthors {} };
    }

    let authors: Resource<Option<Vec<Author>>> =
        use_resource(move || async move { api::fetch_authors(None, None, None).await });
    let query = use_signal(String::new);

    rsx! {
        div { class: "island is-main",
            PageHeader {
                title: "Authors".to_string(),
                subtitle: "Voices of Urdu literature".to_string(),
                actions: rsx! {
                    SearchInput {
                        placeholder: "Search authors…".to_string(),
                        value: query,
                        show_kbd: false,
                    }
                },
            }

            div { class: "is-main-body",
                match &*authors.read() {
                    None => rsx! { div { class: "state-loading", "Loading authors…" } },
                    Some(None) => rsx! { div { class: "state-error", "Could not load authors." } },
                    Some(Some(authors)) => {
                        let q = query.read().to_lowercase();
                        let q = q.trim();
                        let filtered: Vec<&Author> = if q.is_empty() {
                            authors.iter().collect()
                        } else {
                            authors.iter().filter(|a| a.name.to_lowercase().contains(q)).collect()
                        };

                        if filtered.is_empty() {
                            rsx! { div { class: "state-empty", "No authors matched \"{q}\"." } }
                        } else {
                            rsx! {
                                div { class: "author-grid",
                                    for author in filtered {
                                        Link {
                                            key: "{author.id}",
                                            class: "author-card",
                                            to: Route::AuthorDetail { slug: author.slug.clone() },
                                            div { class: "author-avatar",
                                                "{author.name.chars().next().unwrap_or('ا')}"
                                            }
                                            div {
                                                p { class: "author-name", "{author.name}" }
                                                p { class: "author-followers", "{author.followers} followers" }
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
    }
}
