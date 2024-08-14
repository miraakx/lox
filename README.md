# Lox Tree-Walk Interpreter Written in Rust

Rust port from Java of the tree-walk interpreter for the Lox language described in the amazing book Crafting Interpreters, written by [@munificent](https://github.com/munificent).

## Code

* scanner.rs

Allows scanning the source code one character at a time, while also providing the capability to perform N "read ahead" (the lexer actually requires two read aheads to recognize decimal numbers). The read ahead functionality is provded by the struct `NthPeekable`, which uses a circular buffer to stores transient characters.

* lexer.rs

Converts the characters provided by the scanner into tokens, recognizing strings, identifiers, keywords, numbers, operators, etc.

* parser.rs

Performs parsing by applying the grammatical rules of the language. It uses a recursive descent algorithm starting with the lowest precedence grammatical rules. It does not perform backtracking and only admits a single lookahead.

* resolver.rs

Traverses the syntax tree and resolves all variables by associating them with the scope in which they are defined. The information is stored in a dictionary that links the expression ID associated with the variable usage to the distance in "scope" from the respective declaration.

* interpreter.rs

Executes statements sequentially, evaluating any expressions contained within them recursively.

Other files:

* native_functions.rs contains native functions provided by the language. In particular, the clock() function is needed for benchmarks.

* environment.rs is a dictionary that associates variable names with their corresponding values.

### Notes

The implementation has some minor differences from what is described in the book.

* Unlike the book, it supports `break` and `continue` statements.
* It supports UTF-8 strings.
* It includes two extra native functions: `fun assert_eq(actual, expected);` and `fun str(value); `

### Test

End-to-end tests are contained in the `./lox_test` folder and were developed by the book's author.

### Dependencies

Install Rust on your operating system: https://www.rust-lang.org/tools/install

### Usage

Clone and build the project:
```
git clone https://github.com/miraakx/lox
cargo build --release
```
The executable can be found under ./target/release

Run the executable from cmd or bash:
```
cd <exe/folder/path>
lox <file>
```

## Authors

[@miraakx](https://github.com/miraakx)

## Version History

* 0.1
    * Initial Release

## License

* This project is licensed under the MIT License - see the LICENSE.md file for details.
* Copyright for all files located in ./benches, ./lox_test, and the language grammar belongs to the author of the book - see the LICENSE.md file for details.

## Acknowledgments

* [Crafting Interpreters by Robert Nystrom](https://craftinginterpreters.com/)
