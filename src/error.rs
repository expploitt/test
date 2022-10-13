use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct MyError {
    details: String,
}

impl MyError {
    pub fn new(details: String) -> Self {
        MyError {
            details
        }
    }
}

impl Debug for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<std::io::Error> for MyError {
    fn from(e: std::io::Error) -> Self {
        MyError::new(e.to_string())
    }
}