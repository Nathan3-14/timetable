use dioxus::prelude::*;

pub fn Settings() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/pages/settings.scss") }
        "Settings!"
    }
}
