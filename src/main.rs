use clap::error::ErrorKind;
use lfd::lfd::lfd_file::LfdFile;
use lfd::lfd::resources::rmap::Rmap;
use lfd::lfd::traits::lfd_print::LfdPrint;
use lfd::vga::parse_vga_pac_file;
use lfd::vga::VgaPacParseMode;
use std::fs;
use std::string::String;

use clap::CommandFactory;
use clap::Parser;
use clap::Subcommand;

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
    Dump {},
    VgaPac {
        #[arg(short, long)]
        file: String,
        #[arg(value_enum)]
        mode: VgaPacParseMode,
        #[arg(short, long)]
        output: Option<String>,
    },
    Extract {
        #[arg(short, long)]
        file: String,
        #[arg(short, long)]
        destination: String,
    },
    CompressDir {
        #[arg(long)]
        dir: String,
        #[arg(long)]
        dest: String,
        #[arg(short, long)]
        name: String,
    },
}

fn main() {
    let args = Args::parse();
    let mut cmd = Args::command();

    match &args.command {
        Commands::Display { file } => {
            let lfd_file_result = LfdFile::read_from_file(file);

            match lfd_file_result {
                Ok(lfd_file) => {
                    lfd_file.lfd_print(0);
                }
                Err(e) => {
                    cmd.error(ErrorKind::ArgumentConflict, e).exit();
                }
            }
        }
        Commands::VgaPac { file, mode, output } => {
            if let Err(e) = parse_vga_pac_file(file, mode, output) {
                cmd.error(ErrorKind::InvalidValue, e).exit();
            }
        }
        Commands::Extract { file, destination } => {
            println!("Extracting file: {file}, destination: {destination}");
            let lfd_file_result = LfdFile::read_from_file(file);

            match lfd_file_result {
                Ok(lfd_file) => match lfd_file.archive.expand_to_destination(destination) {
                    Ok(_) => println!("Success!"),
                    Err(error) => println!("Error: {error}"),
                },
                Err(e) => {
                    cmd.error(ErrorKind::ArgumentConflict, e).exit();
                }
            }
        }
        Commands::CompressDir { dir, dest, name } => {
            println!("compressing {dir} to {dest}/{name}.LFD");
            let result = Rmap::from_directory(dir, dest, name);
            println!("result: {:?}", result);
        }
        Commands::Dump {} => {
            if let Err(e) = dump() {
                cmd.error(ErrorKind::ArgumentConflict, e).exit();
            }
        }
    }
}

fn dump() -> Result<(), String> {
    println!("LFD Parse Tool");

    let _create_dir_result = fs::create_dir("out/");

    for entry in fs::read_dir("data/").map_err(|e| format!("Error reading directory: {e}"))? {
        let entry = entry.map_err(|e| format!("Invalid entry: {e}"))?;

        let is_lfd = entry.path().extension().is_some_and(|ext| ext.eq("LFD"));

        match entry.path().is_file() && is_lfd {
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
