use crate::mobile_storage::*;
use crate::pages::no_timetable::*;
use chrono::Datelike;
use chrono::Local;
use dioxus::html::g::local;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use rand::seq::IndexedRandom;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::io::Read;
use std::io::{Error, ErrorKind};
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

fn rem_first_and_last(s: &str) -> String {
    let mut s = s.to_string();
    s.pop(); // remove last
    s.pop(); // last 2
    if !s.is_empty() {
        s.remove(0); // remove first
    }
    s
}

fn get_timetables() -> Result<Vec<TimetableJSON>> {
    let init_lessons = vec![Lesson {
        subject: "Nothing".to_string(),
        time: "08:55-10:25".to_string(),
        room: "Import your `timetable.json` file in settings.".to_string(),
    }];

    let local_storage_path = storage_path();

    let home = std::env::var("HOME").expect("HOME not set on iOS");
    let downloads = std::path::PathBuf::from(home).join("Downloads");
    let timetable_fname = downloads.join("timetable.json");
    let file = std::fs::OpenOptions::new()
        .read(true)
        .open(timetable_fname)
        .ok();

    let mut existing_timetables = std::fs::read_to_string(&local_storage_path)?;
    let whole_json: serde_json::Value = serde_json::from_str(&existing_timetables)?;
    let ids = whole_json.as_object().unwrap().keys();
    let mut ids_2 = ids.clone();

    tracing::info!("{:?}", ids);

    if let Some(mut f) = file {
        let mut new_timetable = String::new();
        f.read_to_string(&mut new_timetable)?;
        let new_timetable_json: Value = serde_json::from_str(&new_timetable)?;
        let new_id = new_timetable_json.as_object().unwrap().keys().next();

        let _ = std::fs::write(downloads.join("test"), b"test succeeded");

        if Option::is_some(&ids_2.find(|&x| x == new_id.unwrap())) {
        } else {
            let id_only = rem_first_and_last(&new_timetable);
            let _ = std::fs::write(downloads.join("test"), &id_only);

            existing_timetables.pop();
            existing_timetables.pop();
            // tracing::info!("{existing_timetables}");
            let mut comma = ""; // so that the comma is only added for the not-first timetable
            if ids.len() > 0 {
                comma = ",";
            }
            existing_timetables = existing_timetables + comma + &id_only + "}";

            let _ = std::fs::write(local_storage_path, &existing_timetables);
        }
    }

    let whole_json: serde_json::Value = serde_json::from_str(&existing_timetables)?;
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

fn fetch_timetable_for_id(id: i32, timetables: Vec<TimetableJSON>) -> Result<TimetableJSON> {
    for timetable in timetables {
        if timetable.id == id {
            return Ok(timetable);
        }
    }

    Err(Error::new(
        ErrorKind::InvalidData,
        "Requested ID not in timetables list.",
    )
    .into())
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
    let mut selected_id: Signal<i32> = use_signal(|| match timetables.first() {
        Some(timetable) => timetable.id,
        None => 0_i32,
    });

    let dt = Local::now();
    let day = dt.weekday();
    let mut day_index = use_signal(|| day.number_from_monday() as usize - 1);

    if *day_index.read() > 4 {
        day_index.set(0);
    };

    rsx! {
        document::Stylesheet { href: asset!("/assets/pages/timetable.scss") }
        div { id: "content",
            if !timetables.is_empty() {
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

                    select { onchange: move |e| { selected_id.set(e.value().parse::<i32>().unwrap()) },
                        for timetable in &timetables {
                            option { value: timetable.id, "{timetable.id}" }
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
                        for lesson in fetch_timetable_for_id(*selected_id.read(), timetables)
                            .unwrap()
                            .lessons[*day_index.read()]
                            .clone()
                        {
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
