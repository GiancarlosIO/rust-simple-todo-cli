use crate::MyResult;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};

pub const SEPARATOR: &str = "____";

pub struct Db {
    file: File,
    db_filename: String,
}

impl Db {
    pub fn new(db_filename: String) -> MyResult<Self> {
        // Advanced way to open and file
        let file = OpenOptions::new()
            .read(true)
            // Open the file in append mode
            .append(true)
            // Create the file if it doesn't exists
            .create(true)
            .open(&db_filename)?;

        Ok(Self { file, db_filename })
    }

    pub fn add_todo(&mut self, todo: &str) {
        let todo = format!("{} {} {}\n", todo, SEPARATOR, false);
        match self.file.write_all(todo.as_bytes()) {
            Ok(_) => {
                println!("> Success to add the new todo.");
                self.list();
            }
            Err(err) => {
                eprintln!("Error to create the todo: {}", err)
            }
        }
    }

    pub fn edit_todo(&mut self, todo_id: &usize, name: &String) {
        let lines = self.read_lines();
        let len = lines.len();
        let index_to_edit = *todo_id - 1;

        if len == 0 {
            eprintln!("The database is empty. Use the `add` command to add a new todo.");
            return;
        }

        if index_to_edit > len {
            eprintln!("The given todo id ({}) doesn't exists in the database. Please use the `list` command and check that the todo with id {} exists.", todo_id, todo_id);
            return;
        }

        // create a new vec with the todo edited
        let mut new_lines: Vec<String> = Vec::new();
        for (index, current) in lines {
            // Edit the todo with the given id/index
            if index == index_to_edit {
                let parts: Vec<&str> = current.split(SEPARATOR).collect();
                let completed = if let Some(v) = parts.get(1) {
                    v
                } else {
                    // fallback for false
                    "false"
                };
                new_lines.push(format!("{} {} {}", name, SEPARATOR, completed))
            } else {
                new_lines.push(current)
            }
        }

        // update the file content
        let new_content = new_lines.join("\n");
        match override_db_content(new_content, &self.db_filename) {
            Ok(_) => {
                println!("The todo with id {} was edited.", todo_id);
                self.list();
            }
            Err(err) => eprintln!("Error to update the todo with id {}: ", err),
        }
    }

    /// The `todo_id` is the index + 1 position of the element to be deleted
    pub fn delete_todo(&mut self, todo_id: &usize) {
        let lines = self.read_lines();
        let len = lines.len();
        let mut new_content: Vec<String> = Vec::new();
        // the id is equal to the index + 1;

        if *todo_id > len {
            eprintln!("The given todo id ({}) doesn't exists in the database. Please use the `list` command and check that the todo with id {} exists.", todo_id, todo_id);
            return;
        }

        for (index, line) in lines {
            // do not continue if the id doesn't exists in the db

            if index + 1 != *todo_id {
                new_content.push(line)
            }
        }

        match override_db_content(new_content.join("\n"), &self.db_filename) {
            Ok(_) => {
                println!("Todo with id {} deleted.", todo_id)
            }
            Err(err) => {
                eprintln!(
                    "Error when trying to update the db after deleting a file: {}",
                    err
                )
            }
        }

        self.list();
    }

    pub fn list(&mut self) {
        let lines = self.read_lines();
        let len = lines.len();

        if len == 0 {
            println!("The database is empty. Add new todos using the `add` command.");
            return;
        }

        println!("The TODO List contains the following elements:\n");
        print_row("ID", "Completed", "Name");
        for (index, line) in lines {
            // just an extra code to validate that the todo/line must only have two elements
            let mut parts: Vec<&str> = Vec::with_capacity(2);
            for e in line.split(SEPARATOR).collect::<Vec<&str>>() {
                parts.push(e)
            }

            let todo = if let Some(v) = parts.get(0) {
                v.trim()
            } else {
                "invalid"
            };
            let status = if let Some(v) = parts.get(1) { v } else { "[ ]" };

            let checkbox = if status == "true" { "[x]" } else { "[ ]" };
            print_row((index + 1).to_string().as_str(), checkbox, todo)
        }
    }

    fn read_lines(&mut self) -> Vec<(usize, String)> {
        let mut lines: Vec<(usize, String)> = Vec::new();

        // move the file cursor to the beginning to read all content
        if let Ok(_) = self.file.seek(SeekFrom::Start(0)) {
            let reader = BufReader::new(&self.file);
            for (index, result) in reader.lines().enumerate() {
                match result {
                    Ok(line) => {
                        lines.push((index, line));
                    }
                    Err(err) => {
                        eprintln!(
                            "Error to try to read the file line. Line is {} and error is {}",
                            index, err
                        );
                    }
                }
            }
        }

        lines
    }
}

pub fn create_db(db_filename: &String) -> MyResult<Db> {
    Db::new(db_filename.clone())
}

fn override_db_content(new_content: String, db_filename: &String) -> MyResult<()> {
    // open the file again but this time with only write mode and truncating it to zero length.
    // truncate(true) will delete all the content of the file
    match OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&db_filename)
    {
        Ok(mut file) => file.write_all(new_content.as_bytes())?,
        Err(err) => {
            eprintln!(
                "Error when trying to write the new content in the db: {}",
                err
            )
        }
    }
    Ok(())
}

fn print_row(id: &str, completed: &str, name: &str) {
    println!("{:>4} {:^12} {}", id, completed, name)
}
