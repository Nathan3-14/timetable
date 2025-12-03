use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.scss");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        div { id: "content",
            h1 { "Hello" }
            for _ in 0..100 {
                br {}
                "hello"
            }
            div { id: "bottom-padding" }
        }

        div { id: "navbar" }
    }
}
