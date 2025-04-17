use dioxus::prelude::*;

use crate::components::icons::Icon;

#[component]
pub fn Searchbar() -> Element {
    rsx! {
        div { class: "search-bar justify-self-end",
            label { class: "input input-sm",
                span { class: "h-4 opacity-50",
                    Icon { name: "search", size: 15 }
                }
                input { r#type: "search", class: "grow", placeholder: "Search" }
                kbd { class: "kbd kbd-sm", "⌘" }
                kbd { class: "kbd kbd-sm", "K" }
            }
        }
    }
}
