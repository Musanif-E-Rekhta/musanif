//! Web (WASM) entry point for the Musanif reader.
//!
//! Mounts [`ui::Route`] inside a Dioxus router and applies the
//! persisted theme on first render. Platform-specific chrome lives
//! exclusively on top of [`ui`]; everything reusable is shared from
//! `packages/ui`.

use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::logger::initialize_default();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_effect(move || {
        ui::theme::apply_and_persist(ui::CURRENT_THEME().as_str());
    });

    rsx! {
        document::Title { "Musanif" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<ui::Route> {}
    }
}
