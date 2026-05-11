use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{LdChevronLeft, LdChevronRight, LdTriangleAlert},
    Icon,
};
use ui::api::{self, ChapterDraft};

use crate::components::ConfidenceBar;

#[component]
pub fn ReviewStage(job_id: String) -> Element {
    let drafts = {
        let id = job_id.clone();
        use_resource(move || {
            let id = id.clone();
            async move { api::fetch_chapter_drafts(id).await }
        })
    };

    rsx! {
        div { class: "adm-pane adm-pane-review",
            DraftList { drafts }
        }
    }
}

#[component]
fn DraftList(drafts: Resource<Option<Vec<ChapterDraft>>>) -> Element {
    match &*drafts.read() {
        None => rsx! { div { class: "state-loading", "Loading chapters…" } },
        Some(None) => rsx! { div { class: "state-error", "Could not load chapter drafts." } },
        Some(Some(items)) if items.is_empty() => {
            rsx! { div { class: "state-empty", "No chapter drafts yet." } }
        }
        Some(Some(items)) => {
            let items = items.clone();
            rsx! { ReviewBody { items, drafts } }
        }
    }
}

#[component]
fn ReviewBody(items: Vec<ChapterDraft>, drafts: Resource<Option<Vec<ChapterDraft>>>) -> Element {
    let initial = items
        .iter()
        .find(|c| c.status == "flagged")
        .or_else(|| items.first())
        .map(|c| c.id.clone())
        .unwrap_or_default();
    let mut selected = use_signal(|| initial);
    let active = items
        .iter()
        .find(|c| c.id == *selected.read())
        .or_else(|| items.first())
        .cloned();
    let count = items.len();

    rsx! {
        // PDF preview placeholder. Real per-page rendering will land with
        // the page-preview REST endpoint integration.
        div { class: "adm-pdf",
            div { class: "adm-pdf-toolbar",
                button { class: "adm-icon-btn",
                    Icon { icon: LdChevronLeft, width: 14, height: 14 }
                }
                span { class: "adm-pdf-page", "Page preview" }
                button { class: "adm-icon-btn",
                    Icon { icon: LdChevronRight, width: 14, height: 14 }
                }
                span { class: "adm-pdf-spacer" }
                button { class: "adm-icon-btn", "−" }
                span { class: "adm-pdf-zoom", "100%" }
                button { class: "adm-icon-btn", "+" }
            }
            div { class: "adm-pdf-page-wrap",
                if let Some(a) = active.as_ref() {
                    div { class: "adm-pdf-page-paper", dir: "rtl", lang: "ur",
                        div { class: "adm-pdf-page-h", "{a.title_ur}" }
                        div { class: "adm-pdf-page-running",
                            "Pages {a.page_range}"
                        }
                        if a.confidence < 0.7 {
                            div { class: "adm-pdf-overlay-warn",
                                Icon { icon: LdTriangleAlert, width: 16, height: 16 }
                                div {
                                    div { class: "adm-pdf-warn-title", "Low OCR confidence" }
                                    div { class: "adm-pdf-warn-sub",
                                        "{(a.confidence * 100.0).round() as i32}% — needs review"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Chapter list
        div { class: "adm-chapters",
            div { class: "adm-chapters-head",
                span { "Chapters" }
                span { class: "adm-chapters-count", "{count}" }
            }
            div { class: "adm-chapters-list",
                for c in items.iter().cloned() {
                    ChapterRow {
                        key: "{c.id}",
                        chapter: c.clone(),
                        selected: c.id == *selected.read(),
                        onclick: {
                            let id = c.id.clone();
                            move |_| selected.set(id.clone())
                        },
                    }
                }
            }
        }

        // AI metadata sidebar
        div { class: "adm-meta",
            if let Some(a) = active.as_ref() {
                MetaSidebar { chapter: a.clone(), drafts }
            }
        }
    }
}

#[component]
fn MetaSidebar(
    chapter: ChapterDraft,
    drafts: Resource<Option<Vec<ChapterDraft>>>,
) -> Element {
    let conf = chapter.confidence as f32;
    let flagged = chapter.status == "flagged";
    let summary = chapter.ai_summary.clone().unwrap_or_default();
    let chapter_id = chapter.id.clone();

    rsx! {
        div { class: "adm-meta-block",
            div { class: "adm-meta-label", "AI confidence — chapter {chapter.n}" }
            ConfidenceBar { value: conf }
            if flagged {
                div { class: "adm-meta-issue",
                    Icon { icon: LdTriangleAlert, width: 14, height: 14 }
                    span {
                        if let Some(reason) = chapter.flag_reason.as_deref() {
                            "{reason}"
                        } else {
                            "Needs human review before publish"
                        }
                    }
                }
            }
        }

        if !summary.is_empty() {
            div { class: "adm-meta-block",
                div { class: "adm-meta-label", "AI summary" }
                p { class: "adm-meta-summary", "{summary}" }
            }
        }

        if !chapter.entities.is_empty() {
            div { class: "adm-meta-block",
                div { class: "adm-meta-label", "Detected entities" }
                div { class: "adm-meta-tags",
                    for tag in chapter.entities.iter() {
                        span { key: "{tag}", class: "adm-tag", "{tag}" }
                    }
                }
            }
        }

        if !chapter.themes.is_empty() {
            div { class: "adm-meta-block",
                div { class: "adm-meta-label", "Themes" }
                div { class: "adm-meta-tags",
                    for theme in chapter.themes.iter() {
                        span { key: "{theme}", class: "adm-tag", "{theme}" }
                    }
                }
            }
        }

        div { class: "adm-meta-block",
            div { class: "adm-meta-label", "Suggested actions" }
            div { class: "adm-suggested-actions",
                button {
                    class: "is-btn is-btn--primary",
                    onclick: {
                        let id = chapter_id.clone();
                        let mut drafts = drafts;
                        move |_| {
                            let id = id.clone();
                            spawn(async move {
                                if api::approve_chapter_draft(id).await {
                                    drafts.restart();
                                }
                            });
                        }
                    },
                    "Approve chapter"
                }
                button { class: "is-btn", "Edit summary" }
                button {
                    class: "is-btn adm-btn-danger",
                    onclick: {
                        let id = chapter_id.clone();
                        let mut drafts = drafts;
                        move |_| {
                            let id = id.clone();
                            spawn(async move {
                                let reason = "Flagged for senior editor".to_string();
                                if api::flag_chapter_draft(id, reason).await {
                                    drafts.restart();
                                }
                            });
                        }
                    },
                    "Flag for senior editor"
                }
            }
        }
    }
}

#[component]
fn ChapterRow(chapter: ChapterDraft, selected: bool, onclick: EventHandler<()>) -> Element {
    let mut class = String::from("adm-chapter-row");
    if selected {
        class.push_str(" is-selected");
    }
    let flagged = chapter.status == "flagged";
    if flagged {
        class.push_str(" is-flagged");
    }

    let dot_color = if chapter.confidence >= 0.85 {
        "var(--conf-ok)"
    } else if chapter.confidence >= 0.65 {
        "var(--conf-warn)"
    } else {
        "var(--conf-bad)"
    };

    let title_en = chapter.title_en.as_deref().unwrap_or("").to_string();
    let pct = (chapter.confidence * 100.0).round() as i32;

    rsx! {
        button {
            class: "{class}",
            onclick: move |_| onclick.call(()),
            span { class: "adm-chapter-num", "{chapter.n:02}" }
            span { class: "adm-chapter-meta",
                span { class: "adm-chapter-title-ur", dir: "rtl", lang: "ur", "{chapter.title_ur}" }
                span { class: "adm-chapter-title-en", "{title_en} · pp. {chapter.page_range}" }
            }
            span {
                class: "adm-chapter-conf-mini",
                title: "{pct}%",
                span { class: "adm-conf-dot", style: "background: {dot_color}" }
            }
        }
    }
}
