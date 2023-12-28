# Thoughts
This program should fulfill certian requirements:

- store tasks in a simple file format (.md)
- allow integration with other note-taking apps such as Obsidian or [Logseq](https://github.com/logseq/logseq) (suggestions welcome)
- should do what it does safely/easily and nothing unnessecary

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
todo_path = "/home/user/"
todo_filename = "todo.md"

[format]
# possible formats: "logseq", "md"
checkbox_style = "logseq"

```
