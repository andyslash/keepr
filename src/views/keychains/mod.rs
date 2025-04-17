use crate::{
    components::tab_wrapper::TabWrapper,
    views::keychains::{
        access_keys::AccessKeys, all_items::AllItems, api_tokens::ApiTokens,
        certificates::Certificates, passwords::Passwords, secure_notes::SecureNotes,
    },
};

use dioxus::prelude::*;

pub mod access_keys;
pub mod all_items;
pub mod api_tokens;
pub mod certificates;
pub mod passwords;
pub mod secure_notes;

#[component]
pub fn Keychains() -> Element {
    rsx! {
        div { class: "tabs tabs-sm tabs-lift fixed h-full",

            TabWrapper {
                class: "ml-0",
                tab: "keychains",
                label: "All items",
                icon: "layers",
                checked: true,
                content: rsx! {
                    AllItems {}
                },
            }

            TabWrapper {
                class: "ml-2",
                tab: "keychains",
                label: "Access Keys",
                icon: "fingerprint",
                content: rsx! {
                    AccessKeys {}
                },
            }

            TabWrapper {
                class: "ml-2",
                tab: "keychains",
                label: "API Tokens",
                icon: "ticket",
                content: rsx! {
                    ApiTokens {}
                },
            }

            TabWrapper {
                class: "ml-2",
                tab: "keychains",
                label: "Passwords",
                icon: "password",
                content: rsx! {
                    Passwords {}
                },
            }

            TabWrapper {
                class: "ml-2",
                tab: "keychains",
                label: "Certificates",
                icon: "shield-check",
                content: rsx! {
                    Certificates {}
                },
            }

            TabWrapper {
                class: "ml-2",
                tab: "keychains",
                label: "Secure Notes",
                icon: "file-lock",
                content: rsx! {
                    SecureNotes {}
                },
            }
        }
    }
}
