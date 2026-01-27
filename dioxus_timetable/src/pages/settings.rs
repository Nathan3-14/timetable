use crate::types::*;
use dioxus::{logger::tracing, prelude::*};
use std::{collections::HashMap, io::Read};

fn load_new_timetable() -> Result<()> {
    let local_storage_path = crate::mobile_storage::local_storage_path();

    let local_storage = std::fs::read_to_string(&local_storage_path)?;
    let mut local_data: LocalStorage = serde_json::from_str(&local_storage)?;
    let ids: Vec<String> = local_data.timetables.keys().cloned().collect();

    let home = std::env::var("HOME").expect("HOME not set on iOS");
    // let home = crate::mobile_storage::path::files_dir();
    let downloads = std::path::PathBuf::from(home).join("Downloads");
    // let downloads = home.join("Downloads");
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
            for subject in &new_timetable_json.subjects.clone() {
                local_data
                    .colors
                    .entry(subject.clone())
                    .or_insert("red".to_string());
            }

            local_data
                .timetables
                .insert(new_timetable_json.id.clone(), new_timetable_json);

            let new_json_string = serde_json::to_string_pretty(&local_data).unwrap();
            let _ = std::fs::write(&local_storage_path, new_json_string);
        }
    }
    Ok(())
}

fn load_new_timetable_from_string(new_timetable: String) -> Result<()> {
    let local_storage_path = crate::mobile_storage::local_storage_path();
    tracing::info!("{}", &local_storage_path.display());

    let local_storage = std::fs::read_to_string(&local_storage_path)?;
    let mut local_data: LocalStorage = serde_json::from_str(&local_storage)?;
    tracing::info!("local: {:?}", local_data);
    let ids: Vec<String> = local_data.timetables.keys().cloned().collect();

    tracing::info!("new string: {}", new_timetable);

    let new_timetable_json: Timetable = serde_json::from_str(&new_timetable)?;
    tracing::info!("new: {:?}", new_timetable_json);

    // if Option::is_some(&ids_2.find(|&x| x == new_id.unwrap())) {
    if !ids.contains(&new_timetable_json.id) {
        tracing::info!("new timetable, adding...");
        for subject in &new_timetable_json.subjects.clone() {
            local_data
                .colors
                .entry(subject.clone())
                .or_insert("red".to_string());
        }

        if local_data.default_id == "0" {
            local_data.default_id = new_timetable_json.id.clone();
        }

        local_data
            .timetables
            .insert(new_timetable_json.id.clone(), new_timetable_json);

        tracing::info!("new timetable added, writing...");

        let new_json_string = serde_json::to_string_pretty(&local_data).unwrap();
        let _ = std::fs::write(&local_storage_path, new_json_string);
        tracing::info!("new timetable written");
    }

    Ok(())
}

fn change_color(subject: String, color: String) -> Result<()> {
    let local_storage_path = crate::mobile_storage::local_storage_path();

    let local_storage = std::fs::read_to_string(&local_storage_path)?;
    let mut local_data: LocalStorage = serde_json::from_str(&local_storage)?;

    // local_data.colors.insert(subject, color);
    *local_data.colors.get_mut(&subject).unwrap() = color;

    std::fs::write(
        local_storage_path,
        serde_json::to_string_pretty(&local_data)?,
    )
    .unwrap();

    Ok(())
}

#[component]
pub fn SettingsPage() -> Element {
    let local_storage_path = crate::mobile_storage::local_storage_path();

    let local_storage = std::fs::read_to_string(&local_storage_path)?;
    let mut local_data: LocalStorage = serde_json::from_str(&local_storage)?;

    let mut color_map: HashMap<String, Signal<String>> = HashMap::new();

    for (subject, color) in &local_data.colors {
        color_map.insert(subject.clone(), use_signal(|| color.clone()));
    }

    let mut new_timetable_string = use_signal(String::new);

    rsx! {
        document::Stylesheet { href: asset!("/assets/pages/settings.scss") }
        div { id: "content",
            "Settings!"

            div { id: "colors",
                h1 { "Colours" }
                div { id: "colors-container",
                    for (subject , color) in local_data.colors.clone() {
                        div { class: "color-row",
                            p { class: "subject-text", "{subject}" }
                            div {
                                p {
                                    id: "color-text-{subject}",
                                    class: "color-text",
                                    color: format!("var(--{color})"),
                                    "⬤"
                                }
                                select {
                                    value: "{color}",
                                    onchange: move |e| {
                                        let subjectClone = subject.clone();
                                        async move {
                                            change_color(subjectClone.clone(), e.value()).unwrap();
                                            #[rustfmt::skip]
                                            // let _ = dioxus::document::eval(
                                            //         &format!(
                                            //             r#" document.getElementById("color-text-{subjectClone}").innerText = "{0}" "#,
                                            //             e.value(),
                                            //         ),
                                            //     )
                                            //     .await;
                                            let _ = dioxus::document::eval(
                                                    &format!(
                                                        r#" document.getElementById("color-text-{subjectClone}").style.color = "var(--{0})" "#,
                                                        e.value(),
                                                    ),
                                                )
                                                .await;
                                            tracing::info!("color: {}", e.value());
                                        }
                                    },
                                    option { value: "mauve", color: "var(--mauve)", "Purple" }
                                    option { value: "red", color: "var(--red)", "Red" }
                                    option { value: "peach", color: "var(--peach)", "Orange" }
                                    option { value: "green", color: "var(--green)", "Green" }
                                    option { value: "teal", color: "var(--teal)", "Teal" }
                                    // option { value: "sky", color: "var(--sky)", "Sky Blue" }
                                    option {
                                        value: "sapphire",
                                        color: "var(--sapphire)",
                                        "Sapphire"
                                    }
                                    option { value: "blue", color: "var(--blue)", "Blue" }
                                }
                            }
                        }
                    }
                }
            }

            input {
                id: "new-input",
                r#type: "text",
                onchange: move |e| { new_timetable_string.set(e.value()) },
            }

            button {
                id: "new-timetable-button",
                onclick: move |_| async move {
                    load_new_timetable_from_string(new_timetable_string.read().clone().to_string())
                        .unwrap();
                },
                "Import timetable from clipboard"
            }

            div { id: "button-grid",
                button {
                    id: "load-new-timetable",
                    class: "danger-button",
                    onclick: move |_| {
                        load_new_timetable().unwrap();
                    },
                    "Load New Timetable"
                }
                button {
                    id: "reset-timetables",
                    class: "danger-button",
                    onclick: reset_timetables,
                    "Clear All Timetables"
                }
            }
        }
    }
}
