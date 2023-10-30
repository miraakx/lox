use error::ConsoleErrorLogger;
use interpreter::Interpreter;
use parser_stmt::Parser;
use resolver::Resolver;
use string_interner::StringInterner;

use crate::parser_stmt::Stmt;

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

fn main()
{
   let code =
      "
      fun fib(n) {
         if (n < 2)
            return n;
         return fib(n - 1) + fib(n - 2);
      }
      var before = clock();
      print fib(40);
      var after = clock();
      print after - before;
      ";
   run(code);
}
/*
fn _main()
{
   let args: Vec<String> = env::args().collect();
   let result = match args.len() {
      1 => run_prompt(),
      2 => run_file(&args[1]),
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

fn run_file<'a>(filepath: &'a str) -> Result<(), Box<dyn Error>>
{
   let _ = fs::read_to_string(filepath)?;
   println!("Running from file: {} ...", filepath);
   Ok(())
}

fn run_prompt() -> Result<(), Box<dyn Error>>
{
   loop {
      println!("(input): ");
      let mut line = String::new();
      io::stdin().read_line(&mut line)?;
      println!("(output): ");
      run(&line);
   }
}
*/
fn run(code: &str)
{
   let stmts       : Vec<Stmt>;
   let mut interner: StringInterner;
   {
      let mut parser: Parser = Parser::new(ConsoleErrorLogger{});
      let result  = parser.parse(code);
      match result {
         Ok((r_stmts, r_interner)) => {
            stmts    = r_stmts;
            interner = r_interner;
         },
         Err(_) => {
            println!("\nCompile time error(s) detected. See above.\n");
            return;
         }
      }
   }
   let mut interpreter;
   {
      let mut resolver: Resolver = Resolver::new(ConsoleErrorLogger{}, &mut interner);
      let result = resolver.resolve(&stmts);
      match result {
         Ok(side_table) => {
            interpreter = Interpreter::new(&mut interner, side_table);
         },
         Err(_) => {
            println!("\nCompile time error(s) detected. See above.\n");
            return;
         }
      }
   }
   let result = interpreter.execute(&stmts);
   if result.is_err() {
      println!("\nProgram terminated with errors. See above.\n");
      return;
   }
   println!("\nProgram terminated successfully.\n");
}