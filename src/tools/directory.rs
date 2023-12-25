use crate::tools::directory;
use crate::tools::{error::DirErrors, input};
use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use toml;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub path: TodoPath,
    pub format: TodoFormatting,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoPath {
    pub todo_path: String,
    pub todo_filename: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoFormatting {
    pub checkbox_style: String,
}

// returns tuple with (directory, filename)
pub fn ensure_todofile(
    todo_path: PathBuf,
    todo_filename: PathBuf,
) -> Result<(PathBuf, PathBuf), DirErrors> {
    let unified_path: PathBuf = todo_path.join(todo_filename.clone());

    while !directory::check_for_dir(unified_path.clone()) {
        match input::readinput(format!("create {} ? (y/n)", unified_path.display()).as_str())
            .expect("input failed")
            .as_str()
        {
            "y" => directory::create_path(unified_path.clone()),
            _ => return Err(DirErrors::UserRefused),
        }
    }

    Ok((todo_path, todo_filename))
}

// should check configuration and if that is invalid, assign arguments as default paths
pub fn get_config() -> Config {
    // defaults for content of config
    let mut configuration = Config {
        path: TodoPath {
            todo_path: dirs::home_dir().unwrap().to_string_lossy().to_string(),
            todo_filename: "todo.md".to_string(),
        },
        format: TodoFormatting {
            checkbox_style: "md".to_string(),
        },
    };
    // path to the configuration file
    let path = dirs::config_dir()
        .expect("config dir error")
        .join(PathBuf::from("todo-md-rs"))
        .join(PathBuf::from("config.toml"));
    // check if configuration can be found
    if directory::check_for_dir(path.clone()) {
        let contents: String = fs::read_to_string(path).unwrap();
        configuration = match toml::from_str(&contents) {
            Ok(content) => content,
            Err(_e) => {
                println!("error in your config!");

                println!("using the following defaults: {configuration:?}");
                configuration
            }
        };
    } else {
        match input::readinput("create base configuraton file? (y/n): ")
            .expect("input failed")
            .as_str()
        {
            // create file and write defaults into it
            "y" => {
                directory::create_path(path.clone());

                let _ = export_line(&path, toml::to_string(&configuration).unwrap());
            }
            _ => {
                println!("using temporary path"); // configuration should not have changed from the initialization
                let toml = toml::to_string(&configuration).unwrap();
                println!("{toml:#?}");
            }
        }
    }

    configuration
}

pub fn read_lines(filepath: &PathBuf) -> Vec<String> {
    // let parser = crate::todo::TodoParser::new();
    let mut lines: Vec<String> = Vec::new();
    let file = fs::read_to_string(filepath).expect("file not found");
    for line in file.lines() {
        lines.push(line.to_string())
    }
    // for line in file.lines() {
    //     parser
    //         .todo_list
    //         .push(parser.string_to_todo(line.to_string()));
    //     println!("{line}");
    // }
    lines
}

pub fn export_line(filepath: &PathBuf, line: String) -> std::io::Result<()> {
    let mut file = fs::OpenOptions::new().append(true).open(filepath)?;
    file.write_all(format!("{line}\n").as_bytes())?;

    let newone = fs::read_to_string(filepath).unwrap();
    dbg!(newone);

    Ok(())
}

pub fn check_for_dir(path: PathBuf) -> bool {
    match path.try_exists() {
        Ok(true) => {
            // just return true?
            return true;
        }
        Ok(false) => {
            // just return false?
            return false;
        }
        Err(e) => {
            // error
            println!("attention: {e:?}");
            panic!("unhandled error occured: tried checking for dir");
        }
    }
}

pub fn create_path(path: PathBuf) {
    match check_for_dir(path.clone()) {
        false => {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            println!("creating: {path:?}");
            let _ = File::create(path.clone()).expect("creating file failed");
        }
        true => {
            println!("file already exists");
        }
    }
}
