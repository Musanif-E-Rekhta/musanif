use dioxus::prelude::*;

#[component]
pub fn DesktopWindow(
    children: Element,
    #[props(default = vec!["Discover".to_string()])]
    tabs: Vec<String>,
    #[props(default = 0)]
    active_tab: usize,
    on_tab_click: Option<EventHandler<usize>>,
) -> Element {
    rsx! {
        div { class: "dt-window",
            div { 
                class: "dt-window-chrome",
                onmousedown: move |_| { 
                    #[cfg(feature = "desktop")]
                    dioxus::desktop::window().drag(); 
                },
                
                div { class: "dt-traffic-lights",
                    div { 
                        class: "dt-light dt-light--close",
                        onclick: move |e| {
                            e.stop_propagation();
                            #[cfg(feature = "desktop")]
                            dioxus::desktop::window().close();
                        }
                    }
                    div { 
                        class: "dt-light dt-light--min",
                        onclick: move |e| {
                            e.stop_propagation();
                            #[cfg(feature = "desktop")]
                            dioxus::desktop::window().set_minimized(true);
                        }
                    }
                    div { 
                        class: "dt-light dt-light--max",
                        onclick: move |e| {
                            e.stop_propagation();
                            #[cfg(feature = "desktop")]
                            {
                                let w = dioxus::desktop::window();
                                w.set_maximized(!w.is_maximized());
                            }
                        }
                    }
                }

                div { class: "dt-tabs",
                    for (i , tab) in tabs.iter().enumerate() {
                        div { 
                            key: "{i}",
                            class: if i == active_tab { "dt-tab dt-tab--active" } else { "dt-tab" },
                            onclick: move |e| {
                                e.stop_propagation();
                                if let Some(handler) = on_tab_click {
                                    handler.call(i);
                                }
                            },
                            "{tab}"
                        }
                    }
                }
            }

            div { class: "dt-window-body",
                {children}
            }
        }
    }
}
