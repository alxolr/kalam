use structopt::StructOpt;

use crate::{entry::Action, storage::Storage};

#[derive(StructOpt, Debug)]
pub struct Cmd {
    #[structopt(short, long, help = "The range to report on [today, week, month]")]
    pub range: String,
}

impl Cmd {
    pub fn run(&self, storage: &Storage) {
        let entries = storage.list_entries(None);

        let entries = match self.range.as_str() {
            "today" => {
                let today = chrono::Local::now().date_naive();

                entries
                    .into_iter()
                    .filter(|entry| entry.action == Action::Stop)
                    .filter(|entry| {
                        let entry_date =
                            chrono::DateTime::parse_from_rfc3339(&entry.updated_at).unwrap();

                        entry_date.date_naive() == today
                    })
                    .collect()
            }
            _ => entries,
        };

        let mut total = 0.0;

        for entry in entries {
            let duration = entry.duration_hours();

            total += duration;

            println!(
                "{} {} [{}] {:.2} hours",
                entry.id, entry.title, entry.project, duration
            );
        }

        println!("Total: {:.2} hours", total);
    }
}
