use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{LdBookmark, LdCheck},
    Icon,
};
use ui::api::{self, IngestionJob, PublishCheck};

#[derive(Clone, Copy, PartialEq)]
enum Visibility {
    Public,
    Unlisted,
    Draft,
}

impl Visibility {
    fn slug(self) -> &'static str {
        match self {
            Visibility::Public => "public",
            Visibility::Unlisted => "unlisted",
            Visibility::Draft => "draft",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum PublishState {
    Idle,
    Submitting,
    Done,
    Error,
}

#[component]
pub fn PublishStage(job: IngestionJob) -> Element {
    let checks = {
        let id = job.id.clone();
        use_resource(move || {
            let id = id.clone();
            async move { api::fetch_publish_checks(id).await }
        })
    };

    let mut visibility = use_signal(|| Visibility::Public);
    let mut publish_state = use_signal(|| PublishState::Idle);

    let title = job
        .hint_title
        .clone()
        .unwrap_or_else(|| "Untitled job".to_string());
    let title_ur = job.hint_title_ur.clone().unwrap_or_default();
    let author = job.hint_author.clone().unwrap_or_default();
    let cover_color = job
        .cover_color
        .clone()
        .unwrap_or_else(|| "var(--surface-2)".to_string());
    let cover_glyph = job.cover_glyph.clone().unwrap_or_else(|| "م".to_string());
    let pages = job.pages.unwrap_or(0);
    let chapters = job.chapters_total.unwrap_or(0);
    let job_id = job.id.clone();
    let submitting = matches!(*publish_state.read(), PublishState::Submitting);
    let done = matches!(*publish_state.read(), PublishState::Done);

    rsx! {
        div { class: "adm-pane adm-pane-publish",
            div { class: "adm-publish-left",
                div { class: "adm-publish-preview",
                    div { class: "adm-publish-preview-frame",
                        div {
                            class: "adm-publish-cover",
                            style: "background: {cover_color}",
                            div { class: "adm-publish-cover-glyph", "{cover_glyph}" }
                            div { class: "adm-publish-cover-title", "{title_ur}" }
                            div { class: "adm-publish-cover-author", "{author}" }
                        }
                        div { class: "adm-publish-card-meta",
                            div { class: "adm-publish-card-title", "{title}" }
                            div { class: "adm-publish-card-author", "{author}" }
                            div { class: "adm-publish-card-stats",
                                span { "{pages} pages" }
                                span { class: "adm-dot-sep", "·" }
                                span { "{chapters} chapters" }
                            }
                        }
                    }
                    div { class: "adm-publish-preview-cap", "Preview · how readers will see it" }
                }

                div { class: "adm-card",
                    div { class: "adm-card-title", "Visibility" }
                    div { class: "adm-visibility",
                        VisRow {
                            checked: *visibility.read() == Visibility::Public,
                            label: "Public",
                            sub: "Discoverable, indexed, recommendations on",
                            onclick: move |_| visibility.set(Visibility::Public),
                        }
                        VisRow {
                            checked: *visibility.read() == Visibility::Unlisted,
                            label: "Unlisted",
                            sub: "Direct-link only, no recommendations",
                            onclick: move |_| visibility.set(Visibility::Unlisted),
                        }
                        VisRow {
                            checked: *visibility.read() == Visibility::Draft,
                            label: "Draft",
                            sub: "Internal team only",
                            onclick: move |_| visibility.set(Visibility::Draft),
                        }
                    }
                }
            }

            div { class: "adm-publish-right",
                div { class: "adm-card",
                    div { class: "adm-card-title", "Pre-flight checks" }
                    Checklist { checks }
                }

                div { class: "adm-card adm-card-publish",
                    div { class: "adm-card-title",
                        if done { "Published" } else { "Ready to publish" }
                    }
                    p { class: "adm-publish-note",
                        "Going live makes "
                        strong { "{title}" }
                        " available to all Musanif readers. You can unpublish at any time from the book's admin page."
                    }
                    if matches!(*publish_state.read(), PublishState::Error) {
                        p { class: "adm-publish-error",
                            "Publish failed. Check the pre-flight list above and try again."
                        }
                    }
                    div { class: "adm-publish-actions",
                        button { class: "is-btn", disabled: submitting || done, "Schedule for later" }
                        button {
                            class: "is-btn is-btn--primary adm-btn-publish",
                            disabled: submitting || done,
                            onclick: {
                                let job_id = job_id.clone();
                                move |_| {
                                    let job_id = job_id.clone();
                                    let vis = visibility.read().slug().to_string();
                                    publish_state.set(PublishState::Submitting);
                                    spawn(async move {
                                        let result = api::publish_ingestion_job(job_id, vis).await;
                                        publish_state.set(if result.is_some() {
                                            PublishState::Done
                                        } else {
                                            PublishState::Error
                                        });
                                    });
                                }
                            },
                            Icon { icon: LdBookmark, width: 14, height: 14 }
                            if submitting {
                                "Publishing…"
                            } else if done {
                                "Published"
                            } else {
                                "Publish to library"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Checklist(checks: Resource<Option<Vec<PublishCheck>>>) -> Element {
    match &*checks.read() {
        None => rsx! { div { class: "adm-checklist state-loading", "Loading checks…" } },
        Some(None) => rsx! { div { class: "adm-checklist state-error", "Could not load checks." } },
        Some(Some(items)) if items.is_empty() => {
            rsx! { div { class: "adm-checklist state-empty", "No checks reported." } }
        }
        Some(Some(items)) => rsx! {
            div { class: "adm-checklist",
                for c in items.iter().cloned() {
                    CheckRow { key: "{c.label}", check: c }
                }
            }
        },
    }
}

#[component]
fn CheckRow(check: PublishCheck) -> Element {
    let class = if check.ok {
        "adm-check-row is-ok"
    } else {
        "adm-check-row is-err"
    };
    let detail = check.detail.clone().unwrap_or_default();
    rsx! {
        div { class: "{class}",
            span { class: "adm-check-icon",
                if check.ok {
                    Icon { icon: LdCheck, width: 14, height: 14 }
                } else {
                    "—"
                }
            }
            div { class: "adm-check-meta",
                div { class: "adm-check-label", "{check.label}" }
                div { class: "adm-check-detail", "{detail}" }
            }
        }
    }
}

#[component]
fn VisRow(
    checked: bool,
    label: &'static str,
    sub: &'static str,
    onclick: EventHandler<()>,
) -> Element {
    let class = if checked {
        "adm-vis-row is-checked"
    } else {
        "adm-vis-row"
    };
    rsx! {
        button {
            class: "{class}",
            onclick: move |_| onclick.call(()),
            span { class: "adm-vis-radio", span {} }
            div {
                div { class: "adm-vis-label", "{label}" }
                div { class: "adm-vis-sub", "{sub}" }
            }
        }
    }
}
