/*  Create a Todo App that implements Trait, Struct, Option, Error, Result.
 2.   We should be able to enter a value on the terminal.
 3.   Current value and previous values should be return when we hit ENTER key.
 
 Example: []
 Enter a new value: Cycling
 Returns: [Cycling]
 Enter a new value: Hiking
 Returns: [Cycling, Hiking]
  */

use std::fmt::{Display, Formatter};
use std::io::{self, Read, Write};

pub mod error;
pub mod result;


#[derive(Debug)]
struct Object {
    values: Vec<String>,
}

impl Object {
    pub fn new() -> Self {
        Object { values: vec!() }
    }
}


fn main() {
    let mut object = Object::new();

    loop {
        let mut input = String::new();
        // stdin.read_line(&mut input);
        print!("Enter a new value: ");
        io::stdout().flush().unwrap();
        {
            std::io::stdin().read_line(&mut input).unwrap();
        }
        input.trim_end();
        object.values.push(input);
        println!("\n Returns: {:?}", object.values);
    }
}
