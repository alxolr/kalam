use structopt::StructOpt;

use crate::storage::Storage;

#[derive(StructOpt, Debug)]
pub struct Cmd {
    #[structopt(short, long)]
    pub id: Option<String>,
}

impl Cmd {
    pub fn run(&self, storage: &Storage) {
        let entry = storage.stop(self.id.clone());

        println!("{:#?}", entry);

        let duration = entry.unwrap().duration_hours();

        println!("Duration: {:.2} hours", duration);
    }
}
