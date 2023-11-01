use std::env;

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
mod value;
mod alias;
mod tiny_vec;
mod run;

fn main()
{
   let args: Vec<String> = env::args().collect();
   let result = match args.len() {
      1 => run::run_prompt(),
      2 => run::run_file(&args[1]),
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