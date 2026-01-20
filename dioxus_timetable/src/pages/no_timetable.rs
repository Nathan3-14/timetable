use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NoTimetable() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/pages/noTimetable.scss") }
        p { id: "title", "Timetable not found." }
        p { id: "desc",
            // "Please import the "
            // code { "timetable.json" }
            // " file given by your college in "
            // Link { id: "settingsLink", to: Route::Settings, "settings" }
            // "."
            //
            // "Please import the timetable file given by your college in "
            // Link { id: "settingsLink", to: Route::Settings, "settings" }
            // "."
            //
            "Please import your timetable file."
            br {}
            br {}
            "You can do this by downloading it from your college, then placing it in your Downloads folder under the name "
            code { "timetable.json" }
        }
    }
}
