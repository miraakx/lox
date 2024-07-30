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
    pub const fn parser_error(kind: ParserErrorKind, position: Position) -> Self
    {
        Self { kind: LoxErrorKind::Parser(kind), position }
    }

    pub const fn resolver_error(kind: ResolverErrorKind, position: Position) -> Self
    {
        Self { kind: LoxErrorKind::Resolver(kind), position }
    }

    pub const fn interpreter_error(kind: InterpreterErrorKind, position: Position) -> Self
    {
        Self { kind: LoxErrorKind::Interpreter(kind), position }
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
pub enum LoxErrorKind
{
    Parser(ParserErrorKind),
    Interpreter(InterpreterErrorKind),
    Resolver(ResolverErrorKind)
}

impl fmt::Display for LoxErrorKind
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match &self {
            Self::Parser(error) => {
                write!(f, "Parser error: {}", error)
            },
            Self::Interpreter(error) => {
                write!(f, "Runtime error: {}", error)
            },
            Self::Resolver(error) => {
                write!(f, "Resolver error: {}", error)
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResolverErrorKind
{
    LocalVariableNotFound(String),
    VariableAlreadyExists(String),
    ReturnFromTopLevelCode,
    InvalidThisUsage,
    ReturnFromInitializer
}

impl fmt::Display for ResolverErrorKind
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self {
            Self::LocalVariableNotFound(value) => write!(f, "Can't read local variable {} in its own initializer", value),
            Self::VariableAlreadyExists(value) => write!(f, "Already a variable with name '{}' in this scope", value),
            Self::ReturnFromTopLevelCode => write!(f, "Can't return from top-level code"),
            Self::InvalidThisUsage => write!(f, "Can't use 'this' outside of a class."),
            Self::ReturnFromInitializer => write!(f, "Can't return a value from an initializer"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InterpreterErrorKind
{
    IncompatibleBinaryOpTypes,
    InvalidUnaryType,
    NotCallable,
    WrongArity(usize, usize),
    NativeClockSysTimeError,
    InvalidPropertyAccess,
    UdefinedProperty(String),
    UdefinedVariableUsage(String),
    UdefinedVariableAssignment(String),
    AssertionFailure
}

impl fmt::Display for InterpreterErrorKind
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self {
            Self::IncompatibleBinaryOpTypes                   => write!(f, "Both expressions side are not of the same type"),
            Self::InvalidUnaryType                            => write!(f, "Invalid unary type"),
            Self::NotCallable                                 => write!(f, "Not a callable expression"),
            Self::WrongArity(expected, found) => write!(f, "Expected {} arguments, found {}", expected, found),
            Self::NativeClockSysTimeError                     => write!(f, "System time error calling clock()"),
            Self::InvalidPropertyAccess                       => write!(f, "Only instances have properties"),
            Self::UdefinedProperty(value)            => write!(f, "Undefined property '{}'", value),
            Self::UdefinedVariableUsage(value)       => write!(f, "Undefined variable. Tryng to evaluate undefined variable '{}'", value),
            Self::UdefinedVariableAssignment(value)  => write!(f, "Undefined variable. Tryng to assign to undefined variable '{}'", value),
            Self::AssertionFailure                            => write!(f, "Assertion failure"),
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
    UnexpectedEndOfFile,
    MissingClosingParenthesis,
    ExpectedLiteral(TokenKind),
    ExpectedToken(TokenKind, TokenKind),
    BreakOutsideLoop,
    ExpectedIdentifier(TokenKind),
    TooManyArguments,
    TooManyParameters,
    ExpectedBlock
}

impl fmt::Display for ParserErrorKind
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match &self {
            Self::UnexpectedToken(ch)                => write!(f, "Unexpected token '{}'", ch),
            Self::ParseFloatError(value)           => write!(f, "Cannot parse float '{}'", value),
            Self::UnterminatedString                        => write!(f, "Unterminated string"),
            Self::InvalidEscapeCharacter                    => write!(f, "Invalid escape character"),
            Self::UnexpectedEndOfFile                       => write!(f, "Unexpected end of file"),
            Self::MissingClosingParenthesis                 => write!(f, "Missing closing parenthesis ')'"),
            Self::ExpectedLiteral(token_kind)   => write!(f, "Expected literal, found '{}'", token_kind),
            Self::ExpectedToken(expected, found)    => write!(f, "Expected token '{}', found '{}'", expected, found),
            Self::BreakOutsideLoop                          => write!(f, "Found 'break' keyword outside a loop"),
            Self::ExpectedIdentifier(found)     => write!(f, "Expected identifier, found {}", found),
            Self::TooManyArguments                          => write!(f, "Can't have more than 255 arguments."),
            Self::TooManyParameters                         => write!(f, "Can't have more than 255 parameters."),
            Self::ExpectedBlock                             => write!(f, "Internal error: Expected block found something else.")
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExecutionResult {
    ParserError, ResolverError, RuntimeError, CannotReadFile
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