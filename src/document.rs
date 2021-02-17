
/*#[derive(Debug)]
pub enum DocumentEntry {
    Text(String),
    Foreground(String),
    Clear
}*/

use crate::parser::{Flag, Flags};

#[derive(Debug,PartialEq)]
pub enum DocumentEntry {
    Clear(Flags),
    Text(Flags),
    Foreground(Flags)
}

#[derive(Debug)]
pub enum DocumentError {
    UnrecognizedCommand(String)
}

impl DocumentEntry {
    pub fn new(cmd: &str, flags: Flags) -> Result<Self, DocumentError> {
        match cmd {
            "CL" => Ok(DocumentEntry::Clear(flags)),
            "TX" => Ok(DocumentEntry::Text(flags)),
            "FG" => Ok(DocumentEntry::Foreground(flags)),
            _ => Err(DocumentError::UnrecognizedCommand(String::from(cmd)))
        }
    }
}

pub struct Document {

}

impl Document {

}