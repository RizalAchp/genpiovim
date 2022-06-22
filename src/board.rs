use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
use std::fmt;
use std::process::{Output, Stdio};
use std::{env, error::Error, fs::read_to_string, io, path, process::Command};

use crate::konstan::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoard {
    debug: Option<Value>,
    connectivity: Option<Vec<String>>,
    fcpu: u64,
    frameworks: Vec<String>,
    id: String,
    mcu: String,
    name: String,
    platform: String,
    ram: u64,
    rom: u64,
    url: String,
    vendor: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ListDataBoard {
    list_board: Vec<DataBoard>,
}

impl fmt::Display for ListDataBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\n================================[LIST BOARD]===============================\n",
        )?;
        for (i, board) in self.list_board.iter().enumerate() {
            write!(
                f,
                "\n================================{}===============================\n",
                i
            )?;
            write!(f, "     Board        : {}\n", board.name)?;
            write!(f, "     ID           : {}\n", board.name)?;
            write!(f, "     MCU          : {}\n", board.mcu)?;
            write!(f, "     Platform     : {}\n", board.platform)?;
            write!(f, "     Frameworks   : {}\n", &board.frameworks.join(", "))?;
            match &board.connectivity {
                Some(s) => {
                    write!(f, "     Connectivity: {}\n", &s.join(" "))?;
                }
                None => {
                    write!(f, "     Connectivity: None\n")?;
                }
            }
            write!(
                f,
                "\n================================{}===============================\n",
                i
            )?;
        }
        write!(
            f,
            "\n================================[LIST BOARD]===============================\n",
        )
    }
}

pub fn create_pio_project<'a, P: AsRef<path::Path>>(name: P, boardid: &'a str) -> io::Result<()> {
    match env::current_dir() {
        Ok(curr) => {
            fs::create_dir(curr.join(name))?;

            match spawn_command_pio(vec![
                "project",
                "init",
                "--project-dir",
                curr.to_str().unwrap(),
                "--ide",
                "vim",
                "--board",
                boardid,
            ]) {
                Ok(o) => {
                    println!("{}", o.status)
                }
                Err(_) => {}
            };
        }
        Err(_) => {}
    };
    Ok(())
}
pub fn initialize_pio_project<'a>(boardid: &'a str) -> io::Result<()> {
    Ok(
        match spawn_command_pio(vec!["project", "init", "--ide", "vim", "--board", boardid]) {
            Ok(o) => {
                println!("status: {}", o.status);
                match gen_make_file(String::from(".")) {
                    Ok(_) => {}
                    Err(_) => {}
                }
                match gen_main_cpp(String::from(".")) {
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
            Err(_) => {}
        },
    )
}

pub fn spawn_command_pio<'a>(args: Vec<&'a str>) -> io::Result<Output> {
    let platformio_dir = get_platformio_env();
    Command::new("pio")
        .env("PATH", platformio_dir)
        .args(&args)
        .stdout(Stdio::piped())
        .output()
    // .expect("cannot spawn comand")
}

pub fn parse_json() -> Result<ListDataBoard, Box<dyn Error>> {
    let json_path = check_and_create_config().unwrap_or(path::PathBuf::from("./boardlist.json"));
    if json_path.exists() {
        println!("json from file");
        Ok(from_str(&read_to_string(json_path)?)?)
    } else {
        println!("spawning command io");
        let out = spawn_command_pio(COMMAND_UPDATE.to_vec())?;
        Ok(serde_json::from_str(&String::from_utf8(out.stdout)?)?)
    }
}

use std::fs;
pub fn write_json_data<'a>(data: ListDataBoard) -> Result<(), Box<dyn Error>> {
    Ok(fs::write(
        check_and_create_config().unwrap_or(path::PathBuf::from("./boardlist.json")),
        serde_json::to_vec(&data)?,
    )?)
}
