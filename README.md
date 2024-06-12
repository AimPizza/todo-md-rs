# Installation

this will compile a binary to `~/.cargo/bin` which needs to be in your `$PATH` 
```shell
git clone git@github.com:AimPizza/todo-md-rs.git
cd todo-md-rs
cargo install --path=.
```
# Usage

```shell
# list everything (more than listed here)
todo --help
# list your tasks
todo
# add a task
todo add "show readers how to use this tool"
# mark a task as done
todo done 1
# delete a task
todo remove 1
```

# Configuration

Upon launching the program for the first time it will ask you whether it should create a configuration file in $HOME/.config/todo-md-rs/.
the following parameters are allowed:

| parameter | value | explanation |
| --- | --- | --- |
| todo_path | some valid path | where your todo file will be located |
| todo_filename | some valid name | what your todofile is called |
| checkbox_style | "logseq", "md" | recognizes patterns of completion. logseq: TODO DONE, md: [ ] [X] |

```toml
[path]
todo_path = "/home/username/"
todo_filename = "todo.md"

[format]
# possible formats: "logseq", "md" (default)
checkbox_style = "md"
```

# Thoughts

This program should fulfill certian requirements:

- store tasks in a simple file format (.md)  
- allow integration with other note-taking apps such as Obsidian or [Logseq](https://github.com/logseq/logseq) (suggestions welcome)  
- should do what it does safely/easily and nothing unnessecary
- by default, the program should comply with standards
  - however, as workflows can be very different, the possibility to customize aspects should be given

# Decisions

> Feel free to discuss those. I am always open for suggestions/help.

- **This deviates from todo.md spec**: Tasks with only hypehns, not checkboxes, wouldn't make sense in the context of note-taking since they would stand for a bullet point. I therefore chose not to respect them as a task. Changing this behaviour would require a bit of work.
- `done` acts as `uncheck` for tasks that are already done. I hope this helps with ease of use instead of creating confusion, although a settings parameter would be possible to implement.
- Lines from a todo_file are read and evaluated at runtime. The IDs are generated only then, they might change after adding/removing a task.
  - This approach is prone to user error as users could accidentally remove tasks with the wrong ID.
  - If you choose to list tasks and then act upon the listed IDs, the operation should be succesful, though.

# Features TODO

- [ ] tests / code improvement
  - [ ] a simple test adding a task with all parameters
  - [ ] when adding a new command, one has to add code in multiple places ( get_string, strings_to_todo, list_single and mby more ). This creates multiple places for bugs to occur.
- [x] nicer UI
  - [x] display tags and assigned names
- [ ] config options
  - [ ] ignore confirmations (don't prompt before removing task)
- [x] contrasting actions ( add/remove, done/uncheck )
- [ ] comply with [todo.md](https://github.com/todomd/todo.md)
  - [ ] implement sections / heading parsing
  - [x] handle @name and #tags
- [ ] package for Distros (AUR and nixpkgs is the goal for now)
- [ ] not to overbloat things but now that I finish more and more features, I'd like to dream of things like:
  - [ ] CalDAV 
  - [ ] gui app
