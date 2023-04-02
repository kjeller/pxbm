use std::fs::File;

pub fn to_string(file: &str) -> String {
    std::fs::read_to_string(file).unwrap()
}