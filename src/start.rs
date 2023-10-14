use structopt::StructOpt;

use crate::{storage::Storage, entry::Entry};

#[derive(StructOpt, Debug)]
pub struct Cmd {
    #[structopt(short, long)]
    pub title: String,

    #[structopt(short, long)]
    pub project: String
}

impl Cmd {
    pub fn run(&self) {

        let entry = Entry::start(self.title.clone(), self.project.clone());
        let storage = Storage;
        let entry = storage.start(entry);

        println!("Started: {:?}", entry);
    }
}
