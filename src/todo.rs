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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoPath {
    pub todo_path: PathBuf,
    pub todo_filename: PathBuf,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoFormatting {
    pub checkbox_style: String, // accepted: "md", "logseq"
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConfigFile {
    pub path: TodoPath,
    pub format: TodoFormatting,
}
impl ConfigFile {
    pub fn init() -> ConfigFile {
        // BEGIN GET_CONFIG removeme

        // should check configuration and if that is invalid, assign arguments as default paths
        // defaults for content of config
        let mut conf = ConfigFile {
            format: TodoFormatting {
                checkbox_style: "md".to_string(),
            },
            path: TodoPath {
                todo_path: dirs::home_dir().unwrap(),
                todo_filename: "todo.md".into(),
            },
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
            conf = match toml::from_str(&contents) {
                Ok(content) => content,
                Err(_e) => {
                    println!("error in your config!");

                    println!("using the following defaults: {conf:?}");
                    conf
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

                    let _ = export_line(&path, toml::to_string(&conf).unwrap());
                }
                _ => {
                    println!("using temporary path"); // configuration should not have changed from the initialization
                    let toml = toml::to_string(&conf).unwrap();
                    println!("{toml:#?}");
                }
            }
        }
        // END GET_CONFIG removeme

        let unified_path: PathBuf = conf.path.todo_path.join(conf.path.todo_filename.clone());

        while !check_for_dir(unified_path.clone()) {
            match readinput(format!("create {} ? (y/n)", unified_path.display()).as_str())
                .expect("input failed")
                .as_str()
            {
                "y" => create_path(unified_path.clone()),
                _ => panic!("user refused"),
            }
        }
        return conf;
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

#[derive(Debug, Clone)]
pub struct TodoItem {
    pub id: u32,
    pub line: u32, // should probably be a tuple of start/end (see logseq task block would remain junk)
    pub is_completed: bool,
    pub title: String,
    pub date_due: Option<NaiveDate>,
}
impl TodoItem {
    pub fn new() -> TodoItem {
        TodoItem {
            id: 0,
            line: 0,
            is_completed: false,
            title: String::from(""),
            // date_due: Some(Local::now().date_naive()),
            date_due: None,
        }
    }

    pub fn get_string(todoitem: &TodoItem, conf_todo: &TodoConfig) -> String {
        let mut result_string = String::new();
        result_string.push_str(if todoitem.is_completed {
            &conf_todo.example_done
        } else {
            &conf_todo.example_todo
        });

        result_string.push(' ');
        result_string.push_str(&todoitem.title);
        result_string.push(' ');
        match &todoitem.date_due {
            Some(date) => result_string.push_str(date.to_string().as_str()), // TODO: can this be written nicer?
            None => result_string.push_str(""),
        };

        result_string
    }
}

// TODO: not respecting config yet
pub struct TodoConfig {
    // from TodoParser
    pub completion_style: Regex, // check if line is valid
    pub completion_done: Regex,  // check if valid line is done
    pub date_format: Regex,
    pub example_todo: String,
    pub example_done: String,
}
impl TodoConfig {
    //
    // converted
    //

    pub fn new(conf_file: &ConfigFile) -> TodoConfig {
        let default_md: TodoConfig = TodoConfig {
            completion_style: Regex::new(r"^\s*-\s*\[[ xX]\]").unwrap(),
            completion_done: Regex::new(r"^\s*-\s*\[[^\s]\]").unwrap(),
            date_format: Regex::new(r"(?:^|\s)(\d{4}-\d{2}-\d{2})(?:\s|$)").unwrap(),
            example_todo: String::from("- [ ]"),
            example_done: String::from("- [X]"),
        };

        // Markdown style
        if conf_file.format.checkbox_style == "md" {
            return default_md;
        }
        // Logseq style
        else if conf_file.format.checkbox_style == "logseq" {
            return TodoConfig {
                completion_style: Regex::new(r"^\s*-\s*[A-Z]{4}").unwrap(),
                completion_done: Regex::new(r"^\s*-\s*DONE\s").unwrap(),
                date_format: default_md.date_format,
                example_todo: String::from("- TODO"),
                example_done: String::from("- DONE"),
            };
        }
        // default to Markdown
        else {
            println!("Be careful: your config contains an invalid format! Defaulting to \"md\".");
            return default_md;
        }
    }

    //
    // from TodoHandler
    //

    //
    // from TodoParser
    //
}

pub enum Info {
    Help,
}
pub fn print_info(arg: Info) {
    match arg {
        Info::Help => println!("implement a help page"),
    }
}

// previous "directory" methods
pub fn remove_lines(filepath: &PathBuf, line_nr: Vec<u32>) {
    let original_content = fs::read_to_string(filepath).expect("file not found");
    let indices_zero_indexed: Vec<u32> = line_nr
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

    println!("remove_lines: {:?}", line_nr);

    let mut file = File::create(filepath).unwrap();
    for line in lines {
        let _ = writeln!(file, "{}", line);
    }
}

pub fn change_line(filepath: &PathBuf, line_nr: u32, line_content: String) {
    let original_content = fs::read_to_string(filepath).expect("file not found"); // TODO: is it a problem that we read to string at every call?
    let line_nr_zero_indexed: u32 = if line_nr < 1 { line_nr } else { line_nr - 1 };

    let lines: Vec<String> = original_content
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            // catch the line to be changed
            if i != line_nr_zero_indexed as usize {
                Some(line.to_string()) // existed before
            } else {
                Some(line_content.clone()) // was changed
            }
        })
        .collect();

    let mut file = File::create(filepath).unwrap();
    for line in lines {
        let _ = writeln!(file, "{}", line);
    }
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

pub fn export_line(filepath: &PathBuf, line_content: String) -> std::io::Result<()> {
    let mut file = fs::OpenOptions::new().append(true).open(filepath)?;
    file.write_all(format!("{line_content}\n").as_bytes())?;

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

pub struct Todo {
    todo_list: Vec<TodoItem>,
}
impl Todo {
    pub fn new() -> Todo {
        Todo {
            todo_list: Vec::new(),
        }
    }

    pub fn add(&self, input_content: Vec<String>, conf_todo: &TodoConfig, conf_file: &ConfigFile) {
        let mut todoitem: TodoItem = TodoItem::new();

        let title = input_content.join(" ").trim().to_string();
        println!("adding task: {}", title);
        todoitem.title = title;
        // write item into file
        let _ = export_line(
            &conf_file
                .path
                .todo_path
                .join(conf_file.path.todo_filename.clone()),
            TodoItem::get_string(&todoitem, conf_todo),
        );
    }

    pub fn done(&mut self, indicies: Vec<String>, conf_file: &ConfigFile, conf_todo: &TodoConfig) {
        // keep track of task_ids to then mark as done
        let mut to_check_off: Vec<usize> = Vec::new();

        // iterate arguments and sanitize
        for item in indicies {
            match item.parse::<usize>() {
                Ok(val) => {
                    println!("{item} is indeed a number");
                    if val <= self.todo_list.len() && val > 0 {
                        println!("{} is now done, yay", self.todo_list[val - 1].title); // TODO: should we really go by index or search through the vector?
                        to_check_off.push(val - 1); // valid, can remove safely
                    } else {
                        println!("DEBUG: len is: {}", self.todo_list.len());
                        println!("DEBUG: todo is: {:#?}", self.todo_list);
                        println!("argument {val} is out of range");
                    }
                }
                // just skip invalid arguments
                Err(_) => println!("{item} is not a valid number"),
            };
        }

        // act upon sanitized arguments
        for pos in to_check_off {
            self.todo_list[pos].is_completed = true;
            change_line(
                &conf_file
                    .path
                    .todo_path
                    .join(conf_file.path.todo_filename.clone()),
                self.todo_list[pos].line,
                TodoItem::get_string(&self.todo_list[pos], conf_todo),
            )
        }

        // grab the line from todo vector
        // todoToStr()
        // write into file
    }

    // TODO notice that we're currently removing by line number not by task id
    pub fn remove(&self, id: Vec<String>, path: PathBuf) {
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
        for item in &self.todo_list {
            if sanitized_ids.contains(&item.id) {
                lines_to_rm.push(item.line)
            }
        }
        remove_lines(&path, lines_to_rm);
    }

    // TODO: handle the date if it exists
    pub fn list(&self) {
        for it in &self.todo_list {
            if it.is_completed {
                println!("[X] {} {} {:?}", it.id, it.title, it.date_due);
            } else {
                println!("[ ] {} {} {:?}", it.id, it.title, it.date_due);
            }
        }
    }

    // TODO: complete this method to include all the other fields of Todo
    pub fn strings_to_todo(&mut self, lines: Vec<String>, conf_todo: &TodoConfig) {
        let mut item_list: Vec<TodoItem> = Vec::new();

        for (linecount, line) in lines.iter().enumerate() {
            // new task detected
            // for now, the order is so that the date (and maybe at some point tags) will be removed before feeding the rest of the line into the string
            if conf_todo.completion_style.is_match(&line) {
                let mut item = TodoItem::new();
                let mut l: String = line.to_string();

                item.id = item_list.len() as u32 + 1;
                item.line = linecount as u32 + 1;

                item.is_completed = if conf_todo.completion_done.is_match(&line) {
                    l = conf_todo.completion_done.replace(&l, "").to_string(); // remove the checkbox
                    true
                } else {
                    l = conf_todo.completion_style.replace(&l, "").to_string(); // remove the checkbox
                    false
                };

                item.date_due = match conf_todo.date_format.captures(&line) {
                    Some(date) => {
                        l = conf_todo.date_format.replace(&l, " ").to_string(); // remove the first date and assume it's the due_date
                        Some(date[0].parse::<NaiveDate>().unwrap())
                    }
                    _ => None,
                };

                item.title = l.trim().to_string(); // take what's left for the title

                println!("{:#?}", item);

                item_list.push(item.clone());
            }
        }

        self.todo_list = item_list.clone();
    }
}
