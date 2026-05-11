use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{LdBookmark, LdHeart, LdPenLine, LdSearch, LdSettings},
    Icon,
};
use ui::api::{self, IngestionJob};

use super::BookQueueItem;
use crate::state::CURRENT_JOB;

#[component]
pub fn Sidebar() -> Element {
    let jobs =
        use_resource(move || async move { api::fetch_admin_jobs(None, None, None, None).await });

    rsx! {
        aside { class: "adm-sidebar",
            div { class: "adm-side-brand",
                span { class: "adm-side-mark", "مصنف" }
                div {
                    div { class: "adm-side-name", "Musanif" }
                    div { class: "adm-side-role", "Admin Console" }
                }
            }

            QueueSection { jobs: jobs.clone() }

            div { class: "adm-side-section",
                div { class: "adm-side-section-title", "Library" }
                button { class: "adm-side-link",
                    Icon { icon: LdBookmark, width: 14, height: 14 }
                    "All books"
                }
                button { class: "adm-side-link",
                    Icon { icon: LdPenLine, width: 14, height: 14 }
                    "Authors"
                }
                button { class: "adm-side-link",
                    Icon { icon: LdHeart, width: 14, height: 14 }
                    "Featured"
                }
            }

            div { class: "adm-side-section",
                div { class: "adm-side-section-title", "System" }
                button { class: "adm-side-link",
                    Icon { icon: LdSearch, width: 14, height: 14 }
                    "Reports"
                }
                button { class: "adm-side-link",
                    Icon { icon: LdSettings, width: 14, height: 14 }
                    "Settings"
                }
            }
        }
    }
}

#[component]
fn QueueSection(jobs: Resource<Option<Vec<IngestionJob>>>) -> Element {
    let body = match &*jobs.read() {
        None => rsx! { div { class: "adm-queue-state", "Loading queue…" } },
        Some(None) => rsx! { div { class: "adm-queue-state is-error", "Could not load jobs." } },
        Some(Some(items)) if items.is_empty() => {
            rsx! { div { class: "adm-queue-state", "Queue is empty." } }
        }
        Some(Some(items)) => {
            // First fetch picks an initial CURRENT_JOB if none is set yet.
            if CURRENT_JOB.read().is_none() {
                if let Some(first) = items.first() {
                    *CURRENT_JOB.write() = Some(first.id.clone());
                }
            }
            let active_id = CURRENT_JOB.read().clone();
            rsx! {
                div { class: "adm-queue-list",
                    for job in items.iter().cloned() {
                        BookQueueItem {
                            key: "{job.id}",
                            active: Some(&job.id) == active_id.as_ref(),
                            onclick: {
                                let id = job.id.clone();
                                move |_| { *CURRENT_JOB.write() = Some(id.clone()); }
                            },
                            job,
                        }
                    }
                }
            }
        }
    };

    let count_label = match &*jobs.read() {
        Some(Some(items)) => format!("{}", items.len()),
        _ => "—".to_string(),
    };

    rsx! {
        div { class: "adm-side-section",
            div { class: "adm-side-section-title",
                span { "Ingestion queue" }
                span { class: "adm-side-count", "{count_label}" }
            }
            {body}
        }
    }
}
