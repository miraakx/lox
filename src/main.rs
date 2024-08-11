use std::{env, io};

use lox::run;

fn main()
{
   const VERSION: &str = env!("CARGO_PKG_VERSION");
   let args: Vec<String> = env::args().collect();
   match args.len() {
      2 => {
         let arg = &args[1];
         match arg.as_str() {
            "--bench" => {
               run::bench();
            },
            "--version" => {
               println!("rlox {}", VERSION);
            },
            "--help" => {
               println!("Work in progress...");
            }
            _ => {
               let result = run::run_file(&args[1], &mut io::stdout().lock());
                  if result.is_err() {
                     println!("\nProgram terminated with error(s). See above.");
                     std::process::exit(64);
                  }
            }
         }
      }
      _ => {
         println!("\nUsage: `lox [path/to/script]`. Type `lox --help` for further info.");
         std::process::exit(64);
      }
   };
   println!("\nProgram terminated successfully.");
}