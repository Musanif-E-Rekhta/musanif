use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons::LdShare2, Icon};
use ui::api::IngestionJob;

use super::MiniCover;

#[component]
pub fn BookHeader(job: IngestionJob) -> Element {
    let title = job
        .hint_title
        .clone()
        .unwrap_or_else(|| "Untitled job".to_string());
    let title_ur = job.hint_title_ur.clone();
    let author = job.hint_author.clone();
    let pages = job.pages;
    let size_label = job.size_bytes.map(format_bytes);
    let confidence = job.overall_confidence;

    rsx! {
        header { class: "adm-bookhead",
            MiniCover {
                color: job.cover_color.clone(),
                glyph: job.cover_glyph.clone(),
                size: 48,
            }
            div { class: "adm-bookhead-meta",
                div { class: "adm-bookhead-title-row",
                    h1 { class: "adm-bookhead-title", "{title}" }
                    if let Some(ur) = title_ur {
                        span { class: "adm-bookhead-title-ur", dir: "rtl", lang: "ur", "{ur}" }
                    }
                }
                div { class: "adm-bookhead-sub",
                    if let Some(author) = author {
                        span { "{author}" }
                    }
                    if let Some(p) = pages {
                        span { class: "adm-dot-sep", "·" }
                        span { "{p} pages" }
                    }
                    if let Some(size) = size_label {
                        span { class: "adm-dot-sep", "·" }
                        span { "{size}" }
                    }
                }
            }
            div { class: "adm-bookhead-actions",
                if let Some(conf) = confidence {
                    div { class: "adm-bookhead-conf",
                        div { class: "adm-bookhead-conf-label", "Overall confidence" }
                        div { class: "adm-bookhead-conf-num", "{(conf * 100.0).round() as i32}%" }
                    }
                }
                button { class: "is-btn",
                    Icon { icon: LdShare2, width: 14, height: 14 }
                    "Share"
                }
                button { class: "is-btn", "Open in app" }
            }
        }
    }
}

fn format_bytes(b: i32) -> String {
    let b = b as f64;
    if b >= 1_000_000.0 {
        format!("{:.1} MB", b / 1_000_000.0)
    } else if b >= 1_000.0 {
        format!("{:.1} KB", b / 1_000.0)
    } else {
        format!("{} B", b as i64)
    }
}
