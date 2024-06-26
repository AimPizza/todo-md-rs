use chrono::prelude::*;
use colored::{ColoredString, Colorize};
use dirs;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
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
    /// Check config. If config is invalid, either use defaults or let user configure manually.
    pub fn init() -> ConfigFile {
        // setting some defaults
        let mut conf = ConfigFile {
            format: TodoFormatting {
                checkbox_style: "md".to_string(),
            },
            path: TodoPath {
                todo_path: dirs::home_dir().unwrap(),
                todo_filename: "todo.md".into(),
            },
        };
        let path_to_conf = dirs::config_dir()
            .expect("config dir error")
            .join(PathBuf::from("todo-md-rs"))
            .join(PathBuf::from("config.toml"));

        if check_dir_exists(&path_to_conf) {
            let contents: String = fs::read_to_string(&path_to_conf).unwrap();
            conf = match toml::from_str(&contents) {
                Ok(content) => content,
                Err(_e) => {
                    println!("error in your config!");
                    println!("using the following defaults: {conf:#?}");
                    conf
                }
            };
        } else {
            match readinput("create base configuraton file? ( [y]es / [n]o ): ")
                .expect("input failed")
                .as_str()
            {
                "y" | "yes" => {
                    create_path(&path_to_conf);
                    let _ = export_line(&path_to_conf, toml::to_string(&conf).unwrap(), false);
                }
                _ => {
                    println!("using temporary path");
                    let toml = toml::to_string(&conf).unwrap();
                    println!("{toml:#?}");
                }
            }
        }

        // onto checking for the todo.md file itself

        let mut unified_path: PathBuf = conf.path.todo_path.join(&conf.path.todo_filename);

        while !check_dir_exists(&unified_path) {
            match readinput(&format!(
                "create {} ?\n( [y]es / [n]o / [c]hange ): ",
                unified_path.display()
            ))
            .expect("input failed")
            .as_str()
            {
                "y" | "yes" => create_path(&unified_path),
                "c" | "change" => {
                    println!("so you want to change?");
                    let new_path = PathBuf::from(
                        readinput("desired path to todo.md file (including filename): ").unwrap(),
                    );
                    conf.path.todo_filename = new_path.into_iter().last().unwrap().into();
                    conf.path.todo_path = new_path.parent().unwrap().into();
                    unified_path = conf.path.todo_path.join(conf.path.todo_filename.clone());
                    // create the todo.md file
                    create_path(&unified_path);
                    // update the config file
                    let _ = export_line(&path_to_conf, toml::to_string(&conf).unwrap(), false);
                }
                _ => panic!("user refused"),
            }
        }
        return conf;
    }
}

