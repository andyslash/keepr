use dioxus::prelude::*;

#[component]
pub fn Passwords() -> Element {
    rsx! {
        div { class: "flex flex-col items-start justify-start h-full w-full p-4",
            h1 { class: "text-primary text-xl", "Passwords" }
            div { class: "card w-150 mt-3 bg-base-100 shadow-sm",
                div { class: "card-body",
                    span { class: "badge badge-xs badge-neutral", "Password" }

                    div { class: "flex justify-between",
                        h4 { class: "text-2xl font-bold", "Pass iCloud" }
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
                            th { "Type" }
                            th { "Account" }
                            th { "Last Accessed" }
                            th { "Storage" }
                        }
                    }
                    tbody {
                        tr {
                            th { "1" }
                            td { "iCloud Backup" }
                            td { "Password" }
                            td { "Personal" }
                            td { "2023-09-30" }
                            td { "Cloud" }
                        }
                        tr {
                            th { "2" }
                            td { "Secure Notes" }
                            td { "Note" }
                            td { "Work" }
                            td { "2023-09-15" }
                            td { "Local" }
                        }
                        tr {
                            th { "3" }
                            td { "VPN Config" }
                            td { "VPN Password" }
                            td { "Global" }
                            td { "2023-08-20" }
                            td { "System" }
                        }
                        tr {
                            th { "4" }
                            td { "Bluetooth" }
                            td { "Device Password" }
                            td { "Local" }
                            td { "2023-07-25" }
                            td { "Hardware" }
                        }
                        tr {
                            th { "5" }
                            td { "Development Server" }
                            td { "Server Password" }
                            td { "Development" }
                            td { "2023-06-30" }
                            td { "Cloud" }
                        }
                        tr {
                            th { "6" }
                            td { "Router Admin" }
                            td { "Admin Password" }
                            td { "Local" }
                            td { "2023-05-10" }
                            td { "Local" }
                        }
                        tr {
                            th { "7" }
                            td { "GitHub" }
                            td { "Token" }
                            td { "Work" }
                            td { "2023-04-15" }
                            td { "Cloud" }
                        }
                        tr {
                            th { "8" }
                            td { "Bank App" }
                            td { "Online Banking" }
                            td { "Personal" }
                            td { "2023-04-01" }
                            td { "User" }
                        }
                        tr {
                            th { "9" }
                            td { "SSH Key" }
                            td { "Access Key" }
                            td { "Global" }
                            td { "2023-03-20" }
                            td { "Local" }
                        }
                        tr {
                            th { "10" }
                            td { "Cloud Storage" }
                            td { "Storage Info" }
                            td { "Personal" }
                            td { "2023-03-05" }
                            td { "Cloud" }
                        }
                    }
                }
            }
        }
    }
}
