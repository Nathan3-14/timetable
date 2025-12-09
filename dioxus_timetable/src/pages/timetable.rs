use dioxus::prelude::*;

use crate::{Lesson, LessonEl};

#[component]
pub fn Timetable() -> Element {
    let mut times: Vec<String> = Vec::new();

    let example_lesson: Lesson = Lesson {
        subject: "subject".to_string(),
        teacher_name: "teacher name".to_string(),
        time: "time-time2".to_string(),
        room: "room".to_string(),
    };

    let lessons: Vec<Lesson> = [example_lesson].to_vec();

    for hour in 8..18 {
        let text: String = format!("{0:0>2}", hour.to_string()) + ":00";
        times.push(text);
    }

    rsx! {
        document::Stylesheet { href: asset!("/assets/pages/timetable.scss") }
        div { id: "content",
            h1 { id: "main-title", "Timetable" }

            div { id: "grid-container",
                div { id: "times",
                    for time in times {
                        p { id: "time", "{time}" }
                    }
                }
                div { id: "lessons",
                    for lesson in lessons {
                        LessonEl { lesson }
                    }
                }
            }

            div { id: "bottom-padding" }
        }
    }
}
