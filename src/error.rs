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
    ReturnFromInitializer,
    ClassCantInheritFromItslef,
    CantUseSuperOutsideClass,
    CantUseSuperWithoutSuperClass
}

impl fmt::Display for ResolverErrorKind
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self {
            //book
            Self::ClassCantInheritFromItslef    => write!(f, "A class can't inherit from itself."),
            Self::ReturnFromTopLevelCode        => write!(f, "Can't return from top-level code."),
            Self::ReturnFromInitializer         => write!(f, "Can't return a value from an initializer."),
            Self::CantUseSuperOutsideClass      => write!(f, "Can't use 'super' outside of a class."),
            Self::CantUseSuperWithoutSuperClass => write!(f, "Can't use 'super' in a class with no superclass."),
            Self::InvalidThisUsage              => write!(f, "Can't use 'this' outside of a class."),
            Self::LocalVariableNotFound(_)      => write!(f, "Can't read local variable in its own initializer."),
            Self::VariableAlreadyExists(_)      => write!(f, "Already a variable with name in this scope."),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InterpreterErrorKind
{
    CheckNumberOperands,
    CheckNumberOperand,
    NotCallable,
    WrongArity(usize, usize),
    NativeClockSysTimeError,
    UdefinedProperty(String),
    AssertionFailure,
    SuperclassMustBeAClass,
    InvalidPlusOperands,
    OnlyInstancesHaveProperties,
    OnlyInstancesHaveFields,
    UndefinedVariable(String)
}

impl fmt::Display for InterpreterErrorKind
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self {
            //custom
            Self::NativeClockSysTimeError               => write!(f, "System time error calling clock()."),
            Self::AssertionFailure                      => write!(f, "Assertion failure."),
            //book
            Self::SuperclassMustBeAClass                => write!(f, "Superclass must be a class."),
            Self::InvalidPlusOperands                   => write!(f, "Operands must be two numbers or two strings."),
            Self::NotCallable                           => write!(f, "Can only call functions and classes."),
            Self::WrongArity(expected, found)           => write!(f, "Expected {} arguments but got {}.", expected, found),
            Self::OnlyInstancesHaveProperties           => write!(f, "Only instances have properties."),
            Self::OnlyInstancesHaveFields               => write!(f, "Only instances have fields."),
            Self::UdefinedProperty(value)               => write!(f, "Undefined property '{}'.", value),
            Self::CheckNumberOperand                    => write!(f, "Operand must be a number."),
            Self::CheckNumberOperands                   => write!(f, "Operands must be numbers."),
            Self::UndefinedVariable(name)               => write!(f, "Undefined variable '{}'.", name),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParserErrorKind
{
    UnexpectedToken(char),
    UnterminatedString,
    UnexpectedCharacter,
    InvalidAssignmentTarget,

    ParseFloatError(String),

    InvalidEscapeCharacter,
    UnexpectedEndOfFile,
    MissingClosingParenthesis,
    ExpectedLiteral(TokenKind),
    ExpectedToken(String),
    BreakOutsideLoop,
    ExpectedIdentifier(String),
    TooManyArguments,
    TooManyParameters,
    ExpectedBlock,

}

impl fmt::Display for ParserErrorKind
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match &self {
            Self::UnexpectedToken(ch)               => write!(f, "Unexpected token '{}'.", ch),
            Self::UnterminatedString                => write!(f, "Unterminated string."),
            Self::UnexpectedCharacter               => write!(f, "Unexpected character."),
            Self::InvalidAssignmentTarget           => write!(f, "Invalid assignment target."),
            Self::ParseFloatError(value)            => write!(f, "Cannot parse float '{}'.", value),

            Self::InvalidEscapeCharacter            => write!(f, "Invalid escape character."),
            Self::UnexpectedEndOfFile               => write!(f, "Unexpected end of file."),
            Self::MissingClosingParenthesis         => write!(f, "Missing closing parenthesis ')'."),
            Self::ExpectedLiteral(token_kind)       => write!(f, "Expected literal, found '{}'.", token_kind),
            Self::ExpectedToken(message)            => write!(f, "{}", message),
            Self::BreakOutsideLoop                  => write!(f, "Found 'break' keyword outside a loop."),
            Self::ExpectedIdentifier(message)       => write!(f, "{}", message),
            Self::TooManyArguments                  => write!(f, "Can't have more than 255 arguments."),
            Self::TooManyParameters                 => write!(f, "Can't have more than 255 parameters."),
            Self::ExpectedBlock                     => write!(f, "Internal error: Expected block found something else."),

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