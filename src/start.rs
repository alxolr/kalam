use structopt::StructOpt;

use crate::{entry::Entry, storage::Storage};

#[derive(StructOpt, Debug)]
pub struct Cmd {
    #[structopt(short, long)]
    pub title: String,

    #[structopt(short, long)]
    pub project: String,
}

impl Cmd {
    pub fn run(&self, storage: &Storage) {
        let entry = Entry::start(self.title.clone(), self.project.clone());
        let entry = storage.start(entry);

        println!("{:#?}", entry);
    }
}
