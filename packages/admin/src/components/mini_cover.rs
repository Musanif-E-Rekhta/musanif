use dioxus::prelude::*;

const FALLBACK_COLOR: &str = "var(--surface-2)";
const FALLBACK_GLYPH: &str = "م";

/// Small book cover thumbnail: colored rectangle + Urdu glyph. Color and
/// glyph default to neutral fallbacks when the ingestion job hasn't
/// produced cover hints yet.
#[component]
pub fn MiniCover(color: Option<String>, glyph: Option<String>, size: u32) -> Element {
    let height = (size as f32 * 1.35) as u32;
    let bg = color.as_deref().unwrap_or(FALLBACK_COLOR);
    let style = format!("width: {size}px; height: {height}px; background: {bg}");
    let g = glyph.as_deref().unwrap_or(FALLBACK_GLYPH);
    rsx! {
        span { class: "adm-mini-cover", style: "{style}",
            span { class: "adm-mini-cover-glyph", "{g}" }
        }
    }
}
