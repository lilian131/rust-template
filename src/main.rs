use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;
fn main() {
    let mut args: Vec<String> = env::args().collect();
    let main_path = String::from(&args[0]);
    let user_path = String::as_str(&args[&args.len() - 1]);
    let path = String::from(main_path + user_path);
    println!("My path is {}.", path);
    open_the_file(path);
}

fn open_the_file(path: String) {
    let file = File::open(path);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}