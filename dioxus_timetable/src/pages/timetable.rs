use chrono::Datelike;
use chrono::Local;
use dioxus::prelude::*;
use rand::seq::IndexedRandom;
use serde::Deserialize;
use std::ops::Index;

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct Lesson {
    subject: String,
    // teacher_name: String,
    time: String,
    room: String,
}

#[derive(Debug, Deserialize)]
struct Lessons {
    mon: Vec<Lesson>,
    tue: Vec<Lesson>,
    wed: Vec<Lesson>,
    thu: Vec<Lesson>,
    fri: Vec<Lesson>,
}

#[derive(Debug, Deserialize)]
struct TimetableJSON {
    id: isize,
    lessons: Lessons,
}

#[derive(Clone, Copy)]
struct CurrentLessons {
    lessons: Signal<Vec<Lesson>>,
}

impl Index<usize> for Lessons {
    type Output = Vec<Lesson>;
    fn index(&self, s: usize) -> &Vec<Lesson> {
        match s {
            0 => &self.mon,
            1 => &self.tue,
            2 => &self.wed,
            3 => &self.thu,
            4 => &self.fri,
            _ => panic!("unknown field: {}", s),
        }
    }
}

#[server]
pub async fn get_timetable_json() -> Result<String, ServerFnError> {
    let timetable_json = std::fs::read_to_string("./timetable_100101.json").unwrap();
    Ok(timetable_json)
}

#[component]
pub fn LessonEl(lesson: Lesson) -> Element {
    let colors = [
        "mauve", "red", "blue", "sapphire", "teal", "sky", "maroon", "green",
    ];

    let color: String = colors.choose(&mut rand::rng()).unwrap().to_string();

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
    let start_display: String = format!("{start_hour}:{start_minute:0>2}");

    let mut grid_start: i8 = 1;
    grid_start += (start_hour - 8) * 12; // 12 rows per hour
    grid_start += start_minute / 5; // every 5 minutes is one row

    let end_times: Vec<String> = times[1].split(":").map(|x| x.to_string()).collect();
    let end_hour: i8 = end_times[0].parse::<i8>()?;
    let end_minute: i8 = end_times[1].parse::<i8>()?;
    let end_display: String = format!("{end_hour}:{end_minute:0>2}");

    let mut grid_span: i8 = 0;
    grid_span += (end_hour - start_hour) * 12;
    grid_span += (end_minute - start_minute) / 5;

    rsx! {
        div {
            id: "lesson",
            background_color: "var(--{color})",
            grid_row_start: grid_start,
            grid_row_end: "span {grid_span}",
            h1 { id: "lesson__subject-name", "{lesson.subject}" }
            // p { id: "lesson__teacher-name", "{lesson.teacher_name}" }
            p { id: "lesson__room", "{lesson.room}" }
            p { id: "lesson__time", "{start_display} to {end_display}" }
        }
    }
}

#[component]
pub fn Timetable() -> Element {
    const WEEKDAYS: [&str; 5] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];

    let mut times: Vec<String> = Vec::new();
    for hour in 8..18 {
        let text: String = format!("{0:0>2}", hour.to_string()) + ":00";
        times.push(text);
    }

    let timetable_string = use_server_future(get_timetable_json)?;
    let timetable: TimetableJSON =
        serde_json::from_str(&timetable_string.unwrap().unwrap()).unwrap();

    let dt = Local::now();
    let day = dt.weekday();
    let mut day_index = use_signal(|| day.number_from_monday() as usize - 1);

    if *day_index.read() > 4 {
        day_index.set(0);
    };

    rsx! {
        document::Stylesheet { href: asset!("/assets/pages/timetable.scss") }
        div { id: "content",
            div { id: "title-grid",
                button {
                    id: "day-button",
                    onclick: move |_| {
                        day_index.set((day_index - 1) % 5);
                    },
                    "keyboard_arrow_left"
                }

                h1 { id: "main-title", "{WEEKDAYS[*day_index.read()].trim_end()}" }

                button {
                    id: "day-button",
                    onclick: move |_| {
                        day_index.set((day_index + 1) % 5);
                    },
                    "keyboard_arrow_right"
                }
            }

            div { id: "grid-container",
                div { id: "times",
                    for time in times {
                        p { id: "time", "{time}" }
                    }
                }
                div { id: "lessons",
                    for lesson in timetable.lessons[*day_index.read()].clone() {
                        LessonEl { lesson }
                    }
                }
            }

            div { id: "bottom-padding" }
        }
    }
}
