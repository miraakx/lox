use std::{fs, io::Write};

use string_interner::StringInterner;

use crate::{alias::IdentifierSymbol, error::{ConsoleErrorLogger, ExecutionResult}, interpreter::interpreter::Interpreter, parser::{parser::Parser, types::Stmt, resolver::Resolver}};

pub fn run_file(filepath: &str, writer: &mut dyn Write) -> Result<(), ExecutionResult>
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