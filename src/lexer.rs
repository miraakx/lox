use std::rc::Rc;

use string_interner::StringInterner;

use crate::{common::Scanner, tokens::*, error::*, value::Value};

pub struct Lexer<'a>
{
    scanner        : Scanner<'a>,
    string_interner: &'a mut StringInterner,
    error_logger   : Box<dyn ErrorLogger>,
    end_of_file    : bool,
}

impl<'a> Lexer<'a>
{
    pub fn new(code: &'a str, string_interner: &'a mut StringInterner, error_logger: impl ErrorLogger + 'static) -> Self
    {
        Lexer
        {
           scanner:       Scanner::from_str(code, 2),
           error_logger:  Box::new(error_logger),
           end_of_file:   false,
           string_interner
        }
    }
}

impl<'a> Iterator for Lexer<'a>
{
    type Item = Token;

    fn next(&mut self) -> Option<Token>
    {
        let mut opt_token_kind:  Option<TokenKind>;
        let column_start: u32 = self.scanner.column();
        let line_start: u32 = self.scanner.line();

        loop {

            let opt_ch: Option<char> = self.scanner.next();
            if opt_ch.is_none() {
                if self.end_of_file {
                    return None;
                } else {
                    self.end_of_file = true;
                    return Some(Token{ kind: TokenKind::Eof, position: Position { line: self.scanner.line(), column: self.scanner.column()} });
                }
            }

            let ch: char = opt_ch.unwrap();

            opt_token_kind = None;

            match ch {
                SPACE | TAB => {},
                LINE_FEED =>
                {
                    self.scanner.new_line();
                },
                CARRIAGE_RETURN =>
                {
                    self.scanner.new_line();
                    self.scanner.consume_if_peek_is(LINE_FEED);
                },
                LEFT_PAREN =>
                {
                    opt_token_kind = Some(TokenKind::LeftParen);
                },
                RIGHT_PAREN =>
                {
                    opt_token_kind = Some(TokenKind::RightParen);
                },
                LEFT_BRACE =>
                {
                    opt_token_kind = Some(TokenKind::LeftBrace);
                },
                RIGHT_BRACE =>
                {
                    opt_token_kind = Some(TokenKind::RightBrace);
                },
                COMMA =>
                {
                    opt_token_kind = Some(TokenKind::Comma);
                },
                DOT =>
                {
                    opt_token_kind = Some(TokenKind::Dot);
                },
                SEMICOLON =>
                {
                    opt_token_kind = Some(TokenKind::Semicolon);
                },
                MINUS =>
                {
                    opt_token_kind = Some(TokenKind::Minus);
                },
                PLUS =>
                {
                    opt_token_kind = Some(TokenKind::Plus);
                },
                STAR =>
                {
                    opt_token_kind = Some(TokenKind::Star);
                },
                EQUAL =>
                {
                    if self.scanner.is_peek(EQUAL) {
                        self.scanner.next();
                        opt_token_kind = Some(TokenKind::EqualEqual);
                    } else {
                        opt_token_kind = Some(TokenKind::Equal);
                    }
                },
                BANG =>
                {
                    if self.scanner.is_peek(EQUAL) {
                        self.scanner.next();
                        opt_token_kind = Some(TokenKind::BangEqual);
                    } else {
                        opt_token_kind = Some(TokenKind::Bang);
                    }
                },
                GREATER =>
                {
                    if self.scanner.is_peek(EQUAL) {
                        self.scanner.next();
                        opt_token_kind = Some(TokenKind::GreaterEqual);
                    } else {
                        opt_token_kind = Some(TokenKind::Greater);
                    }
                },
                LESS =>
                {
                    if self.scanner.is_peek(EQUAL) {
                        self.scanner.next();
                        opt_token_kind = Some(TokenKind::LessEqual);
                    } else {
                        opt_token_kind = Some(TokenKind::Less);
                    }
                },
                SLASH =>
                {
                    if !self.scanner.is_peek(SLASH) {
                        opt_token_kind = Some(TokenKind::Slash);
                    } else {
                        //consume the second slash character
                        self.scanner.next();

                        loop {
                            let opt_next_ch = self.scanner.peek();

                            if opt_next_ch.is_none() {
                                break;
                            }

                            let next_ch = opt_next_ch.unwrap();

                            match next_ch {
                                LINE_FEED | CARRIAGE_RETURN => {
                                    break;
                                },
                                _ => {
                                    self.scanner.next();
                                }
                            }
                        }
                    }
                },
                QUOTE =>
                {
                    let mut string = String::new();
                    loop {
                        match self.scanner.next() {
                            Some(BACK_SLASH) => {
                                match self.scanner.peek() {
                                    Some('n') => {
                                        self.scanner.next();
                                        string.push('\n');
                                    },
                                    Some('r') => {
                                        self.scanner.next();
                                        string.push('\r');
                                    },
                                    Some('t') => {
                                        self.scanner.next();
                                        string.push('\t');
                                    },
                                    Some('\\') => {
                                        self.scanner.next();
                                        string.push('\\');
                                    },
                                    Some('0') => {
                                        self.scanner.next();
                                        string.push('\0');
                                    },
                                    Some('"') => {
                                        self.scanner.next();
                                        string.push('"');
                                    },
                                    _=> {
                                        string.push(ch);
                                        self.error_logger.log(LoxError::parser_error(ParserErrorKind::InvalidEscapeCharacter, Position { line: self.scanner.line(), column: self.scanner.column() }));
                                    }
                                }
                            },
                            Some('"') => {
                                opt_token_kind = Some(TokenKind::String(Value::String(Rc::new(string))));
                                break;
                            },
                            None => {
                                self.error_logger.log(LoxError::parser_error(ParserErrorKind::UnterminatedString, Position { line: self.scanner.line(), column: self.scanner.column() }));
                                opt_token_kind = Some(TokenKind::String(Value::String(Rc::new(string))));
                                break;
                            },
                            /*Some(ch) if ch.is => {
                                string.push(ch);
                            },*/
                            Some(ch) => {
                                string.push(ch);
                                //self.error_logger.log(LoxError::parser_error(ParserErrorKind::UnexpectedCharacter, Position { line: self.scanner.line(), column: self.scanner.column() }));
                                //goes on reading the rest of the string
                            }
                        }
                    }
                },
                ch if ch.is_ascii_digit() =>
                {
                    let mut number_string = String::from(ch);
                    let mut flag_decimal_point = false;

                    //read the number and stores the chars into 'number_string'
                    while self.scanner.is_peek_ascii_digit() || self.scanner.is_peek('.') && self.scanner.is_peek_next_ascii_digit() && !flag_decimal_point
                    {
                        if self.scanner.is_peek('.') {
                            flag_decimal_point = true;
                        }
                        number_string.push(self.scanner.unwrap_next());

                        /*//looks for a dot followed by another digit
                        if self.scanner.is_peek('.') && self.scanner.is_peek_next_ascii_digit() && !flag_decimal_point
                        {
                            //save the '.' into the number string
                            number_string.push(self.scanner.unwrap_next());
                            number_string.push(self.scanner.unwrap_next());

                            //go on looking for digits
                            while self.scanner.is_peek_ascii_digit()
                            {
                               number_string.push(self.scanner.unwrap_next());
                            }
                        }*/
                    }
                    match number_string.parse::<f64>() {
                        Ok(number) => {
                            opt_token_kind = Some(TokenKind::Number(Value::Number(number)));
                        }
                        Err(_) => {
                            self.error_logger.log(LoxError::parser_error(ParserErrorKind::ParseFloatError(number_string), Position { line: self.scanner.line(), column: self.scanner.column() }));
                            opt_token_kind = Some(TokenKind::Number(Value::Number(f64::NAN)));
                        }
                    }

                },
                ch if is_identifier(ch) =>
                {
                    let mut identifier = String::from(ch);
                    loop {
                        let opt_next_ch: Option<char> = self.scanner.peek();
                        if opt_next_ch.is_none() {
                            break;
                        }

                        let next_ch = opt_next_ch.unwrap();
                        if !is_identifier_char_allowed(next_ch) {
                            break;
                        }
                        identifier.push(self.scanner.next().unwrap());

                    }
                    if let Some(keyword_token) = find_keyword(identifier.as_str()) {
                        opt_token_kind = Some(keyword_token);

                    } else {
                        let symbol = self.string_interner.get_or_intern(identifier);
                        opt_token_kind  = Some(TokenKind::Identifier(Identifier {name: symbol, position: Position { line: self.scanner.line(), column: self.scanner.column() }}));
                    }
                },
                _ =>
                {
                    self.error_logger.log(LoxError::parser_error(ParserErrorKind::UnexpectedToken(ch), Position { line: self.scanner.line(), column: self.scanner.column() }));
                    opt_token_kind = Some(TokenKind::UnexpectedToken);
                }
            }
            if let Some(token_kind) = opt_token_kind {
                return Some(
                    Token{
                        kind: token_kind,
                        position: Position {
                            line: line_start,
                            column: column_start
                        }
                    }
                );
            }
        }
    }
}

const fn is_identifier(ch: char) -> bool
{
    ch.is_ascii_alphabetic() || ch == '_'
}

const fn is_identifier_char_allowed(ch: char) -> bool
{
    ch.is_ascii_alphabetic() || ch == '_' || ch.is_ascii_digit()
}

const fn is_alpha(c: char) -> bool{
    c.is_ascii_alphabetic() ||  c == '_'
}

#[cfg(test)]
mod tests {
    use std::{fs, rc::Rc};

