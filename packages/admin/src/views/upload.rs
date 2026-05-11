use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{LdChevronRight, LdUpload},
    Icon,
};
use ui::api::{self, IngestionJob};

use crate::components::MiniCover;
use crate::state::stage_label;

#[component]
pub fn UploadStage() -> Element {
    let jobs =
        use_resource(move || async move { api::fetch_admin_jobs(None, None, None, None).await });

    rsx! {
        div { class: "adm-pane",
            div { class: "adm-dropzone",
                div { class: "adm-dropzone-icon",
                    Icon { icon: LdUpload, width: 28, height: 28 }
                }
                div { class: "adm-dropzone-title", "Drop PDFs to queue" }
                div { class: "adm-dropzone-sub",
                    "PDF · EPUB · scanned images. Up to 200 MB per file."
                }
                div { class: "adm-dropzone-actions",
                    button { class: "is-btn is-btn--primary",
                        Icon { icon: LdUpload, width: 14, height: 14 }
                        "Choose files"
                    }
                    button { class: "is-btn", "From URL" }
                    button { class: "is-btn", "From Rekhta archive" }
                }
                div { class: "adm-dropzone-hint",
                    span { dir: "rtl", lang: "ur", "یہاں فائلیں چھوڑیں" }
                }
            }

            div { class: "adm-section-title", "Recently uploaded" }
            RecentTable { jobs }
        }
    }
}

#[component]
fn RecentTable(jobs: Resource<Option<Vec<IngestionJob>>>) -> Element {
    match &*jobs.read() {
        None => rsx! { div { class: "state-loading", "Loading jobs…" } },
        Some(None) => rsx! {
            div { class: "state-error", "Could not load ingestion jobs." }
        },
        Some(Some(items)) if items.is_empty() => {
            rsx! { div { class: "state-empty", "No ingestion jobs yet." } }
        }
        Some(Some(items)) => rsx! {
            table { class: "adm-table",
                thead {
                    tr {
                        th { "File" }
                        th { "Size" }
                        th { "Pages" }
                        th { "Status" }
                        th { "Stage" }
                        th {}
                    }
                }
                tbody {
                    for job in items.iter().rev().cloned() {
                        JobRow { key: "{job.id}", job }
                    }
                }
            }
        },
    }
}

#[component]
fn JobRow(job: IngestionJob) -> Element {
    let title = job
        .hint_title
        .as_deref()
        .unwrap_or("Untitled job")
        .to_string();
    let author = job.hint_author.as_deref().unwrap_or("—").to_string();
    let size = job.size_bytes.map(format_bytes).unwrap_or_else(|| "—".into());
    let pages = job
        .pages
        .map(|p| p.to_string())
        .unwrap_or_else(|| "—".into());
    let stage = job.stage as u8;
    let label = stage_label(stage);

    rsx! {
        tr {
            td { class: "adm-cell-file",
                MiniCover {
                    color: job.cover_color.clone(),
                    glyph: job.cover_glyph.clone(),
                    size: 24,
                }
                div {
                    div { class: "adm-cell-title", "{title}" }
                    div { class: "adm-cell-sub", "{author}" }
                }
            }
            td { "{size}" }
            td { "{pages}" }
            td { class: "adm-cell-muted", "{job.status}" }
            td {
                span { class: "adm-stage-pill is-stage-{stage}", "{label}" }
            }
            td {
                button { class: "adm-icon-btn",
                    Icon { icon: LdChevronRight, width: 14, height: 14 }
                }
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
