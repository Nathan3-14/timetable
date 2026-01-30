use crate::{mobile_storage::local_storage_path, types::*};
use dioxus::{logger::tracing, prelude::*};
use linked_hash_map::LinkedHashMap;

fn load_new_timetable_from_string(new_timetable: String) -> anyhow::Result<()> {
    let local_storage_path = crate::mobile_storage::local_storage_path();
    tracing::info!("{}", &local_storage_path.display());

    let local_storage = std::fs::read_to_string(&local_storage_path)?;
    let mut local_data: LocalStorage = serde_json::from_str(&local_storage)?;
    // tracing::info!("local: {:?}", local_data);
    let ids: Vec<String> = local_data.timetables.keys().cloned().collect();

    // tracing::info!("new string: {}", new_timetable);

    let new_timetable_json: Timetable = serde_json::from_str(&new_timetable)?;
    // tracing::info!("new: {:?}", new_timetable_json);

    for lessons_vec in new_timetable_json.lessons.clone() {
        for lesson in lessons_vec {
            if !new_timetable_json.subjects.contains(&lesson.subject) {
                return Err(anyhow::anyhow!(
                    "Lesson `{}` not present in `subjects` array.",
                    &lesson.subject
                ));
            }
            tracing::info!("lesson {} in subjects array", &lesson.subject);
        }
    }

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

        let new_json_string = serde_json::to_string_pretty(&local_data)?;
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

fn clear_timetables() {
    let init_data = LocalStorage {
        colors: LinkedHashMap::new(),
        default_id: String::from("0"),
        timetables: LinkedHashMap::new(),
    };
    let _ = std::fs::write(
        local_storage_path(),
        serde_json::to_string_pretty(&init_data).unwrap(),
    );
}

#[component]
pub fn SettingsPage() -> Element {
    let local_storage_path = crate::mobile_storage::local_storage_path();

    let local_storage = std::fs::read_to_string(&local_storage_path)?;
    let local_data: LocalStorage = serde_json::from_str(&local_storage)?;

    let mut new_timetable_string = use_signal(String::new);

    // popups
    let mut confirm_clear_popup_showing: Signal<bool> = use_signal(|| false);
    let mut timetable_added_popup_showing: Signal<bool> = use_signal(|| false);
    let mut failed_adding_timetable_popup_showing: Signal<bool> = use_signal(|| false);
    let mut failed_adding_timetable_text: Signal<String> = use_signal(String::new);

    rsx! {
        document::Stylesheet { href: asset!("/assets/pages/settings.scss") }
        div { id: "content",
            h1 { "Settings" }

            div { id: "colors",
                h2 { "Colours" }
                div { id: "colors-container",
                    if !local_data.colors.is_empty() {
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
                                        option {
                                            value: "mauve",
                                            color: "var(--mauve)",
                                            "Purple"
                                        }
                                        option { value: "red", color: "var(--red)", "Red" }
                                        option {
                                            value: "peach",
                                            color: "var(--peach)",
                                            "Orange"
                                        }
                                        option {
                                            value: "green",
                                            color: "var(--green)",
                                            "Green"
                                        }
                                        option {
                                            value: "teal",
                                            color: "var(--teal)",
                                            "Teal"
                                        }
                                        // option { value: "sky", color: "var(--sky)", "Sky Blue" }
                                        option {
                                            value: "sapphire",
                                            color: "var(--sapphire)",
                                            "Sapphire"
                                        }
                                        option {
                                            value: "blue",
                                            color: "var(--blue)",
                                            "Blue"
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        p { "No subjects to colour!" }
                    }
                }
            }

            h2 { id: "import-header", "Timetables" }
            div { id: "timetables-settings-container",
                div { class: "timetables-settings-row",
                    input {
                        id: "new-timetable-input",
                        r#type: "text",
                        onchange: move |e| { new_timetable_string.set(e.value()) },
                        placeholder: "paste your timetable here...",
                    }

                    button {
                        id: "new-timetable-button",
                        onclick: move |_| async move {
                            match load_new_timetable_from_string(
                                new_timetable_string.read().clone().to_string(),
                            ) {
                                Ok(_) => timetable_added_popup_showing.set(true),
                                Err(e) => {
                                    failed_adding_timetable_text.set(e.to_string());
                                    failed_adding_timetable_popup_showing.set(true);
                                }
                            };
                        },
                        "Import"
                    }
                }
                div { class: "timetables-settings-row",
                    p { id: "clear-timetables-description", "Clear stored timetables" }
                    button {
                        id: "clear-timetables-button",
                        onclick: move |_| { confirm_clear_popup_showing.set(true) },
                        "Clear"
                    }
                }
            }
            div { id: "spacer" }
        }

        if confirm_clear_popup_showing() {
            div { id: "confirm-clear-timetables-popup-bg", class: "popup-bg",
                div {
                    id: "confirm-clear-timetables-popup-container",
                    class: "popup-container",
                    p {
                        id: "confirm-clear-timetables-popup-text",
                        class: "popup-text",
                        "Are you sure you want to clear your timetables? You will need to re-import any you wish to use."
                    }
                    div {
                        id: "confirm-clear-timetables-popup-buttons-grid",
                        class: "popup-buttons-grid has-two-buttons",
                        // "grid-template-columns": "1fr 1fr", // so that the grid works right
                        button {
                            class: "popup-button",
                            onclick: move |_| {
                                confirm_clear_popup_showing.set(false);
                            },
                            "Cancel"
                        }
                        button {
                            class: "popup-button bad-button",
                            onclick: move |_| {
                                clear_timetables();
                                confirm_clear_popup_showing.set(false);
                            },
                            "Clear"
                        }
                    }
                }
            }
        }

        if timetable_added_popup_showing() {
            div { class: "popup-bg",
                div { class: "popup-container",
                    p { class: "popup-text", "Successfully added timetable!" }
                    div { class: "popup-buttons-grid",
                        button {
                            class: "popup-button",
                            onclick: move |_| timetable_added_popup_showing.set(false),
                            "Cool!"
                        }
                    }
                }
            }
        }

        if failed_adding_timetable_popup_showing() {
            div { class: "popup-bg",
                div { class: "popup-container",
                    div { class: "popup-text",
                        "Failed to add timetable:"
                        p { id: "error-text", {failed_adding_timetable_text()} }
                        "Perhaps you didn't paste the whole timetable?"
                    }
                    div { class: "popup-buttons-grid",
                        button {
                            class: "popup-button",
                            onclick: move |_| failed_adding_timetable_popup_showing.set(false),
                            "Oh :("
                        }
                    }
                }
            }
        }
    }
}
