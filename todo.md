init:
  check for the config and return defaults
  use those values to check for the todofile and ask if it should be created


https://github.com/todo-md

# Regex
// match any indentation of task
"^\s*-\s*\[x\].*"gm
// completion with any symbol
"^\s*-\s*\[[^\s]\]"gm

// due
"due:\d{4}-\d{2}-\d{2}"gm

```markdown
# TODO

This text is not a task.

## Section

And this text neither.

- [ ] This task is open @owner
  - [ ] And it has a subtask!

# BACKLOG

- [ ] This task is postponed

# DONE

- [x] This task is done #prio1
- [-] This task has been declined
```

Each subheader is a todo section. They help grouping and sorting the tasks.

The tasks themself are one liners that start with either `'- [ ] '`, `'- [-] '` or `'- [x] '`.

A task can be in the following states:

* open
* declined
* done
* deleted (removed from the document)

## Metadata

Tasks can be assigned to people using `@USERNAME` format.

Tasks can be tagged using the `#TAG` format.

## Hierarchy

When managing multiple markdown todo files the following hierarchy applies to all tasks.

1. Project: Name of the folder where the `TODO.md` reside.
2. Section: Subheader of `TODO.md` markdown content.
3. Task: File lines starting with `'- [ |-|x] '`

This hierarchy helps organizing and listing tasks either manually or using a software tool
