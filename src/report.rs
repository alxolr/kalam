use std::collections::HashSet;

use structopt::StructOpt;

use crate::{entry::Action, storage::Storage};

#[derive(StructOpt, Debug)]
pub struct Cmd {
    #[structopt(short, long, help = "The range to report on [today, week, all]")]
    pub range: String,

    #[structopt(short, long, help = "The project to report on")]
    pub project: Option<String>,
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

        let entries: Vec<crate::entry::Entry> = if self.project.is_some() {
            entries
                .into_iter()
                .filter(|entry| entry.project == self.project.clone().unwrap())
                .collect()
        } else {
            entries
        };

        let projects = entries
            .iter()
            .map(|entry| entry.project.clone())
            .collect::<HashSet<String>>();

        let mut total = 0.0;

        for project in projects {
            let project_entries = entries
                .iter()
                .filter(|entry| entry.project == project)
                .collect::<Vec<&crate::entry::Entry>>();

            let project_total = project_entries
                .iter()
                .map(|entry| entry.duration_hours())
                .sum::<f64>();

            println!("\n{} [{:.2} hours]", project, project_total);

            for entry in &project_entries {
                let date = chrono::DateTime::parse_from_rfc3339(&entry.created_at).unwrap();

                println!(
                    "{:>14.8} # {:<40} ({:<15}) {:>5} hours",
                    entry.id,
                    entry.title,
                    date.format("%d-%m-%Y %H:%M"),
                    format!("+{:.2}", entry.duration_hours())
                );
            }

            total += project_total;
        }

        println!("\nTotal: {:.2} hours", total);
    }
}
