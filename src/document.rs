
/*#[derive(Debug)]
pub enum DocumentEntry {
    Text(String),
    Foreground(String),
    Clear
}*/

use crate::parser::{Flag, Flags, Arguments};

#[derive(Debug,PartialEq)]
pub enum DocumentEntry {
    Clear(Flags, Arguments),
    Text(Flags, Arguments),
    Foreground(Flags, Arguments)
}

#[derive(Debug)]
pub enum DocumentError {
    UnrecognizedCommand(String)
}

impl DocumentEntry {
    pub fn new(cmd: &str, flags: Flags, args: Arguments) -> Result<Self, DocumentError> {
        match cmd {
            "CL" => Ok(DocumentEntry::Clear(flags, args)),
            "TX" => Ok(DocumentEntry::Text(flags, args)),
            "FG" => Ok(DocumentEntry::Foreground(flags, args)),
            _ => Err(DocumentError::UnrecognizedCommand(String::from(cmd)))
        }
    }
}

pub struct Document {

}

impl Document {

}