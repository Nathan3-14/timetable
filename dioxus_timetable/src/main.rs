mod components;
mod pages;

use crate::components::*;
use crate::pages::*;
use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Timetable,
}

fn main() {
    dioxus::launch(|| {
        rsx! {
            document::Stylesheet { href: asset!("/assets/root.scss") }
            document::Link { rel: "icon", href: "data:," }
            document::Title { "Timetable" }
            Router::<Route> {}
        }
    });
}
