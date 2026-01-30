mod components;
mod mobile_storage;
mod pages;
mod types;

use crate::components::*;
use crate::pages::*;
use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    TimetablePage,

    #[route("/settings")]
    SettingsPage,
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
            document::Meta { content: "width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover" }
            document::Stylesheet { href: asset!("/assets/root.scss") }
            document::Link { rel: "icon", href: "data:," }
            document::Title { "Timetable" }
            Router::<Route> {}
        }
    });
}
