use dioxus::prelude::*;
use dioxus_desktop::use_window;

use crate::{components::icons::Icon, Route};

#[component]
pub fn Sidebar() -> Element {
    const HEADER_SVG: Asset = asset!("/assets/header.svg");

    rsx! {
        div { class: "flex flex-col w-64  bg-white h-full border-r border-gray-200",
            div { class: "flex items-center justify-around h-13",
                div { class: "flex gap-2 items-center",
                    // Close
                    button {
                        class: "w-3 h-3 rounded-full bg-red-500 hover:bg-red-400 relative",
                        onclick: move |_| {
                            let window = use_window();
                            let _ = window.close();
                        },
                    }
                    button {
                        class: "w-3 h-3 rounded-full bg-yellow-400 hover:bg-yellow-300",
                        onclick: move |_| {
                            let window = use_window();
                            let _ = window.set_minimized(true);
                        },
                    }
                    // Maximize/Restore
                    button {
                        class: "w-3 h-3 rounded-full bg-green-400 hover:bg-green-300",
                        onclick: move |_| {
                            let window = use_window();
                            let is_maximized = window.is_maximized();
                            let _ = window.set_maximized(!is_maximized);
                        },
                    }
                }
                div { class: "wl-2 w-1/3",
                    img { src: HEADER_SVG, height: "auto" }
                }
            }
            div { class: "overflow-y-auto overflow-x-hidden flex-grow",
                ul { class: "flex flex-col py-3 space-y-1",
                    Link {
                        to: Route::Recents {},
                        class: "relative flex flex-row items-center h-11 focus:outline-none hover:bg-neutral-content text-gray-600 hover:text-gray-800 border-l-4 border-transparent hover:border-primary pr-6",
                        div { class: "inline-flex justify-center items-center ml-4",
                            Icon { name: "history" }
                        }
                        span { class: "ml-2 text-sm tracking-wide truncate", "Recents" }
                    }

                    li {
                        Link {
                            to: Route::Projects {},
                            class: "relative flex flex-row items-center h-11 focus:outline-none hover:bg-neutral-content text-gray-600 hover:text-gray-800 border-l-4 border-transparent hover:border-primary pr-6",
                            span { class: "inline-flex justify-center items-center ml-4",
                                Icon { name: "folder" }
                            }
                            span { class: "ml-2 text-sm tracking-wide truncate", "Projects" }
                        }
                    }

                    li {
                        Link {
                            to: Route::Keychains {},
                            class: "relative flex flex-row items-center h-11 focus:outline-none hover:bg-neutral-content text-gray-600 hover:text-gray-800 border-l-4 border-transparent hover:border-primary pr-6",
                            span { class: "inline-flex justify-center items-center ml-4",
                                Icon { name: "key" }
                            }
                            span { class: "ml-2 text-sm tracking-wide truncate", "Keychains" }
                        }
                    }

                    li {
                        Link {
                            to: Route::Dotfiles {},
                            class: "relative flex flex-row items-center h-11 focus:outline-none hover:bg-neutral-content text-gray-600 hover:text-gray-800 border-l-4 border-transparent hover:border-primary pr-6",
                            span { class: "inline-flex justify-center items-center ml-4",
                                Icon { name: "dots" }
                            }
                            span { class: "ml-2 text-sm tracking-wide truncate", "Dotfiles" }
                        }
                    }

                    li {
                        Link {
                            to: Route::Snippets {},
                            class: "relative flex flex-row items-center h-11 focus:outline-none hover:bg-neutral-content text-gray-600 hover:text-gray-800 border-l-4 border-transparent hover:border-primary pr-6",
                            span { class: "inline-flex justify-center items-center ml-4",
                                Icon { name: "snippets" }
                            }
                            span { class: "ml-2 text-sm tracking-wide truncate", "Snippets & Scripts" }
                        }
                    }

                    li {
                        Link {
                            to: Route::Processes {},
                            class: "relative flex flex-row items-center h-11 focus:outline-none hover:bg-neutral-content text-gray-600 hover:text-gray-800 border-l-4 border-transparent hover:border-primary pr-6",
                            span { class: "inline-flex justify-center items-center ml-4",
                                Icon { name: "chip" }
                            }
                            span { class: "ml-2 text-sm tracking-wide truncate", "Processes" }
                        }
                    }

                    li {
                        Link {
                            to: Route::LogsExplorer {},
                            class: "relative flex flex-row items-center h-11 focus:outline-none hover:bg-neutral-content text-gray-600 hover:text-gray-800 border-l-4 border-transparent hover:border-primary pr-6",
                            span { class: "inline-flex justify-center items-center ml-4",
                                Icon { name: "logs" }
                            }
                            span { class: "ml-2 text-sm tracking-wide truncate", "Logs explorer" }
                        }
                    }



                    li { class: "px-5",
                        div { class: "flex flex-row divider divider-start items-center h-8",
                            div { class: "text-sm font-light tracking-wide text-gray-500",
                                "Extensions"
                            }
                        }
                    }

                    li {
                        Link {
                            to: Route::DockerContainers {},
                            class: "relative flex flex-row items-center h-11 focus:outline-none hover:bg-neutral-content text-gray-600 hover:text-gray-800 border-l-4 border-transparent hover:border-primary pr-6",
                            span { class: "inline-flex justify-center items-center ml-4",
                                Icon { name: "container" }
                            }
                            span { class: "ml-2 text-sm tracking-wide truncate", "Containers" }
                        }
                    }

                    li {
                        Link {
                            to: Route::DockerImages {},
                            class: "relative flex flex-row items-center h-11 focus:outline-none hover:bg-neutral-content text-gray-600 hover:text-gray-800 border-l-4 border-transparent hover:border-primary pr-6",
                            span { class: "inline-flex justify-center items-center ml-4",
                                Icon { name: "focus" }
                            }
                            span { class: "ml-2 text-sm tracking-wide truncate", "Images" }
                        }
                    }
                    li {
                        Link {
                            to: Route::DockerVolumes {},
                            class: "relative flex flex-row items-center h-11 focus:outline-none hover:bg-neutral-content text-gray-600 hover:text-gray-800 border-l-4 border-transparent hover:border-primary pr-6",
                            span { class: "inline-flex justify-center items-center ml-4",
                                Icon { name: "drive" }
                            }
                            span { class: "ml-2 text-sm tracking-wide truncate", "Volumes" }
                        }
                    }
                    li {
                        Link {
                            to: Route::DockerBuilds {},
                            class: "relative flex flex-row items-center h-11 focus:outline-none hover:bg-neutral-content text-gray-600 hover:text-gray-800 border-l-4 border-transparent hover:border-primary pr-6",
                            span { class: "inline-flex justify-center items-center ml-4",
                                Icon { name: "block" }
                            }
                            span { class: "ml-2 text-sm tracking-wide truncate", "Builds" }
                        }
                    }
                }
            }
            div {
                Link {
                    to: "#",
                    class: "relative flex flex-row items-center h-10 focus:outline-none hover:bg-neutral-content text-gray-600 hover:text-gray-800 border-l-4 border-transparent hover:border-primary pr-6",
                    span { class: "inline-flex justify-center items-center ml-4",
                        Icon { name: "sparkles" }
                    }
                    span { class: "ml-2 text-sm tracking-wide truncate", "What's New" }
                }
                div { class: "join relative self-end flex flex-row items-center h-10 text-secondary-content border-t-1 border-primary pr-6",
                    span { class: "inline-flex justify-center items-center ml-4",
                        Icon { name: "swap", size: 16 }
                    }
                    span { class: "join-item ml-4 text-sm tracking-wide truncate", "Default Vault" }
                    span { class: "join-item px-2 py-0.5 ml-auto text-xs font-medium tracking-wide",
                        Icon { name: "help", size: 16 }
                    }
                    span { class: "join-item px-2 py-0.5 text-xs font-medium tracking-wide",
                        Icon { name: "settings", size: 16 }
                    }
                }
            }
        }
    }
}
