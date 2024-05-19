use clap::{Parser, Subcommand};
use std::error::Error;

pub mod db;

pub const DB_NAME: &str = "db.txt";
type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(
    version,
    about = "A simple todo CLI created in rust",
    author = "Giancarlos Isasi"
)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// custom db text filename
    #[arg(short = 'd', long, default_value = DB_NAME)]
    db_name: String,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new todo
    Add {
        /// The name of the todo to add
        #[arg(required = true)]
        name: String,
    },

    Delete {
        /// The id of the todo to be deleted
        #[arg(required = true)]
        id: usize,
    },

    Edit {
        /// The name of the todo to edit
        #[arg(required = true)]
        id: usize,

        /// The new todo
        #[arg(required = true)]
        name: String,
    },

    List,
}

pub fn new_cli() -> MyResult<Cli> {
    let cli = Cli::parse();

    Ok(cli)
}

pub fn run(cli: Cli) -> MyResult<()> {
    let mut db = db::create_db(&cli.db_name)?;

    match &cli.command {
        Commands::Add { name } => {
            db.add_todo(name);
        }
        Commands::Edit { id, name } => db.edit_todo(id, name),
        Commands::Delete { id } => db.delete_todo(id),
        Commands::List => db.list(),
    }
    Ok(())
}
