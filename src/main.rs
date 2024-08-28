use std::{env, io};

use rlox::run;

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
               let help =
"NAME
      rlox - Lox language tree-walk interpreter writtern in Rust.

SYNOPSIS
      rlox [OPTION] [FILE]

OPTIONS
      --bench     benchmark the interpreter on a standard set of tests written by the author of the Lox language and display the outcome.

      --version   output version information and exit

      --help      output help information and exit

AUTHOR
      Written by miraakx (https://github.com/miraakx)

COPYRIGHT
      Copyrights 2024 miraakx (https://github.com/miraakx) - MIT Licence (https://github.com/miraakx/lox?tab=License-1-ov-file#readme)

";             println!("{}", help);
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
         println!("\nUsage: `rlox [path/to/script]`. Type `rlox --help` for further info.");
         std::process::exit(64);
      }
   };
   println!("\nProgram terminated successfully.");
}