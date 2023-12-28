use regex::Regex;
use std::path::PathBuf;

use crate::tools::directory;

// testing with time crates
use chrono::prelude::*;

fn get_date() -> String {
    let date = Local::now();
    date.to_string()
}

#[derive(Debug, Clone)]
pub struct Todo {
    pub id: u32,
    pub line: u32, // should probably be a tuple of start/end (see logseq task block would remain junk)
    pub is_completed: bool,
    pub priority: char,
    pub creation_date: String,
    pub title: String,
}
impl Todo {
    pub fn new() -> Todo {
        Todo {
            id: 0,
            line: 0,
            is_completed: false,
            priority: 'Z',
            creation_date: get_date(),
            title: String::from(""),
        }
    }

    pub fn get_string(todoitem: Todo, parser: TodoParser) -> String {
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

#[derive(Debug)]
pub struct TodoHandler {
    pub path: PathBuf,
    pub filename: PathBuf,
    pub complete_path: PathBuf,
}
impl TodoHandler {
    pub fn init(config: &directory::Config) -> TodoHandler {
        match directory::ensure_todofile(
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

    pub fn add(&self, input_content: Vec<String>, parser: TodoParser) {
        let mut todoitem: Todo = Todo::new();

        let title = input_content.join(" ").trim().to_string();
        println!("adding task: {}", title);
        todoitem.title = title;
        // write item into file
        let _ = directory::export_line(
            &self.path.join(&self.filename),
            Todo::get_string(todoitem, parser),
        );
    }

    pub fn done(&self, indicies: Vec<String>) {
        println!("{:?}", indicies);
    }

    // TODO notice that we're currently removing by line number not by task id
    pub fn remove(&self, id: Vec<String>, path: PathBuf, parser: TodoParser) {
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
        directory::remove_lines(&path, lines_to_rm);
    }

    pub fn list(&self, todos: Vec<Todo>) {
        for todoitem in todos {
            if todoitem.is_completed {
                println!("[X] {} {}", todoitem.id, todoitem.title);
            } else {
                println!("[ ] {} {}", todoitem.id, todoitem.title);
            }
        }
    }
}

// TODO: not respecting config yet
pub struct TodoParser {
    pub completion_style: Regex, // check if line is valid
    pub completion_done: Regex,  // check if valid line is done
    pub example_todo: String,
    pub example_done: String,
    pub todo_list: Vec<Todo>,
}
impl TodoParser {
    pub fn new(config: &directory::Config) -> TodoParser {
        let default_md: TodoParser = TodoParser {
            completion_style: Regex::new(r"^\s*-\s*\[[ xX]\]").unwrap(),
            completion_done: Regex::new(r"^\s*-\s*\[[^\s]\]").unwrap(),
            example_todo: String::from("- [ ]"),
            example_done: String::from("- [X]"),
            todo_list: Vec::new(),
        };

        // Markdown style
        if config.format.checkbox_style == "md" {
            return default_md;
        }
        // Logseq style
        else if config.format.checkbox_style == "logseq" {
            return TodoParser {
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
        let mut item_list: Vec<Todo> = Vec::new();

        for (linecount, line) in lines.iter().enumerate() {
            // new task detected
            if self.completion_style.is_match(&line) {
                let mut item = Todo::new();

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
