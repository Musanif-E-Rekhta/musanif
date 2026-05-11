use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons::LdCheck, Icon};
use ui::api::{self, JobEventSub, JobLogEntry, PipelineStep};

use crate::state::{CURRENT_MODEL, CURRENT_PROVIDER};

struct Provider {
    id: &'static str,
    label: &'static str,
    models: &'static [Model],
}
struct Model {
    id: &'static str,
    label: &'static str,
    note: &'static str,
}

const PROVIDERS: &[Provider] = &[
    Provider {
        id: "claude",
        label: "Anthropic",
        models: &[
            Model { id: "sonnet-4.5", label: "Claude Sonnet 4.5", note: "best for Urdu/Hindi" },
            Model { id: "opus-4",     label: "Claude Opus 4",     note: "deep analysis · 4× cost" },
            Model { id: "haiku-4",    label: "Claude Haiku 4.5",  note: "fast · light QA" },
        ],
    },
    Provider {
        id: "openai",
        label: "OpenAI",
        models: &[
            Model { id: "gpt-5",      label: "GPT-5",      note: "general purpose" },
            Model { id: "gpt-5-mini", label: "GPT-5 mini", note: "fast" },
        ],
    },
    Provider {
        id: "google",
        label: "Google",
        models: &[
            Model { id: "gemini-2.5", label: "Gemini 2.5 Pro", note: "very long context" },
        ],
    },
];

#[component]
pub fn ProcessStage(job_id: String) -> Element {
    let provider_id = *CURRENT_PROVIDER.read();
    let model_id = *CURRENT_MODEL.read();
    let active_provider = PROVIDERS
        .iter()
        .find(|p| p.id == provider_id)
        .unwrap_or(&PROVIDERS[0]);

    // Seed from the existing queries; the WS subscription folds further
    // updates into these signals. The initial fetch covers the case of
    // an already-completed job (no live events to receive) and gives
    // the UI something to render while the socket is opening.
    let mut steps = use_signal::<Option<Vec<PipelineStep>>>(|| None);
    let mut log = use_signal::<Option<Vec<JobLogEntry>>>(|| None);

    {
        let id = job_id.clone();
        use_future(move || {
            let id = id.clone();
            async move {
                if let Some(s) = api::fetch_job_steps(id.clone()).await {
                    steps.set(Some(s));
                }
                if let Some(l) = api::fetch_job_log(id, None, None).await {
                    log.set(Some(l));
                }
            }
        });
    }

    // Live updates over graphql-transport-ws. On native (no WS support
    // yet), `subscribe_job_events` returns immediately and the UI keeps
    // showing the seeded snapshot.
    {
        let id = job_id.clone();
        use_future(move || {
            let id = id.clone();
            async move {
                let mut log_seq: u64 = 0;
                let cb: api::JobEventCallback = Box::new(move |ev: JobEventSub| {
                    fold_event(ev, &mut steps, &mut log, &mut log_seq);
                });
                api::subscribe_job_events(id, cb).await;
            }
        });
    }

    rsx! {
        div { class: "adm-pane adm-pane-process",
            div { class: "adm-process-left",
                div { class: "adm-now-running",
                    div { class: "adm-now-running-spinner",
                        span {} span {} span {}
                    }
                    div { class: "adm-now-running-meta",
                        div { class: "adm-now-running-title", "Processing job" }
                        div { class: "adm-now-running-sub", "Live pipeline status below" }
                    }
                    button { class: "is-btn", "Pause" }
                    button { class: "is-btn", "Cancel" }
                }

                PipelineList { steps }
                LogList { log }
            }

            div { class: "adm-process-right",
                div { class: "adm-card",
                    div { class: "adm-card-title", "AI Provider" }
                    div { class: "adm-provider-tabs",
                        for p in PROVIDERS.iter() {
                            button {
                                key: "{p.id}",
                                class: if provider_id == p.id { "adm-provider-tab is-active" } else { "adm-provider-tab" },
                                onclick: {
                                    let id = p.id;
                                    move |_| {
                                        *CURRENT_PROVIDER.write() = id;
                                        if let Some(provider) = PROVIDERS.iter().find(|x| x.id == id) {
                                            if let Some(first) = provider.models.first() {
                                                *CURRENT_MODEL.write() = first.id;
                                            }
                                        }
                                    }
                                },
                                "{p.label}"
                            }
                        }
                    }
                    div { class: "adm-model-list",
                        for m in active_provider.models.iter() {
                            button {
                                key: "{m.id}",
                                class: if model_id == m.id { "adm-model-row is-selected" } else { "adm-model-row" },
                                onclick: {
                                    let id = m.id;
                                    move |_| { *CURRENT_MODEL.write() = id; }
                                },
                                div { class: "adm-model-radio",
                                    div { class: "adm-model-radio-dot" }
                                }
                                div { class: "adm-model-meta",
                                    div { class: "adm-model-name", "{m.label}" }
                                    div { class: "adm-model-note", "{m.note}" }
                                }
                            }
                        }
                    }
                }

                div { class: "adm-card",
                    div { class: "adm-card-title", "Pipeline settings" }
                    div { class: "adm-settings",
                        SettingRow {
                            label: "OCR for scanned pages",
                            sub: "Adds ~30s per 100 pages",
                            on: true,
                        }
                        SettingRow {
                            label: "Auto-flag low confidence",
                            sub: "Threshold < 70%",
                            on: true,
                        }
                        SettingRow {
                            label: "Generate audio narration",
                            sub: "Urdu TTS, 1 voice",
                            on: false,
                        }
                    }
                }
            }
        }
    }
}

