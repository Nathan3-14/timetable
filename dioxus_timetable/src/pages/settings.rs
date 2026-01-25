use crate::types::*;
use dioxus::{logger::tracing, prelude::*};
use std::io::Read;

fn load_new_timetable() -> Result<()> {
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

    [
        "mauve", "red", "blue", "sapphire", "teal", "sky", "maroon", "green",
    ];

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
                            p {
                                class: "color-text",
                                color: format!("var(--{color})"),
                                "{color}"
                            }
                            select {
                                value: "{color}",
                                onchange: move |e| {
                                    change_color(subject.clone(), e.value()).unwrap();
                                    tracing::info!("color: {}", e.value());
                                },
                                option { value: "mauve", color: "var(--mauve)", "mauve" }
                                option { value: "red", color: "var(--red)", "red" }
                                option { value: "maroon", color: "var(--maroon)", "maroon" }
                                option { value: "green", color: "var(--green)", "green" }
                                option { value: "teal", color: "var(--teal)", "teal" }
                                option { value: "sky", color: "var(--sky)", "sky" }
                                option {
                                    value: "sapphire",
                                    color: "var(--sapphire)",
                                    "sapphire"
                                }
                                option { value: "blue", color: "var(--blue)", "blue" }
                            }
                        }
                    }
                }
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
                button { id: "reset-timetables", class: "danger-button", "Clear All Timetables" }
            }
        }
    }
}
