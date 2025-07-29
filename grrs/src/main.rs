use std;
use chrono::{DateTime, Local};

/*
rust comments

 */

fn main() {
    println!("Hello, world!");
    let local: DateTime<Local> = Local::now();
    println!("Today is {}", local.format("%A"));
}
