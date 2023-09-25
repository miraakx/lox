use std::error::Error;
use std::fmt;

use crate::tokens::{Position, TokenKind};

#[derive(Clone, Debug, PartialEq)]
pub enum LoxErrorKind
{
    UnexpectedToken(char),
    ParseFloatError(String),
    UnterminatedString,
    InvalidEscapeCharacter,
    UnexpectedToken2(String),
    UnexpectedEndOfFile,
    MissingClosingParenthesis,
    LiteralExpected(TokenKind),
    MissingSemicolon,
    VariableNameExpected,
    ExpectedToken(TokenKind),
    UdefinedVariable(String),
    BreakOutsideLoop,
    NotCallable,
    WrongArity(u32, u32),
    NativeClockSysTimeError,
    ResolverLocalVariableNotFound(String)
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoxError
{
    pub kind: LoxErrorKind,
    pub position: Position
}

impl LoxError
{
    pub fn new(kind: LoxErrorKind, position: Position) -> LoxError
    {
        LoxError { kind, position }
    }
}

impl Error for LoxError {}

impl fmt::Display for LoxError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match &self.kind {
            LoxErrorKind::UnexpectedToken(ch)       => write!(f, "Unexpected token '{}', at {}",             ch,    self.position),
            LoxErrorKind::ParseFloatError(value)  => write!(f, "Cannot parse float '{}', at {}",           value, self.position),
            LoxErrorKind::UnterminatedString               => write!(f, "Unterminated string at {}",                       self.position),
            LoxErrorKind::InvalidEscapeCharacter           => write!(f, "Invalid escape character at {}",                  self.position),
            LoxErrorKind::UnexpectedToken2(ch)    => write!(f, "Unexpected token '{}', at {}",             ch,    self.position),
            LoxErrorKind::UnexpectedEndOfFile              => write!(f, "Unexpected end of file, at {}",                   self.position),
            LoxErrorKind::MissingClosingParenthesis        => write!(f, "Missing closing parenthesis ')', at {}",          self.position),
            LoxErrorKind::LiteralExpected(kind)=> write!(f, "Expected literal, found '{:?}' at {}",     kind,  self.position),
            LoxErrorKind::MissingSemicolon                 => write!(f, "Missing semicolon ';' at {}",                     self.position),
            LoxErrorKind::VariableNameExpected             => write!(f, "Expected variable name after 'var' at {}",        self.position),
            LoxErrorKind::ExpectedToken(kind)  => write!(f, "Expected token '{:?}' at {}",              kind,  self.position),
            LoxErrorKind::UdefinedVariable(name)  => write!(f, "Undefined variable '{}' at {}",            name,  self.position),
            LoxErrorKind::BreakOutsideLoop                 => write!(f, "Found 'break' keyword outside a loop at {}",      self.position),
            LoxErrorKind::NotCallable                      => write!(f, "Not a callable expression at {}",                 self.position),
            LoxErrorKind::WrongArity(expected, found) => write!(f, "Expected {} arguments, found {}, at {}",   expected, found, self.position),
            LoxErrorKind::NativeClockSysTimeError          => write!(f, "System time error calling clock(), at {}",        self.position),
            LoxErrorKind::ResolverLocalVariableNotFound(name) => write!(f, "Can't read local variable {} in its own initializer, at {}", name, self.position),
        }
    }
}

pub trait ErrorLogger
{
    fn log(&mut self, error: LoxError);
}

pub struct ConsoleErrorLogger;

impl ErrorLogger for ConsoleErrorLogger
{
    fn log(&mut self, error: LoxError) {
        println!("{}", error);
    }
}