// creates file and parent path
pub fn create_path(path: &PathBuf) {
    match check_dir_exists(&path) {
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

/// Parses a Vec of raw Strings and writes them into the Todo container.
/// A task will only be recognised by the configured `TodoConfig.completion_style`
pub fn strings_to_todo(lines: Vec<String>, conf_todo: &TodoConfig) -> Vec<TodoItem> {
    let mut item_list: Vec<TodoItem> = Vec::new();

    for (linecount, line) in lines.iter().enumerate() {
        // - task detected by completion pattern -
        // for now, the order is so that the date (and maybe at some point tags) will be removed before
        // feeding the rest of the line into the string
        if conf_todo.completion_style.is_match(&line) {
            let mut item = TodoItem::new();
            let mut l_mut: String = line.to_string();

            // ID and LINE, duh
            item.id = item_list.len() + 1;
            item.line = linecount + 1;

            // COMPLETION
            item.is_completed = if conf_todo.completion_done.is_match(&line) {
                l_mut = conf_todo.completion_done.replace(&l_mut, "").into(); // remove the checkbox
                true
            } else {
                l_mut = conf_todo.completion_style.replace(&l_mut, "").into(); // remove the checkbox
                false
            };

            // DATE
            item.date_due = match conf_todo.date_format.captures(&line) {
                Some(date) => {
                    l_mut = conf_todo.date_format.replace(&l_mut, " ").into(); // remove the first date and assume it's the due_date
                    Some(date[0].parse::<NaiveDate>().unwrap())
                }
                _ => None,
            };

            // TAG
            let tag_re = Regex::new(r"#\w+").unwrap();
            tag_re
                .captures_iter(&line)
                .map(|captures| captures.get(0).unwrap().as_str())
                .for_each(|tag| item.tags.push(tag.into()));
            l_mut = tag_re.replace_all(&l_mut, "").into();

            // NAME
            let name_re = Regex::new(r"@\w+").unwrap();
            name_re
                .captures_iter(&line)
                .map(|captures| captures.get(0).unwrap().as_str())
                .for_each(|name| item.names.push(name.into()));
            l_mut = name_re.replace_all(&l_mut, "").into();

            // TITLE
            item.title = l_mut.trim().to_string(); // take what's left for the title

            item_list.push(item.clone());
        }
    }

    item_list
}

#[derive(Debug, Clone)]
pub struct TodoItem {
    pub id: usize,
    pub line: usize, // should probably be a tuple of start/end (see logseq task block would remain junk)
    pub is_completed: bool,
    pub title: String,
    pub date_due: Option<NaiveDate>,
    pub tags: Vec<String>,
    pub names: Vec<String>,
}
impl TodoItem {
    pub fn new() -> TodoItem {
        TodoItem {
            id: 0,
            line: 0,
            is_completed: false,
            title: String::from(""),
            date_due: None,
            tags: Vec::new(),
            names: Vec::new(),
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
        if let Some(date) = &todoitem.date_due {
            result_string.push_str(date.to_string().as_str());
        }
        if &todoitem.tags.len() > &0 {
            for tag in &todoitem.tags {
                result_string.push_str(&format!(" {tag}"))
            }
        }
        if &todoitem.names.len() > &0 {
            for name in &todoitem.names {
                result_string.push_str(&format!(" {name}"))
            }
        }

        result_string
    }
}

pub struct TodoConfig {
    pub completion_style: Regex, // check if line is valid
    pub completion_done: Regex,  // check if valid line is done
    pub date_format: Regex,
    pub example_todo: String,
    pub example_done: String,
}
impl TodoConfig {
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
            default_md
        }
        // Logseq style
        else if conf_file.format.checkbox_style == "logseq" {
            TodoConfig {
                completion_style: Regex::new(r"^\s*-\s*[A-Z]{4}").unwrap(),
                completion_done: Regex::new(r"^\s*-\s*DONE\s").unwrap(),
                date_format: default_md.date_format,
                example_todo: String::from("- TODO"),
                example_done: String::from("- DONE"),
            }
        }
        // default to Markdown
        else {
            println!("Be careful: your config contains an invalid format! Defaulting to \"md\".");
            return default_md;
        }
    }
}

pub fn remove_lines(filepath: &PathBuf, line_nr: Vec<usize>) {
    let original_content = fs::read_to_string(filepath).expect("file not found");
    let indices_zero_indexed: Vec<usize> = line_nr
        .iter()
        .map(|&nr| if nr < 1 { nr } else { nr - 1 })
        .collect();

    let lines: Vec<String> = original_content
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            // line number is 1-indexed but we'll remove by counting 0-indexed
            if !indices_zero_indexed.contains(&i) {
                Some(line.to_string())
            } else {
                None
            }
        })
        .collect();

    let mut file = File::create(filepath).unwrap();
    for line in lines {
        let _ = writeln!(file, "{}", line);
    }
}

pub fn change_line(filepath: &PathBuf, line_nr: usize, line_content: String) {
    // TODO: Avoid reading from the filesystem at every call. Batch all of it into one operation.
    let original_content = fs::read_to_string(filepath).expect("file not found");
    let line_nr_zero_indexed: usize = if line_nr < 1 { line_nr } else { line_nr - 1 };

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

    lines
}

/// `append: false` would overwrite the existing content
pub fn export_line(filepath: &PathBuf, line_content: String, append: bool) -> std::io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(append)
        .open(filepath)?;
    file.write_all(format!("{line_content}\n").as_bytes())?;

    let _newone = fs::read_to_string(filepath).unwrap();

    Ok(())
}

pub fn check_dir_exists(path: &PathBuf) -> bool {
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
// read some string from the command line as user input
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
    pub todo_list: Vec<TodoItem>,
}
impl Todo {
    pub fn new() -> Todo {
        Todo {
            todo_list: Vec::new(),
        }
    }

    /// input_content is of type `&Vec<String>` because that's what clap uses to capture all argument after the command
    pub fn add(&self, input_content: &Vec<String>, conf_todo: &TodoConfig, conf_file: &ConfigFile) {
        let mut item: TodoItem = strings_to_todo(
            vec![format!(
                "{} {}",
                conf_todo.example_todo,
                input_content.join(" ")
            )],
            &conf_todo,
        )
        .get(0)
        .expect("unable to convert your input into a task, sorry")
        .to_owned();
        item.id = self.todo_list.len() + 1;

        let _ = export_line(
            &conf_file
                .path
                .todo_path
                .join(conf_file.path.todo_filename.clone()),
            TodoItem::get_string(&item, conf_todo),
            true,
        );

        Self::list_single(&item, "adding");
    }

