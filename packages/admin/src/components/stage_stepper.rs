use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons::LdCheck, Icon};

use crate::state::{Stage, STAGES};

/// 5-stage horizontal stepper. The job's own `stage` decides which steps
/// are reachable; the operator can click any reachable step to switch the
/// main pane via the `onchange` event.
#[component]
pub fn StageStepper(active: u8, job_stage: u8, onchange: EventHandler<u8>) -> Element {
    rsx! {
        div { class: "adm-stepper",
            for (i, s) in STAGES.iter().enumerate() {
                StageStepperStep {
                    key: "{s.n}",
                    stage: *s,
                    is_last: i == STAGES.len() - 1,
                    active_n: active,
                    job_stage,
                    onchange,
                }
            }
        }
    }
}

#[component]
fn StageStepperStep(
    stage: Stage,
    is_last: bool,
    active_n: u8,
    job_stage: u8,
    onchange: EventHandler<u8>,
) -> Element {
    let done = job_stage > stage.n;
    let current = active_n == stage.n;
    let reachable = stage.n <= job_stage;

    let mut class = String::from("adm-step");
    if current {
        class.push_str(" is-current");
    }
    if done {
        class.push_str(" is-done");
    }
    if !reachable {
        class.push_str(" is-locked");
    }

    let connector_class = if done {
        "adm-step-connector is-done"
    } else {
        "adm-step-connector"
    };

    rsx! {
        button {
            class: "{class}",
            disabled: !reachable,
            onclick: move |_| {
                if reachable {
                    onchange.call(stage.n);
                }
            },
            span { class: "adm-step-num",
                if done {
                    Icon { icon: LdCheck, width: 14, height: 14 }
                } else {
                    "{stage.n}"
                }
            }
            span { class: "adm-step-label",
                span { class: "adm-step-en", "{stage.label}" }
                span { class: "adm-step-ur", dir: "rtl", lang: "ur", "{stage.ur}" }
            }
        }
        if !is_last {
            div { class: "{connector_class}" }
        }
    }
}
