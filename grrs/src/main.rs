// use std;
use chrono::{DateTime, Local};

/*
rust comments
uppercase for rust constants
underscore for variable names
variables are immutable by default
Data Types:  Integer , Floating-Point , Boolean , Character
Be aware of primitives and their correct usage
=	 | Assignment | Assigning a value to a variable
:	| Type annotation or pattern label  |	Declaring the type of something
as   | is the type cast operator. | It converts a value from one type into another, unsafely but explicitly.
arithmetic is strict and explicit| no silent type coercion, no unchecked overflows, and no guessing
In Rust:
! before = logical (let dead = !alive)
! after = macro (like println!)
if , else if , while, for , loop, while, break, continue , for loop = for variable in lower_bound_number..upper_bound_number,
.. is not inclusive by  nature and needs = to be inclusive
enums exist
match is used for its naming scheme
if let expression in Rust is a shorthand for a match expression with only one pattern/arm to match.
 What is => in Rust?
=> separates a pattern from the code to run when it matches.
What is _ in Rust?
_ is a wildcard pattern.
Option<T> is used when a value might be present or not.
Result<T, E> is used when an operation can succeed or fail, and you want to know why it failed.
In Rust, -> means “returns”.
[] = arr
mut is for mutability
A slice is a reference to a section of a collection, not a copy.

 */



fn process(numbers: &[i32]) {
    for &n in numbers {
        if n < 0 {

            continue; // Skip negative numbers
        }

        if n == 0 {
            break; // Stop processing
        }

        println!("Processed: {}", n * 2);
    }
}

fn count() {
    let mut number = 0;

    // infinite loop starts here
    loop {
        number += 1;
        println!("{}", number);

        if number >= 10 {
            // exit the loop
            break;
        }
    }
}
fn print_it() {
    println!("This is a macro!"); // Works
}

fn bad_print(msg: &str) {
    println!("{}", msg); // ❌ ERROR: format string must be a literal
}


fn main() {
    println!("Hello, world!");
    // print!("hi");
    let local: DateTime<Local> = Local::now();
    println!("Today is {}", local.format("%A"));
    print_it();
    bad_print("hi");
    count();
    let data = [3, -1, 5, -4, 0, 8, 9];
    process(&data);

}
