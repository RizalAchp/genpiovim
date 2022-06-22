use dirs;
use std::{self, fs, path::PathBuf};

pub const COMMAND_UPDATE: [&str; 2] = ["boards", "--json-output"];
pub const MAKEFILE: &str = "\nall:\tpio -f -c vim run\n\nupload:\tpio -f -c vim run --target upload\n\nclean:\tpio -f -c vim run --target clean\n\nprogram:\tpio -f -c vim run --target program\n\nuploadfs:\tpio -f -c vim run --target uploadfs\n\nupdate:\tpio -f -c vim update\n\nmonitor:\tpio device monitor\n";
pub const MAIN_CPP: &str = "\n#include <Arduino.h>\n\nvoid setup(){}\n\nvoid loop(){}\n";
pub const MAKEFILE_PATH: &str = "genpio/Makefile.bak";

pub fn get_file_config<'a>(path: &'a str) -> Option<PathBuf> {
    Some(dirs::config_dir()?.join(path))
}

pub fn get_exist_makefile_dir() -> Option<PathBuf> {
    Some(dirs::config_dir()?.join(MAKEFILE_PATH))
}

pub fn get_board_list_path() -> PathBuf {
    get_file_config("genpio/boardlist.json").unwrap_or(PathBuf::from("./boardlist.json"))
}

pub fn check_and_create_config() -> Option<PathBuf> {
    let gnpio = dirs::config_dir()?.join("genpio");
    if gnpio.exists() {
        Some(gnpio.join("boardlist.json"))
    } else {
        fs::create_dir(&gnpio).unwrap();
        Some(gnpio.join("boardlist.json"))
    }
}

pub fn get_platformio_env() -> PathBuf {
    if cfg!(windows) {
        dirs::home_dir().unwrap().join(".platformio/penv/Scripts")
    } else {
        dirs::home_dir().unwrap().join(".platformio/penv/bin")
    }
}

pub fn gen_make_file(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let currdir = std::env::current_dir()
        .unwrap_or(PathBuf::from("./"))
        .join(name + "/Makefile");
    match get_exist_makefile_dir() {
        Some(matchs) => {
            if matchs.exists() {
                match std::fs::copy(matchs, currdir) {
                    Ok(_) => {}
                    Err(_) => {}
                }
            } else {
                match std::fs::write(currdir, MAKEFILE) {
                    Ok(_) => {}
                    Err(_) => {}
                };
            }
        }
        None => {
            std::fs::write(currdir, MAKEFILE).unwrap();
        }
    };
    Ok(())
}

pub fn gen_main_cpp(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let currdir = std::env::current_dir().unwrap_or(PathBuf::from("./"));
    std::fs::write(currdir.join(name + "/src/main.cpp"), MAIN_CPP).unwrap();
    Ok(())
}
