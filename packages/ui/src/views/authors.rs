use dioxus::prelude::*;

use crate::{api, models::Author, Route};

#[component]
pub fn Authors() -> Element {
    let authors: Resource<Option<Vec<Author>>> =
        use_resource(move || async move { api::fetch_authors(None).await });

    rsx! {
        div { class: "island is-main",
            div { class: "is-main-header",
                h2 { class: "is-main-title", "Authors" }
                span { class: "is-main-subtitle", "Voices of Urdu literature" }
            }

            div { class: "is-main-body",
                match &*authors.read() {
                    None => rsx! { div { class: "state-loading", "Loading authors…" } },
                    Some(None) => rsx! { div { class: "state-error", "Could not load authors." } },
                    Some(Some(authors)) if authors.is_empty() => rsx! {
                        div { class: "state-empty", "No authors found." }
                    },
                    Some(Some(authors)) => rsx! {
                        div { style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(220px, 1fr)); gap: 16px",
                            for author in authors {
                                Link {
                                    key: "{author.id}",
                                    class: "is-book",
                                    style: "display: flex; gap: 14px; padding: 14px; background: var(--bg-color); border-radius: 12px; align-items: center",
                                    to: Route::AuthorDetail { slug: author.slug.clone() },
                                    div { style: "width: 48px; height: 48px; border-radius: 50%; background: var(--accent-light); color: var(--primary); display: flex; align-items: center; justify-content: center; font-family: var(--font-urdu); font-size: 20px; font-weight: 700; flex-shrink: 0",
                                        "{author.name.chars().next().unwrap_or('ا')}"
                                    }
                                    div {
                                        p { style: "font-size: 14px; font-weight: 700; margin: 0 0 2px", "{author.name}" }
                                        p { style: "font-size: 11px; color: var(--text-muted); margin: 0", "{author.followers} followers" }
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
