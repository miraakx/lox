use std::{cell::RefCell, fs, io::Write, rc::Rc};

use string_interner::StringInterner;

use crate::{alias::IdentifierSymbol, benches::{BINARY_TREES_LOX, EQUALITY_LOX, FIB_LOX, INSTANTIATION_LOX, INVOCATION_LOX, METHOD_CALL_LOX, PROPERTIES_LOX, STRING_EQUALITY_LOX, TREES_LOX, ZOO_BATCH_LOX, ZOO_LOX}, error::ExecutionResult, interpreter::interpreter::Interpreter, parser::{parser::Parser, resolver::Resolver, types::Stmt}};

/// Executes a file.
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

/// Executes the supplied code.
pub fn run<T:Write>(code: &str, writer: T) -> Result<(), ExecutionResult>
{
   let stmts: Vec<Stmt>;
   let mut interner: StringInterner = StringInterner::default();
   let _ = interner.get_or_intern_static("this");
   let _ = interner.get_or_intern_static("super");
   let init_symbol: IdentifierSymbol = interner.get_or_intern_static("init");
   let writer = Rc::new(RefCell::new(writer));
   {
      let mut parser: Parser<T> = Parser::new(Rc::clone(&writer), init_symbol);
      stmts = parser.parse(code, &mut interner)?;
   }
   let mut interpreter;
   {
      let mut resolver: Resolver<T> = Resolver::new(Rc::clone(&writer), &mut interner);
      let side_table = resolver.resolve(&stmts)?;
      interpreter = Interpreter::new_with_writer(&mut interner, side_table, Rc::clone(&writer));
   }
   interpreter.execute(&stmts)
}

/// Runs the benchmarks designed by the autor of the language and prints out the results.
pub fn bench() {
   let benches = [BINARY_TREES_LOX, EQUALITY_LOX, FIB_LOX, INSTANTIATION_LOX, INVOCATION_LOX, METHOD_CALL_LOX, PROPERTIES_LOX, STRING_EQUALITY_LOX, TREES_LOX, ZOO_BATCH_LOX, ZOO_LOX];
   println!("{:<16} ;{:<10}", "TEST TYPE", "ELAPSED");
   for (test_index, bench) in benches.iter().enumerate() {
      let mut buf_output = Vec::<u8>::new();
      let _ = run(bench, &mut buf_output);
      let lines: Vec<&str> = std::str::from_utf8(&buf_output).unwrap().lines().collect();
      let mut text: &str = "";
      let mut result: f64 = -1.0;
      for (index, line) in lines.into_iter().enumerate() {
         match index {
            0 => {
               if !line.contains("elapsed") {
                  panic!("first line {} do not contains 'elapsed'", line);
               }
            },
            1 => {
               text = match test_index {
                  0 => {"BINARY_TREES"},
                  1 => {"EQUALITY"},
                  2 => {"FIB"},
                  3 => {"INSTANTIATION"},
                  4 => {"INVOCATION"},
                  5 => {"METHOD_CALL"},
                  6 => {"PROPERTIES"},
                  7 => {"STRING_EQUALITY"},
                  8 => {"TREES"},
                  9 => {"ZOO_BATCH"},
                  10 => {"ZOO"},
                  _ => {panic!("inexpected banch type")}
               };
               result = line.to_string().parse::<f64>().unwrap();
            }
            _ => {
               panic!()
            }
         }
      }
      println!("{:<16} ;{:>9}", text, format!("{:.3}", result))
   }
}