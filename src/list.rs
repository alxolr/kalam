use structopt::StructOpt;

use crate::storage::Storage;

#[derive(StructOpt, Debug)]
pub struct Cmd {
    #[structopt(short, long, default_value = "10")]
    pub limit: usize,
}

impl Cmd {
    pub fn run(&self, storage: &Storage) {
        let entries = storage.list_entries(Some(self.limit));

        for entry in entries {
            println!("{:#?}", entry);

            let duration = entry.duration_hours();
            println!("Duration: {:.2} hours", duration);
        }
    }
}
