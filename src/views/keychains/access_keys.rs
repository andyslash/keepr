use dioxus::prelude::*;

#[component]
pub fn AccessKeys() -> Element {
    rsx! {
        div { class: "flex flex-col items-start justify-start h-full w-full p-4",
            h1 { class: "text-primary text-xl", "Access Keys" }
            div { class: "card w-150 mt-3 bg-base-100 shadow-sm",
                div { class: "card-body",
                    span { class: "badge badge-xs badge-info", "Access Key" }

                    div { class: "flex justify-between",
                        h4 { class: "text-2xl font-bold", "B1289CE4-A672-4CDE-B2E9-0A703614FAE9" }
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
                            th { "Label" }
                            th { "Kind" }
                            th { "Access Range" }
                            th { "Updated On" }
                            th { "Keychain" }
                        }
                    }
                    tbody {
                        tr {
                            th { "1" }
                            td { "iCloud" }
                            td { "Password" }
                            td { "Global" }
                            td { "2023-10-05" }
                            td { "System" }
                        }
                        tr {
                            th { "2" }
                            td { "Local Items" }
                            td { "Password" }
                            td { "Local" }
                            td { "2023-09-30" }
                            td { "System" }
                        }
                        tr {
                            th { "3" }
                            td { "AirDrop" }
                            td { "App Password" }
                            td { "Global" }
                            td { "2023-08-15" }
                            td { "System" }
                        }
                        tr {
                            th { "4" }
                            td { "Safari AutoFill" }
                            td { "Web Password" }
                            td { "Local" }
                            td { "2023-07-01" }
                            td { "User" }
                        }
                        tr {
                            th { "5" }
                            td { "Time Machine" }
                            td { "Backup Secret" }
                            td { "Global" }
                            td { "2023-06-20" }
                            td { "System" }
                        }
                        tr {
                            th { "6" }
                            td { "Wi-Fi Network" }
                            td { "Network Secret" }
                            td { "Global" }
                            td { "2023-05-15" }
                            td { "System" }
                        }
                        tr {
                            th { "7" }
                            td { "Disk Utility" }
                            td { "Certificate" }
                            td { "Local" }
                            td { "2023-04-25" }
                            td { "System" }
                        }
                        tr {
                            th { "8" }
                            td { "Mail.app" }
                            td { "E-mail Secret" }
                            td { "Local" }
                            td { "2023-04-10" }
                            td { "User" }
                        }
                        tr {
                            th { "9" }
                            td { "Messages" }
                            td { "Messaging Secret" }
                            td { "Global" }
                            td { "2023-03-30" }
                            td { "System" }
                        }
                        tr {
                            th { "10" }
                            td { "Apple Pay" }
                            td { "Payment Details" }
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
