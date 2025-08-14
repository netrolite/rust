use std::{fs::File, io::ErrorKind};

fn main() {
    let file_path = "Cargo.tomll";
    read_file_or_create_if_not_found(file_path);
}

fn read_file_or_create_if_not_found(file_path: &str) {
    match File::open(file_path) {
        Ok(_) => println!("Found file {file_path}"),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                println!("File {file_path} not found");
                println!("Attempting to create file named {file_path}");
                match File::create(file_path) {
                    Ok(_) => read_file_or_create_if_not_found(file_path),
                    Err(err) => println!("Failed to create file. Error: {err}"),
                }
            }
            err => {
                println!("Unknown error when reading file: {err}")
            }
        },
    };
}
