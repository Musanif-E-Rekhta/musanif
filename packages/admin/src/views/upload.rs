use dioxus::prelude::*;
use dioxus::html::{FileData, HasFileData};
use dioxus_free_icons::{
    icons::ld_icons::{LdChevronRight, LdUpload},
    Icon,
};
use ui::api::{self, uploads, IngestionJob};

use crate::components::MiniCover;
use crate::state::{stage_label, CURRENT_JOB, CURRENT_STAGE};

#[derive(Clone, PartialEq)]
enum UploadState {
    Idle,
    Reading(String),
    Uploading(String),
    CreatingJob(String),
    Error(String),
}

#[component]
pub fn UploadStage() -> Element {
    let mut jobs =
        use_resource(move || async move { api::fetch_admin_jobs(None, None, None, None).await });
    let mut upload_state = use_signal(|| UploadState::Idle);
    let mut drag_active = use_signal(|| false);

    let busy = !matches!(*upload_state.read(), UploadState::Idle | UploadState::Error(_));

    let mut start_upload = move |files: Vec<FileData>| {
        let Some(file) = files.into_iter().next() else { return };
        let filename = file.name();
        let mime_from_event = file.content_type();
        upload_state.set(UploadState::Reading(filename.clone()));
        spawn(async move {
            let bytes = match file.read_bytes().await {
                Ok(b) => b.to_vec(),
                Err(_) => {
                    upload_state.set(UploadState::Error("Couldn't read the file.".to_string()));
                    return;
                }
            };
            let mime = mime_from_event
                .as_deref()
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| mime_for(&filename))
                .to_string();
            upload_state.set(UploadState::Uploading(filename.clone()));
            let asset = match uploads::upload_admin_asset(&filename, &mime, bytes).await {
                Ok(a) => a,
                Err(_) => {
                    upload_state.set(UploadState::Error(format!("Upload failed for {filename}.")));
                    return;
                }
            };
            upload_state.set(UploadState::CreatingJob(filename.clone()));
            match api::create_ingestion_job(asset.id, None, None).await {
                Some(job) => {
                    *CURRENT_JOB.write() = Some(job.id);
                    *CURRENT_STAGE.write() = 2;
                    upload_state.set(UploadState::Idle);
                    jobs.restart();
                }
                None => {
                    upload_state.set(UploadState::Error(
                        "Couldn't create the ingestion job.".to_string(),
                    ));
                }
            }
        });
    };

    let dropzone_class = if *drag_active.read() {
        "adm-dropzone is-drag"
    } else {
        "adm-dropzone"
    };

    rsx! {
        div { class: "adm-pane",
            div {
                class: "{dropzone_class}",
                ondragover: move |e| {
                    e.prevent_default();
                    if !*drag_active.read() {
                        drag_active.set(true);
                    }
                },
                ondragleave: move |_| drag_active.set(false),
                ondrop: move |e| {
                    e.prevent_default();
                    drag_active.set(false);
                    if busy { return; }
                    let files = e.files();
                    if !files.is_empty() {
                        start_upload(files);
                    }
                },
                div { class: "adm-dropzone-icon",
                    Icon { icon: LdUpload, width: 28, height: 28 }
                }
                div { class: "adm-dropzone-title",
                    match &*upload_state.read() {
                        UploadState::Idle => rsx! { "Drop PDFs to queue" },
                        UploadState::Reading(f) => rsx! { "Reading {f}…" },
                        UploadState::Uploading(f) => rsx! { "Uploading {f}…" },
                        UploadState::CreatingJob(f) => rsx! { "Creating job for {f}…" },
                        UploadState::Error(msg) => rsx! { "{msg}" },
                    }
                }
                div { class: "adm-dropzone-sub",
                    "PDF · EPUB · scanned images. Up to 200 MB per file."
                }
                div { class: "adm-dropzone-actions",
                    label {
                        r#for: "adm-upload-input",
                        class: if busy { "is-btn is-btn--primary is-disabled" } else { "is-btn is-btn--primary" },
                        Icon { icon: LdUpload, width: 14, height: 14 }
                        if busy { "Working…" } else { "Choose files" }
                    }
                    input {
                        id: "adm-upload-input",
                        r#type: "file",
                        accept: ".pdf,application/pdf,.epub,application/epub+zip",
                        style: "position:absolute;width:1px;height:1px;opacity:0;pointer-events:none",
                        disabled: busy,
                        onchange: move |e| {
                            let files = e.files();
                            if !files.is_empty() {
                                start_upload(files);
                            }
                        },
                    }
                    button {
                        class: "is-btn",
                        r#type: "button",
                        disabled: true,
                        title: "Not available yet",
                        "From URL"
                    }
                    button {
                        class: "is-btn",
                        r#type: "button",
                        disabled: true,
                        title: "Not available yet",
                        "From Rekhta archive"
                    }
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
        None => rsx! { div { class: "adm-queue-state", "Loading jobs…" } },
        Some(None) => rsx! {
            div { class: "adm-queue-state is-error", "Could not load ingestion jobs." }
        },
        Some(Some(items)) if items.is_empty() => {
            rsx! { div { class: "adm-queue-state", "No ingestion jobs yet." } }
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
    let job_id = job.id.clone();

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
                button {
                    class: "adm-icon-btn",
                    r#type: "button",
                    onclick: move |_| {
                        *CURRENT_JOB.write() = Some(job_id.clone());
                        *CURRENT_STAGE.write() = stage.max(1);
                    },
                    Icon { icon: LdChevronRight, width: 14, height: 14 }
                }
            }
        }
    }
}

fn mime_for(filename: &str) -> &'static str {
    let lower = filename.to_ascii_lowercase();
    if lower.ends_with(".pdf") {
        "application/pdf"
    } else if lower.ends_with(".epub") {
        "application/epub+zip"
    } else if lower.ends_with(".png") {
        "image/png"
    } else if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
        "image/jpeg"
    } else if lower.ends_with(".webp") {
        "image/webp"
    } else {
        "application/octet-stream"
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
