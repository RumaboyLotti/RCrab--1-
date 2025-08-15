use std::io::stdin;
use std::io::stdout;
use std::{fs, io, str};
use glob::glob;
use std::result;
use std::result::Result;
use std::string::String;
use std::fs::read_dir;
use std::path::Path;
/*
What am i trying to solve:
file sorter
read the directory
look for types of files
move them into the correct folder


 */



fn find_dir(input: &str) {
    read_dir(Path::new("./src")).ok();

}



fn main() {

    let mut user_input = String::new();


    stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line"); // Handle error if reading fails

    let directory_to_access = format!("{}{}", user_input.trim(), "{}");

    // finds given directory
    find_dir(&*directory_to_access);


}