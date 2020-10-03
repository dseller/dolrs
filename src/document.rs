
/*#[derive(Debug)]
pub enum DocumentEntry {
    Text(String),
    Foreground(String),
    Clear
}*/

#[derive(Debug)]
pub enum DocumentEntry {
    Clear,
    Text(String),
    Foreground(String)
}

#[derive(Debug)]
pub enum DocumentError {
    UnrecognizedCommand(String)
}

impl DocumentEntry {
    pub fn new(cmd: &str) -> Result<Self, DocumentError> {
        match cmd {
            "CL" => Ok(DocumentEntry::Clear),
            "TX" => Ok(DocumentEntry::Text(String::from(""))),
            "FG" => Ok(DocumentEntry::Foreground(String::from("whatever"))),
            _ => Err(DocumentError::UnrecognizedCommand(String::from(cmd)))
        }
    }
}

pub struct Document {

}

impl Document {

}