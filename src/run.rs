use std::{fs, io::Write};

use string_interner::StringInterner;

use crate::{alias::IdentifierSymbol, error::{ConsoleErrorLogger, ExecutionResult}, interpreter::Interpreter, parser_stmt::{Parser, Stmt}, resolver::Resolver};

pub fn run_file<'a>(filepath: &'a str, writer: &mut dyn Write) -> Result<(), ExecutionResult>
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

/*pub fn run_prompt() -> Result<(), ()>
{
   let mut string_interner: StringInterner = StringInterner::default();
   let mut resolver: Resolver = Resolver::new(ConsoleErrorLogger{}, &mut string_interner);
   let mut tokens: Vec<Token> = vec![];
   //let side_table = resolver.resolve(&stmts)?;
   loop {
      println!("(input): ");
      let mut line = String::new();
      let result = io::stdin().read_line(&mut line);
      let error_logger: ConsoleErrorLogger = ConsoleErrorLogger{};
      match result {
         Ok(_) => {
            println!("(output): ");
            let result = run(&line, Box::new(&mut io::stdout().lock()));
            match result {
               Ok(_) => {
                  println!("\nProgram terminated successfully.\n");
                  return Ok(());
               },
               Err(_) => {
                  println!("\nProgram terminated with error(s). See above.\n");
                  return Err(());
               },
            }
         },
         Err(error) => {
            println!("\nCannot read line: {}\n", error);
            return Err(());
         },
      }
   }
}*/

pub fn run(code: &str, writer: &mut dyn Write) -> Result<(), ExecutionResult>
{
   let stmts: Vec<Stmt>;
   let mut interner: StringInterner = StringInterner::default();
   let _ = interner.get_or_intern_static("this");
   let _ = interner.get_or_intern_static("super");
   let init_symbol: IdentifierSymbol = interner.get_or_intern_static("init");
   {
      let mut parser: Parser = Parser::new(ConsoleErrorLogger{}, init_symbol);
      stmts = parser.parse(code, &mut interner)?;
   }
   let mut interpreter;
   {
      let mut resolver: Resolver = Resolver::new(ConsoleErrorLogger{}, &mut interner);
      let side_table = resolver.resolve(&stmts)?;
      interpreter = Interpreter::new_with_writer(&mut interner, side_table, writer);
   }
   interpreter.execute(&stmts)
}