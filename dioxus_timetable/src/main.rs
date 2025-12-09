mod components;
mod pages;

use crate::components::*;
use crate::pages::*;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Lesson {
    subject: String,
    teacher_name: String,
    time: String,
    room: String,
}

#[component]
pub fn LessonEl(lesson: Lesson) -> Element {
    let times: Vec<String> = lesson
        .time
        .split("-")
        .map(|time| time.to_string())
        .collect();

    if times.len() != 2 {
        return rsx! {};
    }

    let start_time = &times[0];
    let end_time = &times[1];

    rsx! {
        div { id: "lesson", background_color: "var(--red)",
            h1 { id: "lesson__subject-name", "{lesson.subject}" }
            p { id: "lesson__teacher-name", "{lesson.teacher_name}" }
            p { id: "lesson__room", "{lesson.room}" }
            p { id: "lesson__time", "from {start_time} to {end_time}" }
        }
    }
}

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
            Router::<Route> {}
        }
    });
}
