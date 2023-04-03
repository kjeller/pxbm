use std::ffi::OsStr;

pub fn to_string(file: &OsStr) -> String {
    std::fs::read_to_string(file).unwrap()
}
