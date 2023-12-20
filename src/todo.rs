use regex::Regex;
use std::path::PathBuf;

use crate::tools::directory;

// testing with time crates
use chrono::prelude::*;

fn get_date() -> String {
    let date = Local::now();
    date.to_string()
}

#[derive(Debug)]
pub struct Todo {
    pub is_completed: bool,
    pub priority: char,
    pub creation_date: String,
    pub title: String,
}
impl Todo {
    pub fn new() -> Todo {
        Todo {
            is_completed: false,
            priority: 'Z',
            creation_date: get_date(),
            title: String::from(""),
        }
    }
}
impl From<Todo> for String {
    fn from(todoitem: Todo) -> String {
        let mut result_string = String::new();
        result_string.push_str(if todoitem.is_completed {
            "- [x]"
        } else {
            "- [ ]"
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
}
impl TodoHandler {
    pub fn init(config: &directory::Config) -> TodoHandler {
        match directory::ensure_todofile(
            config.path.todo_path.clone().into(),
            config.path.todo_filename.clone().into(),
        ) {
            Ok(result) => {
                return TodoHandler {
                    path: PathBuf::from(result.0),
                    filename: PathBuf::from(result.1),
                }
            }
            Err(_e) => panic!("could not verify path for a file"),
        }
    }

    pub fn add(&self, input_content: Vec<String>) {
        println!("adding task: {}", input_content.join(" "));
        // testing, create default todoitem
        let mut todoitem: Todo = Todo::new();
        todoitem.title = input_content.join(" ");
        //let _ = files::export_line(std::path::Path::new(LISTPATH), title.join(" "));
        // write item into file
        let _ = directory::export_line(&self.path.join(&self.filename), String::from(todoitem));
    }

    pub fn done(&self, indicies: Vec<String>) {
        println!("{:?}", indicies);
    }

    pub fn remove(&self, id: Vec<String>) {
        let mut sanitized_ids: Vec<u32> = Vec::new();
        for item in id {
            match item.parse::<u32>() {
                Ok(number) => sanitized_ids.push(number),
                Err(_) => continue,
            }
        }
        println!("removing task with IDs: {:?}", sanitized_ids);
    }

    pub fn list(&self, todos: Vec<Todo>) {
        for todoitem in todos {
            if todoitem.is_completed {
                println!("[X] {}", todoitem.title);
            } else {
                println!("[ ] {}", todoitem.title);
            }
        }
    }
}

// TODO: not respecting config yet
pub struct TodoParser {
    pub completion_style: Regex, // check if line is valid
    pub completion_done: Regex,  // check if valid line is done
    pub todo_list: Vec<Todo>,
}
impl TodoParser {
    pub fn new(config: &directory::Config) -> TodoParser {
        let default_md: TodoParser = TodoParser {
            completion_style: Regex::new(r"^\s*-\s*\[[ xX]\]").unwrap(),
            completion_done: Regex::new(r"^\s*-\s*\[[^\s]\]").unwrap(),
            todo_list: Vec::new(),
        };

        // Markdown style
        if config.format.checkbox_style == "md" {
            println!("TodoParser::new(): md format found");
            return default_md;
        }
        // Logseq style
        else if config.format.checkbox_style == "logseq" {
            println!("TodoParser::new(): Logseq format found");
            return TodoParser {
                completion_style: Regex::new(r"^\s*-\s*[A-Z]{4}").unwrap(),
                completion_done: Regex::new(r"^\s*-\s*DONE\s").unwrap(),
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
        for line in lines.iter() {
            if self.completion_style.is_match(&line) {
                let mut item = Todo::new();
                item.is_completed = self.completion_done.is_match(&line);

                // item.title = line[3..].to_string();

                let completion_part = match self.completion_style.captures(&line) {
                    Some(part) => part[0].to_string(),
                    _ => "".to_string(),
                };
                item.title = line[completion_part.len() + 1..].to_string();

                self.todo_list.push(item);
            }
        }
    }
}

pub fn print_info(arg: i32) {
    match arg {
        1 => println!("printing out all your tasks..."),
        2 => println!("adding task"),
        3 => println!("removing task"),
        4 => println!("illegal operation"),
        5 => println!("no help page implemented yet"),
        i32::MIN..=0_i32 | 5_i32..=i32::MAX => println!("your task was not recognised"),
    }
}
