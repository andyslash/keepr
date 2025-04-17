use crate::components::icons::Icon;
use dioxus::prelude::*;

#[component]
pub fn LogsExplorer() -> Element {
    rsx! {
        div { class: "mt-10 p-6 h-full flex flex-col bg-base-100 text-base-content ",
            h1 { class: "text-primary text-xl", "Live Logs" }

            div { class: "mt-5 tabs tabs-md w-full",
                a { class: "tab tab-active", "All" }
                a { class: "tab", "Info" }
                a { class: "tab", "Warning" }
                a { class: "tab", "Error" }
            }
            div {
                id: "log-view",
                class: "overflow-y-auto mockup-code h-1/2 rounded-box overflow-hidden shadow p-4 font-mono text-sm whitespace-pre-wrap h-[500px] scroll-smooth",

                {
                    (0..3)
                        .flat_map(|_| {
                            vec![
                                rsx!(
                                    div { class : " text-success",
                                    "[12:42:01] [INFO] Server started successfully on port 8080" }
                                ),
                                rsx!(
                                    div { class : "text-info",
                                    "[12:42:05] [DEBUG] Initializing modules..." }
                                ),
                                rsx!(
                                    div { class : "text-warning",
                                    "[12:42:10] [WARN] Connection latency above threshold" }
                                ),
                                rsx!(
                                    div { class : "text-error",
                                    "[12:42:12] [ERROR] Failed to load configuration file" }
                                ),
                                rsx!(
                                    div { class : "text-info",
                                    "[12:42:15] [DEBUG] Retrying in 5 seconds..." }
                                ),
                                rsx!(
                                    div { class : "text-success",
                                    "[12:42:20] [INFO] Recovery successful" }
                                ),
                                rsx!(
                                    div { class : "text-info",
                                    "[12:42:22] [DEBUG] New user session: id=3093, ip=192.168.1.32" }
                                ),
                                rsx!(
                                    div { class : "text-success",
                                    "[12:42:23] [INFO] Authenticated user johndoe" }
                                ),
                                rsx!(
                                    div { class : "text-warning",
                                    "[12:42:30] [WARN] API response time degraded" }
                                ),
                                rsx!(
                                    div { class : "text-error",
                                    "[12:42:32] [ERROR] Unexpected token in JSON at position 128" }
                                ),
                                rsx!(
                                    div { class : "text-info",
                                    "[12:42:35] [DEBUG] Session keepalive signal sent" }
                                ),
                                rsx!(
                                    div { class : "text-success",
                                    "[12:42:40] [INFO] Background job completed: cleanup-temp-files"
                                    }
                                ),
                                rsx!(
                                    div { class : "text-warning",
                                    "[12:42:45] [WARN] High memory usage detected: 82%" }
                                ),
                                rsx!(
                                    div { class : "text-error",
                                    "[12:42:48] [ERROR] Docker container crashed: redis-cache" }
                                ),
                                rsx!(
                                    div { class : "text-success",
                                    "[12:42:52] [INFO] Redis container restarted successfully" }
                                ),
                            ]
                        })
                }
            
            }
            div { class: "justify-end my-5 flex",
                button { class: "btn btn-sm btn-outline btn-error",
                    Icon { name: "trash" }
                    " Clear logs"
                }
            }
        }
    }
}
