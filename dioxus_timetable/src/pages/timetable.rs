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

    let start_times: Vec<String> = times[0].split(":").map(|x| x.to_string()).collect();
    let start_hour: i8 = start_times[0].parse::<i8>()?;
    let start_minute: i8 = start_times[1].parse::<i8>()?;

    let mut grid_start: i8 = 1;
    grid_start += (start_hour - 8) * 12; // 12 rows per hour
    grid_start += start_minute / 5; // every 5 minutes is one row

    let end_times: Vec<String> = times[1].split(":").map(|x| x.to_string()).collect();
    let end_hour: i8 = end_times[0].parse::<i8>()?;
    let end_minute: i8 = end_times[1].parse::<i8>()?;

    let mut grid_span: i8 = 0;
    grid_span += (end_hour - start_hour) * 12;
    grid_span += (end_minute - start_minute) / 5;

    rsx! {
        div {
            id: "lesson",
            background_color: "var(--mauve)",
            grid_row_start: grid_start,
            grid_row_end: "span {grid_span}",
            h1 { id: "lesson__subject-name", "{lesson.subject}" }
            p { id: "lesson__teacher-name", "{lesson.teacher_name}" }
            p { id: "lesson__room", "{lesson.room} {start_hour} {start_minute} {grid_start}" }
            p { id: "lesson__time", "from {start_times:#?} to {end_times:#?}" }
        }
    }
}

#[component]
pub fn Timetable() -> Element {
    let mut times: Vec<String> = Vec::new();

    let example_lesson: Lesson = Lesson {
        subject: "subject".to_string(),
        teacher_name: "teacher name".to_string(),
        time: "9:30-11:30".to_string(),
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
