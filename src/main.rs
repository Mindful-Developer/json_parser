use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::env;
use std::process::exit;

mod language_tools;
mod tests;


fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: check_json <file>");
        return;
    }
    let file_path = Path::new(&args[1]);
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut parser = language_tools::parser::Parser::new(&contents);
    let result = parser.parse();
    match result {
        Ok(_) => exit(0),
        Err(e) => {
            println!("Error: {}", e);
            exit(1);
        },
    }
}
