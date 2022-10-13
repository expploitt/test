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

use crate::result::ObjectResult;

pub mod error;
pub mod result;


#[derive(Debug)]
struct Object {
    values: Vec<String>,
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, v) in self.values.iter().enumerate() {
            if i > 0 { write!(f, ",")?; }
            write!(f, "{}", v.trim_end())?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl Object {
    pub fn new() -> Self {
        Object { values: vec!() }
    }
}


fn main() -> ObjectResult<()> {
    let mut object = Object::new();

    loop {
        let mut input = String::new();
        // stdin.read_line(&mut input);
        print!("Enter a new value: ");
        io::stdout().flush()?;
        {
            std::io::stdin().read_line(&mut input)?;
        }

        if input != "" && input != "\n" {
            object.values.push(input);
            println!("\n Returns: {}", object);
        }
    }
}
