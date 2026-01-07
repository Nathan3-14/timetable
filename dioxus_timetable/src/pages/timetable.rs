use crate::mobile_storage::*;
use crate::pages::no_timetable::*;
use chrono::Datelike;
use chrono::Local;
use dioxus::prelude::*;
use rand::seq::IndexedRandom;
use serde::Deserialize;
use serde::Serialize;
use std::ops::Index;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Lesson {
    subject: String,
    // teacher_name: String,
    time: String,
    room: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Lessons {
    mon: Vec<Lesson>,
    tue: Vec<Lesson>,
    wed: Vec<Lesson>,
    thu: Vec<Lesson>,
    fri: Vec<Lesson>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct TimetableJSON {
    id: i32,
    lessons: Lessons,
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

fn get_timetables() -> Result<Vec<TimetableJSON>> {
    // let timetable_json = std::fs::read_to_string("./timetable_100101.json").unwrap();
    // let path = mobile_storage::storage_path();
    let init_lessons = vec![Lesson {
        subject: "Nothing".to_string(),
        time: "08:55-10:25".to_string(),
        room: "Import your `timetable.json` file in settings.".to_string(),
    }];

    let path = storage_path();

    let timetables_str = std::fs::read_to_string(&path)?;
    let whole_json: serde_json::Value = serde_json::from_str(&timetables_str)?;
    let ids = whole_json.as_object().unwrap().keys();
    let mut timetables: Vec<TimetableJSON> = vec![];

    for id in ids {
        let id = id.as_str();
        if let Some(timetable) = whole_json.get(id) {
            let timetable: TimetableJSON = TimetableJSON {
                id: id.parse::<i32>()?,
                lessons: serde_json::from_value(timetable.get("lessons").unwrap().clone())?,
            };
            timetables.push(timetable);
        }
    }

    let init_data = TimetableJSON {
        id: 100101,
        lessons: Lessons {
            mon: init_lessons.clone(),
            tue: init_lessons.clone(),
            wed: init_lessons.clone(),
            thu: init_lessons.clone(),
            fri: init_lessons,
        },
    };

    let timetable_json = init_data;

    Ok(timetables)
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
    let start_display: String = format!("{start_hour:0>2}:{start_minute:0>2}");

    let mut grid_start: i8 = 1;
    grid_start += (start_hour - 8) * 12; // 12 rows per hour
    grid_start += start_minute / 5; // every 5 minutes is one row

    let end_times: Vec<String> = times[1].split(":").map(|x| x.to_string()).collect();
    let end_hour: i8 = end_times[0].parse::<i8>()?;
    let end_minute: i8 = end_times[1].parse::<i8>()?;
    let end_display: String = format!("{end_hour:0>2}:{end_minute:0>2}");

    let mut grid_span: i8 = 0;
    grid_span += (end_hour - start_hour) * 12;
    grid_span += (end_minute - start_minute) / 5;

    rsx! {
        div {
            id: "lesson",
            background_color: "var(--{color})",
            grid_row_start: grid_start,
            grid_row_end: "span {grid_span}",
            div { id: "lesson__header",
                h1 { id: "lesson__subject-name", "{lesson.subject}" }
                p { id: "lesson__room", "{lesson.room}" }
            }
            // p { id: "lesson__teacher-name", "{lesson.teacher_name}" }
            p { id: "lesson__time-start", "{start_display}" }
            p { id: "lesson__time-end", "{end_display}" }
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

    // let timetable_string = use_server_future(get_timetable_json)?;
    // let timetable: TimetableJSON =
    // serde_json::from_str(&timetable_string.unwrap().unwrap()).unwrap();

    let timetables: Vec<TimetableJSON> = get_timetables().unwrap();
    let timetable = timetables.get(0);

    let dt = Local::now();
    let day = dt.weekday();
    let mut day_index = use_signal(|| day.number_from_monday() as usize - 1);

    if *day_index.read() > 4 {
        day_index.set(0);
    };

    rsx! {
        document::Stylesheet { href: asset!("/assets/pages/timetable.scss") }
        div { id: "content",
            if Option::is_some(&timetable) {
                div { id: "title-grid",
                    button {
                        id: "day-button",
                        onclick: move |_| {
                            // + 4 then mod 5 is the same as - 1 then mod 5
                            day_index.set((day_index + 4) % 5);
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

                    select {
                        for timetable in &timetables {
                            option { value: "{timetable.id}", "{timetable.id}" }
                        }
                    }
                }

                div { id: "grid-container",
                    div { id: "times",
                        for time in times {
                            p { id: "time", "{time}" }
                        }
                    }
                    div { id: "lessons",
                        for lesson in timetable.unwrap().lessons[*day_index.read()].clone() {
                            LessonEl { lesson }
                        }
                    }
                }
            } else {
                NoTimetable {}
            }

            div { id: "bottom-padding" }
        }
    }
}
