//! Musanif admin console entry point.
//!
//! Drives the Sarab ingestion pipeline (upload → process → review →
//! edit → publish) against merk's admin GraphQL surface. All data
//! access goes through [`ui::api::*`], which re-exports cynic-typed
//! operations from `ui::graphql::admin`.
//!
//! The `web` feature targets WASM; `desktop` opens in a tao/wry window
//! sharing the same `WindowTabStrip` chrome the reader's desktop shell
//! uses.

use dioxus::prelude::*;
use ui::api::{self, IngestionJob};
use ui::components::{WindowTab, WindowTabStrip};

use admin::components::{BookHeader, Sidebar, StageStepper};
use admin::state::{AdminSection, CURRENT_JOB, CURRENT_SECTION, CURRENT_STAGE};
use admin::views::{EditStage, ProcessStage, PublishStage, ReviewStage, UploadStage};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const ADMIN_CSS: Asset = asset!("/assets/admin.css");

fn main() {
    dioxus::logger::initialize_default();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_effect(move || {
        ui::theme::apply_and_persist(ui::CURRENT_THEME().as_str());
    });

    rsx! {
        document::Title { "Musanif Admin" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: ADMIN_CSS }

        WindowTabStrip {
            WindowTab {
                title: "Ingestion queue".to_string(),
                glyph: Some("م".to_string()),
                active: *CURRENT_SECTION.read() == AdminSection::Ingestion,
                onclick: move |_| { *CURRENT_SECTION.write() = AdminSection::Ingestion; },
            }
            WindowTab {
                title: "Library".to_string(),
                glyph: Some("م".to_string()),
                active: *CURRENT_SECTION.read() == AdminSection::Library,
                onclick: move |_| { *CURRENT_SECTION.write() = AdminSection::Library; },
            }
            WindowTab {
                title: "Analytics".to_string(),
                glyph: Some("م".to_string()),
                active: *CURRENT_SECTION.read() == AdminSection::Analytics,
                onclick: move |_| { *CURRENT_SECTION.write() = AdminSection::Analytics; },
            }
        }

        match *CURRENT_SECTION.read() {
            AdminSection::Ingestion => rsx! { IngestionShell {} },
            AdminSection::Library => rsx! { Placeholder { title: "Library".to_string() } },
            AdminSection::Analytics => rsx! { Placeholder { title: "Analytics — coming soon".to_string() } },
        }
    }
}

#[component]
fn IngestionShell() -> Element {
    let job_id = CURRENT_JOB.read().clone();
    let job = use_resource(move || {
        let id = CURRENT_JOB.read().clone();
        async move {
            match id {
                Some(id) => api::fetch_admin_job(id).await,
                None => None,
            }
        }
    });

    rsx! {
        div { class: "adm-shell",
            Sidebar {}

            div { class: "adm-main",
                match job_id {
                    None => rsx! { EmptyShell {} },
                    Some(_) => rsx! { LoadedShell { job: job.clone() } },
                }
            }
        }
    }
}

#[component]
fn LoadedShell(job: Resource<Option<IngestionJob>>) -> Element {
    match &*job.read() {
        None => rsx! { div { class: "adm-loading", "Loading job…" } },
        Some(None) => rsx! {
            div { class: "adm-error",
                p { "Could not load this ingestion job." }
            }
        },
        Some(Some(j)) => {
            let job_stage = j.stage as u8;
            // Snap the stage panel to the job's current stage when the
            // operator switches jobs and the previous selection is past
            // the new job's reachable stages.
            let snap = job_stage;
            use_effect(move || {
                if *CURRENT_STAGE.peek() > snap || *CURRENT_STAGE.peek() == 0 {
                    *CURRENT_STAGE.write() = snap.max(1);
                }
            });

            let stage = *CURRENT_STAGE.read();
            let job_clone = j.clone();
            let job_id = j.id.clone();

            rsx! {
                BookHeader { job: job_clone.clone() }

                StageStepper {
                    active: stage,
                    job_stage,
                    onchange: move |n: u8| { *CURRENT_STAGE.write() = n; },
                }

                div { class: "adm-stage-body",
                    match stage {
                        1 => rsx! { UploadStage {} },
                        2 => rsx! { ProcessStage { job_id: job_id.clone() } },
                        3 => rsx! { ReviewStage { job_id: job_id.clone() } },
                        4 => rsx! { EditStage { job_id: job_id.clone() } },
                        5 => rsx! { PublishStage { job: job_clone.clone() } },
                        _ => rsx! { UploadStage {} },
                    }
                }
            }
        }
    }
}

#[component]
fn EmptyShell() -> Element {
    rsx! {
        div { class: "adm-empty",
            p { "Select a job from the queue to begin." }
        }
    }
}

#[component]
fn Placeholder(title: String) -> Element {
    rsx! {
        div { class: "adm-shell",
            div {
                style: "flex: 1; display: flex; align-items: center; justify-content: center; color: var(--text-muted); font-size: 14px",
                "{title}"
            }
        }
    }
}
