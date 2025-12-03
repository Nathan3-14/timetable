use dioxus::prelude::*;

#[component]
pub fn Timetable() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/pages/timetable.scss") }
        div { id: "content",
            h1 { id: "main-title", "Timetable" }
            for _ in 0..100 {
                br {}
                "hello"
            }
            div { id: "bottom-padding" }
        }
    }
}
