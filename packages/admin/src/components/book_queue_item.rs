use dioxus::prelude::*;
use ui::api::IngestionJob;

use super::MiniCover;
use crate::state::stage_label;

/// One row in the sidebar queue: cover + title/sub + stage pill. Active
/// item gets the `is-active` modifier from the parent.
#[component]
pub fn BookQueueItem(job: IngestionJob, active: bool, onclick: EventHandler<()>) -> Element {
    let class = if active {
        "adm-queue-item is-active"
    } else {
        "adm-queue-item"
    };
    let title = job
        .hint_title
        .as_deref()
        .unwrap_or("Untitled job")
        .to_string();
    let pages = job.pages;
    let stage = job.stage as u8;
    let label = stage_label(stage);

    rsx! {
        button {
            class: "{class}",
            onclick: move |_| onclick.call(()),
            MiniCover {
                color: job.cover_color.clone(),
                glyph: job.cover_glyph.clone(),
                size: 32,
            }
            div { class: "adm-queue-meta",
                div { class: "adm-queue-title", "{title}" }
                div { class: "adm-queue-sub",
                    if let Some(p) = pages {
                        span { "{p}p" }
                    }
                    span { class: "adm-dot-sep", "·" }
                    span { class: "adm-queue-status", "{job.status}" }
                }
            }
            div { class: "adm-queue-stage",
                span { class: "adm-stage-pill is-stage-{stage}", "{label}" }
            }
        }
    }
}
