use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_effect(move || {
        let theme = ui::CURRENT_THEME().as_str();
        let _ = document::eval(&format!(
            "document.documentElement.setAttribute('data-theme', '{theme}');"
        ));
    });

    rsx! {
        document::Title { "Musanif" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<ui::Route> {}
        ui::components::ThemeSwitcher {}
    }
}