    use string_interner::StringInterner;

    use crate::{error::ConsoleErrorLogger, lexer::Identifier, tokens::{Token, TokenKind}, value::Value};

    use super::Lexer;

    fn tokenize(code: &str) -> Vec<Token>
    {
        let mut interner = StringInterner::default();
        let lexer = Lexer::new(code, &mut interner, ConsoleErrorLogger{});
        lexer.collect()
    }

    fn tokenize_with_interner(code: &str, string_interner: &mut StringInterner) -> Vec<Token>
    {
        let lexer = Lexer::new(code, string_interner, ConsoleErrorLogger{});
        lexer.collect()
    }

    #[test]
    fn test_parens()
    {
        use crate::tokens::TokenKind;

        assert_eq!(&tokenize("{").get(0).unwrap().kind, &TokenKind::LeftBrace);
        assert_eq!(&tokenize("}").get(0).unwrap().kind, &TokenKind::RightBrace);
        assert_eq!(&tokenize("(").get(0).unwrap().kind, &TokenKind::LeftParen);
        assert_eq!(&tokenize(")").get(0).unwrap().kind, &TokenKind::RightParen);
        let tokens = tokenize("({ })");
        assert_eq!(&tokens.get(0).unwrap().kind, &TokenKind::LeftParen);
        assert_eq!(&tokens.get(1).unwrap().kind, &TokenKind::LeftBrace);
        assert_eq!(&tokens.get(2).unwrap().kind, &TokenKind::RightBrace);
        assert_eq!(&tokens.get(3).unwrap().kind, &TokenKind::RightParen);
    }

