use crate::{
    entry::{Action, Entry},
    storage::Storage,
};

pub struct Cmd;

impl Cmd {
    pub fn run(&self, storage: &Storage) {
        let entries = storage.list_entries(None);

        let active_entries = entries
            .into_iter()
            .filter(|entry| entry.action == Action::Start)
            .collect::<Vec<Entry>>();

        for entry in active_entries {
            let now = chrono::Local::now().to_rfc3339();

            let start: chrono::DateTime<chrono::FixedOffset> =
                chrono::DateTime::parse_from_rfc3339(&entry.created_at).unwrap();
            let end = chrono::DateTime::parse_from_rfc3339(&now).unwrap();

            let duration = end - start;

            println!(
                "{} {} [{}] {:.2} hours",
                entry.id,
                entry.title,
                entry.project,
                duration.num_seconds() as f64 / 60.0 / 60.0
            );
        }
    }
}
