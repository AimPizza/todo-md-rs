use regex::Regex;
use std::path::PathBuf;

// testing with time crates
use chrono::prelude::*;
use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use toml;

enum Error {
    UserRefused,
}

pub fn readinput(prompt: &str) -> io::Result<String> {
    let mut buffer = String::new();
    print!("{prompt}");
    io::stdout().flush()?;
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    // remove trailing newline
    let input = buffer.trim().to_string();

    Ok(input)
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConfigFile {
    pub format: String, // accepted: "md", "logseq"
    pub todo_path: String,
    pub todo_filename: String,
}

// returns tuple with (directory, filename)
pub fn ensure_todofile(
    todo_path: PathBuf,
    todo_filename: PathBuf,
) -> Result<(PathBuf, PathBuf), Error> {
    let unified_path: PathBuf = todo_path.join(todo_filename.clone());

    while !check_for_dir(unified_path.clone()) {
        match readinput(format!("create {} ? (y/n)", unified_path.display()).as_str())
            .expect("input failed")
            .as_str()
        {
            "y" => create_path(unified_path.clone()),
            _ => return Err(Error::UserRefused),
        }
    }

    Ok((todo_path, todo_filename))
}

// should check configuration and if that is invalid, assign arguments as default paths
pub fn get_config() -> ConfigFile {
    // defaults for content of config
    let mut configuration = ConfigFile {
        path: TodoPath {
            todo_path: dirs::home_dir().unwrap().to_string_lossy().to_string(),
            todo_filename: "todo.md".to_string(),
        },
        format: "md".to_string()    
    };
    // path to the configuration file
    let path = dirs::config_dir()
        .expect("config dir error")
        .join(PathBuf::from("todo-md-rs"))
        .join(PathBuf::from("config.toml"));

    // check if configuration can be found
    if check_for_dir(path.clone()) {
        // config found
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
        // config not found
        match readinput("create base configuraton file? (y/n): ")
            .expect("input failed")
            .as_str()
        {
            // create file and write defaults into it
            "y" => {
                create_path(path.clone());

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

pub fn remove_lines(filepath: &PathBuf, indices: Vec<u32>) {
    let original_content = fs::read_to_string(filepath).expect("file not found");
    let indices_zero_indexed: Vec<u32> = indices
        .iter()
        .map(|&nr| if nr < 1 { nr } else { nr - 1 })
        .collect();

    let lines: Vec<String> = original_content
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            // line number is 1-indexed but we'll remove by counting 0-indexed
            if !indices_zero_indexed.contains(&(i as u32)) {
                Some(line.to_string())
            } else {
                None
            }
        })
        .collect();

    println!("remove_lines: {:?}", indices);

    let mut file = File::create(filepath).unwrap();
    for line in lines {
        let _ = writeln!(file, "{}", line);
    }
}

pub fn export_line(filepath: &PathBuf, line: String) -> std::io::Result<()> {
    let mut file = fs::OpenOptions::new().append(true).open(filepath)?;
    file.write_all(format!("{line}\n").as_bytes())?;

    let _newone = fs::read_to_string(filepath).unwrap();

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
fn get_date() -> String {
    let date = Local::now();
    date.to_string()
}

#[derive(Debug, Clone)]
pub struct TodoItem {
    pub id: u32,
    pub line: u32, // should probably be a tuple of start/end (see logseq task block would remain junk)
    pub is_completed: bool,
    pub priority: char,
    pub creation_date: String,
    pub title: String,
}
impl TodoItem {
    pub fn new() -> TodoItem {
        TodoItem {
            id: 0,
            line: 0,
            is_completed: false,
            priority: 'Z',
            creation_date: get_date(),
            title: String::from(""),
        }
    }

    pub fn get_string(todoitem: TodoItem, parser: Config) -> String {
        let mut result_string = String::new();
        result_string.push_str(if todoitem.is_completed {
            &parser.example_done
        } else {
            &parser.example_todo
        });

        result_string.push(' ');
        result_string.push(todoitem.priority);
        result_string.push(' ');
        result_string.push_str(&todoitem.creation_date);
        result_string.push(' ');
        result_string.push_str(&todoitem.title);

        result_string
    }
}

// TODO: not respecting config yet
pub struct Config {
    // from TodoParser
    pub completion_style: Regex, // check if line is valid
    pub completion_done: Regex,  // check if valid line is done
    pub example_todo: String,
    pub example_done: String,
    pub todo_list: Vec<TodoItem>,
    // from TodoHandler
    pub path: PathBuf,
    pub filename: PathBuf,
    pub complete_path: PathBuf,
}
impl Config {

    //
    // converted
    //

    //
    // from TodoHandler
    //
    pub fn init(config: &ConfigFile) -> TodoHandler {
        match ensure_todofile(
            config.path.todo_path.clone().into(),
            config.path.todo_filename.clone().into(),
        ) {
            Ok(result) => {
                let path = PathBuf::from(result.0);
                let filename = PathBuf::from(result.1);
                let complete_path: PathBuf = path.join(filename.clone());
                return TodoHandler {
                    path,
                    filename,
                    complete_path,
                };
            }
            Err(_e) => panic!("could not verify path for a file"),
        }
    }

    pub fn add(&self, input_content: Vec<String>, parser: Config) {
        let mut todoitem: TodoItem = TodoItem::new();

        let title = input_content.join(" ").trim().to_string();
        println!("adding task: {}", title);
        todoitem.title = title;
        // write item into file
        let _ = export_line(
            &self.path.join(&self.filename),
            TodoItem::get_string(todoitem, parser),
        );
    }

    pub fn done(&self, indicies: Vec<String>, mut parser: Config) {

        // keep track of task_ids to then mark as done
        let mut to_check_off: Vec<usize> = Vec::new();

        // iterate arguments and sanitize
        for item in indicies {
            match item.parse::<usize>() {
                Ok(val) => {
                    println!("{item} is indeed a number");
                    if val < parser.todo_list.len() {
                        println!("{} is now done, yay", parser.todo_list[val].title);
                        to_check_off.push(val);
                    } else {
                        println!("DEBUG: len is: {}", parser.todo_list.len());
                        println!("DEBUG: todo is: {:?}", parser.todo_list);
                        println!("argument {val} is out of range");
                    }
                },
                // just skip invalid arguments
                Err(_) => println!("{item} is not a valid number"),
            };        
        }

        // act upon sanitized arguments
        for pos in to_check_off {
            // TODO rewrite file in that position
        }
    }

    // TODO notice that we're currently removing by line number not by task id
    pub fn remove(&self, id: Vec<String>, path: PathBuf, parser: Config) {
        // sanitize the given arguments
        let mut sanitized_ids: Vec<u32> = Vec::new();
        for item in id {
            match item.parse::<u32>() {
                Ok(number) => sanitized_ids.push(number),
                Err(_) => continue,
            }
        }
        println!("removing task with IDs: {:?}", sanitized_ids);

        // delete those tasks
        let mut lines_to_rm: Vec<u32> = Vec::new();
        for item in parser.todo_list {
            if sanitized_ids.contains(&item.id) {
                lines_to_rm.push(item.line)
            }
        }
        remove_lines(&path, lines_to_rm);
    }

    pub fn list(&self, todos: Vec<TodoItem>) {
        for todoitem in todos {
            if todoitem.is_completed {
                println!("[X] {} {}", todoitem.id, todoitem.title);
            } else {
                println!("[ ] {} {}", todoitem.id, todoitem.title);
            }
        }
    }

    //
    // from TodoParser
    //
    pub fn new(config: &ConfigFile) -> Config {
        let default_md: Config = Config {
            completion_style: Regex::new(r"^\s*-\s*\[[ xX]\]").unwrap(),
            completion_done: Regex::new(r"^\s*-\s*\[[^\s]\]").unwrap(),
            example_todo: String::from("- [ ]"),
            example_done: String::from("- [X]"),
            todo_list: Vec::new(),
        };

        // Markdown style
        if config.format == "md" {
            return default_md;
        }
        // Logseq style
        else if config.format == "logseq" {
            return Config {
                completion_style: Regex::new(r"^\s*-\s*[A-Z]{4}").unwrap(),
                completion_done: Regex::new(r"^\s*-\s*DONE\s").unwrap(),
                example_todo: String::from("- TODO"),
                example_done: String::from("- DONE"),
                todo_list: default_md.todo_list,
            };
        }
        // default to Markdown
        else {
            println!("Be careful: your config contains an invalid format! Defaulting to \"md\".");
            return default_md;
        }
    }
    // TODO: complete this method to include all the other fields of Todo
    pub fn strings_to_todo(&mut self, lines: Vec<String>) {
        let mut item_list: Vec<TodoItem> = Vec::new();

        for (linecount, line) in lines.iter().enumerate() {
            // new task detected
            if self.completion_style.is_match(&line) {
                let mut item = TodoItem::new();

                item.id = item_list.len() as u32 + 1;

                item.line = linecount as u32 + 1;

                item.is_completed = self.completion_done.is_match(&line);

                // item.title = line[3..].to_string();

                let completion_part = match self.completion_style.captures(&line) {
                    Some(part) => part[0].to_string(),
                    _ => "".to_string(),
                };
                item.title = line[completion_part.len() + 1..].to_string();

                item_list.push(item.clone());
            }
        }

        self.todo_list = item_list.clone();
    }
}

pub enum Info {
    Help,
}
pub fn print_info(arg: Info) {
    match arg {
        Info::Help => println!("implement a help page"),
    }
}
