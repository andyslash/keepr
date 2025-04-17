use crate::components::icons::Icon;
use dioxus::prelude::*;

#[component]
pub fn TabWrapper(
    tab: &'static str,
    label: &'static str,
    icon: &'static str,
    #[props(optional)] class: Option<&'static str>,
    #[props(optional)] checked: Option<bool>,
    #[props(optional)] content: Option<Element>,
) -> Element {
    let class = class.unwrap_or("");
    let is_checked = checked.unwrap_or(false);

    rsx! {
        label { class: "tab {class} text-sm",
            input { r#type: "radio", name: "{tab}", checked: is_checked }
            Icon { name: icon, size: 14 }
            span { class: "text-base-100 ml-1", "{label}" }
        }

        div { class: "tab-content bg-base-100", {content} }
    }
}
