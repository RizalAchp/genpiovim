mod board;
mod konstan;
use crate::board::*;
use crate::konstan::*;
use clap::*;
use std::error::Error;

/// simple cli app for generating pio project for vim
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// commands
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// project command for generating makefile, initialize project, and create new projects [-h for help message]
    Project {
        #[clap(short, long, group = "input", default_value = "projectdir")]
        name: String,

        #[clap(short, long, group = "boardid", default_value = "uno")]
        board_id: String,

        /// generate Makefiles for pio project in current dir
        #[clap(short, long, action)]
        gen: bool,

        /// initialize current folder for pio project vim
        #[clap(short, long, requires = "boardid", action)]
        init: bool,

        /// new project folder for pio vim
        #[clap(short, long, requires = "input", requires = "boardid", action)]
        new: bool,
    },

    /// configuration command for listing board and updating board list. [-h for help message]
    Config {
        /// list all platform board
        #[clap(short, long)]
        list: bool,

        /// update data board json
        #[clap(short, long)]
        update: bool,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let json_path = get_board_list_path();
    let args: Args = Args::parse();

    match &args.command {
        Some(Command::Project {
            name,
            board_id,
            gen,
            init,
            new,
        }) => {
            if *gen {
                match gen_make_file(String::from(".")) {
                    Ok(_) => println!("SUCCESS: Creating makefile for pio project done!"),
                    Err(_) => eprintln!("ERROR: Terjadi Kesalahan saat membuat makefile!"),
                }
            } else if *init {
                match initialize_pio_project(&board_id) {
                    Ok(_) => {}
                    Err(_) => {}
                };
                return Ok(());
            } else if *new {
                match create_pio_project(name, &board_id) {
                    Ok(_) => {}
                    Err(_) => {}
                }
                return Ok(());
            }
        }
        Some(Command::Config { list, update }) => {
            if *list {
                match parse_json() {
                    Ok(d) => {
                        println!("{}", d)
                    }
                    Err(ref e) => {
                        eprintln!(
                            "ERROR: terjadi kesalahan saat membaca file json! pada file: {:#?} {:#?}",
                            json_path, e
                        );
                    }
                };
                return Ok(());
            } else if *update {
                match write_json_data(parse_json()?) {
                    Ok(k) => {
                        println!("SUCCES: Updating board data Done! on: {:#?}", json_path);
                        return Ok(k);
                    }
                    Err(_) => {}
                }
                return Ok(());
            }
        }
        None => eprintln!("ERROR: No Arguments"),
    }
    Ok(())
}
