use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Cli {
    List(list::Cmd),
    Start(start::Cmd),
    Stop(stop::Cmd),
    Report(report::Cmd),
    Active,
    Path,
}

mod active;
mod entry;
mod list;
mod start;
mod stop;
mod storage;
mod report;

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
            cmd.run(&storage);
        }
        Cli::Stop(cmd) => {
            cmd.run(&storage);
        }
        Cli::Report(cmd) => {
            cmd.run(&storage);
        }
        Cli::List(list) => {
            list.run(&storage);
        },
        Cli::Path => {
            println!("{}", storage.storage_path().display());
        },
        Cli::Active => {
            active::Cmd.run(&storage);
        },
        
    }

    Ok(())
}
