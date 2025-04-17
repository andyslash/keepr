use crate::components::icons::Icon;
use dioxus::prelude::*;

#[component]
pub fn Dotfiles() -> Element {
    rsx! {
        div { class: "mt-8 flex flex-col h-full overflow-hidden",
            div { class: "flex flex-grow overflow-hidden",
                // Sidebar
                div { class: "w-1/4 bg-base-200 p-4 border-r border-base-300 space-y-4 overflow-y-auto",
                    h2 { class: "text-xs uppercase font-bold opacity-60", "Dotfiles" },

                    div { class: "flex flex-col gap-2 text-sm",
                        span { class: "font-semibold text-sm opacity-50", "Global" },
                        ul { class: "space-y-1",
                            li { class: "flex justify-between items-center px-2 py-1 rounded hover:bg-base-300 cursor-pointer", span { ".bashrc" }, span { class: "badge badge-success badge-xs", "✔" } },
                            li { class: "flex justify-between items-center px-2 py-1 rounded hover:bg-base-300 cursor-pointer", span { ".zshrc" }, span { class: "badge badge-success badge-xs", "✔" } },
                            li { class: "flex justify-between items-center px-2 py-1 rounded hover:bg-base-300 cursor-pointer", span { ".gitconfig" }, span { class: "badge badge-warning badge-xs", "!" } },
                            li { class: "flex justify-between items-center px-2 py-1 rounded hover:bg-base-300 cursor-pointer", span { ".ssh/config" }, span { class: "badge badge-error badge-xs", "✘" } },
                            li { class: "flex justify-between items-center px-2 py-1 rounded hover:bg-base-300 cursor-pointer", span { ".vimrc" }, span { class: "badge badge-success badge-xs", "✔" } },
                            li { class: "flex justify-between items-center px-2 py-1 rounded hover:bg-base-300 cursor-pointer", span { ".editorconfig" }, span { class: "badge badge-neutral badge-xs", "~" } },
                        },

                        span { class: "font-semibold text-xs opacity-50 mt-4", "Project: api-server" },
                        ul { class: "space-y-1",
                            li { class: "px-2 py-1 rounded hover:bg-base-300 cursor-pointer", ".env" },
                            li { class: "px-2 py-1 rounded hover:bg-base-300 cursor-pointer", ".npmrc" },
                            li { class: "px-2 py-1 rounded hover:bg-base-300 cursor-pointer", "docker-compose.yml" },
                            li { class: "px-2 py-1 rounded hover:bg-base-300 cursor-pointer", ".dockerignore" },
                        },

                        span { class: "font-semibold text-xs opacity-50 mt-4", "Project: machine-learning" },
                        ul { class: "space-y-1",
                            li { class: "px-2 py-1 rounded hover:bg-base-300 cursor-pointer", "pyproject.toml" },
                            li { class: "px-2 py-1 rounded hover:bg-base-300 cursor-pointer", ".tool-versions" },
                            li { class: "px-2 py-1 rounded hover:bg-base-300 cursor-pointer", "requirements.txt" },
                            li { class: "px-2 py-1 rounded hover:bg-base-300 cursor-pointer", ".env" },
                        }
                    }
                },

                // Main content
                div { class: "w-3/4 p-6 flex flex-col gap-6 overflow-y-auto min-h-0",
                    h1 { class: "text-xl font-semibold text-primary", ".npmrc" },
                    p { class: "text-sm opacity-70", "Configuration file for Node.js & NPM registry auth and scope setup." },
                    div { class: "text-sm opacity-60", "Location: ~/projects/api-server/.npmrc" },
                    div { class: "badge badge-accent w-fit", "Attached to project: api-server" },

                    div {
                        class: "mockup-code rounded-box p-4 font-mono text-sm whitespace-pre-wrap shadow-inner",
                        code {
                            "//registry.npmjs.org/:_authToken=abc123\nstrict-ssl=true\n"
                            "//registry.npmjs.org/:_authToken=abc123\nstrict-ssl=true\n"
                            "//registry.npmjs.org/:_authToken=abc123\nstrict-ssl=true\n"
                        }
                    },

                    div { class: "flex gap-2 justify-end",
                        button { class: "btn btn-sm btn-outline btn-primary", Icon { name: "eye" }, "View" },
                        button { class: "btn btn-sm btn-outline btn-warning", Icon { name: "edit" }, "Edit" },
                        button { class: "btn btn-sm btn-outline btn-secondary", Icon { name: "download" }, "Export" }
                    },

                    div { class: "stats shadow w-full",
                        div { class: "stat",
                            div { class: "stat-figure text-primary", Icon { name: "folder-open" } },
                            div { class: "stat-title", "Dotfiles Detected" },
                            div { class: "stat-value text-primary", "20" },
                            div { class: "stat-desc", "8 global, 12 project-specific" }
                        },
                        div { class: "stat",
                            div { class: "stat-figure text-secondary", Icon { name: "box" } },
                            div { class: "stat-title", "Tools Detected" },
                            div { class: "stat-value text-secondary", "7" },
                            div { class: "stat-desc", "Node.js, Docker, Poetry, Git, Vim, Astro, Tailwind" }
                        }
                    },

                    div { class: "bg-base-100 border border-base-300 rounded-box p-4 space-y-2",
                        h2 { class: "text-sm font-semibold text-neutral-content", "Available Presets" },
                        p { class: "text-xs opacity-70", "Based on detected tools, you can quickly install and link suggested dotfiles:" },
                        div { class: "grid grid-cols-2 gap-2 text-sm",
                            div { class: "bg-base-200 p-3 rounded-box flex flex-col gap-1",
                                span { class: "font-semibold", ".npmrc" },
                                span { class: "text-xs opacity-60", "NPM registry & auth config" },
                                button { class: "btn btn-xs btn-outline btn-success mt-1 w-fit", Icon { name: "plus" }, " Add preset" }
                            },
                            div { class: "bg-base-200 p-3 rounded-box flex flex-col gap-1",
                                span { class: "font-semibold", "pyproject.toml" },
                                span { class: "text-xs opacity-60", "Poetry / Python project settings" },
                                button { class: "btn btn-xs btn-outline btn-success mt-1 w-fit", Icon { name: "plus" }, " Add preset" }
                            },
                            div { class: "bg-base-200 p-3 rounded-box flex flex-col gap-1",
                                span { class: "font-semibold", ".docker/config.json" },
                                span { class: "text-xs opacity-60", "Docker Hub credentials and settings" },
                                button { class: "btn btn-xs btn-outline btn-success mt-1 w-fit", Icon { name: "plus" }, " Add preset" }
                            }
                        }
                    }
                }
            }
        }
    }
}