    #[test]
    fn test_equalities()
    {
        assert_eq!(&tokenize("=").get(0).unwrap().kind,  &TokenKind::Equal);
        assert_eq!(&tokenize("!").get(0).unwrap().kind,  &TokenKind::Bang);
        assert_eq!(&tokenize("==").get(0).unwrap().kind, &TokenKind::EqualEqual);
        assert_eq!(&tokenize("!=").get(0).unwrap().kind, &TokenKind::BangEqual);
        assert_eq!(&tokenize(">").get(0).unwrap().kind,  &TokenKind::Greater);
        assert_eq!(&tokenize(">=").get(0).unwrap().kind, &TokenKind::GreaterEqual);
        assert_eq!(&tokenize("<").get(0).unwrap().kind,  &TokenKind::Less);
        assert_eq!(&tokenize("<=").get(0).unwrap().kind, &TokenKind::LessEqual);
        let tokens = tokenize("==!=<=>> =");
        assert_eq!(&tokens.get(0).unwrap().kind, &TokenKind::EqualEqual);
        assert_eq!(&tokens.get(1).unwrap().kind, &TokenKind::BangEqual);
        assert_eq!(&tokens.get(2).unwrap().kind, &TokenKind::LessEqual);
        assert_eq!(&tokens.get(3).unwrap().kind, &TokenKind::Greater);
        assert_eq!(&tokens.get(4).unwrap().kind, &TokenKind::Greater);
        assert_eq!(&tokens.get(5).unwrap().kind, &TokenKind::Equal);
    }

