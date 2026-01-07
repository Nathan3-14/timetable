use dioxus::prelude::*;

pub fn Settings() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/pages/settings.scss") }
        div { id: "content",
            "Settings!"

            input {
                r#type: "file",
                // accept: ".json",
                // multiple: "false",
                onchange: move |e| async move {
                    let a = "hello";
                    let b = "hello2";
                },
            }
        }
    }
}
