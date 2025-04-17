use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
        div { class: "p-8 text-xl text-base-content", "Projects Page" }
    }
}
