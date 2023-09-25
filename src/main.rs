#![allow(dead_code)]

use std::env;
use std::error::Error;
use std::fs;
use std::io;

use error::ConsoleErrorLogger;
use interpreter::Interpreter;
use lexer::Lexer;
use parser_stmt::Parser;
use resolver::Resolver;

mod common;
mod error;
mod interpreter;
mod lexer;
mod parser_stmt;
mod parser_expr;
mod tokens;
mod environment;
mod native;
mod resolver;

fn main()
{
   //let code = "fun ciao() { return \"ciao\"; } fun stampa(fn) { print fn(); } stampa(ciao);";
   let code = "var a = \"global\"; { fun showA() {print a;} showA(); var a = \"block\"; showA(); }";
   run(code);
}

fn _main()
{
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

fn run_file<'a>(filepath: &'a str) -> Result<(), Box<dyn Error>>
{
   let _ = fs::read_to_string(filepath)?;
   println!("Running from file: {} ...", filepath);
   Ok(())
}

fn run_prompt() -> Result<(), Box<dyn Error>>
{
   loop {
      println!("(input): ");
      let mut line = String::new();
      io::stdin().read_line(&mut line)?;
      println!("(output): ");
      run(&line);
   }
}

fn run(code: &str)
{
   let mut lexer = Lexer::new(code, ConsoleErrorLogger{});
   let mut parser: Parser = Parser::new(ConsoleErrorLogger{});
   let r_stmts  = parser.parse(&mut lexer);
   match r_stmts
   {
      Ok(stmts) => {
         let mut interpreter = Interpreter::new();
         let mut resolver: Resolver = Resolver::new(&mut interpreter);
         resolver.resolve(&stmts[..]);
         let _ = interpreter.execute(&stmts[..]);
      },
      Err(err) => {
         println!("{}", err);
      }
   }
}