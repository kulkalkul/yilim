use std::fs;

const TOKEN_PATH: &'static str = "./config/token";

pub fn read_token() -> String {
    fs::read(TOKEN_PATH)
        .map(String::from_utf8)
        .expect("Error while reading token file")
        .expect("Error while parsing token file")
}