pub mod lfd;

use clap::error::ErrorKind;
use std::fs;
use std::string::String;

use crate::lfd::lfd_file::LfdFile;
use crate::lfd::traits::lfd_print::LfdPrint;
use clap::CommandFactory;
use clap::Parser;
use clap::Subcommand;

/// LFD Tool
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Display {
        #[arg(short, long)]
        file: String,
    },
}

fn _main() {
    let args = Args::parse();

    match &args.command {
        Commands::Display { file } => {
            let lfd_file_result = LfdFile::read_from_file(file);

            match lfd_file_result {
                Ok(lfd_file) => {
                    lfd_file.lfd_print(0);
                }
                Err(e) => {
                    let mut cmd = Args::command();
                    cmd.error(ErrorKind::ArgumentConflict, e).exit();
                }
            }
        }
    }
}

fn main() -> Result<(), String> {
    println!("LFD Parse Tool");

    let _create_dir_result = fs::create_dir("out/");

    for entry in fs::read_dir("data/").map_err(|e| format!("Error reading directory: {e}"))? {
        let entry = entry.map_err(|e| format!("Invalid entry: {e}"))?;

        let _is_species = entry.path().starts_with("data/SPECIES.LFD");

        let _is_tourdesk = entry.path().starts_with("data/TOURDESK.LFD");
        let is_empire_lfd = entry.path().starts_with("data/EMPIRE.LFD");
        let is_scene4 = entry.path().starts_with("data/SCENE4.LFD");
        let is_platform = entry.path().starts_with("data/PLATFORM.LFD");

        let _should_parse = is_empire_lfd || is_scene4 || is_platform;

        let _is_city = entry.path().starts_with("data/CITY.LFD");
        let _is_battle = entry.path().starts_with("data/BATTLE2.LFD");
        let _is_test = entry.path().starts_with("data/LAUNCH.LFD");

        // || {
        // entry.path().starts_with("data/SPECIES2.LFD")
        // || entry.path().starts_with("data/SPECIES3.LFD")
        // };

        match entry.path().is_file() && _is_species {
            true => {
                let lfd_file = LfdFile::read_from_file(
                    entry.path().to_str().expect("Failed to get file name."),
                )?;

                lfd_file.lfd_print(0);

                // let output_file = LfdFile {
                //     file_name: lfd_file.file_name.replace("data/", "out/"),
                //     archive: lfd_file.archive,
                // };
                // output_file.write_to_file()?;
            }
            false => (),
        }
    }

    Ok(())
}
