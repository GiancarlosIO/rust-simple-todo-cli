## Simple todo CLI

A simple todo cli to add, remove, edit and list items.

## Commands

````text
A simple todo CLI created in rust

Usage: todo-cli-3.exe [OPTIONS] <COMMAND>

Commands:
  add     Add a new todo
  delete
  edit
  list
  help    Print this message or the help of the given subcommand(s)

Options:
  -d, --db-name <DB_NAME>  custom db text filename [default: db.txt]
  -h, --help               Print help
  -V, --version            Print version
````

### List
````bash
cargo run -- list
````

### Add new todos
````bash
cargo run -- add "Its my todo"
````

### Edit a todo
````bash
cargo run -- edit "1" "Edit this todo with id 1"
````

### Delete a todo
````bash
cargo run -- delete 1
````

## Run tests
```bash
cargo test
```