use dioxus::prelude::*;

use crate::Route;

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
        div { class: "island is-main",
            div { class: "is-main-header",
                h2 { class: "is-main-title", "Authors" }
                span { class: "is-main-subtitle", "Voices of Urdu literature" }
            }

            div { class: "is-main-body",
                div { style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(220px, 1fr)); gap: 16px",
                    for (name, years, count, m) in authors {
                        Link {
                            key: "{name}",
                            class: "is-book",
                            style: "display: flex; gap: 14px; padding: 14px; background: var(--bg-color); border-radius: 12px; align-items: center",
                            to: Route::AuthorDetail { slug: name.to_lowercase().replace(' ', "-") },
                            div { style: "width: 48px; height: 48px; border-radius: 50%; background: var(--accent-light); color: var(--primary); display: flex; align-items: center; justify-content: center; font-family: var(--font-urdu); font-size: 20px; font-weight: 700; flex-shrink: 0",
                                "{m}"
                            }
                            div {
                                p { style: "font-size: 14px; font-weight: 700; margin: 0 0 2px", "{name}" }
                                p { style: "font-size: 11px; color: var(--text-muted); margin: 0", "{years} · {count} books" }
                            }
                        }
                    }
                }
            }
        }
    }
}
