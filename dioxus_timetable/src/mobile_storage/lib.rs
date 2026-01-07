use crate::mobile_storage::path;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::OnceLock,
};

const FILENAME: &str = "timetables.json";
static STORAGE: OnceLock<PathBuf> = OnceLock::new();

pub fn storage_path() -> PathBuf {
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

            let metadata = file.metadata().expect("Failed to get file metadata");
            if metadata.len() == 0 {
                file.write_all(b"{}")
                    .expect("Failed to write initial JSON to storage file");
            }

            file_path
        })
        .clone()
}
