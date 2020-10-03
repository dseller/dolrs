use crate::document::{DocumentEntry, DocumentError};
use std::str::Chars;
use std::borrow::{BorrowMut};
use crate::document::DocumentEntry::Text;
use std::iter::Peekable;
use crate::parser::Token::Comma;
use crate::parser::LexError::{UnexpectedCharacter, UnexpectedEndOfStream, DollarExpected};

#[derive(Debug)]
pub enum ParseError {
    GeneralError,
    UnexpectedEOF,
    DocError(DocumentError)
}

impl From<DocumentError> for ParseError {
    fn from(err: DocumentError) -> Self {
        ParseError::DocError(err)
    }
}

#[derive(Debug, PartialEq)]
enum Token {
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

    Ok(Text(str))
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
            //result.push(parse_command(iterator.borrow_mut())?);

            println!("{:?}", lex(iterator.borrow_mut()));
        } else {
            result.push(read_text(iterator.borrow_mut())?);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

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