    pub fn done(&mut self, indicies: &Vec<usize>, conf_file: &ConfigFile, conf_todo: &TodoConfig) {
        // to keep track of valid ids
        let mut to_check_off: Vec<usize> = Vec::new();
        let mut to_uncheck: Vec<usize> = Vec::new();

        // sanitize input
        for item in indicies {
            if item <= &self.todo_list.len() && item > &0 {
                // valid, can remove safely
                if self.todo_list[item - 1].is_completed {
                    to_uncheck.push(*item);
                } else {
                    to_check_off.push(item - 1);
                }
            } else {
                println!("argument {item} is out of range");
            }
        }

        // write into file and list
        for pos in to_check_off {
            self.todo_list[pos].is_completed = true;
            change_line(
                &conf_file
                    .path
                    .todo_path
                    .join(conf_file.path.todo_filename.clone()),
                self.todo_list[pos].line,
                TodoItem::get_string(&self.todo_list[pos], conf_todo),
            );

            Self::list_single(&self.todo_list[pos], "done");
        }

        if to_uncheck.len() > 0 {
            self.uncheck(&to_uncheck, conf_file, conf_todo)
        };
    }

    pub fn uncheck(&mut self, ids: &Vec<usize>, conf_file: &ConfigFile, conf_todo: &TodoConfig) {
        let mut to_uncheck: Vec<usize> = Vec::new(); // to store sanitized ids

        // sanitize input
        for item in ids {
            if item <= &self.todo_list.len() && item > &0 {
                to_uncheck.push(item - 1); // valid, can remove safely
            } else {
                println!("argument {item} is out of range");
            }
        }

        // write into file and list
        for pos in to_uncheck {
            self.todo_list[pos].is_completed = false;
            change_line(
                &conf_file
                    .path
                    .todo_path
                    .join(conf_file.path.todo_filename.clone()),
                self.todo_list[pos].line,
                TodoItem::get_string(&self.todo_list[pos], conf_todo),
            );

            Self::list_single(&self.todo_list[pos], "unchecked");
        }
    }

    // TODO: Notice how we're currently removing by line number, not by task id? The api should be more robust than that.
    pub fn remove(&self, ids: &Vec<usize>, path: PathBuf) {
        let mut delete_all = false;
        let mut lines_to_rm: Vec<usize> = Vec::new();
        for item in &self.todo_list {
            if ids.contains(&item.id) {
                Self::list_single(&item, "to remove");
                if !delete_all {
                    match readinput("delete that task? ( [y]es / [n]o / [a]ll ): ")
                        .expect("input failed")
                        .as_str()
                    {
                        "y" | "yes" => lines_to_rm.push(item.line),
                        "a" | "all" => {
                            delete_all = true;
                            lines_to_rm.push(item.line);
                        }
                        _ => (),
                    }
                } else {
                    lines_to_rm.push(item.line);
                }
            }
        }
        remove_lines(&path, lines_to_rm);
    }

    /// `prefix` is prepended with ": " or ignored if empty
    fn list_single(item: &TodoItem, prefix: &str) {
        let mut line: ColoredString = "".into();

        if !prefix.is_empty() {
            line = format!("{prefix}: ").into();
        }

        // every task has these
        if item.is_completed {
            line = format!("{line}[x] ").into();
        } else {
            line = format!("{line}[ ] ").into();
        }
        line = format!("{line} {} {}", item.id, item.title).into();

        // optional parts
        if let Some(date) = item.date_due {
            line = format!("{line}{}{}", " | ", date.to_string().red()).into();
        }
        if !item.tags.is_empty() {
            line = format!("{line}{}{}", " | ", item.tags.join(" ").green()).into();
        }
        if !item.names.is_empty() {
            line = format!("{line}{}{}", " | ", item.names.join(" ").cyan()).into();
        }

        // modifications to the whole string
        if item.is_completed {
            line = format!("{}", line).strikethrough();
        };

        println!("{line}");
    }

    pub fn list_all(&self) {
        for it in &self.todo_list {
            Todo::list_single(it, "");
        }
    }
}
