use crate::components::Cover;
use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{LdPencil, LdShare2},
    Icon,
};

#[component]
pub fn Profile() -> Element {
    let stats = [
        ("142", "Books finished"),
        ("1,847", "Hours read"),
        ("23", "Day streak"),
        ("412", "Highlights"),
    ];

    let recently_finished = [
        (
            "Udas Naslein",
            "Abdullah Hussein",
            "#e6c8b5",
            "اداس",
            "ع",
            "Mar 14",
        ),
        (
            "Aab-e-Hayat",
            "Muhammad Husain Azad",
            "#d4b894",
            "آب حیات",
            "م",
            "Feb 28",
        ),
        (
            "Toba Tek Singh",
            "Saadat Hasan Manto",
            "#eed7c2",
            "ٹوبہ",
            "س",
            "Feb 09",
        ),
    ];

    let highlights = [
        (
            "The night is long; even the moon is tired of waiting.",
            "Diwan-e-Ghalib · Ghazal 47",
        ),
        (
            "Some sentences are doors. You can stand outside them for years.",
            "Aag Ka Darya · Part II",
        ),
        (
            "I am not a stranger to my own house. The house has become a stranger.",
            "Toba Tek Singh",
        ),
    ];

    rsx! {
        div { class: "island is-main",
            div { class: "is-main-header",
                h2 { class: "is-main-title", "Profile" }
                span { class: "is-main-subtitle", "Reading life · since March 2024" }
                div { class: "is-main-actions",
                    button { class: "is-btn",
                        Icon { icon: LdShare2, width: 14, height: 14 }
                        "Share"
                    }
                    button { class: "is-btn",
                        Icon { icon: LdPencil, width: 14, height: 14 }
                        "Edit"
                    }
                }
            }

            div { class: "is-main-body",
                // User card
                div { style: "display: flex; gap: 20px; padding: 20px; background: var(--bg-color); border-radius: 14px; margin-bottom: 20px",
                    div { style: "width: 72px; height: 72px; border-radius: 50%; background: var(--primary); color: var(--bg-card); display: flex; align-items: center; justify-content: center; font-size: 28px; font-weight: 700; flex-shrink: 0",
                        "U"
                    }
                    div { style: "flex: 1",
                        h3 { style: "margin: 0 0 4px; font-size: 22px; font-weight: 700", "Usaira Imisani" }
                        p { style: "margin: 0 0 10px; color: var(--text-muted); font-size: 13px",
                            "Lahore · Reading mostly poetry and partition-era fiction"
                        }
                        div { style: "display: flex; gap: 6px; flex-wrap: wrap",
                            span { class: "is-chip", "ghazal" }
                            span { class: "is-chip", "partition" }
                            span { class: "is-chip", "mysticism" }
                            span { class: "is-chip", "19th-century" }
                        }
                    }
                }

                // Stats grid
                div { style: "display: grid; grid-template-columns: repeat(4, 1fr); gap: 12px; margin-bottom: 28px",
                    for (val, label) in stats {
                        div {
                            key: "{label}",
                            style: "padding: 16px; background: var(--bg-color); border-radius: 12px",
                            p { style: "margin: 0; font-size: 24px; font-weight: 700; font-family: var(--font-serif)", "{val}" }
                            p { style: "margin: 2px 0 0; font-size: 11px; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.06em; font-weight: 600", "{label}" }
                        }
                    }
                }

                // Reading goal
                div { class: "island is-rail-card", style: "margin-bottom: 28px",
                    div { class: "is-rail-eyebrow", "2026 Goal" }
                    p { style: "margin: 8px 0 4px; font-family: var(--font-serif)",
                        span { style: "font-size: 32px; font-weight: 700", "14" }
                        span { style: "font-size: 14px; color: var(--text-muted)", " / 24 books" }
                    }
                    div { class: "is-progress", style: "height: 6px; margin-top: 8px",
                        div { class: "is-progress-fill", style: "width: 58%" }
                    }
                    p { style: "margin: 10px 0 0; font-size: 12px; color: var(--text-muted)",
                        "You're 2 books ahead of pace. Next milestone: 18 by April end."
                    }
                }

                // Two-column layout
                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 24px",
                    div {
                        h4 { style: "font-size: 11px; text-transform: uppercase; letter-spacing: 0.1em; color: var(--text-muted); margin: 0 0 12px; font-weight: 700",
                            "Recently Finished"
                        }
                        div { style: "display: flex; flex-direction: column; gap: 10px",
                            for (t, a, c, u, m, d) in recently_finished {
                                div { key: "{t}", style: "display: flex; gap: 12px; align-items: center",
                                    div { style: "width: 44px; flex-shrink: 0",
                                        div { class: "is-continue-cover", style: "background: {c}; width: 44px",
                                            Cover { urdu: u, mono: m }
                                        }
                                    }
                                    div { style: "flex: 1; min-width: 0",
                                        p { style: "margin: 0; font-size: 13px; font-weight: 600", "{t}" }
                                        p { style: "margin: 0; font-size: 11px; color: var(--text-muted)", "{a}" }
                                    }
                                    span { style: "font-size: 11px; color: var(--text-muted)", "{d}" }
                                }
                            }
                        }
                    }

                    div {
                        h4 { style: "font-size: 11px; text-transform: uppercase; letter-spacing: 0.1em; color: var(--text-muted); margin: 0 0 12px; font-weight: 700",
                            "Recent Highlights"
                        }
                        div { style: "display: flex; flex-direction: column; gap: 12px",
                            for (q, b) in highlights {
                                div { key: "{q}", style: "border-left: 2px solid var(--primary); padding-left: 12px",
                                    p { class: "is-quote", style: "margin: 0 0 4px; font-size: 13px", "\u{201C}{q}\u{201D}" }
                                    p { class: "is-quote-source", "{b}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
