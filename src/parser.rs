use crate::document::{DocumentEntry, DocumentError, Document};
use std::str::Chars;
use std::borrow::{BorrowMut, Borrow};
use crate::document::DocumentEntry::Text;
use std::iter::Peekable;
use crate::parser::Token::Comma;
use crate::parser::LexError::{UnexpectedCharacter, UnexpectedEndOfStream, DollarExpected};
use crate::parser::ParseError::{ExpectedTokenError, UnexpectedTokenError};
use std::collections::HashMap;

peg::parser!{
    grammar doldoc() for str {
        rule literal() -> String
            = s:$(['A'..='Z']+) { String::from(s) }

        rule flag() -> Flag
            = status:$(['+'|'-']) code:$(['A'..='Z']+) { Flag::new(status, code) }

        rule flag_list() -> Vec<Flag>
            = f:flag()* { f }

        rule arg() -> Argument
            = "," key:$(['A'..='Z']+)? "="? value:$(['A'..='Z']+) { Argument::new(key, value) }

        rule arg_list() -> Vec<Argument>
            = a:arg()* { a }

        pub rule command() -> Result<DocumentEntry, DocumentError>
            = "$" cmd:literal() flags:flag_list() args:arg_list() "$" { DocumentEntry::new(cmd.as_str(), Flags::new(flags), Arguments::new(args)) }
    }
}

#[derive(Debug, PartialEq)]
pub struct Flag {
    status: bool,
    code: String
}

