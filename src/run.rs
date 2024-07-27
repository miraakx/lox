use std::{fs, io::{self, Write}};

use string_interner::StringInterner;

use crate::{parser_stmt::{Stmt, Parser}, error::{ConsoleErrorLogger, ExecutionResult}, resolver::Resolver, interpreter::Interpreter};

pub fn run_file<'a>(filepath: &'a str, writer: Box<&mut dyn Write>) -> Result<(), ExecutionResult>
{
    let r_code = fs::read_to_string(filepath);
    match r_code {
    Ok(code) => {
        run(&code, writer)
    },
    Err(error) => {
        println!("\nCannot read file: {}\n", error);
        Err(ExecutionResult::CannotReadFile)
    },
}


}

pub fn run_prompt() -> Result<(), ()>
{
   loop {
      print!(">>> ");
      let _ = io::stdout().flush();
      let mut line = String::new();
      let result = io::stdin().read_line(&mut line);
      match result {
         Ok(_) => {
            //println!("");
            let _ = run(&line, Box::new(&mut io::stdout().lock()));
         },
         Err(error) => {
            println!("\nCannot read line: {}\n", error);
            return Err(());
         },
      }
   }
}

pub fn run(code: &str, writer: Box<&mut dyn Write>) -> Result<(), ExecutionResult>
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
      interpreter = Interpreter::new_with_writer(&mut interner, side_table, writer);
   }
   interpreter.execute(&stmts)
}