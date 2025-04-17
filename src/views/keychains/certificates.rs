use dioxus::prelude::*;

#[component]
pub fn Certificates() -> Element {
    rsx! {
        div { class: "flex flex-col items-start justify-start h-full w-full p-4",
            h1 { class: "text-primary text-xl", "Certificates" }
            div { class: "card w-150 mt-3 bg-base-100 shadow-sm",
                div { class: "card-body",
                    span { class: "badge badge-xs badge-warning", "Certificate" }

                    div { class: "flex justify-between",
                        h4 { class: "text-2xl font-bold", "Certificat-XYZ-123456789" }
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
                table { class: "table table-compact table-xs table-auto text-neutral",
                    thead { class: "text-secondary",
                        tr {
                            th {}
                            th { "Certificate Name" }
                            th { "Features" }
                            th { "Scope" }
                            th { "Issued On" }
                            th { "Authority" }
                        }
                    }
                    tbody {
                        tr {
                            th { "1" }
                            td { "iCloud Access" }
                            td { "Secure Cloud Storage" }
                            td { "Global" }
                            td { "2023-10-05" }
                            td { "Apple" }
                        }
                        tr {
                            th { "2" }
                            td { "Local Backup" }
                            td { "On-premise Storage" }
                            td { "Local" }
                            td { "2023-09-30" }
                            td { "Local Authority" }
                        }
                        tr {
                            th { "3" }
                            td { "Quick Share" }
                            td { "File Transfer" }
                            td { "Global" }
                            td { "2023-08-15" }
                            td { "System" }
                        }
                        tr {
                            th { "4" }
                            td { "AutoFill Security" }
                            td { "Password Management" }
                            td { "Local" }
                            td { "2023-07-01" }
                            td { "User" }
                        }
                        tr {
                            th { "5" }
                            td { "System Restore" }
                            td { "Automatic Backups" }
                            td { "Global" }
                            td { "2023-06-20" }
                            td { "System" }
                        }
                        tr {
                            th { "6" }
                            td { "Network Access" }
                            td { "Secure Connectivity" }
                            td { "Global" }
                            td { "2023-05-15" }
                            td { "System" }
                        }
                        tr {
                            th { "7" }
                            td { "Utilities Certification" }
                            td { "System Tools" }
                            td { "Local" }
                            td { "2023-04-25" }
                            td { "System" }
                        }
                        tr {
                            th { "8" }
                            td { "Email Access" }
                            td { "Mail Encryption" }
                            td { "Local" }
                            td { "2023-04-10" }
                            td { "User" }
                        }
                        tr {
                            th { "9" }
                            td { "Messaging Service" }
                            td { "Secure Messaging" }
                            td { "Global" }
                            td { "2023-03-30" }
                            td { "System" }
                        }
                        tr {
                            th { "10" }
                            td { "Payment Security" }
                            td { "Transaction Encryption" }
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
