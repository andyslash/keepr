use dioxus::prelude::*;

#[component]
pub fn DockerBuilds() -> Element {
    rsx! {
        div { class: "p-8 text-xl text-base-content", "Docker Builds Page" }
    }
}
