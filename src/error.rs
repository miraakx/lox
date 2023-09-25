use std::error::Error;
use std::fmt;

use crate::tokens::{Position, TokenKind};

#[derive(Clone, Debug, PartialEq)]
pub struct LoxError
{
    pub kind: LoxErrorKind,
    pub position: Position
}

impl LoxError
{
    pub fn parser_error(kind: ParserErrorKind, position: Position) -> LoxError
    {
        LoxError { kind: LoxErrorKind::ParserErrorKind(kind), position }
    }

    pub fn resolver_error(kind: ResolverErrorKind, position: Position) -> LoxError
    {
        LoxError { kind: LoxErrorKind::ResolverErrorKind(kind), position }
    }

    pub fn interpreter_error(kind: InterpreterErrorKind, position: Position) -> LoxError
    {
        LoxError { kind: LoxErrorKind::InterpreterErrorKind(kind), position }
    }
}

impl Error for LoxError {}

impl fmt::Display for LoxError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}, at {}", self.kind, self.position)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LoxErrorKind {
    ParserErrorKind(ParserErrorKind), InterpreterErrorKind(InterpreterErrorKind), ResolverErrorKind(ResolverErrorKind)
}

impl fmt::Display for LoxErrorKind
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match &self {
            LoxErrorKind::ParserErrorKind(error) => {
                write!(f, "Parser error: {}", error)
            },
            LoxErrorKind::InterpreterErrorKind(error) => {
                write!(f, "Runtime error: {}", error)
            },
            LoxErrorKind::ResolverErrorKind(_) => {
                todo!();
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ResolverErrorKind
{
    LocalVariableNotFound(String)
}

impl fmt::Display for ResolverErrorKind
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self {
            ResolverErrorKind::LocalVariableNotFound(name) => write!(f, "Can't read local variable {} in its own initializer", name),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum InterpreterErrorKind
{
    IncompatibleBinaryOpTypes,
    InvalidUnaryType,
    InvalidBinaryType,
    NotCallable,
    WrongArity(u32, u32),
    NativeClockSysTimeError,
}

impl fmt::Display for InterpreterErrorKind
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self {
            InterpreterErrorKind::IncompatibleBinaryOpTypes                 => write!(f, "Both expressions side are not of the same type"),
            InterpreterErrorKind::InvalidUnaryType                          => write!(f, "Invalid unary type"),
            InterpreterErrorKind::InvalidBinaryType                         => write!(f, "Invalid binary type"),
            InterpreterErrorKind::NotCallable                               => write!(f, "Not a callable expression"),
            InterpreterErrorKind::WrongArity(expected, found)   => write!(f, "Expected {} arguments, found {}", expected, found),
            InterpreterErrorKind::NativeClockSysTimeError                   => write!(f, "System time error calling clock()"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParserErrorKind
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
}

impl fmt::Display for ParserErrorKind
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match &self {
            ParserErrorKind::UnexpectedToken(ch)       => write!(f, "Unexpected token '{}'", ch),
            ParserErrorKind::ParseFloatError(value)  => write!(f, "Cannot parse float '{}'", value),
            ParserErrorKind::UnterminatedString               => write!(f, "Unterminated string"),
            ParserErrorKind::InvalidEscapeCharacter           => write!(f, "Invalid escape character"),
            ParserErrorKind::UnexpectedToken2(ch)    => write!(f, "Unexpected token '{}'", ch),
            ParserErrorKind::UnexpectedEndOfFile              => write!(f, "Unexpected end of file"),
            ParserErrorKind::MissingClosingParenthesis        => write!(f, "Missing closing parenthesis ')'"),
            ParserErrorKind::LiteralExpected(kind)=> write!(f, "Expected literal, found '{:?}'", kind),
            ParserErrorKind::MissingSemicolon                 => write!(f, "Missing semicolon ';'"),
            ParserErrorKind::VariableNameExpected             => write!(f, "Expected variable name after 'var'"),
            ParserErrorKind::ExpectedToken(kind)  => write!(f, "Expected token '{:?}'", kind),
            ParserErrorKind::UdefinedVariable(name)  => write!(f, "Undefined variable '{}'", name),
            ParserErrorKind::BreakOutsideLoop                 => write!(f, "Found 'break' keyword outside a loop"),
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