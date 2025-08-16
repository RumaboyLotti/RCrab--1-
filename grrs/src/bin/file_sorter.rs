use std::io::{stdin, Write};
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
    //Need to re-write this to match the given pattern
    for path in glob(input).expect("Failed to read path") {
        match path {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("{:?}", e),
        }
    }

}

fn sort_into_folder () {

}

fn main() {

    let mut user_input = String::new();


    // let mut out  = io::stdout();
    // out.write_all("please enter the folder you want to sort ".as_ref()).unwrap();
    println!("please enter the folder you want to sort");

    //takes user input and stores it in a var to be iterated over
    stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line"); // Handle error if reading fails


    // takes user input and formats it for glob
    let directory_to_access = format!("{}{}", user_input.trim(), "{}");

    // finds given directory
    find_dir(&*directory_to_access);

    sort_into_folder();
}