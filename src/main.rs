use std::{env, io};

mod utils;
mod error;
mod interpreter;
mod parser;
mod alias;
mod run;

fn main()
{
   let args: Vec<String> = env::args().collect();
   match args.len() {
      2 => {
         let result = run::run_file(&args[1], &mut io::stdout().lock());
         if result.is_err() {
            println!("\nProgram terminated with error(s). See above.");
            std::process::exit(64);
         }
      }
      _ => {
         println!("\nUsage: rlox [path/to/script]");
         std::process::exit(64);
      }
   };
   println!("\nProgram terminated successfully.");
}