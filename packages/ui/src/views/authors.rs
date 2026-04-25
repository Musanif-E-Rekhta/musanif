use dioxus::prelude::*;

#[component]
pub fn Authors() -> Element {
    let authors = [
        ("Mirza Ghalib", "1797–1869", 4, "غ"),
        ("Allama Iqbal", "1877–1938", 3, "ا"),
        ("Qurratulain Hyder", "1927–2007", 6, "ق"),
        ("Saadat Hasan Manto", "1912–1955", 8, "س"),
        ("Bano Qudsia", "1928–2017", 5, "ب"),
        ("Abdullah Hussein", "1931–2015", 2, "ع"),
    ];

    rsx! {
        div { class: "is-main",
            if cfg!(feature = "mobile") {
                div { class: "is-mob-header",
                    div {
                        p { class: "is-mob-greet", "Voices of Urdu" }
                        h1 { class: "is-mob-title", "Authors" }
                    }
                }
            } else {
                div { class: "is-main-header",
                    h2 { class: "is-main-title", "Authors" }
                    span { class: "is-main-subtitle", "Explore the masters of Urdu literature" }
                }
            }

            div { class: "is-mob-search",
                svg {
                    width: "18", height: "18", fill: "none", stroke: "currentColor",
                    stroke_width: "2", view_box: "0 0 24 24", stroke_linecap: "round", stroke_linejoin: "round",
                    circle { cx: "11", cy: "11", r: "8" }
                    path { d: "M21 21l-4.35-4.35" }
                }
                input { placeholder: "Search authors…" }
            }

            div { class: "is-main-body",
                div { style: "display: flex; flex-direction: column; gap: 8px",
                    for (name, years, count, m) in authors {
                        div { class: "island", style: "display: flex; gap: 12px; padding: 14px; align-items: center",
                            div { 
                                style: "width: 42px; height: 42px; border-radius: 50%; \
                                        background: var(--accent-light); color: var(--primary); \
                                        display: flex; align-items: center; justify-content: center; \
                                        font-family: var(--font-urdu); fontSize: 18px; font-weight: 700",
                                "{m}"
                            }
                            div { style: "flex: 1",
                                p { style: "font-size: 14px; font-weight: 700; margin: 0 0 2px", "{name}" }
                                p { style: "font-size: 11px; color: var(--text-muted); margin: 0", "{years} · {count} books" }
                            }
                            span { style: "color: var(--text-muted)", "›" }
                        }
                    }
                }
            }
        }
    }
}
