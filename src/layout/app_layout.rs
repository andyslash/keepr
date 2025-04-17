use dioxus::prelude::*;
use dioxus_desktop::use_window;

use crate::components;
use crate::Route;
use components::{searchbar::Searchbar, sidebar::Sidebar};

const APP_THEME: &str = "winter";

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div {
            class: "rounded-xl overflow-hidden bg-base-400 h-screen w-screen",
            onmousedown: move |_| {
                let window = use_window();
                let _ = window.drag_window();
            },
            ondoubleclick: move |_| {
                let window = use_window();
                let is_maximized = window.is_maximized();
                if is_maximized {
                    let _ = window.set_maximized(false);
                } else {
                    let _ = window.set_maximized(true);
                }
            },
            div { class: "flex flex-row", "data-theme": APP_THEME,
                div { class: "flex min-h-screen flex-col flex-auto flex-shrink-0 antialiased bg-base-200",
                    Sidebar {}
                }
                div { class: "flex min-h-screen w-full flex-col items-center justify-start",
                    div { class: "flex flex-row items-center justify-around px-4 h-12 bg-primary border-b border-base-300 w-full z-0",
                        div { class: "flex flex-grow-2" }
                        Searchbar {}
                    }
                    div { class: "main-content -mt-8 flex flex-col w-full h-full overflow-y-auto",
                        Outlet::<Route> {}
                    }
                                // div { class:"flex align-self-end bg-neutral-content w-full h-10 items-center justify-center ",
                //     "FOOTER" }
                }
            }
        }
    }
}
