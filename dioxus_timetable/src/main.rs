mod components;
mod mobile_storage;
mod pages;

use crate::components::*;
use crate::pages::*;
use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Timetable,

    #[route("/settings")]
    Settings,
}

pub fn capitalise(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
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
