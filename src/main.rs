use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Cli {
    List(list::Cmd),
    Start(start::Cmd),
    Stop,
    Report,
}

mod entry;
mod list;
mod start;
mod storage;

fn main() {
    if let Err(e) = do_main() {
        println!("Error: {}", e);
        std::process::exit(1);
    }
}

fn do_main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    let storage = storage::Storage;

    storage.create_path_if_not_exists();

    match args {
        Cli::Start(cmd) => {
            cmd.run();
        }
        Cli::Stop => {
            println!("Stopping...");
        }
        Cli::Report => {
            println!("Reporting...");
        }
        Cli::List(list) => {
            list.run();
        }
    }

    Ok(())
}
