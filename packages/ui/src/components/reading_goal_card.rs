use dioxus::prelude::*;

/// Right-rail card showing yearly reading goal progress: big counter, thin
/// bar, and a contextual hint about pace.
#[component]
pub fn ReadingGoalCard(
    year: i32,
    completed: i32,
    target: i32,
    pace_hint: Option<String>,
) -> Element {
    let pct = if target > 0 {
        (completed as f64 / target as f64 * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };
    rsx! {
        div { class: "island is-rail-card",
            div { class: "is-rail-eyebrow", "Reading Goal" }
            div { class: "is-goal-headline",
                span { class: "is-goal-count", "{completed}" }
                span { class: "is-goal-target", "/ {target} books in {year}" }
            }
            div { class: "is-progress is-progress--lg",
                div { class: "is-progress-fill", style: "width: {pct:.0}%" }
            }
            if let Some(hint) = pace_hint.as_deref().filter(|s| !s.is_empty()) {
                p { class: "is-goal-hint", "{hint}" }
            }
        }
    }
}