    #[test]
    fn test_numbers()
    {
        let vec = tokenize("0000.0000245");
        let token = &vec.get(0).unwrap().kind;
        assert_eq!(token, &TokenKind::Number(Value::Number(0.0000245)));

        let vec = tokenize("0001.. ..");
        let token = &vec.get(0).unwrap().kind;
        assert_eq!(token, &TokenKind::Number(Value::Number(1.0)));

        let vec = tokenize("10.0245");
        let token = &vec.get(0).unwrap().kind;
        assert_eq!(token, &TokenKind::Number(Value::Number(10.0245)));

        let vec = tokenize("8 .1.0");
        let token = &vec.get(0).unwrap().kind;
        assert_eq!(token, &TokenKind::Number(Value::Number(8.0)));
        let token = &vec.get(1).unwrap().kind;
        assert_eq!(token, &TokenKind::Dot);
        let token = &vec.get(2).unwrap().kind;
        assert_eq!(token, &TokenKind::Number(Value::Number(1.0)));

        let vec = tokenize("0.001");
        let token = &vec.get(0).unwrap().kind;
        assert_eq!(token, &TokenKind::Number(Value::Number(0.001)));

        let vec = tokenize("9.9");
        let token = &vec.get(0).unwrap().kind;
        assert_eq!(token, &TokenKind::Number(Value::Number(9.9)));

        let vec = tokenize("1.");
        let token = &vec.get(0).unwrap().kind;
        assert_eq!(token, &TokenKind::Number(Value::Number(1.0)));
        let token = &vec.get(1).unwrap().kind;
        assert_eq!(token, &TokenKind::Dot);

        let vec = tokenize(".1");
        let token = &vec.get(0).unwrap().kind;
        assert_eq!(token, &TokenKind::Dot);
        let token = &vec.get(1).unwrap().kind;
        assert_eq!(token, &TokenKind::Number(Value::Number(1.0)));

    }

    #[test]
    fn test_strings()
    {
        let vec = tokenize("\"funzionerà? 😀 成\"");
        let token = &vec.get(0).unwrap().kind;
        assert_eq!(token, &TokenKind::String(Value::String(Rc::new("funzionerà? 😀 成".to_owned()))));

        let vec = tokenize("\"\\n \\0 \\r \\t \\\\ \\\"\"");
        let token = &vec.get(0).unwrap().kind;
        assert_eq!(token, &TokenKind::String(Value::String(Rc::new("\n \0 \r \t \\ \"".to_owned()))));
    }

    #[test]
    fn test_keywords()
    {
        assert_eq!(&tokenize("true").get(0).unwrap().kind, &TokenKind::True(Value::Bool(true)));
        assert_eq!(&tokenize("false").get(0).unwrap().kind, &TokenKind::False(Value::Bool(false)));
        assert_eq!(tokenize("if").get(0).unwrap().kind, TokenKind::If);
        assert_eq!(tokenize("else").get(0).unwrap().kind, TokenKind::Else);
        assert_eq!(tokenize("for").get(0).unwrap().kind, TokenKind::For);
        assert_eq!(tokenize("while").get(0).unwrap().kind, TokenKind::While);
        assert_eq!(tokenize("or").get(0).unwrap().kind, TokenKind::Or);
        assert_eq!(tokenize("and").get(0).unwrap().kind, TokenKind::And);
        assert_eq!(tokenize("class").get(0).unwrap().kind, TokenKind::Class);
        assert_eq!(tokenize("fun").get(0).unwrap().kind, TokenKind::Fun);
        assert_eq!(tokenize("super").get(0).unwrap().kind, TokenKind::Super);
        assert_eq!(tokenize("this").get(0).unwrap().kind, TokenKind::This);
        assert_eq!(tokenize("var").get(0).unwrap().kind, TokenKind::Var);
        assert_eq!(tokenize("nil").get(0).unwrap().kind, TokenKind::Nil);
        assert_eq!(tokenize("print").get(0).unwrap().kind, TokenKind::Print);
        assert_eq!(tokenize("return").get(0).unwrap().kind, TokenKind::Return);

        assert_eq!(&tokenize("true!").get(0).unwrap().kind, &TokenKind::True(Value::Bool(true)));
        assert_eq!(&tokenize("false) ").get(0).unwrap().kind, &TokenKind::False(Value::Bool(false)));
        assert_eq!(tokenize(" if else ").get(0).unwrap().kind, TokenKind::If);
    }