impl Flag {
    pub fn new(status: &str, code: &str) -> Self {
        Flag {
            status: status == "+",
            code: String::from(code)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Flags {
    flags: Vec<Flag>
}

impl Flags {
    pub fn new(flags: Vec<Flag>) -> Self {
        Flags {
            flags
        }
    }

    pub fn empty() -> Self {
        Flags {
            flags: Vec::new()
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Argument {
    key: Option<String>,
    value: String
}

impl Argument {
    pub fn new(key: Option<&str>, value: &str) -> Self {
        Argument {
            key: key.map(|s| String::from(s)),
            value: String::from(value)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Arguments {
    values: Vec<Argument>
}

impl Arguments {
    pub fn new(values: Vec<Argument>) -> Self {
        Arguments {
            values
        }
    }

    pub fn empty() -> Self {
        Arguments::new(Vec::new())
    }

    /*fn get(&self, key: &str) -> Option<&Argument> {
        self.values.iter().find(|arg| arg.key == key)
    }*/
}

#[derive(Debug)]
pub enum ParseError {
    GeneralError,
    ExpectedTokenError(Token),
    UnexpectedTokenError(Token),
    UnexpectedEOF,
    DocError(DocumentError)
}

impl From<DocumentError> for ParseError {
    fn from(err: DocumentError) -> Self {
        ParseError::DocError(err)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Literal(String),
    Comma,
    Dollar,
    Equals,
    QuotedString(String),
    Plus,
    Minus
}

#[derive(Debug)]
enum LexError {
    UnexpectedCharacter(char),
    UnexpectedEndOfStream,
    DollarExpected
}

/*fn lex_expect(iterator: &mut Peekable<Chars>, expected: char) -> Result<(), LexError> {
    let ch = iterator.peek();
    if ch.is_none() {
        return Err(ExpectedCharacter(expected));
    }

    if *ch.unwrap() != expected {
        return Err(ExpectedCharacter(expected));
    }

    Ok(())
}*/

fn lex(iterator: &mut Peekable<Chars>) -> Result<Vec<Token>, LexError> {
    //lex_expect(iterator, '$')?;
    let mut result = Vec::new();

    if iterator.next().unwrap() != '$' {
        return Err(DollarExpected);
    }
    result.push(Token::Dollar);

    loop {
        match iterator.next().unwrap() {
            '$' => {
                result.push(Token::Dollar);
                break;
            },
            '=' => result.push(Token::Equals),
            ',' => result.push(Token::Comma),
            '+' => result.push(Token::Plus),
            '-' => result.push(Token::Minus),
            ch @ '0'..='9' |
            ch @ 'A'..='Z' |
            ch @ 'a'..='z' |
            ch @ '_'  => {
                // read literal
                let mut literal = String::new();
                literal.push(ch);

                loop {
                    let ch = iterator.peek();
                    if ch.is_none() {
                        return Err(UnexpectedEndOfStream);
                    }

                    let ch = ch.unwrap();
                    if !ch.is_alphanumeric() {
                        break;
                    }

                    let ch = iterator.next().unwrap();
                    literal.push(ch);
                }

                result.push(Token::Literal(literal));
            },
            '"' => {
                let mut quoted_string = String::new();
                loop {
                    let ch = iterator.next();
                    if ch.is_none() {
                        return Err(UnexpectedEndOfStream);
                    }

                    let ch = ch.unwrap();
                    if ch == '"' {
                        break;
                    }

                    quoted_string.push(ch);
                }

                result.push(Token::QuotedString(quoted_string));
            },
            unexpected @ _ => return Err(UnexpectedCharacter(unexpected))
        }
    }

    Ok(result)
}

fn read_text(iterator: &mut Peekable<Chars>) -> Result<DocumentEntry, ParseError> {
    let mut str = String::new();

    loop {
        let peekaboo = iterator.peek();
        if peekaboo.is_none() || *peekaboo.unwrap() == '$' {
            break;
        }

        let ch = match iterator.next() {
            Some(ch) => ch,
            None => break
        };

        str.push(ch);
    }

    Ok(Text(Flags::empty(), Arguments::empty()))
}

pub fn parse(str: &str) -> Result<Vec<DocumentEntry>, ParseError> {
    let mut iterator = str.chars().peekable();
    let mut result = Vec::<DocumentEntry>::new();

    loop {
        let ch = match iterator.peek() {
            Some(ch) => ch,
            None => break
        };

        if *ch == '$' {
            // Skip first dollar.
            iterator.next();

            let mut buffer = String::new();
            while *iterator.peek().unwrap() != '$' {
                buffer.push(iterator.by_ref().next().unwrap());
            }

            // Skip last dollar.
            iterator.by_ref().next();

            let command = doldoc::command(format!("${}$", buffer).as_str());
            println!("{:?}", command);
        } else {
            result.push(read_text(iterator.borrow_mut())?);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_parses_plus_flag() {
        let result = doldoc::command("$CL+X$").unwrap().unwrap();

        assert_eq!(result, DocumentEntry::Clear(Flags::new(vec![Flag::new("+", "X")]), Arguments::empty()));
    }

    #[test]
    fn test_parses_minus_flag() {
        let result = doldoc::command("$CL-Y$").unwrap().unwrap();

        assert_eq!(result, DocumentEntry::Clear(Flags::new(vec![Flag::new("-", "Y")]), Arguments::empty()));
    }

    #[test]
    fn test_parses_multiple_flags() {
        let result = doldoc::command("$CL+X-Y$").unwrap().unwrap();

        assert_eq!(result, DocumentEntry::Clear(Flags::new(
            vec![Flag::new("+", "X"), Flag::new("-", "Y")]),
            Arguments::empty()));
    }

    #[test]
    fn test_parse_text() -> Result<(), ParseError> {
        let result = parse("Hello World!")?;

        assert_eq!(1, result.len());
        // assert_eq!(DocumentEntry::Text(String::from("Hello World!")), *result.get(0).unwrap());

        Ok(())
    }

    #[test]
    fn test_parse_text_with_dollars() -> Result<(), ParseError> {
        let result = parse("That costs $$50.00!")?;

        assert_eq!(3, result.len());
        /*assert_eq!(DocumentEntry::Text(String::from("That costs ")), *result.get(0).unwrap());
        assert_eq!(DocumentEntry::Text(String::from("$")), *result.get(1).unwrap());
        assert_eq!(DocumentEntry::Text(String::from("50.00!")), *result.get(2).unwrap());*/

        Ok(())
    }

    #[test]
    fn test_lex_literals() -> Result<(), LexError> {
        let mut chars = "$CL$".chars().peekable();
        let result = lex(chars.borrow_mut())?;

        assert_eq!(3, result.len());
        assert_eq!(Token::Dollar, result[0]);
        assert_eq!(Token::Literal(String::from("CL")), result[1]);
        assert_eq!(Token::Dollar, result[2]);

        Ok(())
    }

    #[test]
    fn test_lex_arguments() -> Result<(), LexError> {
        let mut chars = "$TEST,A=\"Value\"$".chars().peekable();
        let result = lex(chars.borrow_mut())?;

        assert_eq!(7, result.len());
        assert_eq!(Token::Dollar, result[0]);
        assert_eq!(Token::Literal(String::from("TEST")), result[1]);
        assert_eq!(Token::Comma, result[2]);
        assert_eq!(Token::Literal(String::from("A")), result[3]);
        assert_eq!(Token::Equals, result[4]);
        assert_eq!(Token::QuotedString(String::from("Value")), result[5]);
        assert_eq!(Token::Dollar, result[6]);

        Ok(())
    }

    #[test]
    fn test_lex_quoted_strings() -> Result<(), LexError> {
        let mut chars = "$TX,\"Test\"$".chars().peekable();
        let result = lex(chars.borrow_mut())?;

        assert_eq!(5, result.len());
        assert_eq!(Token::Dollar, result[0]);
        assert_eq!(Token::Literal(String::from("TX")), result[1]);
        assert_eq!(Token::Comma, result[2]);
        assert_eq!(Token::QuotedString(String::from("Test")), result[3]);
        assert_eq!(Token::Dollar, result[4]);

        Ok(())
    }

    #[test]
    fn test_lex_flags() -> Result<(), LexError> {
        let mut chars = "$TX+B-C$".chars().peekable();
        let result = lex(chars.borrow_mut())?;

        assert_eq!(7, result.len());
        assert_eq!(Token::Dollar, result[0]);
        assert_eq!(Token::Literal(String::from("TX")), result[1]);
        assert_eq!(Token::Plus, result[2]);
        assert_eq!(Token::Literal(String::from("B")), result[3]);
        assert_eq!(Token::Minus, result[4]);
        assert_eq!(Token::Literal(String::from("C")), result[5]);
        assert_eq!(Token::Dollar, result[6]);

        Ok(())
    }

    #[test]
    fn test_lex_all() -> Result<(), LexError> {
        let mut chars = "$TX+B-C,\"Hello\",S=\"World!\"$".chars().peekable();
        let result = lex(chars.borrow_mut())?;

        assert_eq!(13, result.len());
        assert_eq!(Token::Dollar, result[0]);
        assert_eq!(Token::Literal(String::from("TX")), result[1]);
        assert_eq!(Token::Plus, result[2]);
        assert_eq!(Token::Literal(String::from("B")), result[3]);
        assert_eq!(Token::Minus, result[4]);
        assert_eq!(Token::Literal(String::from("C")), result[5]);
        assert_eq!(Token::Comma, result[6]);
        assert_eq!(Token::QuotedString(String::from("Hello")), result[7]);
        assert_eq!(Token::Comma, result[8]);
        assert_eq!(Token::Literal(String::from("S")), result[9]);
        assert_eq!(Token::Equals, result[10]);
        assert_eq!(Token::QuotedString(String::from("World!")), result[11]);
        assert_eq!(Token::Dollar, result[12]);

        Ok(())
    }
}