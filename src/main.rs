#![allow(dead_code)]

use std::env;
use std::error::Error;
use std::fs;
use std::io;

use crate::error::ErrorRepoVec;
use crate::lexer::Lexer;
use crate::lexer::TokenSourceLazy;
use crate::tokens::DebugRepoHashMap;
use crate::parser::Parser;

mod lexer;
mod parser;
mod tokens;
mod error;
mod common;
mod interpreter;
fn main() {
   let code = "5-3-(1-2)*2<0==4";
   run(code);
}
/*
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

fn run_file<'a>(filepath: &'a str) -> Result<(), Box<dyn Error>> {
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
      run(line);
   }
}
*/
fn run(code: &str) {
   
   let error_repo_lexer = ErrorRepoVec::new();
   let debug_repo = DebugRepoHashMap::new();

   let lexer = Lexer::new(&code, Box::new(error_repo_lexer), Box::new(debug_repo));
   let token_source = TokenSourceLazy::new_lexer(lexer);
   let error_repo_parser = ErrorRepoVec::new();
   let mut parser = Parser::new(Box::new(token_source), Box::new(error_repo_parser));
   parser.parse();
}


