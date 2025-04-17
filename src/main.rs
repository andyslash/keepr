use dioxus::prelude::*;
use dioxus_desktop::{
    tao::platform::macos::WindowBuilderExtMacOS, Config, LogicalSize, WindowBuilder,
};

mod components;
mod layout;
mod route;
mod views;

use crate::route::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    LaunchBuilder::desktop()
        .with_cfg(
            Config::new().with_window(
                WindowBuilder::new()
                    .with_title("Keepr")
                    .with_inner_size(LogicalSize::new(1366.0, 900.0))
                    .with_focused(true)
                    .with_has_shadow(true)
                    .with_movable_by_window_background(true)
                    .with_maximizable(true)
                    .with_transparent(true)
                    .with_decorations(false),
            ),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
