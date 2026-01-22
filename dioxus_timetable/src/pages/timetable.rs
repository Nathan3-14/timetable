use crate::mobile_storage::*;
use crate::pages::no_timetable::*;
use crate::types::*;
use chrono::{Datelike, Local};
use dioxus::prelude::*;
use dioxus::{html::g::local, logger::tracing};
use rand::seq::IndexedRandom;
use serde_json::Value;
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Read};

fn rem_first_and_last(s: &str) -> String {
    let mut s = s.to_string();
    s.pop(); // remove last
    s.pop(); // last 2

    if !s.is_empty() {
        s.remove(0); // remove first
    }

    s
}

fn get_local_data() -> Result<LocalStorage> {
    let local_storage_path = crate::mobile_storage::local_storage_path();

    let local_storage = std::fs::read_to_string(&local_storage_path)?;
    let mut local_data: LocalStorage = serde_json::from_str(&local_storage)?;
    let ids: Vec<String> = local_data.timetables.keys().cloned().collect();

    let home = std::env::var("HOME").expect("HOME not set on iOS");
    let downloads = std::path::PathBuf::from(home).join("Downloads");
    let new_timetable_path = downloads.join("timetable.json");
    let file = std::fs::OpenOptions::new()
        .read(true)
        .open(&new_timetable_path)
        .ok();

    // tracing::info!("{:?}", ids);

    if let Some(mut f) = file {
        let mut new_timetable = String::new();
        f.read_to_string(&mut new_timetable)?;
        let new_timetable_json: Timetable = serde_json::from_str(&new_timetable)?;

        let _ = std::fs::write(downloads.join("test"), b"test succeeded");

        // if Option::is_some(&ids_2.find(|&x| x == new_id.unwrap())) {
        if !ids.contains(&new_timetable_json.id) {
            local_data
                .timetables
                .insert(new_timetable_json.id.clone(), new_timetable_json);

            let new_json_string = serde_json::to_string_pretty(&local_data).unwrap();
            let _ = std::fs::write(&local_storage_path, new_json_string);
        }
    }

    Ok(local_data)
}

fn fetch_timetable_for_id(id: String, timetables: HashMap<String, Timetable>) -> Result<Timetable> {
    for timetable in timetables.values() {
        if timetable.id == id {
            return Ok(timetable.clone());
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
pub fn TimetablePage() -> Element {
    const WEEKDAYS: [&str; 5] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];

    let mut times: Vec<String> = Vec::new();
    for hour in 8..18 {
        let text: String = format!("{0:0>2}", hour.to_string()) + ":00";
        times.push(text);
    }

    // let timetable_string = use_server_future(get_timetable_json)?;
    // let timetable: Timetable =
    // serde_json::from_str(&timetable_string.unwrap().unwrap()).unwrap();

    let data: LocalStorage = get_local_data().unwrap();

    let mut ids: Vec<String> = data.timetables.keys().cloned().collect();
    // ids.sort();

    let mut selected_id: Signal<String> = use_signal(|| data.default_id.clone());
    // tracing::info!("id is initially: {}", selected_id.read());

    tracing::info!("{:?}", ids);

    let dt = Local::now();
    let day = dt.weekday();
    let mut day_index = use_signal(|| day.number_from_monday() as usize - 1);

    if *day_index.read() > 4 {
        day_index.set(0);
    };

    rsx! {
        document::Stylesheet { href: asset!("/assets/pages/timetable.scss") }
        div { id: "content",
            if &*selected_id.read() != "0" {
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
                        onchange: move |e| async move {
                            tracing::info!("changing id to: {}", e.value());
                            *selected_id.write() = e.value();
                            tracing::info!("id is now: {}", selected_id);
                        },
                        value: selected_id(),

                        for id in ids {
                            option { value: id.clone(), "{&id}" }
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
                        for lesson in data.timetables.get(&selected_id()).unwrap().lessons[day_index()].clone() {
                            {
                                tracing::info!("lesson for {}: {:?}", selected_id, lesson);
                            }
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
