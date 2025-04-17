use dioxus::prelude::*;

#[component]
pub fn ApiTokens() -> Element {
    rsx! {
        div { class: "flex flex-col items-start justify-start h-full w-full p-4",
            h1 { class: "text-primary text-xl", "Api Tokens" }
            div { class: "card w-150 mt-3 bg-base-100 shadow-sm",
                div { class: "card-body",
                    span { class: "badge badge-xs badge-accent", "Api Token" }

                    div { class: "flex justify-between",
                        h4 { class: "text-2xl font-bold", "A1B2C3D4-E5F6-G7H8-I9J0-KLMNOPQRSTU" }
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
                            th { "Name" }
                            th { "Token Type" }
                            th { "Permissions" }
                            th { "Last Used" }
                            th { "Source" }
                        }
                    }
                    tbody {
                        tr {
                            th { "1" }
                            td { "Service A" }
                            td { "Bearer" }
                            td { "Read/Write" }
                            td { "2023-10-01" }
                            td { "Internal" }
                        }
                        tr {
                            th { "2" }
                            td { "Service B" }
                            td { "API Key" }
                            td { "Read Only" }
                            td { "2023-09-20" }
                            td { "External" }
                        }
                        tr {
                            th { "3" }
                            td { "Service C" }
                            td { "OAuth" }
                            td { "Full Access" }
                            td { "2023-08-18" }
                            td { "Internal" }
                        }
                        tr {
                            th { "4" }
                            td { "Service D" }
                            td { "Bearer" }
                            td { "Limited Access" }
                            td { "2023-07-25" }
                            td { "External" }
                        }
                        tr {
                            th { "5" }
                            td { "Service E" }
                            td { "API Key" }
                            td { "Read/Write" }
                            td { "2023-06-30" }
                            td { "Internal" }
                        }
                        tr {
                            th { "6" }
                            td { "Service F" }
                            td { "OAuth" }
                            td { "Read Only" }
                            td { "2023-05-10" }
                            td { "External" }
                        }
                        tr {
                            th { "7" }
                            td { "Service G" }
                            td { "Bearer" }
                            td { "Full Access" }
                            td { "2023-04-20" }
                            td { "Internal" }
                        }
                        tr {
                            th { "8" }
                            td { "Service H" }
                            td { "API Key" }
                            td { "Limited Access" }
                            td { "2023-03-15" }
                            td { "External" }
                        }
                        tr {
                            th { "9" }
                            td { "Service I" }
                            td { "OAuth" }
                            td { "Read/Write" }
                            td { "2023-03-01" }
                            td { "Internal" }
                        }
                        tr {
                            th { "10" }
                            td { "Service J" }
                            td { "Bearer" }
                            td { "Full Access" }
                            td { "2023-02-18" }
                            td { "External" }
                        }
                    }
                }
            }
        }
    }
}
