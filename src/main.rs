#![allow(dead_code)]

use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;

use lexer::Position;

mod lexer;
mod parser;
mod common;
mod interpreter;

fn main() {
   run("5-3-(1-2)*2<0==4");
}

fn _main() {

   let args: Vec<String> = env::args().collect();
   let result = match args.len() {
      1 => run_prompt(),
      2 => run_file(&args[1]),
      _ => { 
               println!("Usage: rlox [script]"); 
               std::process::exit(64); 
            }
   };
   match result {
      Ok(()) => {}
      Err(_) => {
         std::process::exit(64); 
      }
   }
}

fn run_file(filepath: &str) -> Result<(), Box<dyn Error>> {
   let _ = fs::read_to_string(filepath)?;
   println!("Running from file: {} ...", filepath); 
   Ok(())
}

fn run_prompt() -> Result<(), Box<dyn Error>> {
   loop { 
      println!("\n(input):\n");
      let mut line = String::new();
      io::stdin().read_line(&mut line)?;
      println!("\n(output):\n");
      run(&line);
   }
}

fn run(code: &str) {
   println!("running...\n");

   //let result = lexer::tokenize(code);
   let tree = parser::parse(code).unwrap().unwrap();
   parser::print(&tree);
   print!("\n");
}

#[derive(Clone, Debug, PartialEq)]
pub enum LoxErrorKind {
   UnexpectedToken(char), ParseFloatError(String), UnterminatedString, InvalidEscapeCharacter, UnexpectedToken2(String)
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoxError {
   kind: LoxErrorKind,
   position: Position
}

impl LoxError {
   pub fn new(kind: LoxErrorKind, position: Position) -> LoxError {
      LoxError { position, kind }
   }
}

impl Error for LoxError{}

impl fmt::Display for LoxError {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match &self.kind {
         LoxErrorKind::UnexpectedToken(ch) => write!(f, "Unexpected token '{}', at line: {}, column: {}.", ch, self.position.line, self.position.column),
         LoxErrorKind::ParseFloatError(value) => write!(f, "Cannot parse float '{}', at line: {}, column: {}.", value, self.position.line, self.position.column),
         LoxErrorKind::UnterminatedString => write!(f, "Unterminated string at line: {}, column: {}.", self.position.line, self.position.column),
         LoxErrorKind::InvalidEscapeCharacter => write!(f, "Invalid escape character at line: {}, column: {}.", self.position.line, self.position.column),
         LoxErrorKind::UnexpectedToken2(ch) => write!(f, "Unexpected token '{}', at line: {}, column: {}.", ch, self.position.line, self.position.column),
      }  
   }
}

