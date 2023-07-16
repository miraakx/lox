use std::error::Error;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum LoxErrorKind {
   UnexpectedToken(char), ParseFloatError(String), UnterminatedString, InvalidEscapeCharacter, UnexpectedToken2(String)
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoxError {
   pub kind:    LoxErrorKind,
   pub line:    u32,
   pub column:  u32,
}

impl LoxError {
   pub fn new(kind: LoxErrorKind, line: u32, column: u32) -> LoxError {
      LoxError { kind, line, column }
   }
}

impl Error for LoxError{}

impl fmt::Display for LoxError {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match &self.kind {
         LoxErrorKind::UnexpectedToken(ch) => write!(f, "Unexpected token '{}', at line: {}, column: {}.", ch, self.line, self.column),
         LoxErrorKind::ParseFloatError(value) => write!(f, "Cannot parse float '{}', at line: {}, column: {}.", value, self.line, self.column),
         LoxErrorKind::UnterminatedString => write!(f, "Unterminated string at line: {}, column: {}.", self.line, self.column),
         LoxErrorKind::InvalidEscapeCharacter => write!(f, "Invalid escape character at line: {}, column: {}.", self.line, self.column),
         LoxErrorKind::UnexpectedToken2(ch) => write!(f, "Unexpected token '{}', at line: {}, column: {}.", ch, self.line, self.column),
      }  
   }
}

pub trait ErrorRepo {
   fn save(&mut self, error_kind: LoxErrorKind, line: u32, column: u32);
}

pub struct ErrorRepoVec {
   errors: Vec<LoxError>
}

impl ErrorRepo for ErrorRepoVec {

   fn save(&mut self, error_kind: LoxErrorKind, line: u32, column: u32) {
      self.errors.push(LoxError::new(error_kind, line, column));
   }
}

impl ErrorRepoVec {
   pub fn new() -> Self {
      ErrorRepoVec{errors: vec!()}
   }
}

trait ErrorSource: Iterator{}
