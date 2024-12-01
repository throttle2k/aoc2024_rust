use std::{fs, path::PathBuf};

pub fn read_input(file_name: &str) -> String {
    let path = PathBuf::from(format!("inputs/{file_name}"));
    fs::read_to_string(path).unwrap()
}