/// Apply one subscription event into the live `steps`/`log` signals.
///
/// `kind` discriminates the event variant per the schema:
///   step_update         → upsert by `n` into steps
///   log_entry           → append to log with a synthetic id
///   chapter_draft_added → no-op here (handled by the drafts view)
///   pipeline_completed  → no-op here (the next status query reflects it)
fn fold_event(
    ev: JobEventSub,
    steps: &mut Signal<Option<Vec<PipelineStep>>>,
    log: &mut Signal<Option<Vec<JobLogEntry>>>,
    log_seq: &mut u64,
) {
    match ev.kind.as_str() {
        "step_update" => {
            let Some(n) = ev.n else { return };
            let mut current = steps.read().clone().unwrap_or_default();
            let updated = PipelineStep {
                id: format!("step:{n}"),
                n,
                label: ev.label.unwrap_or_default(),
                status: ev.status.unwrap_or_default(),
                detail: ev.detail,
                started_at: ev.started_at,
                finished_at: ev.finished_at,
            };
            if let Some(slot) = current.iter_mut().find(|s| s.n == n) {
                *slot = updated;
            } else {
                current.push(updated);
                current.sort_by_key(|s| s.n);
            }
            steps.set(Some(current));
        }
        "log_entry" => {
            *log_seq += 1;
            let entry = JobLogEntry {
                id: format!("live:{}", *log_seq),
                t: ev.t,
                kind: ev.log_kind.unwrap_or_else(|| "info".to_string()),
                message: ev.message.unwrap_or_default(),
            };
            let mut current = log.read().clone().unwrap_or_default();
            current.push(entry);
            log.set(Some(current));
        }
        _ => {}
    }
}

#[component]
fn PipelineList(steps: ReadSignal<Option<Vec<PipelineStep>>>) -> Element {
    match &*steps.read() {
        None => rsx! { div { class: "adm-pipeline state-loading", "Loading pipeline…" } },
        Some(items) if items.is_empty() => {
            rsx! { div { class: "adm-pipeline state-empty", "No pipeline steps yet." } }
        }
        Some(items) => rsx! {
            div { class: "adm-pipeline",
                for step in items.iter().cloned() {
                    PipelineRow { key: "{step.id}", step }
                }
            }
        },
    }
}

#[component]
fn LogList(log: ReadSignal<Option<Vec<JobLogEntry>>>) -> Element {
    let head = rsx! {
        div { class: "adm-log-head",
            span { "Live log" }
            span { class: "adm-log-cursor", "●" }
        }
    };
    match &*log.read() {
        None => rsx! {
            div { class: "adm-log",
                {head}
                div { class: "adm-log-body state-loading", "Loading log…" }
            }
        },
        Some(items) => rsx! {
            div { class: "adm-log",
                {head}
                div { class: "adm-log-body",
                    for line in items.iter().cloned() {
                        LogRow { key: "{line.id}", line }
                    }
                }
            }
        },
    }
}

#[component]
fn PipelineRow(step: PipelineStep) -> Element {
    let done = step.status == "done";
    let active = step.status == "running" || step.status == "active";
    let mut class = String::from("adm-pipeline-row");
    if done {
        class.push_str(" is-done");
    }
    if active {
        class.push_str(" is-active");
    }
    let detail = step.detail.unwrap_or_default();
    rsx! {
        div { class: "{class}",
            div { class: "adm-pipeline-status",
                if done {
                    Icon { icon: LdCheck, width: 14, height: 14 }
                } else if active {
                    span { class: "adm-pulse" }
                } else {
                    span { class: "adm-pipeline-dot" }
                }
            }
            div { class: "adm-pipeline-label", "{step.label}" }
            div { class: "adm-pipeline-detail", "{detail}" }
        }
    }
}

#[component]
fn LogRow(line: JobLogEntry) -> Element {
    let time = line.t.unwrap_or_default();
    rsx! {
        div { class: "adm-log-row is-kind-{line.kind}",
            span { class: "adm-log-time", "{time}" }
            span { class: "adm-log-msg", "{line.message}" }
        }
    }
}

#[component]
fn SettingRow(label: &'static str, sub: &'static str, on: bool) -> Element {
    let toggle_class = if on { "adm-toggle is-on" } else { "adm-toggle" };
    rsx! {
        div { class: "adm-setting-row",
            div {
                div { class: "adm-setting-label", "{label}" }
                div { class: "adm-setting-sub", "{sub}" }
            }
            div { class: "{toggle_class}", span {} }
        }
    }
}
