use dioxus::prelude::*;

use crate::layout::app_layout::AppLayout;

use crate::views::{
    docker_builds::DockerBuilds, docker_containers::DockerContainers, docker_images::DockerImages,
    docker_volumes::DockerVolumes, dotfiles::Dotfiles, keychains::Keychains,
    logs_explorer::LogsExplorer, processes::Processes, projects::Projects, recents::Recents,
    snippets::Snippets, PageNotFound,
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/")]
            #[redirect("/recents",|| Route::Recents {})]
        #[route("/docker/builds")]
        DockerBuilds {},
        #[route("/docker/containers")]
        DockerContainers {},

        #[route("/docker/images")]
        DockerImages {},

        #[route("/docker/volumes")]
        DockerVolumes {},

        #[route("/dotfiles")]
        Dotfiles {},

        #[route("/keychains")]
        Keychains {},

        #[route("/logs_explorer")]
        LogsExplorer {},

        #[route("/processes")]
        Processes {},

        #[route("/projects")]
        Projects {},

        #[route("/recents")]
        Recents {},

        #[route("/snippets")]
        Snippets {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
