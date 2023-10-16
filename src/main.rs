#![allow(dead_code)]

use std::cell::RefCell;
use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::rc::Rc;

use error::ConsoleErrorLogger;
use interpreter::Interpreter;
use lexer::Lexer;
use parser_stmt::Parser;
use resolver::Resolver;
use string_interner::StringInterner;

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

fn main()
{
   //let code = "fun ciao() { return \"ciao\"; } fun stampa(fn) { print fn(); } stampa(ciao);";
   //let code = "var a = \"global\"; { fun showA() {print a;} showA(); var a = \"block\"; showA(); }";
   //let code = "class Car { start() { print \"engine on\"; } stop() { print \"engine off\"; } } var panda = Car(); panda.start(); print panda.stop();";
   let code =
   /*"
      {
         var closure = 5;
         fun prova() {
            var inner = closure;
         }
      }
      prova();
   ";*/
   "

   var speak = nil;
   {
      class Parrot {
         init() {
            this.word = \"squeak\";
            return;
         }
         speak(){
            print this.word;
         }
      }
      var parrot = Parrot();
      speak = parrot.speak;
   }
   speak();
   ";
   run(code);
   //todo!("stop() {{ print \"engine off\"; }} senza punto e virgola panica!");
}

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

fn run(code: &str)
{
   let interner = Rc::new(RefCell::new(StringInterner::default()));
   let r_stmts;
   {
      let mut lexer = Lexer::new(code, ConsoleErrorLogger{}, Rc::clone(&interner));
      let mut parser: Parser = Parser::new(ConsoleErrorLogger{});
      r_stmts  = parser.parse(&mut lexer);
   }
   if r_stmts.is_err() {
      println!("\nCompile time error(s) detected. See above.\n");
      return;
   }
   let stmts = &r_stmts.unwrap()[..];
   let mut interpreter = Interpreter::new(Rc::clone(&interner));
   {
      let mut resolver: Resolver = Resolver::new(&mut interpreter, ConsoleErrorLogger{}, Rc::clone(&interner));
      let result = resolver.resolve(stmts);
      if result.is_err() {
         println!("\nCompile time error(s) detected. See above.\n");
         return;
      }
   }
   let result = interpreter.execute(stmts);
   if result.is_err() {
      println!("\nProgram terminated with errors. See above.\n");
      return;
   }
   println!("\nProgram terminated successfully.\n");
}