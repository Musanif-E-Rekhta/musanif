use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{LdCheck, LdChevronLeft, LdChevronRight, LdHeart},
    Icon,
};
use ui::api::{self, ChapterDraft};

#[component]
pub fn EditStage(job_id: String) -> Element {
    let drafts = {
        let id = job_id.clone();
        use_resource(move || {
            let id = id.clone();
            async move { api::fetch_chapter_drafts(id).await }
        })
    };

    rsx! {
        div { class: "adm-pane adm-pane-edit",
            EditBody { drafts }
        }
    }
}

#[component]
fn EditBody(drafts: Resource<Option<Vec<ChapterDraft>>>) -> Element {
    match &*drafts.read() {
        None => rsx! { div { class: "state-loading", "Loading drafts…" } },
        Some(None) => rsx! { div { class: "state-error", "Could not load chapter drafts." } },
        Some(Some(items)) if items.is_empty() => {
            rsx! { div { class: "state-empty", "No chapter drafts to edit yet." } }
        }
        Some(Some(items)) => {
            // Prefer the first flagged or non-approved draft so the editor
            // lands somewhere actionable.
            let chapter = items
                .iter()
                .find(|c| c.status == "flagged")
                .or_else(|| items.iter().find(|c| c.status != "approved"))
                .or_else(|| items.first())
                .cloned()
                .unwrap();
            rsx! { Diff { chapter, drafts } }
        }
    }
}

#[component]
fn Diff(chapter: ChapterDraft, drafts: Resource<Option<Vec<ChapterDraft>>>) -> Element {
    let pct = (chapter.confidence * 100.0).round() as i32;
    let title_en = chapter.title_en.as_deref().unwrap_or("").to_string();
    let ai_content = chapter.ai_content.clone();
    let human = chapter.human_content.clone();
    let chapter_id = chapter.id.clone();

    rsx! {
        div { class: "adm-edit-toolbar",
            button { class: "is-btn",
                Icon { icon: LdChevronLeft, width: 14, height: 14 }
                "Previous"
            }
            div { class: "adm-edit-title",
                span { class: "adm-edit-title-ur", dir: "rtl", lang: "ur",
                    "{chapter.title_ur}"
                }
                span { class: "adm-edit-title-en",
                    "Chapter {chapter.n} · {title_en} · pp. {chapter.page_range}"
                }
            }
            button { class: "is-btn",
                "Next"
                Icon { icon: LdChevronRight, width: 14, height: 14 }
            }
            span { style: "flex: 1" }
            button {
                class: "is-btn",
                onclick: {
                    let id = chapter_id.clone();
                    let mut drafts = drafts;
                    move |_| {
                        let id = id.clone();
                        spawn(async move {
                            let reason = "AI extraction rejected during edit".to_string();
                            if api::flag_chapter_draft(id, reason).await {
                                drafts.restart();
                            }
                        });
                    }
                },
                "Reject AI"
            }
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
                Icon { icon: LdCheck, width: 14, height: 14 }
                "Approve edit"
            }
        }

        div { class: "adm-diff",
            div { class: "adm-diff-side",
                div { class: "adm-diff-head",
                    span { class: "adm-diff-tag is-ai", "AI extracted" }
                    span { class: "adm-diff-conf", "{pct}% confidence" }
                }
                div { class: "adm-diff-body", dir: "rtl", lang: "ur",
                    p { "{ai_content}" }
                }
            }

            div { class: "adm-diff-side",
                div { class: "adm-diff-head",
                    span { class: "adm-diff-tag is-human", "Human edit" }
                    span { class: "adm-diff-conf",
                        if human.is_some() { "Edited" } else { "Not yet edited" }
                    }
                }
                div { class: "adm-diff-body", dir: "rtl", lang: "ur",
                    if let Some(h) = human.as_ref() {
                        p { "{h}" }
                    } else {
                        p { class: "adm-diff-placeholder",
                            "Open this chapter to begin editing."
                        }
                    }
                }
            }
        }

        div { class: "adm-edit-footer",
            div { class: "adm-edit-changes",
                span { class: "adm-edit-note",
                    "Status: {chapter.status}"
                }
            }
            div { class: "adm-edit-comments",
                Icon { icon: LdHeart, width: 14, height: 14 }
                span { "Comments" }
            }
        }
    }
}
