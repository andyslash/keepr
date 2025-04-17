use dioxus::prelude::*;

#[component]
pub fn Snippets() -> Element {
    rsx! {
        div { class: "p-8 text-xl text-base-content", "Snippets Page" }
    }
}
