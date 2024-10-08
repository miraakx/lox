use std::error::Error;
use std::fmt;

use crate::parser::position::Position;

#[derive(Clone, Debug, PartialEq)]
pub struct LoxError
{
    pub kind: LoxErrorKind,
    pub position: Option<Position>
}

impl LoxError
{
    pub const fn internal_error(kind: InternalErrorKind) -> Self
    {
        Self { kind: LoxErrorKind::Internal(kind), position: None }
    }

    pub const fn parser_error(kind: ParserErrorKind, position: Position) -> Self
    {
        Self { kind: LoxErrorKind::Parser(kind), position: Some(position) }
    }

    pub const fn resolver_error(kind: ResolverErrorKind, position: Position) -> Self
    {
        Self { kind: LoxErrorKind::Resolver(kind), position: Some(position) }
    }

    pub const fn interpreter_error(kind: InterpreterErrorKind, position: Position) -> Self
    {
        Self { kind: LoxErrorKind::Interpreter(kind), position: Some(position) }
    }
}

impl Error for LoxError {}

impl fmt::Display for LoxError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self.position {
            Some(position) => {
                write!(f, "[line {}] {}", position.line, self.kind)
            },
            None => {
                write!(f, "[line N/A] {}", self.kind)
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LoxErrorKind
{
    Parser(ParserErrorKind),
    Interpreter(InterpreterErrorKind),
    Resolver(ResolverErrorKind),
    Internal(InternalErrorKind)
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
            Self::Internal(error) => {
                write!(f, "Internal error: {}", error)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InternalErrorKind
{
    ExpectedBlock,
    ExpectToken
}

impl fmt::Display for InternalErrorKind
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self {
            Self::ExpectedBlock => write!(f, "Expected block found something else."),
            Self::ExpectToken   => write!(f, "Expected token."),
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
    InvalidAssignmentTarget,
    ParseFloatError(String),
    InvalidEscapeCharacter,
    ExpectedExpression,
    ExpectedToken(String),
    BreakOutsideLoop,
    ContinueOutsideLoop,
    ExpectedIdentifier(String),
    TooManyArguments,
    TooManyParameters,
}

impl fmt::Display for ParserErrorKind
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match &self {
            Self::UnexpectedToken(ch)               => write!(f, "Unexpected token '{}'.", ch),
            Self::UnterminatedString                => write!(f, "Unterminated string."),
            Self::InvalidAssignmentTarget           => write!(f, "Invalid assignment target."),
            Self::ParseFloatError(value)            => write!(f, "Cannot parse float '{}'.", value),
            Self::ExpectedExpression                => write!(f, "Expect expression"),
            Self::ExpectedIdentifier(message)       => write!(f, "{}", message),
            Self::ExpectedToken(message)            => write!(f, "{}", message),
            Self::TooManyArguments                  => write!(f, "Can't have more than 255 arguments."),
            Self::TooManyParameters                 => write!(f, "Can't have more than 255 parameters."),
            Self::BreakOutsideLoop                  => write!(f, "Can't use 'break' outside of a loop."),
            Self::ContinueOutsideLoop               => write!(f, "Can't use 'continue' outside of a loop."),
            Self::InvalidEscapeCharacter            => write!(f, "Invalid escape character."),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExecutionResult {
    ParserError, ResolverError, RuntimeError, CannotReadFile
 }