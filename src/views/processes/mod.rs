use crate::components::icons::Icon;
use dioxus::prelude::*;

#[component]
pub fn Processes() -> Element {
    rsx! {
        div { class: "mt-10 flex flex-col p-6 gap-6",

            // Real-time Usage Graph (Mocked)
            div { class: "w-full bg-base-100 p-4 rounded-box shadow flex flex-col gap-2",
                div { class: "flex items-center justify-between",
                    div {
                        h2 { class: "text-sm font-semibold text-primary", "CPU Usage (Live)" }
                        p { class: "text-xs opacity-60", "Updated every second" }
                    }
                    span { class: "text-xs text-neutral", "4 Cores active" }
                }

                // Fake animated graph placeholder
                div { class: "w-full h-90 bg-gradient-to-r from-primary/20 via-primary/30 to-primary/10 rounded-box animate-pulse" }
            }

            // Stats Summary
            div { class: "stats shadow w-full",
                div { class: "stat",
                    div { class: "stat-figure text-primary",
                        Icon { name: "zap" }
                    }
                    div { class: "stat-title", "CPU Usage" }
                    div { class: "stat-value text-primary", "43%" }
                    div { class: "stat-desc", "4 cores used on 8" }
                }
                div { class: "stat",
                    div { class: "stat-figure text-secondary",
                        Icon { name: "memory" }
                    }
                    div { class: "stat-title", "Memory Usage" }
                    div { class: "stat-value text-secondary", "3.2 GB / 8 GB" }
                    div { class: "stat-desc", "40% used" }
                }
                div { class: "stat",
                    div { class: "stat-figure text-accent",
                        Icon { name: "layers" }
                    }
                    div { class: "stat-title", "Processes" }
                    div { class: "stat-value text-accent", "152" }
                    div { class: "stat-desc", "12 running, 3 zombie" }
                }
            }

            // Process List
            div { class: "overflow-auto rounded-box border border-base-300 divide-y divide-base-200",
                div { class: "grid grid-cols-[2fr_1fr_1fr_1fr_0.5fr_0.5fr] text-xs font-semibold text-neutral-content px-4 py-2 bg-base-200",
                    span { "Name" }
                    span { "PID" }
                    span { "CPU" }
                    span { "Memory" }
                    span { "User" }
                    span { "" } // actions
                }

                // Example row
                div { class: "grid grid-cols-[2fr_1fr_1fr_1fr_0.5fr_0.5fr] items-center px-4 py-3 text-sm hover:bg-base-100",
                    span { class: "font-medium", "nginx" }
                    span { "2134" }
                    span { class: "text-accent", "12%" }
                    span { "30 MB" }
                    span { "root" }
                    div { class: "flex gap-1 justify-end",
                        button { class: "btn btn-xs btn-ghost",
                            Icon { name: "pause" }
                        }
                        button { class: "btn btn-xs btn-ghost",
                            Icon { name: "trash" }
                        }
                    }
                }

                div { class: "grid grid-cols-[2fr_1fr_1fr_1fr_0.5fr_0.5fr] items-center px-4 py-3 text-sm hover:bg-base-100",
                    span { class: "font-medium", "postgres" }
                    span { "4021" }
                    span { class: "text-warning", "32%" }
                    span { "512 MB" }
                    span { "postgres" }
                    div { class: "flex gap-1 justify-end",
                        button { class: "btn btn-xs btn-ghost",
                            Icon { name: "pause" }
                        }
                        button { class: "btn btn-xs btn-ghost",
                            Icon { name: "trash" }
                        }
                    }
                }
            }
        }
    }
}
