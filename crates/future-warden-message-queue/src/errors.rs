use std::{error::Error, fmt};

#[derive(Debug, Eq, PartialEq)]
pub struct SendError;

#[derive(Debug, Eq, PartialEq)]
pub struct RecvError;

impl fmt::Display for SendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "message queue closed")
    }
}

impl Error for SendError {}

impl fmt::Display for RecvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "message queue closed")
    }
}

impl Error for RecvError {}
