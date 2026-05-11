use dioxus::prelude::*;

/// ID of the ingestion job the operator is currently focused on. `None`
/// until the sidebar's first fetch resolves and selects an initial job.
pub static CURRENT_JOB: GlobalSignal<Option<String>> = Signal::global(|| None);

/// Which of the 5 ingestion stages is shown in the main pane. The job's
/// own progress (`stage` field) determines which stages are reachable —
/// the stepper locks anything past it.
pub static CURRENT_STAGE: GlobalSignal<u8> = Signal::global(|| 1);

/// Top-level admin tab — Ingestion (queue) / Library / Analytics.
#[derive(Clone, Copy, PartialEq)]
pub enum AdminSection {
    Ingestion,
    Library,
    Analytics,
}

pub static CURRENT_SECTION: GlobalSignal<AdminSection> =
    Signal::global(|| AdminSection::Ingestion);

/// AI provider settings for the Process panel.
pub static CURRENT_PROVIDER: GlobalSignal<&'static str> = Signal::global(|| "claude");
pub static CURRENT_MODEL: GlobalSignal<&'static str> = Signal::global(|| "sonnet-4.5");

/// One pipeline-stage label. The stepper renders these in order and the
/// stage views key off `n` to decide which body component to mount.
#[derive(Clone, Copy, PartialEq)]
pub struct Stage {
    pub n: u8,
    pub label: &'static str,
    pub ur: &'static str,
}

pub const STAGES: &[Stage] = &[
    Stage { n: 1, label: "Upload",  ur: "اپ لوڈ" },
    Stage { n: 2, label: "Process", ur: "تجزیہ" },
    Stage { n: 3, label: "Review",  ur: "جائزہ" },
    Stage { n: 4, label: "Edit",    ur: "ترمیم" },
    Stage { n: 5, label: "Publish", ur: "شائع" },
];

pub fn stage_label(n: u8) -> &'static str {
    STAGES
        .get((n.saturating_sub(1)) as usize)
        .map(|s| s.label)
        .unwrap_or("")
}
