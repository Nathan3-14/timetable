use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/components/navbar.scss") }
        div { id: "navbar",
            Link { to: Route::Timetable, "Timetable" }
            Link { class: "link link--settings", to: Route::Settings, "settings" }
        }
        Outlet::<Route> {}
    }
}
