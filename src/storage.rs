use std::fs::read_to_string;

use crate::entry::Entry;

pub struct Storage;

impl Storage {
    pub fn storage_path(&self) -> std::path::PathBuf {
        let home_dir = home::home_dir().unwrap();
        home_dir.join(".config").join("kalam").join("storage.json")
    }

    pub fn create_path_if_not_exists(&self) {
        let storage_path: std::path::PathBuf = self.storage_path();

        if !storage_path.exists() {
            // create the directory if it doesn't exist
            let storage_dir = storage_path.parent().unwrap();
            if !storage_dir.exists() {
                std::fs::create_dir_all(storage_dir).unwrap();
            }

            // create the file
            std::fs::File::create(&storage_path).unwrap();

            // write an empty array to the file
            let empty_entries: Vec<Entry> = Vec::new();
            let empty_entries_json = serde_json::to_string_pretty(&empty_entries).unwrap();
            std::fs::write(&storage_path, empty_entries_json).unwrap();
        }
    }

    pub fn start(&self, entry: Entry) -> Entry {
        let storage_path = self.storage_path();

        let mut entries = self.list_entries(None);
        entries.push(entry.clone());

        let entries_json = serde_json::to_string_pretty(&entries).unwrap();
        std::fs::write(&storage_path, entries_json).unwrap();

        entry
    }

    pub fn list_entries(&self, limit: Option<usize>) -> Vec<Entry> {
        let storage_path = self.storage_path();

        let file = read_to_string(&storage_path).unwrap();
        let entries: Vec<Entry> = serde_json::from_str(&file).unwrap();

        if let Some(limit) = limit {
            entries.into_iter().rev().take(limit).collect()
        } else {
            entries
        }
    }
}