    #[test]
    fn test_others()
    {
        assert_eq!(tokenize("+").get(0).unwrap().kind, TokenKind::Plus);
        assert_eq!(tokenize("-").get(0).unwrap().kind, TokenKind::Minus);
        assert_eq!(tokenize("/").get(0).unwrap().kind, TokenKind::Slash);
        assert_eq!(tokenize("*").get(0).unwrap().kind, TokenKind::Star);
        assert_eq!(tokenize(".").get(0).unwrap().kind, TokenKind::Dot);
        assert_eq!(tokenize(",").get(0).unwrap().kind, TokenKind::Comma);
        assert_eq!(tokenize(";").get(0).unwrap().kind, TokenKind::Semicolon);
    }

    #[test]
    fn test_unexpected_tokens()
    {
        assert_eq!(tokenize(":").get(0).unwrap().kind, TokenKind::UnexpectedToken);
        assert_eq!(tokenize("&").get(0).unwrap().kind, TokenKind::UnexpectedToken);
        assert_eq!(tokenize("&&").get(0).unwrap().kind, TokenKind::UnexpectedToken);
        assert_eq!(tokenize("|").get(0).unwrap().kind, TokenKind::UnexpectedToken);
        assert_eq!(tokenize("||").get(0).unwrap().kind, TokenKind::UnexpectedToken);
    }

    #[test]
    fn test_identifier_file() {
        test("./lox_test/scanning/identifiers.lox");
    }

    #[test]
    fn test_keywords_file() {
        test("./lox_test/scanning/keywords.lox");
    }

    #[test]
    fn test_numbers_file() {
        test("./lox_test/scanning/numbers.lox");
    }

    #[test]
    fn test_punctuators_file() {
        test("./lox_test/scanning/punctuators.lox");
    }

    #[test]
    fn test_strings_file() {
        test("./lox_test/scanning/strings.lox");
    }

    #[test]
    fn test_whitespace_file() {
        test("./lox_test/scanning/whitespace.lox");
    }

    #[test]
    fn test_literals_file() {
        test("./lox_test/scanning/numbers2.lox");
    }

