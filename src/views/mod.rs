pub mod docker_builds;
pub mod docker_containers;
pub mod docker_images;
pub mod docker_volumes;
pub mod dotfiles;
pub mod keychains;
pub mod logs_explorer;
pub mod processes;
pub mod projects;
pub mod recents;
pub mod snippets;

use dioxus::prelude::*;

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    let full_path = format!("/{}", route.join("/"));

    rsx!(
        div { class: "flex flex-col items-center justify-center h-full text-center p-8 text-base-content",
            h1 { class: "text-4xl font-bold mb-4", "404 - Page Not Found" }
            p { class: "text-lg mb-2",
                "La route "
                code { class: "font-mono bg-base-200 px-2 py-1 rounded", "{full_path}" }
                " n'existe pas."
            }
        }
    )
}
