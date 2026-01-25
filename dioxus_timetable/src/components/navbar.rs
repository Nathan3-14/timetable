use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/components/navbar.scss") }
        div { id: "navbar",
            Link {
                id: "link",
                active_class: "active",
                to: Route::TimetablePage,
                "calendar_today"
            }
            Link { id: "link", active_class: "active", to: Route::SettingsPage, "settings" }
        }
        Outlet::<Route> {}
    }
}
