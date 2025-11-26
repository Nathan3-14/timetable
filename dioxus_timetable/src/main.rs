use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.scss");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        div { id: "root", class: "test-appearance",
            div { id: "content",
                h1 { "Hello!" }
            }

            div { id: "navbar" }
        }
    }
}
