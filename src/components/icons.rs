use dioxus::prelude::*;

#[component]
pub fn Icon(
    name: &'static str,
    #[props(optional)] class: Option<&'static str>,
    #[props(optional)] size: Option<u8>,
) -> Element {
    let size = size.unwrap_or(20).to_string();
    let class = class.unwrap_or("text-current");

    match name {
        "memory" => rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                class: "lucide lucide-memory-stick-icon lucide-memory-stick",

                path { d: "M6 19v-3" }
                path { d: "M10 19v-3" }
                path { d: "M14 19v-3" }
                path { d: "M18 19v-3" }
                path { d: "M8 11V9" }
                path { d: "M16 11V9" }
                path { d: "M12 11V9" }
                path { d: "M2 15h20" }
                path { d: "M2 7a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v1.1a2 2 0 0 0 0 3.837V17a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-5.1a2 2 0 0 0 0-3.837Z" }
            }
        ),
        "zap" => rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                class: "lucide lucide-zap-icon lucide-zap",

                path { d: "M4 14a1 1 0 0 1-.78-1.63l9.9-10.2a.5.5 0 0 1 .86.46l-1.92 6.02A1 1 0 0 0 13 10h7a1 1 0 0 1 .78 1.63l-9.9 10.2a.5.5 0 0 1-.86-.46l1.92-6.02A1 1 0 0 0 11 14z" }
            }
        ),
        "trash" => rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                class: "{class}",

                path { d: "M3 6h18" }
                path { d: "M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" }
                path { d: "M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" }
                line {
                    x1: "10",
                    y1: "11",
                    x2: "10",
                    y2: "17",
                }
                line {
                    x1: "14",
                    y1: "11",
                    x2: "14",
                    y2: "17",
                }
            }
        ),
        "pause" => rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                class: "{class}",
                rect {
                    x: "14",
                    y: "4",
                    width: "4",
                    height: "16",
                    rx: "1",
                }
                rect {
                    x: "6",
                    y: "4",
                    width: "4",
                    height: "16",
                    rx: "1",
                }
            }
        ),
        "play" => rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                class: "lucide lucide-play-icon lucide-play",
                polygon { points: "6 3 20 12 6 21 6 3" }
            }
        ),
        "history" => rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{size}",
                height: "{size}",
                view_box: "0 0 24 24",
                class: "{class}",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" }
                path { d: "M3 3v5h5" }
                path { d: "M12 7v5l4 2" }
            }
        ),
        "search" => rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{size}",
                height: "{size}",
                view_box: "0 0 24 24",
                class: "{class}",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                circle { cx: "11", cy: "11", r: "8" }
                path { d: "m21 21-4.3-4.3" }
            }
        ),
        "logs" => rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{size}",
                height: "{size}",
                view_box: "0 0 24 24",
                class: "{class}",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M3 3v16a2 2 0 0 0 2 2h16" }
                path { d: "M7 16h8" }
                path { d: "M7 11h12" }
                path { d: "M7 6h3" }
            }
        ),
        "folder" => rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{size}",
                height: "{size}",
                view_box: "0 0 24 24",
                class: "{class}",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z" }
                path { d: "M8 10v4" }
                path { d: "M12 10v2" }
                path { d: "M16 10v6" }
            }
        ),
        "key" => rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{size}",
                height: "{size}",
                view_box: "0 0 24 24",
                class: "{class}",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M2.586 17.414A2 2 0 0 0 2 18.828V21a1 1 0 0 0 1 1h3a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h1a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h.172a2 2 0 0 0 1.414-.586l.814-.814a6.5 6.5 0 1 0-4-4z" }
                circle {
                    cx: "16.5",
                    cy: "7.5",
                    r: ".5",
                    fill: "currentColor",
                }
            }
        ),
        "snippets" => rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{size}",
                height: "{size}",
                view_box: "0 0 24 24",
                class: "{class}",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M15 12h-5" }
                path { d: "M15 8h-5" }
                path { d: "M19 17V5a2 2 0 0 0-2-2H4" }
                path { d: "M8 21h12a2 2 0 0 0 2-2v-1a1 1 0 0 0-1-1H11a1 1 0 0 0-1 1v1a2 2 0 1 1-4 0V5a2 2 0 1 0-4 0v2a1 1 0 0 0 1 1h3" }
            }
        ),
        "dots" => rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{size}",
                height: "{size}",
                view_box: "0 0 24 24",
                class: "{class}",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M10.1 2.18a9.93 9.93 0 0 1 3.8 0" }
                path { d: "M17.6 3.71a9.95 9.95 0 0 1 2.69 2.7" }
                path { d: "M21.82 10.1a9.93 9.93 0 0 1 0 3.8" }
                path { d: "M20.29 17.6a9.95 9.95 0 0 1-2.7 2.69" }
                path { d: "M13.9 21.82a9.94 9.94 0 0 1-3.8 0" }
                path { d: "M6.4 20.29a9.95 9.95 0 0 1-2.69-2.7" }
                path { d: "M2.18 13.9a9.93 9.93 0 0 1 0-3.8" }
                path { d: "M3.71 6.4a9.95 9.95 0 0 1 2.7-2.69" }
                circle { cx: "12", cy: "12", r: "1" }
            }
        ),
        "focus" => rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{size}",
                height: "{size}",
                view_box: "0 0 24 24",
                class: "{class}",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                circle { cx: "12", cy: "12", r: "3" }
                path { d: "M3 7V5a2 2 0 0 1 2-2h2" }
                path { d: "M17 3h2a2 2 0 0 1 2 2v2" }
                path { d: "M21 17v2a2 2 0 0 1-2 2h-2" }
                path { d: "M7 21H5a2 2 0 0 1-2-2v-2" }
            }
        ),
        "block" => rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{size}",
                height: "{size}",
                view_box: "0 0 24 24",
                class: "{class}",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                rect {
                    width: "7",
                    height: "7",
                    x: "14",
                    y: "3",
                    rx: "1",
                }
                path { d: "M10 21V8a1 1 0 0 0-1-1H4a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-5a1 1 0 0 0-1-1H3" }
            }
        ),
        "drive" => rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{size}",
                height: "{size}",
                view_box: "0 0 24 24",
                class: "{class}",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                line {
                    x1: "22",
                    x2: "2",
                    y1: "12",
                    y2: "12",
                }
                path { d: "M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z" }
                line {
                    x1: "6",
                    x2: "6.01",
                    y1: "16",
                    y2: "16",
                }
                line {
                    x1: "10",
                    x2: "10.01",
                    y1: "16",
                    y2: "16",
                }
            }
        ),
        "swap" => rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{size}",
                height: "{size}",
                view_box: "0 0 24 24",
                class: "{class}",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "m21 16-4 4-4-4" }
                path { d: "M17 20V4" }
                path { d: "m3 8 4-4 4 4" }
                path { d: "M7 4v16" }
            }
        ),
        "chip" => rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{size}",
                height: "{size}",
                view_box: "0 0 24 24",
                class: "{class}",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M18 12h2" }
                path { d: "M18 16h2" }
                path { d: "M18 20h2" }
                path { d: "M18 4h2" }
                path { d: "M18 8h2" }
                path { d: "M4 12h2" }
                path { d: "M4 16h2" }
                path { d: "M4 20h2" }
                path { d: "M4 4h2" }
                path { d: "M4 8h2" }
                path { d: "M8 2a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V4a2 2 0 0 0-2-2h-1.5c-.276 0-.494.227-.562.495a2 2 0 0 1-3.876 0C9.994 2.227 9.776 2 9.5 2z" }
            }
        ),
        "container" => rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{size}",
                height: "{size}",
                view_box: "0 0 24 24",
                class: "{class}",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M22 7.7c0-.6-.4-1.2-.8-1.5l-6.3-3.9a1.72 1.72 0 0 0-1.7 0l-10.3 6c-.5.2-.9.8-.9 1.4v6.6c0 .5.4 1.2.8 1.5l6.3 3.9a1.72 1.72 0 0 0 1.7 0l10.3-6c.5-.3.9-1 .9-1.5Z" }
                path { d: "M10 21.9V14L2.1 9.1" }
                path { d: "m10 14 11.9-6.9" }
                path { d: "M14 19.8v-8.1" }
                path { d: "M18 17.5V9.4" }
            }
        ),
        "sparkles" => rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{size}",
                height: "{size}",
                view_box: "0 0 24 24",
                class: "{class}",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M9.937 15.5A2 2 0 0 0 8.5 14.063l-6.135-1.582a.5.5 0 0 1 0-.962L8.5 9.936A2 2 0 0 0 9.937 8.5l1.582-6.135a.5.5 0 0 1 .963 0L14.063 8.5A2 2 0 0 0 15.5 9.937l6.135 1.581a.5.5 0 0 1 0 .964L15.5 14.063a2 2 0 0 0-1.437 1.437l-1.582 6.135a.5.5 0 0 1-.963 0z" }
                path { d: "M20 3v4" }
                path { d: "M22 5h-4" }
                path { d: "M4 17v2" }
                path { d: "M5 18H3" }
            }
        ),
        "help" => rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{size}",
                height: "{size}",
                view_box: "0 0 24 24",
                class: "{class}",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                circle { cx: "12", cy: "12", r: "10" }
                path { d: "M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" }
                path { d: "M12 17h.01" }
            }
        ),
        "settings" => rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{size}",
                height: "{size}",
                view_box: "0 0 24 24",
                class: "{class}",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z" }
            }
        ),
        "layers" => rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{size}",
                height: "{size}",
                view_box: "0 0 24 24",
                class: "{class}",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.83z" }
                path { d: "M2 12a1 1 0 0 0 .58.91l8.6 3.91a2 2 0 0 0 1.65 0l8.58-3.9A1 1 0 0 0 22 12" }
                path { d: "M2 17a1 1 0 0 0 .58.91l8.6 3.91a2 2 0 0 0 1.65 0l8.58-3.9A1 1 0 0 0 22 17" }
            }
        ),
        "fingerprint" => rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{size}",
                height: "{size}",
                view_box: "0 0 24 24",
                class: "{class}",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M12 10a2 2 0 0 0-2 2c0 1.02-.1 2.51-.26 4" }
                path { d: "M14 13.12c0 2.38 0 6.38-1 8.88" }
                path { d: "M17.29 21.02c.12-.6.43-2.3.5-3.02" }
                path { d: "M2 12a10 10 0 0 1 18-6" }
                path { d: "M2 16h.01" }
                path { d: "M21.8 16c.2-2 .131-5.354 0-6" }
                path { d: "M5 19.5C5.5 18 6 15 6 12a6 6 0 0 1 .34-2" }
                path { d: "M8.65 22c.21-.66.45-1.32.57-2" }
                path { d: "M9 6.8a6 6 0 0 1 9 5.2v2" }
            }
        ),
        "ticket" => rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{size}",
                height: "{size}",
                view_box: "0 0 24 24",
                class: "{class}",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M2 9a3 3 0 0 1 0 6v2a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-2a3 3 0 0 1 0-6V7a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z" }
                path { d: "M13 5v2" }
                path { d: "M13 17v2" }
                path { d: "M13 11v2" }
            }
        ),
        "password" => rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{size}",
                height: "{size}",
                view_box: "0 0 24 24",
                class: "{class}",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                rect {
                    width: "20",
                    height: "12",
                    x: "2",
                    y: "6",
                    rx: "2",
                }
                path { d: "M12 12h.01" }
                path { d: "M17 12h.01" }
                path { d: "M7 12h.01" }
            }
        ),
        "shield-check" => rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{size}",
                height: "{size}",
                view_box: "0 0 24 24",
                class: "{class}",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z" }
                path { d: "m9 12 2 2 4-4" }
            }
        ),
        "file-lock" => rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "{size}",
                height: "{size}",
                view_box: "0 0 24 24",
                class: "{class}",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v1" }
                path { d: "M14 2v4a2 2 0 0 0 2 2h4" }
                rect {
                    width: "8",
                    height: "5",
                    x: "2",
                    y: "13",
                    rx: "1",
                }
                path { d: "M8 13v-2a2 2 0 1 0-4 0v2" }
            }
        ),
        _ => rsx!(
            span { class: "text-red-500 text-sm", "?" }
        ),
    }
}
