use std::error::Error;
use std::fmt;

use crate::tokens::{Position, TokenKind};

#[derive(Clone, Debug, PartialEq)]
pub enum LoxErrorKind {
    UnexpectedToken(char),
    ParseFloatError(String),
    UnterminatedString,
    InvalidEscapeCharacter,
    UnexpectedToken2(String),
    UnexpectedEndOfFile,
    MissingClosingParenthesis,
    LiteralExpected(TokenKind)
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoxError {
    pub kind: LoxErrorKind,
    pub position: Position
}

impl LoxError {
    pub fn new(kind: LoxErrorKind, position: Position) -> LoxError {
        LoxError { kind, position }
    }
}

impl Error for LoxError {}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            LoxErrorKind::UnexpectedToken(ch)       => write!(f, "Unexpected token '{}', at {}.",   ch,    self.position),
            LoxErrorKind::ParseFloatError(value)  => write!(f, "Cannot parse float '{}', at {}.", value, self.position),
            LoxErrorKind::UnterminatedString               => write!(f, "Unterminated string at {}.",             self.position),
            LoxErrorKind::InvalidEscapeCharacter           => write!(f, "Invalid escape character at {}.",        self.position),
            LoxErrorKind::UnexpectedToken2(ch)    => write!(f, "Unexpected token '{}', at {}.",   ch,    self.position),
            LoxErrorKind::UnexpectedEndOfFile              => write!(f, "Unexpected end of file, at {}.",         self.position),
            LoxErrorKind::MissingClosingParenthesis        => write!(f, "Missing closing parenthesis ')', at {}.",self.position),
            LoxErrorKind::LiteralExpected(kind)=> write!(f, "Expected literal, found {:?} at {}",     kind, self.position),
        }
    }
}

pub fn println_hadle_error(error: LoxError) {
    println!("{}", error);
}