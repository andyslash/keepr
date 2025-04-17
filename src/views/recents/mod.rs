use dioxus::prelude::*;

#[component]
pub fn Recents() -> Element {
    rsx! {
        div { class: "p-8 text-xl text-base-content", "Recents Page" }
    }
}
