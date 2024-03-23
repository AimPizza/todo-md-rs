mod todo;
use crate::todo::*;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// lists all tasks
    #[clap(alias = "ls")]
    List {},
    /// adds a task
    #[clap(alias = "a")]
    Add {
        /// title and other properties of the task to be added
        content: Vec<String>,
    },
    /// remove one or more tasks
    #[clap(alias = "rm")]
    Remove {
        /// IDs of the tasks to remove
        ids: Vec<usize>,
    },
    /// check off a task
    #[clap(alias = "d")]
    Done {
        /// IDs of the tasks to mark as done
        ids: Vec<usize>,
    },
}

fn main() {
    let args = Args::parse();

    // set up the configuration
    let conf_file: ConfigFile = ConfigFile::init();
    let conf_todo = TodoConfig::new(&conf_file);
    let complete_path = conf_file.path.todo_path.join(&conf_file.path.todo_filename); // TODO: nasty workaround but best until fixed

    // get the todos
    let mut todos = Todo::new(); // get empty parser
    todos.strings_to_todo(read_lines(&complete_path), &conf_todo); // populate parser

    match &args.command {
        Some(Commands::List {}) => todos.list_all(),
        Some(Commands::Add { content }) => todos.add(content, &conf_todo, &conf_file),
        Some(Commands::Remove { ids }) => todos.remove(ids, complete_path),
        Some(Commands::Done { ids }) => todos.done(ids, &conf_file, &conf_todo),
        None => todos.list_all(),
    }
}
