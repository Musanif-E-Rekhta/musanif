use crate::components::Cover;
use crate::{Route, CURRENT_USER};
use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{LdLogIn, LdLogOut, LdPencil, LdShare2, LdUserPlus},
    Icon,
};

#[component]
pub fn Profile() -> Element {
    let stats = [
        ("0", "Books finished"),
        ("0", "Hours read"),
        ("0", "Day streak"),
        ("0", "Highlights"),
    ];

    let recently_finished: [(&str, &str, &str, &str, &str, &str); 0] = [];

    let highlights: [(&str, &str); 0] = [];

    let current_user = CURRENT_USER.read();
    let is_authenticated = current_user.is_some();

    rsx! {
        div { class: "island is-main",
            div { class: "is-main-header",
                h2 { class: "is-main-title", "Profile" }
                span { class: "is-main-subtitle", "Reading life · since March 2024" }
                div { class: "is-main-actions",
                    if is_authenticated {
                        button { class: "is-btn",
                            Icon { icon: LdShare2, width: 14, height: 14 }
                            "Share"
                        }
                        button { class: "is-btn",
                            Icon { icon: LdPencil, width: 14, height: 14 }
                            "Edit"
                        }
                        button {
                            class: "is-btn",
                            onclick: move |_| {
                                *CURRENT_USER.write() = None;
                            },
                            Icon { icon: LdLogOut, width: 14, height: 14 }
                            "Sign out"
                        }
                    } else {
                        Link {
                            to: Route::Login {},
                            class: "is-btn",
                            style: "background: var(--primary); color: var(--bg-card); border-color: var(--primary)",
                            Icon { icon: LdLogIn, width: 14, height: 14 }
                            "Sign In"
                        }
                        Link {
                            to: Route::Signup {},
                            class: "is-btn",
                            Icon { icon: LdUserPlus, width: 14, height: 14 }
                            "Sign Up"
                        }
                    }
                }
            }

            div { class: "is-main-body",
                // User card
                div { style: "display: flex; gap: 20px; padding: 20px; background: var(--bg-color); border-radius: 14px; margin-bottom: 20px",
                    div { style: "width: 72px; height: 72px; border-radius: 50%; background: var(--primary); color: var(--bg-card); display: flex; align-items: center; justify-content: center; font-size: 28px; font-weight: 700; flex-shrink: 0",
                        if let Some(user) = current_user.as_ref() {
                            "{user.username.chars().next().unwrap_or('?').to_ascii_uppercase()}"
                        } else {
                            "G"
                        }
                    }
                    div { style: "flex: 1; display: flex; flex-direction: column; justify-content: center;",
                        if let Some(user) = current_user.as_ref() {
                            h3 { style: "margin: 0 0 4px; font-size: 22px; font-weight: 700", "{user.username}" }
                            p { style: "margin: 0; color: var(--text-muted); font-size: 13px",
                                "{user.email}"
                            }
                        } else {
                            h3 { style: "margin: 0 0 4px; font-size: 22px; font-weight: 700", "Guest" }
                            p { style: "margin: 0; color: var(--text-muted); font-size: 13px",
                                "Sign in to sync your progress and highlights."
                            }
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