    fn test(path: &str) {
        let code: String = fs::read_to_string(path).unwrap();
        let mut string_interner: StringInterner = StringInterner::default();
        let result: Vec<Token> = tokenize_with_interner(code.as_str(), &mut string_interner);
        let expect = regex::Regex::new(r"// expect: ").expect("Errore nell'espressione regolare 'expect'");
        let mut expected_results: Vec<Vec<&str>> = Vec::new();
        for line in code.lines()
        {
            if expect.is_match(line)
            {
                let filter_expect: Vec<&str> = line.split("// expect: ").collect();
                if let Some(result) = filter_expect.last()
                {
                    let expected_result: Vec<&str> = result.split(" ").into_iter().collect();
                    expected_results.push(expected_result);
                }
            }
        }

        let it = expected_results.iter().zip(result.iter());
        for (_, (expected, token)) in it.enumerate() {
            match expected[0] {
                "NUMBER" => {
                    let expected_tk_kind = TokenKind::Number(Value::Number(expected[2].parse::<f64>().unwrap()));
                    match (&token.kind, &expected_tk_kind) {
                        (TokenKind::Number(Value::Number(number)), TokenKind::Number(Value::Number(expected))) => {
                            assert_eq!(number, expected)
                        },
                        _ => {
                            panic!("expected identifier got {}", token.kind)
                        }
                    }
                }
                //strings
                "STRING" => {
                    let expected_tk_kind = TokenKind::String(Value::String(Rc::new(expected[2].to_string())));
                    match (&token.kind, &expected_tk_kind) {
                        (TokenKind::String(Value::String(string)), TokenKind::String(Value::String(expected))) => {
                            assert_eq!(string, expected)
                        },
                        _ => {
                            panic!("expected string got {}", token.kind)
                        }
                    }
                },
                "IDENTIFIER" => {
                    let id = string_interner.get_or_intern(expected[1]);
                    let expected_tk_kind = TokenKind::Identifier(Identifier { name: id, position: token.position });
                    match (&token.kind, &expected_tk_kind) {
                        (TokenKind::Identifier(identifier), TokenKind::Identifier(expected_identifier)) => {
                            assert_eq!(identifier.name, expected_identifier.name)
                        },
                        _ => {
                            panic!("expected identifier got {}", token.kind)
                        }
                    }
                },
                "DOT" => {
                    assert_eq!(token.kind, TokenKind::Dot)
                },
                "EOF" => {
                    assert_eq!(token.kind, TokenKind::Eof)
                },
                //keywords
                "CLASS" => {
                    assert_eq!(token.kind, TokenKind::Class)
                }
                "ELSE"=> {
                    assert_eq!(token.kind, TokenKind::Else)
                }
                "FALSE"=> {
                    assert_eq!(token.kind, TokenKind::False(Value::Bool(false)));
                }
                "FOR"=> {
                    assert_eq!(token.kind, TokenKind::For);
                }
                "FUN"=> {
                    assert_eq!(token.kind, TokenKind::Fun);
                }
                "IF"=> {
                    assert_eq!(token.kind, TokenKind::If);
                }
                "NIL"=> {
                    assert_eq!(token.kind, TokenKind::Nil);
                }
                "OR"=> {
                    assert_eq!(token.kind, TokenKind::Or);
                }
                "RETURN"=> {
                    assert_eq!(token.kind, TokenKind::Return);
                }
                "SUPER"=> {
                    assert_eq!(token.kind, TokenKind::Super);
                }
                "THIS"=> {
                    assert_eq!(token.kind, TokenKind::This);
                }
                "TRUE"=> {
                    assert_eq!(token.kind, TokenKind::True(Value::Bool(true)));
                }
                "VAR"=> {
                    assert_eq!(token.kind, TokenKind::Var);
                }
                "WHILE" => {
                    assert_eq!(token.kind, TokenKind::While);
                }
                "AND" => {
                    assert_eq!(token.kind, TokenKind::And);
                },
                //punctuators
                "LEFT_PAREN" => {
                    assert_eq!(token.kind, TokenKind::LeftParen);
                },
                "RIGHT_PAREN" => {
                    assert_eq!(token.kind, TokenKind::RightParen);
                },
                "LEFT_BRACE" => {
                    assert_eq!(token.kind, TokenKind::LeftBrace);
                },
                "RIGHT_BRACE" => {
                    assert_eq!(token.kind, TokenKind::RightBrace);
                },
                "SEMICOLON" => {
                    assert_eq!(token.kind, TokenKind::Semicolon);
                },
                "COMMA" => {
                    assert_eq!(token.kind, TokenKind::Comma);
                },
                "PLUS" => {
                    assert_eq!(token.kind, TokenKind::Plus);
                },
                "MINUS" => {
                    assert_eq!(token.kind, TokenKind::Minus);
                },
                "STAR" => {
                    assert_eq!(token.kind, TokenKind::Star);
                },
                "BANG_EQUAL" => {
                    assert_eq!(token.kind, TokenKind::BangEqual);
                },
                "EQUAL_EQUAL" => {
                    assert_eq!(token.kind, TokenKind::EqualEqual);
                },
                "LESS_EQUAL" => {
                    assert_eq!(token.kind, TokenKind::LessEqual);
                },
                "GREATER_EQUAL" => {
                    assert_eq!(token.kind, TokenKind::GreaterEqual);
                },
                "LESS" => {
                    assert_eq!(token.kind, TokenKind::Less);
                },
                "GREATER" => {
                    assert_eq!(token.kind, TokenKind::Greater);
                },
                "SLASH" => {
                    assert_eq!(token.kind, TokenKind::Slash);
                },

                 _ => {
                    panic!("unexpected type")
                 }
            }
        }
    }


}