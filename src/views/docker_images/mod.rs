use crate::components::{icons::Icon, tab_wrapper::TabWrapper};
use dioxus::prelude::*;

#[component]
fn DockerImageDetails() -> Element {
    rsx! {
        div { class: "flex flex-row justify-start gap-2",
            h1 { class: "pl-5 mt-10 text-primary text-xl",
                "ubuntu:latest"
                span { class: "ml-5 badge text-white badge-xs badge-info", "IN USE" }
            }
        }
        div { class: "grid grid-cols-2 grid-rows-2 gap-5 h-11/12 w-9/12 pl-5 pb-20",
            div { class: "flex flex-col justify-start items-start",
                h2 { class: "divider divider-start font-semibold text-sm text-secondary",
                    "Base Images"
                }
                div { class: "w-full h-full overflow-y-auto",
                    div { class: "space-y-1",
                        {(0..11).map(|i| rsx! {
                            div { class: "flex items-start justify-between bg-base-100 p-2 shadow-md text-xs",
                                div { class: "font-mono", "FROM apt-get install -y package-{i}" }
                                div { class: "text-right opacity-60 whitespace-nowrap", "20 MB" }
                            }
                        })}
                    }
                }
            }
            div { class: "flex flex-col justify-start items-start",
                h2 { class: "divider divider-start font-semibold text-sm text-secondary",
                    "Overview"
                }
                div { class: "stats stats-vertical shadow",
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
            }
            div { class: "flex flex-col justify-start items-start",
                h2 { class: "divider divider-start font-semibold text-sm text-secondary",
                    "Image Layers"
                }
                div { class: "w-full h-full overflow-y-auto",
                    table { class: "table table-sm text-neutral",
                        tbody {
                            tr {
                                th { "1" }
                                td { "RUN apt-get update" }
                                td { "50 MB" }
                            }
                            tr {
                                th { "2" }
                                td { "RUN apt-get install -y python3" }
                                td { "200 MB" }
                            }
                            tr {
                                th { "3" }
                                td { "COPY . /app" }
                                td { "15 MB" }
                            }
                            tr {
                                th { "4" }
                                td { "RUN pip install -r requirements.txt" }
                                td { "100 MB" }
                            }
                            tr {
                                th { "5" }
                                td { "CMD [\"python3\", \"app.py\"]" }
                                td { "5 MB" }
                            }
                            tr {
                                th { "6" }
                                td { "RUN apt-get clean" }
                                td { "1 MB" }
                            }
                            tr {
                                th { "7" }
                                td { "RUN mkdir /newdir" }
                                td { "1 MB" }
                            }
                            tr {
                                th { "8" }
                                td { "RUN touch /newdir/file.txt" }
                                td { "2 MB" }
                            }
                            tr {
                                th { "9" }
                                td { "RUN chmod 755 /newdir/file.txt" }
                                td { "1 MB" }
                            }
                            tr {
                                th { "10" }
                                td { "RUN echo \"Hello World\" > /newdir/file.txt" }
                                td { "1 MB" }
                            }
                            tr {
                                th { "11" }
                                td { "RUN chown user:user /newdir" }
                                td { "1 MB" }
                            }
                            tr {
                                th { "12" }
                                td { "RUN rm -rf /var/cache/apt" }
                                td { "1 MB" }
                            }
                        }
                    }
                }
            }
            div { class: "flex flex-col justify-start items-start",
                h2 { class: "divider divider-start font-semibold text-sm text-secondary",
                    "Layer Details"
                }
                div { class: "w-full h-full",
                    div { class: "tabs w-full h-10/12 tabs-border",

                        input {
                            r#type: "radio",
                            name: "my_tabs_2",
                            class: "tab",
                            aria_label: "Vulnerabilities",
                        }
                        div { class: "tab-content border-base-300 bg-base-100 p-10",
                            "Tab content 1"
                        }

                        input {
                            r#type: "radio",
                            name: "my_tabs_2",
                            class: "tab",
                            aria_label: "Packages",
                            checked: true,
                        }
                        div { class: "tab-content border-base-300 bg-base-100 p-10",
                            "Tab content 2"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn DockerImages() -> Element {
    rsx! {
        div { class: "tabs tabs-sm tabs-lift fixed h-full w-full",
            TabWrapper {
                class: "ml-0",
                tab: "docker",
                label: "Images",
                icon: "focus",
                checked: true,
                content: rsx! {
                    DockerImageDetails {}
                },
            }
        }
    }
}
