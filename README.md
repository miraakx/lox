# Lox Tree-Walk Interpreter Written in Rust

Rust port from Java of the tree-walk interpreter for the Lox language described in the amazing book Crafting Interpreters, written by [@munificent](https://github.com/munificent).

## Code

* scanner.rs

Allows scanning the source code one character at a time, while also providing the capability to perform N "read ahead" (the lexer actually requires two read aheads to recognize decimal numbers). The read ahead functionality is provded by the struct `NthPeekable`, which uses a circular buffer to store transient characters.

* lexer.rs

Converts the characters provided by the scanner into tokens, recognizing strings, identifiers, keywords, numbers, operators, etc.

* parser.rs

Performs parsing by applying the grammatical rules of the language. It uses a recursive descent algorithm starting with the lowest precedence grammatical rules. It does not perform backtracking and only admits a single lookahead. The single look ahead functionality is provided by the struct `Peekable`.

* resolver.rs

Traverses the syntax tree and resolves all variables by associating them with the scope in which they are defined. The information is stored in a dictionary that links the expression ID associated with the variable usage to the distance in "scopes" from the respective declaration.

* interpreter.rs

Executes all the statements sequentially, recursively evaluating any expression.

Other files:

* native_functions.rs contains native functions provided by the language. In particular, the `clock()` function is needed for benchmarks.

* environment.rs defines a structure to store all the program variables and their respective values.

### Notes

The implementation has some minor differences from what is described in the book.

* Unlike the book, it supports `break` and `continue` statements.
* It supports UTF-8 strings.
* It includes two extra native functions: `fun assert_eq(actual, expected);` and `fun str(value); `

### Tests

End-to-end tests are contained in the `./lox_test` folder and were developed by the book's author.

### Benchmarks

You can benchmark the interpreter with the following command:
```
lox --bench
```

Results:

Benchmark        | Rust lox (average) | Java lox (average)  | Winner
---------------- | ------------------ | --------------------|--------
BINARY_TREES     |  9,038 +/- 0,071   |  8,318 +/- 0,135    | Java
EQUALITY         |  9,376 +/- 0,025   |  4,596 +/- 0,172    | Java
FIB              |  6,389 +/- 0,024   |  4,268 +/- 0,126    | Java
INSTANTIATION    |  3,162 +/- 0,002   |  1,486 +/- 0,051    | Java
INVOCATION       |  2,531 +/- 0,022   |  1,431 +/- 0,068    | Java
METHOD_CALL      |  1,429 +/- 0,003   |  2,133 +/- 0,325    | Rust
PROPERTIES       |  3,462 +/- 0,003   |  4,578 +/- 0,018    | Rust
STRING_EQUALITY  |  0,333 +/- 0,002   |  0,989 +/- 0,527    | Rust
TREES            | 17,959 +/- 0,035   | 28,744 +/- 0,713    | Rust
ZOO_BATCH        | 10,006 +/- 0,005   | 10,017 +/- 0,004    | Rust
ZOO              |  2,522 +/- 0,004   |  5,002 +/- 0,072    | Rust

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


## License

* This project is licensed under the MIT License - see the LICENSE.md file for details.
* Copyright for all files located in ./benches, ./lox_test, and the language grammar belongs to the author of the book - see the LICENSE.md file for details.

## Acknowledgments

* [Crafting Interpreters by Robert Nystrom](https://craftinginterpreters.com/)
