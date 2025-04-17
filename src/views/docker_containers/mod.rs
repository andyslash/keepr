use crate::components::{icons::Icon, radial::Radial, tab_wrapper::TabWrapper};
use dioxus::prelude::*;

#[component]
fn Containers() -> Element {
    rsx! {
        div { class: "mt-5 flex flex-col items-start justify-start h-full p-4",
            h1 { class: "text-primary text-xl", "Docker Containers" }
            div { class: "mt-10 h-1/4 stats shadow",
                // CPU Usage
                div { class: "stat",
                    div { class: "stat-figure text-primary",
                        Icon { name: "zap" }
                    }
                    div { class: "stat-title", "Container CPU Usage" }
                    div { class: "stat-value text-primary", "0.22% / 600%" }
                    div { class: "stat-desc", "(6 cores allocated)" }
                }

                // Memory Usage
                div { class: "stat",
                    div { class: "stat-figure text-secondary",
                        Icon { name: "memory" }
                    }
                    div { class: "stat-title", "Container Memory Usage" }
                    div { class: "stat-value text-secondary", "17.24 MB / 7.56 GB" }
                    div { class: "stat-desc", "Across 10 containers" }
                }
            }
            div { class: "mt-10 w-4/5 h-full",
                ul { class: "list bg-base-100 rounded-box",

                    li { class: "p-4 pb-2 text-xs  tracking-wide", "Running containers" }

                    li { class: "list-row hover:shadow-sm cursor-pointer",
                        div { class: "text-2xl font-thin  tabular-nums w-8 mt-2",
                            Icon { name: "layers", class: "text-success" }
                        }

                        div { class: "list-col-grow",
                            div { class: "font-semibold", "web_app" }
                            div { class: "text-xs  grid grid-cols-[160px_80px_80px_1fr] gap-x-3 items-center",

                                span { class: "opacity-60 truncate", "haproxy:2.3" }

                                span { class: "badge badge-outline badge-sm badge-success w-[80px] text-center",
                                    "Running"
                                }

                                span { class: "opacity-60 w-[80px] text-neutral-500",
                                    "443/tcp"
                                }

                                span { class: "opacity-60 whitespace-nowrap text-neutral-500",
                                    "Last started: 4 hours ago"
                                }
                            }
                        }
                        div { class: "text-xs text-right pr-2",
                            Radial {
                                class: "text-accent",
                                value: 70,
                                size: "3rem",
                            }
                        }
                        button { class: "btn btn-square btn-ghost",
                            Icon { name: "play", size: 12 }
                        }
                        button { class: "btn btn-square btn-ghost",
                            Icon { name: "pause", size: 12 }
                        }
                        button { class: "btn btn-square btn-ghost",
                            Icon { name: "trash", size: 12 }
                        }
                    }

                    li { class: "list-row hover:shadow-sm cursor-pointer",
                        div { class: "text-2xl font-thin  tabular-nums w-8 mt-2",
                            Icon { name: "layers", class: "text-error" }
                        }
                        div { class: "list-col-grow",
                            div { class: "font-semibold", "db_service" }
                            div { class: "text-xs grid grid-cols-[160px_80px_80px_1fr] gap-x-3 items-center",

                                span { class: "opacity-60 truncate", "postgres:13" }

                                span { class: "badge badge-outline badge-sm badge-error w-[80px] text-center",
                                    "Exited"
                                }

                                span { class: "opacity-60 w-[80px] text-neutral-500",
                                    "5432/tcp"
                                }

                                span { class: "opacity-60 whitespace-nowrap text-neutral-500",
                                    "Last started: 2 hours ago"
                                }
                            }
                        }
                        div { class: "text-xs text-right pr-2",
                            Radial {
                                class: "text-accent",
                                value: 0,
                                size: "3rem",
                            }
                        }
                        button { class: "btn btn-square btn-ghost",
                            Icon { name: "play", size: 12 }
                        }
                        button { class: "btn btn-square btn-ghost",
                            Icon { name: "pause", size: 12 }
                        }
                        button { class: "btn btn-square btn-ghost",
                            Icon { name: "trash", size: 12 }
                        }
                    }

                    li { class: "list-row hover:shadow-sm cursor-pointer",
                        div { class: "text-2xl font-thin tabular-nums w-8 mt-2",
                            Icon { name: "layers", class: "text-success" }
                        }
                        div { class: "list-col-grow",
                            div { class: "font-semibold", "cache_server" }
                            div { class: "text-xs grid grid-cols-[160px_80px_80px_1fr] gap-x-3 items-center",

                                span { class: "opacity-60 truncate", "redis:alpine" }

                                span { class: "badge badge-outline badge-sm  badge-success w-[80px] text-center",
                                    "Running"
                                }

                                span { class: "opacity-60 w-[80px] text-neutral-500",
                                    "6379/tcp"
                                }

                                span { class: "opacity-60 whitespace-nowrap text-neutral-500",
                                    "Last started: 2 days ago"
                                }
                            }
                        }
                        div { class: "text-xs text-right pr-2",
                            Radial {
                                class: "text-accent",
                                value: 5,
                                size: "3rem",
                            }
                        }
                        button { class: "btn btn-square btn-ghost",
                            Icon { name: "play", size: 12 }
                        }
                        button { class: "btn btn-square btn-ghost",
                            Icon { name: "pause", size: 12 }
                        }
                        button { class: "btn btn-square btn-ghost",
                            Icon { name: "trash", size: 12 }
                        }
                    }

                    li { class: "list-row hover:shadow-sm cursor-pointer",
                        div { class: "text-2xl font-thin  tabular-nums w-8 mt-2",
                            Icon { name: "layers", class: "text-success" }
                        }
                        div { class: "list-col-grow",
                            div { class: "font-semibold", "load_balancer" }
                            div { class: "text-xs grid grid-cols-[160px_80px_80px_1fr] gap-x-3 items-center",

                                span { class: "opacity-60 truncate", "haproxy:2.3" }

                                span { class: "badge badge-outline badge-sm badge-success w-[80px] text-center",
                                    "Running"
                                }

                                span { class: "opacity-60 w-[80px] text-neutral-500",
                                    "443/tcp"
                                }

                                span { class: "opacity-60 whitespace-nowrap text-neutral-500",
                                    "Last started: 4 hours ago"
                                }
                            }
                        }
                        div { class: "text-xs text-right pr-2",
                            Radial {
                                class: "text-accent",
                                value: 20,
                                size: "3rem",
                            }
                        }
                        button { class: "btn btn-square btn-ghost",
                            Icon { name: "play", size: 12 }
                        }
                        button { class: "btn btn-square btn-ghost",
                            Icon { name: "pause", size: 12 }
                        }
                        button { class: "btn btn-square btn-ghost",
                            Icon { name: "trash" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn DockerContainers() -> Element {
    rsx! {
        div { class: "tabs tabs-sm tabs-lift fixed h-full w-full",
            TabWrapper {
                class: "ml-0",
                tab: "docker",
                label: "Containers",
                icon: "container",
                checked: true,
                content: rsx! {
                    Containers {}
                },
            }
            TabWrapper {
                class: "ml-2",
                tab: "docker",
                label: "e7a0c5b2",
                icon: "drive",
                checked: false,
                content: rsx! {
                    h1 { "This is a dynamic content" }
                },
            }
            TabWrapper {
                class: "ml-2",
                tab: "docker",
                label: "+",
                icon: "layers",
                checked: false,
                content: rsx! {},
            }
        }
    }
}
