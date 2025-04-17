use dioxus::prelude::*;

#[component]
pub fn Radial(
    #[props(optional)] class: Option<&'static str>,
    #[props(optional)] value: Option<u8>,
    #[props(optional)] size: Option<&'static str>,
) -> Element {
    let class = class.unwrap_or("text-current");
    let value = value.unwrap_or(0).to_string();
    let size = size.unwrap_or("5rem").to_string();

    rsx! {
        div {
            class: "{class} radial-progress",
            style: "--value:{value}; --size:{size};",
            aria_valuenow: "{value}",
            role: "progressbar",
            "{value}%"
        }
    }
}
