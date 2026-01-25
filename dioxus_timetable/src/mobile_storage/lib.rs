use crate::mobile_storage::path;
use crate::types::LocalStorage;
use linked_hash_map::LinkedHashMap;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::OnceLock,
};

const FILENAME: &str = "timetables.json";
static STORAGE: OnceLock<PathBuf> = OnceLock::new();

pub fn local_storage_path() -> PathBuf {
    STORAGE
        .get_or_init(|| {
            // aquí ya llamas a tu path::files_dir()
            let dir = path::files_dir();
            let file_path = dir.join(FILENAME);

            let mut file = std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(false)
                .open(&file_path)
                .expect("Failed to open storage file");

            let init_data: LocalStorage = LocalStorage {
                colors: LinkedHashMap::new(),
                default_id: "0".to_string(),
                timetables: LinkedHashMap::new(),
            };

            let init_string: String =
                serde_json::to_string_pretty(&init_data).expect("Failed to parse initial data");

            let metadata = file.metadata().expect("Failed to get file metadata");
            if metadata.len() == 0 {
                file.write_all(init_string.as_bytes())
                    .expect("Failed to write initial JSON to storage file");
            }

            file_path
        })
        .clone()
}
