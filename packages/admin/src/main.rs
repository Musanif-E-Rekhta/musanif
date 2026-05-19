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
                active: *CURRENT_SECTION.read() == AdminSection::Ingestion,
                onclick: move |_| { *CURRENT_SECTION.write() = AdminSection::Ingestion; },
            }
            WindowTab {
                title: "Library".to_string(),
                active: *CURRENT_SECTION.read() == AdminSection::Library,
                onclick: move |_| { *CURRENT_SECTION.write() = AdminSection::Library; },
            }
            WindowTab {
                title: "Analytics".to_string(),
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
        None => rsx! {
            div { class: "adm-loading",
                div { class: "adm-loading-frame",
                    div { class: "adm-loading-pulse",
                        span {} span {} span {}
                    }
                    div { class: "adm-loading-text", "Loading job" }
                }
            }
        },
        Some(None) => rsx! {
            div { class: "adm-error",
                div { class: "adm-error-frame",
                    div { class: "adm-error-mark", "!" }
                    h2 { class: "adm-error-title", "Couldn't load this job." }
                    p { class: "adm-error-hint",
                        "The job may have been removed, or the server didn't respond. Pick another job from the queue, or refresh."
                    }
                }
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
                        2 => rsx! { ProcessStage { job: job_clone.clone() } },
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
    let jobs = use_resource(move || async move {
        api::fetch_admin_jobs(None, None, None, None).await
    });

    rsx! {
        div { class: "adm-empty",
            div { class: "adm-empty-frame",
                div { class: "adm-empty-mark", dir: "rtl", lang: "ur", "مصنف" }
                div { class: "adm-empty-eyebrow", "Sarab pipeline" }
                h2 { class: "adm-empty-title", "Ready when you are." }
                p { class: "adm-empty-hint",
                    "Pick a job from the queue to advance it through upload, processing, review, edits, and publish."
                }
                match &*jobs.read() {
                    Some(Some(items)) if !items.is_empty() => {
                        let stages: [(u8, &str); 5] = [
                            (1, "Upload"),
                            (2, "Process"),
                            (3, "Review"),
                            (4, "Edit"),
                            (5, "Publish"),
                        ];
                        let counts: Vec<(u8, &str, usize)> = stages
                            .iter()
                            .map(|(n, label)| {
                                let count = items.iter().filter(|j| j.stage as u8 == *n).count();
                                (*n, *label, count)
                            })
                            .collect();
                        rsx! {
                            ul { class: "adm-empty-counts",
                                for (n, label, count) in counts {
                                    li { key: "{n}", class: "adm-empty-count",
                                        span {
                                            class: if count == 0 { "adm-empty-count-num is-zero" } else { "adm-empty-count-num" },
                                            "{count}"
                                        }
                                        span { class: "adm-empty-count-label", "{label}" }
                                    }
                                }
                            }
                        }
                    }
                    _ => rsx! {}
                }
            }
        }
    }
}

#[component]
fn Placeholder(title: String) -> Element {
    rsx! {
        div { class: "adm-placeholder",
            div { class: "adm-empty-frame",
                div { class: "adm-empty-mark", dir: "rtl", lang: "ur", "مصنف" }
                div { class: "adm-empty-eyebrow", "Coming soon" }
                h2 { class: "adm-empty-title", "{title}" }
                p { class: "adm-empty-hint",
                    "We're focused on ingestion first. Check back once that flow has settled."
                }
            }
        }
    }
}
