use crate::pages::no_timetable::*;
use crate::types::*;
use chrono::{Datelike, Local};
use dioxus::logger::tracing;
use dioxus::prelude::*;

fn get_local_data() -> Result<LocalStorage> {
    let local_storage_path = crate::mobile_storage::local_storage_path();

    let local_storage = std::fs::read_to_string(&local_storage_path)?;
    let local_data: LocalStorage = serde_json::from_str(&local_storage)?;

    Ok(local_data)
}

#[component]
pub fn LessonEl(lesson: Lesson) -> Element {
    let colors = get_local_data().unwrap().colors;
    let color = colors.get(&lesson.subject).unwrap();

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

    let subject_name = match lesson.subject.as_str() {
        "Computer Science" => "C.S.",
        "Uniformed Protective Services" => "U.P.S.",
        _ => &lesson.subject,
    };

    rsx! {
        div {
            class: "lesson",
            background_color: "var(--{color})",
            grid_row_start: grid_start,
            grid_row_end: "span {grid_span}",
            div { class: "lesson__header",
                p { class: "lesson__subject-name", "{subject_name}" }
                p { class: "lesson__room", "{lesson.room}" }
            }
            p { class: "lesson__teacher-name", "{lesson.teacher_name}" }
            p { class: "lesson__time-start", "{start_display}" }
            p { class: "lesson__time-end", "{end_display}" }
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

    let data: LocalStorage = get_local_data().unwrap();

    let mut unsorted_ids: Vec<String> = data.timetables.keys().cloned().collect();
    unsorted_ids.sort();

    // remove the default_id from the vec
    unsorted_ids.retain(|x| *x != data.default_id);

    let mut ids: Vec<String> = vec![data.default_id.clone()];
    // append the rest of the ids so that the default_id is first
    ids.append(&mut unsorted_ids);
    // this is because the first element in the vec is the one selected by default in the `select`,
    // so the default_id needs to be first for it to make sense and work as intended.

    let mut selected_id: Signal<String> = use_signal(|| data.default_id.clone());
    // tracing::info!("id is initially: {}", selected_id.read());

    tracing::info!("{:?}", ids);

    let dt = Local::now();
    let day = dt.weekday();
    let mut day_index = use_signal(|| day.number_from_monday() as usize - 1);

    if day_index() > 4 {
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
                        for lesson in data.timetables[&selected_id()].lessons[day_index()].clone() {
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
