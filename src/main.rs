fn main() {
    if let Err(err) = todo_cli_3::new_cli().and_then(todo_cli_3::run) {
        eprintln!("Critical Error: {}", err);
        std::process::exit(1);
    }
}
