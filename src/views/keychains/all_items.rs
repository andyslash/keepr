use dioxus::prelude::*;

#[component]
pub fn AllItems() -> Element {
    rsx! {
        div { class: "flex flex-col items-start justify-start h-full w-full p-4",
            h1 { class: "text-primary text-xl", "All Keychains" }
            div { class: "card w-150 mt-3 bg-base-100 shadow-sm",
                div { class: "card-body",
                    span { class: "badge badge-xs badge-warning", "Certificate" }

                    div { class: "flex justify-between",
                        h4 { class: "text-2xl font-bold", "A79FBE3B-D593-FCEA-D1AA-BA8899AA1E54" }
                    }

                    ul { class: "mt-6 flex flex-col gap-2 text-xs",

                        li { "High-resolution image generation" }
                        li { "Customizable style templates" }
                        li { "Batch processing capabilities" }
                        li { "AI-driven image enhancements" }
                    }
                }
            }
            div { class: "overflow-x-auto rounded-box w-full mt-10",
                table { class: "table border-separate bg-base-100 table-compact table-xs table-auto text-neutral",
                    thead { class: "text-secondary",
                        tr {
                            th {}
                            th { "Label" }
                            th { "Type" }
                            th { "Scope" }
                            th { "Date modified" }
                            th { "Keychain" }
                        }
                    }
                    tbody {
                        tr { class: "border-b border-gray-200 bg-gray-50",
                            th { "1" }
                            td { "iCloud" }
                            td { "Password" }
                            td { "Global" }
                            td { "2023-10-05" }
                            td { "System" }
                        }
                        tr { class: "bg-base-50 border-b border-gray-200",
                            th { "2" }
                            td { "Local Items" }
                            td { "Password" }
                            td { "Local" }
                            td { "2023-09-30" }
                            td { "System" }
                        }
                        tr { class: "border-b border-gray-200 bg-gray-50",
                            th { "3" }
                            td { "AirDrop" }
                            td { "Application Password" }
                            td { "Global" }
                            td { "2023-08-15" }
                            td { "System" }
                        }
                        tr { class: "bg-base-50 border-b border-gray-200",
                            th { "4" }
                            td { "Safari AutoFill" }
                            td { "Internet Password" }
                            td { "Local" }
                            td { "2023-07-01" }
                            td { "User" }
                        }
                        tr { class: "border-b border-gray-200 bg-gray-50",
                            th { "5" }
                            td { "Time Machine" }
                            td { "Backup Password" }
                            td { "Global" }
                            td { "2023-06-20" }
                            td { "System" }
                        }
                        tr { class: "bg-base-50 border-b border-gray-200",
                            th { "6" }
                            td { "Wi-Fi Network" }
                            td { "Network Password" }
                            td { "Global" }
                            td { "2023-05-15" }
                            td { "System" }
                        }
                        tr { class: "border-b border-gray-200 bg-gray-50",
                            th { "7" }
                            td { "Disk Utility" }
                            td { "Certificate" }
                            td { "Local" }
                            td { "2023-04-25" }
                            td { "System" }
                        }
                        tr { class: "bg-base-50 border-b border-gray-200",
                            th { "8" }
                            td { "Mail.app" }
                            td { "Email Password" }
                            td { "Local" }
                            td { "2023-04-10" }
                            td { "User" }
                        }
                        tr { class: "border-b border-gray-200 bg-gray-50",
                            th { "9" }
                            td { "Messages" }
                            td { "Messaging Password" }
                            td { "Global" }
                            td { "2023-03-30" }
                            td { "System" }
                        }
                        tr { class: "bg-base-50 border-b border-gray-200",
                            th { "10" }
                            td { "Apple Pay" }
                            td { "Payment Info" }
                            td { "Local" }
                            td { "2023-03-05" }
                            td { "User" }
                        }
                        {
                            (0..10)
                                .map(|i| {
                                    let class = if i % 2 == 0 {
                                        "bg-base-50 border-b border-gray-200"
                                    } else {
                                        "bg-gray-50 border-b border-gray-200"
                                    };
                                    rsx! {
                                        tr { class: "{class}",
                                            th { "" }
                                            td { " " }
                                            td { " " }
                                            td { " " }
                                            td { " " }
                                            td { " " }
                                        }
                                    }
                                })
                        }
                    }
                }
            }
        }
    }
}
