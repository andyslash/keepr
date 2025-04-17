use dioxus::prelude::*;

#[component]
pub fn DockerVolumes() -> Element {
    rsx! {
        div { class: "p-8 text-xl text-base-content", "Docker Volumes Page" }
    }
}
