#![allow(dead_code)]

use std::env;
use std::error::Error;
use std::fs;
use std::io;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::parse;

mod common;
mod error;
mod interpreter;
mod lexer;
mod parser;
mod tokens;
mod environment;

fn main() {
   let code = "var a = 88+5; { var a=a; var b = 5; print b+a; { var a = 7; print a;} } if(false) {print a;}";
   run(code);
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

fn run_file<'a>(filepath: &'a str) -> Result<(), Box<dyn Error>> {
   let _ = fs::read_to_string(filepath)?;
   println!("Running from file: {} ...", filepath);
   Ok(())
}

fn run_prompt() -> Result<(), Box<dyn Error>> {
   loop {
      println!("(input): ");
      let mut line = String::new();
      io::stdin().read_line(&mut line)?;
      println!("(output): ");
      run(&line);
   }
}

fn run(code: &str) {
   let mut lexer = Lexer::new(code);
   let r_parse  = parse(&mut lexer);
   if r_parse.is_ok() {
      let mut interpreter = Interpreter::new();
      let r_inter = interpreter.interpret(r_parse.unwrap());
   }
}