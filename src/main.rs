use std::{env, io};

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
mod run;

fn main()
{
   let args: Vec<String> = env::args().collect();
   match args.len() {
      /*1 => {
         let result = run::run_prompt();
         if let Err(_) = result {
            println!("\nProgram terminated with error(s). See above.");
            std::process::exit(64);
         }
      }*/
      2 => {
         let result = run::run_file(&args[1], &mut io::stdout().lock());
         if result.is_err() {
            println!("\nProgram terminated with error(s). See above.");
            std::process::exit(64);
         }
      }
      _ => {
         println!("\nUsage: rlox [script]");
         std::process::exit(64);
      }
   };
   println!("\nProgram terminated successfully.");
}