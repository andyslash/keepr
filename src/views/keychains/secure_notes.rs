use dioxus::prelude::*;

#[component]
pub fn SecureNotes() -> Element {
    rsx! {
        div { class: "flex flex-col items-start justify-start h-full w-full p-4",
            h1 { class: "text-primary text-xl", "Secure Notes" }
            div { class: "card w-150 mt-3 bg-base-100 shadow-sm",
                div { class: "card-body",
                    span { class: "badge badge-xs badge-error", "Secure Notes" }

                    div { class: "flex justify-between",
                        h4 { class: "text-2xl font-bold", "B02C1F4B-E673-BCFD-A2CC-DC5678EE2F98" }
                    }

                    ul { class: "mt-6 flex flex-col gap-2 text-xs",

                        li { "High-resolution image generation" }
                        li { "Customizable style templates" }
                        li { "Batch processing capabilities" }
                        li { "AI-driven image enhancements" }
                    }
                }
            }
            div { class: "overflow-x-auto rounded-box border border-base-content/5 bg-base-100 w-full mt-10",
                table { class: "table table-compact table-sm table-auto text-neutral",
                    thead { class: "text-secondary",
                        tr {
                            th {}
                            th { "Title" }
                            th { "Category" }
                            th { "Visibility" }
                            th { "Date Updated" }
                            th { "Vault" }
                        }
                    }
                    tbody {
                        tr {
                            th { "1" }
                            td { "Secure Note 1" }
                            td { "Confidential" }
                            td { "Global" }
                            td { "2023-10-05" }
                            td { "System" }
                        }
                        tr {
                            th { "2" }
                            td { "Secure Note 2" }
                            td { "Private" }
                            td { "Local" }
                            td { "2023-09-30" }
                            td { "System" }
                        }
                        tr {
                            th { "3" }
                            td { "Secure Archive" }
                            td { "Sensitive" }
                            td { "Global" }
                            td { "2023-08-15" }
                            td { "System" }
                        }
                        tr {
                            th { "4" }
                            td { "Secure Reminder" }
                            td { "Personal" }
                            td { "Local" }
                            td { "2023-07-01" }
                            td { "User" }
                        }
                        tr {
                            th { "5" }
                            td { "Secure Backup" }
                            td { "Critical" }
                            td { "Global" }
                            td { "2023-06-20" }
                            td { "System" }
                        }
                        tr {
                            th { "6" }
                            td { "Secure Network" }
                            td { "Network Sensitive" }
                            td { "Global" }
                            td { "2023-05-15" }
                            td { "System" }
                        }
                        tr {
                            th { "7" }
                            td { "Utility Notes" }
                            td { "Internal" }
                            td { "Local" }
                            td { "2023-04-25" }
                            td { "System" }
                        }
                        tr {
                            th { "8" }
                            td { "Email Notes" }
                            td { "Confidential" }
                            td { "Local" }
                            td { "2023-04-10" }
                            td { "User" }
                        }
                        tr {
                            th { "9" }
                            td { "Communication Notes" }
                            td { "Messaging Sensitive" }
                            td { "Global" }
                            td { "2023-03-30" }
                            td { "System" }
                        }
                        tr {
                            th { "10" }
                            td { "Payment Notes" }
                            td { "Financial" }
                            td { "Local" }
                            td { "2023-03-05" }
                            td { "User" }
                        }
                    }
                }
            }
        }
    }
}
