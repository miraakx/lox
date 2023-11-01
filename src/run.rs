use std::{fs, error::Error, io};

use string_interner::StringInterner;

use crate::{parser_stmt::{Stmt, Parser}, error::{ConsoleErrorLogger, LoxError}, resolver::Resolver, interpreter::Interpreter};

pub fn run_file<'a>(filepath: &'a str) -> Result<(), Box<dyn Error>>
{
   let code = fs::read_to_string(filepath)?;
   let result = run(&code);
    match result {
        Ok(_) => {
            println!("\nProgram terminated successfully.\n");
        },
        Err(_) => {
            println!("\nProgram terminated with error(s). See above.\n");
        },
    }
   Ok(())
}

pub fn run_prompt() -> Result<(), Box<dyn Error>>
{
    loop {
       println!("(input): ");
       let mut line = String::new();
       io::stdin().read_line(&mut line)?;
       println!("(output): ");
       let result = run(&line);
       match result {
            Ok(_) => {
                println!("\nProgram terminated successfully.\n");
            },
            Err(_) => {
                println!("\nProgram terminated with error(s). See above.\n");
            },
        }
    }
}

pub fn run(code: &str) -> Result<(), ()>
{
   let stmts       : Vec<Stmt>;
   let mut interner: StringInterner;
   {
      let mut parser: Parser = Parser::new(ConsoleErrorLogger{});
      let result  = parser.parse(code)?;
      stmts    = result.0;
      interner = result.1;
   }
   let mut interpreter;
   {
      let mut resolver: Resolver = Resolver::new(ConsoleErrorLogger{}, &mut interner);
      let side_table = resolver.resolve(&stmts)?;
      interpreter = Interpreter::new(&mut interner, side_table);
   }
   interpreter.execute(&stmts)?;
   Ok(())
}