use std::rc::Rc;

use string_interner::StringInterner;

use crate::{error::*, utils::utils::Scanner};

use super::{position::Position, tokens::{Token, TokenKind}, keywords::*};

pub struct Lexer<'a>
{
    scanner        : Scanner<'a>,
    string_interner: &'a mut StringInterner,
    error_logger   : Box<dyn ErrorLogger>,
    end_of_file    : bool,
    line           : u32,
    column         : u32
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
           string_interner,
           line: 1,
           column: 1
        }
    }
}

impl <'a> Lexer<'a>
{
    #[inline]
    fn new_line(&mut self) {
        self.line   += 1;
        self.column = 1;
    }

    #[inline]
    fn advance_column(&mut self) {
        self.column += 1;
    }

    #[inline]
    fn get_position(&self) -> Position {
        Position { line: self.line, column: self.column }
    }

}

impl<'a> Iterator for Lexer<'a>
{
    type Item = Token;

    fn next(&mut self) -> Option<Token>
    {
        let mut opt_token_kind     : Option<TokenKind>;
        let mut token_start_column : u32 = self.column;
        let mut token_start_line   : u32 = self.line;
        let mut is_token_started   : bool = false;

        loop {

            let opt_ch: Option<char> = self.scanner.next();
            if opt_ch.is_none() {
                if self.end_of_file {
                    return None;
                } else {
                    self.end_of_file = true;
                    return Some(Token{ kind: TokenKind::Eof, position: self.get_position() });
                }
            }

            let ch: char = opt_ch.unwrap();

            opt_token_kind = None;

            match ch {
                SPACE | TAB => {
                    if !is_token_started {
                        token_start_column += 1;
                    }
                    self.advance_column();
                },
                //handle '\r'
                CARRIAGE_RETURN =>
                {
                    if !is_token_started {
                        token_start_line   += 1;
                        token_start_column = 1;
                    }
                    self.new_line();
                    //handle Windows new line '\r\n'
                    self.scanner.consume_if_peek_is(LINE_FEED);
                },
                //handle '\n'
                LINE_FEED =>
                {
                    if !is_token_started {
                        token_start_line   += 1;
                        token_start_column = 1;
                    }
                    self.new_line();
                },

                LEFT_PAREN =>
                {
                    is_token_started = true;
                    self.advance_column();
                    opt_token_kind = Some(TokenKind::LeftParen);
                },
                RIGHT_PAREN =>
                {
                    is_token_started = true;
                    self.advance_column();
                    opt_token_kind = Some(TokenKind::RightParen);
                },
                LEFT_BRACE =>
                {
                    is_token_started = true;
                    self.advance_column();
                    opt_token_kind = Some(TokenKind::LeftBrace);
                },
                RIGHT_BRACE =>
                {
                    is_token_started = true;
                    self.advance_column();
                    opt_token_kind = Some(TokenKind::RightBrace);
                },
                COMMA =>
                {
                    is_token_started = true;
                    self.advance_column();
                    opt_token_kind = Some(TokenKind::Comma);
                },
                DOT =>
                {
                    is_token_started = true;
                    self.advance_column();
                    opt_token_kind = Some(TokenKind::Dot);
                },
                SEMICOLON =>
                {
                    is_token_started = true;
                    self.advance_column();
                    opt_token_kind = Some(TokenKind::Semicolon);
                },
                MINUS =>
                {
                    is_token_started = true;
                    self.advance_column();
                    opt_token_kind = Some(TokenKind::Minus);
                },
                PLUS =>
                {
                    is_token_started = true;
                    self.advance_column();
                    opt_token_kind = Some(TokenKind::Plus);
                },
                STAR =>
                {
                    is_token_started = true;
                    self.advance_column();
                    opt_token_kind = Some(TokenKind::Star);
                },
                EQUAL =>
                {
                    is_token_started = true;
                    self.advance_column();
                    if self.scanner.is_peek(EQUAL) {
                        self.scanner.next();
                        self.advance_column();
                        opt_token_kind = Some(TokenKind::EqualEqual);
                    } else {
                        opt_token_kind = Some(TokenKind::Equal);
                    }
                },
                BANG =>
                {
                    is_token_started = true;
                    self.advance_column();
                    if self.scanner.is_peek(EQUAL) {
                        self.scanner.next();
                        self.advance_column();
                        opt_token_kind = Some(TokenKind::BangEqual);
                    } else {
                        opt_token_kind = Some(TokenKind::Bang);
                    }
                },
                GREATER =>
                {
                    is_token_started = true;
                    self.advance_column();
                    if self.scanner.is_peek(EQUAL) {
                        self.scanner.next();
                        self.advance_column();
                        opt_token_kind = Some(TokenKind::GreaterEqual);
                    } else {
                        opt_token_kind = Some(TokenKind::Greater);
                    }
                },
                LESS =>
                {
                    is_token_started = true;
                    self.advance_column();
                    if self.scanner.is_peek(EQUAL) {
                        self.scanner.next();
                        self.advance_column();
                        opt_token_kind = Some(TokenKind::LessEqual);
                    } else {
                        opt_token_kind = Some(TokenKind::Less);
                    }
                },
                SLASH =>
                {
                    self.advance_column();
                    if !self.scanner.is_peek(SLASH) {
                        is_token_started = true;
                        opt_token_kind = Some(TokenKind::Slash);
                    } else {
                        //consume the second slash character
                        self.scanner.next();
                        self.advance_column();

                        loop {
                            let opt_next_ch = self.scanner.peek();

                            if opt_next_ch.is_none() {
                                break;
                            }

                            let next_ch = opt_next_ch.unwrap();

                            match next_ch {
                                LINE_FEED | CARRIAGE_RETURN => {
                                    //advance line on next iteration
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
                    is_token_started = true;
                    self.advance_column();
                    let mut string = String::new();
                    loop {
                        match self.scanner.next() {
                            Some(BACK_SLASH) => {
                                self.advance_column();
                                match self.scanner.peek() {
                                    Some('n') => {
                                        self.scanner.next();
                                        self.advance_column();
                                        string.push('\n');
                                    },
                                    Some('r') => {
                                        self.scanner.next();
                                        self.advance_column();
                                        string.push('\r');
                                    },
                                    Some('t') => {
                                        self.scanner.next();
                                        self.advance_column();
                                        string.push('\t');
                                    },
                                    Some('\\') => {
                                        self.scanner.next();
                                        self.advance_column();
                                        string.push('\\');
                                    },
                                    Some('0') => {
                                        self.scanner.next();
                                        self.advance_column();
                                        string.push('\0');
                                    },
                                    Some('"') => {
                                        self.scanner.next();
                                        self.advance_column();
                                        string.push('"');
                                    },
                                    _=> {
                                        string.push(ch);
                                        self.error_logger.log(LoxError::parser_error(ParserErrorKind::InvalidEscapeCharacter, self.get_position()));
                                    }
                                }
                            },
                            Some('"') => {
                                self.advance_column();
                                opt_token_kind = Some(TokenKind::String(Rc::new(string)));
                                break;
                            },
                            None => {
                                self.error_logger.log(LoxError::parser_error(ParserErrorKind::UnterminatedString, self.get_position()));
                                opt_token_kind = Some(TokenKind::String(Rc::new(string)));
                                break;
                            },
                            Some(ch) => {
                                self.advance_column();
                                string.push(ch);
                            }
                        }
                    }
                },
                ch if ch.is_ascii_digit() =>
                {
                    is_token_started = true;
                    self.advance_column();
                    let mut number_string = String::from(ch);
                    let mut flag_decimal_point = false;

                    //read the number and stores the chars into 'number_string'
                    while self.scanner.is_peek_ascii_digit() || self.scanner.is_peek('.') && self.scanner.is_peek_next_ascii_digit() && !flag_decimal_point
                    {
                        self.advance_column();
                        if self.scanner.is_peek('.') {
                            flag_decimal_point = true;
                        }
                        number_string.push(self.scanner.unwrap_next());
                    }
                    match number_string.parse::<f64>() {
                        Ok(number) => {
                            opt_token_kind = Some(TokenKind::Number(number));
                        }
                        Err(_) => {
                            self.error_logger.log(LoxError::parser_error(ParserErrorKind::ParseFloatError(number_string), self.get_position()));
                            opt_token_kind = Some(TokenKind::Number(f64::NAN));
                        }
                    }

                },
                ch if is_identifier(ch) =>
                {
                    is_token_started = true;
                    self.advance_column();
                    let mut identifier = String::from(ch);

                    while self.scanner.is_peek_identifier_char()
                    {
                        self.advance_column();
                        identifier.push(self.scanner.unwrap_next());
                    }

                    if let Some(keyword_token) = find_keyword(identifier.as_str()) {
                        opt_token_kind = Some(keyword_token);
                    } else {
                        let symbol = self.string_interner.get_or_intern(identifier);
                        opt_token_kind  = Some(TokenKind::Identifier(symbol));
                    }
                },
                _ =>
                {
                    self.error_logger.log(LoxError::parser_error(ParserErrorKind::UnexpectedToken(ch), self.get_position()));
                    opt_token_kind = Some(TokenKind::UnexpectedToken);
                }
            }
            if let Some(token_kind) = opt_token_kind {
                return Some(
                    Token{
                        kind: token_kind,
                        position: Position {
                            line  : token_start_line,
                            column: token_start_column
                        }
                    }
                );
            }
        }
    }
}

#[inline]
const fn is_identifier(ch: char) -> bool
{
    ch.is_ascii_alphabetic() || ch == '_'
}

pub fn find_keyword(str: &str) -> Option<TokenKind>
{
    const TOKEN_FALSE: TokenKind = TokenKind::False;
    const TOKEN_TRUE:  TokenKind = TokenKind::True;
    if str.len() < IF.len() || str.len() > CONTINUE.len() {
        return None;
    }
    let mut chars = str.chars();
    match chars.next()?
    {
        'f' =>
        {
            match chars.next()?
            {
                'a' => { compare(str, FALSE, TOKEN_FALSE) },
                'o' => { compare(str, FOR,   TokenKind::For  ) },
                'u' => { compare(str, FUN,   TokenKind::Fun  ) },
                _ =>   { None }
            }
        },
        't' =>
        {
            match chars.next()?
            {
                'h' => { compare(str, THIS, TokenKind::This) },
                'r' => { compare(str, TRUE, TOKEN_TRUE) },
                _ =>   { None }
            }
        },
        'v' => { compare(str, VAR,    TokenKind::Var   ) },
        'a' => { compare(str, AND,    TokenKind::And   ) },
        'c' =>
        {
            match chars.next()?
            {
                'l' => { compare(str, CLASS, TokenKind::Class) },
                'o' => { compare(str, CONTINUE, TokenKind::Continue) },
                _ =>   { None }
            }
        },
        'e' => { compare(str, ELSE,   TokenKind::Else  ) },
        'i' => { compare(str, IF,     TokenKind::If    ) },
        'n' => { compare(str, NIL,    TokenKind::Nil   ) },
        'o' => { compare(str, OR,     TokenKind::Or    ) },
        'p' => { compare(str, PRINT,  TokenKind::Print ) },
        'r' => { compare(str, RETURN, TokenKind::Return) },
        's' => { compare(str, SUPER,  TokenKind::Super ) },
        'w' => { compare(str, WHILE,  TokenKind::While ) },
        'b' => { compare(str, BREAK,  TokenKind::Break ) },
        _ => { None }
    }
}

fn compare(str: &str, keyword: &str, token_kind: TokenKind) -> Option<TokenKind>
{
    if str.len() == keyword.len() && str.eq(keyword) {
        Some(token_kind)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, rc::Rc};

    use string_interner::StringInterner;

    use crate::{error::ConsoleErrorLogger,  parser::tokens::{Token, TokenKind}};

    use super::{Lexer, Position};

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
        use crate::parser::tokens::TokenKind;

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
        assert_eq!(token, &TokenKind::Number(0.0000245));

        let vec = tokenize("0001.. ..");
        let token = &vec.get(0).unwrap().kind;
        assert_eq!(token, &TokenKind::Number(1.0));

        let vec = tokenize("10.0245");
        let token = &vec.get(0).unwrap().kind;
        assert_eq!(token, &TokenKind::Number(10.0245));

        let vec = tokenize("8 .1.0");
        let token = &vec.get(0).unwrap().kind;
        assert_eq!(token, &TokenKind::Number(8.0));
        let token = &vec.get(1).unwrap().kind;
        assert_eq!(token, &TokenKind::Dot);
        let token = &vec.get(2).unwrap().kind;
        assert_eq!(token, &TokenKind::Number(1.0));

        let vec = tokenize("0.001");
        let token = &vec.get(0).unwrap().kind;
        assert_eq!(token, &TokenKind::Number(0.001));

        let vec = tokenize("9.9");
        let token = &vec.get(0).unwrap().kind;
        assert_eq!(token, &TokenKind::Number(9.9));

        let vec = tokenize("1.");
        let token = &vec.get(0).unwrap().kind;
        assert_eq!(token, &TokenKind::Number(1.0));
        let token = &vec.get(1).unwrap().kind;
        assert_eq!(token, &TokenKind::Dot);

        let vec = tokenize(".1");
        let token = &vec.get(0).unwrap().kind;
        assert_eq!(token, &TokenKind::Dot);
        let token = &vec.get(1).unwrap().kind;
        assert_eq!(token, &TokenKind::Number(1.0));

    }

    #[test]
    fn test_strings()
    {
        let vec = tokenize("\"funzionerà? 😀 成\"");
        let token = &vec.get(0).unwrap().kind;
        assert_eq!(token, &TokenKind::String(Rc::new("funzionerà? 😀 成".to_owned())));

        let vec = tokenize("\"\\n \\0 \\r \\t \\\\ \\\"\"");
        let token = &vec.get(0).unwrap().kind;
        assert_eq!(token, &TokenKind::String(Rc::new("\n \0 \r \t \\ \"".to_owned())));
    }

    #[test]
    fn test_keywords()
    {
        assert_eq!(&tokenize("true").get(0).unwrap().kind, &TokenKind::True);
        assert_eq!(&tokenize("false").get(0).unwrap().kind, &TokenKind::False);
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

        assert_eq!(&tokenize("true!").get(0).unwrap().kind, &TokenKind::True);
        assert_eq!(&tokenize("false) ").get(0).unwrap().kind, &TokenKind::False);
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
    fn test_no_tokens() {
        assert_eq!(tokenize("").get(0).unwrap().kind, TokenKind::Eof);
        assert_eq!(tokenize("\r\n").get(0).unwrap().kind, TokenKind::Eof);
        assert_eq!(tokenize("\n").get(0).unwrap().kind, TokenKind::Eof);
        assert_eq!(tokenize("\r").get(0).unwrap().kind, TokenKind::Eof);
        assert_eq!(tokenize("//Hello World!").get(0).unwrap().kind, TokenKind::Eof);
        assert_eq!(tokenize("//").get(0).unwrap().kind, TokenKind::Eof);
        assert_eq!(tokenize("\t").get(0).unwrap().kind, TokenKind::Eof);
        assert_eq!(tokenize("\t\t\t").get(0).unwrap().kind, TokenKind::Eof);
        assert_eq!(tokenize(" ").get(0).unwrap().kind, TokenKind::Eof);
        assert_eq!(tokenize("     ").get(0).unwrap().kind, TokenKind::Eof);
        assert_eq!(tokenize("////").get(0).unwrap().kind, TokenKind::Eof);
        assert_eq!(tokenize("//\\").get(0).unwrap().kind, TokenKind::Eof);
        assert_eq!(tokenize("//\n//\n//").get(0).unwrap().kind, TokenKind::Eof);
        assert_eq!(tokenize("//\r\n//\r\n//").get(0).unwrap().kind, TokenKind::Eof);
        assert_eq!(tokenize("\n\n\n").get(0).unwrap().kind, TokenKind::Eof);
        assert_eq!(tokenize("\r\n\r\n\r\n").get(0).unwrap().kind, TokenKind::Eof);
        assert_eq!(tokenize("\r\n\n\r\n\n").get(0).unwrap().kind, TokenKind::Eof);
        assert_eq!(tokenize("\r\n\r\r\n\r").get(0).unwrap().kind, TokenKind::Eof);
        assert_eq!(tokenize("//Hello World!\n//Hello World!\n//Hello World!").get(0).unwrap().kind, TokenKind::Eof);
    }

    #[test]
    fn test_position_1() {
        assert_eq!(tokenize("").get(0).unwrap().position, Position { line: 1, column: 1});
    }

    #[test]
    fn test_position_2() {
        assert_eq!(tokenize(" ").get(0).unwrap().position, Position { line: 1, column: 2});
    }

    #[test]
    fn test_position_3() {
        assert_eq!(tokenize("   ").get(0).unwrap().position, Position { line: 1, column: 4});
    }

    #[test]
    fn test_position_4() {
        assert_eq!(tokenize("\r\n").get(0).unwrap().position, Position { line: 2, column: 1});
    }

    #[test]
    fn test_position_5() {
        assert_eq!(tokenize("\r").get(0).unwrap().position, Position { line: 2, column: 1});
    }

    #[test]
    fn test_position_6() {
        assert_eq!(tokenize("\n").get(0).unwrap().position, Position { line: 2, column: 1});
    }

    #[test]
    fn test_position_7() {
        assert_eq!(tokenize("\n\r").get(0).unwrap().position, Position { line: 3, column: 1});
    }

    #[test]
    fn test_position_8() {
        assert_eq!(tokenize("\t").get(0).unwrap().position, Position { line: 1, column: 2});
    }

    #[test]
    fn test_position_9() {
        assert_eq!(tokenize("//Hello World!").get(0).unwrap().position, Position { line: 1, column: 3});
    }

    #[test]
    fn test_position_10() {
        assert_eq!(tokenize("//Hello World!\n").get(0).unwrap().position, Position { line: 2, column: 1});
    }

    #[test]
    fn test_position_11() {
        assert_eq!(tokenize("\r\n\n\r").get(0).unwrap().position, Position { line: 4, column: 1});
    }

    #[test]
    fn test_position_13() {
        let tokens = tokenize("var foo;");

        assert_eq!(tokens.get(0).unwrap().position, Position { line: 1, column: 1});

        assert_eq!(tokens.get(1).unwrap().position, Position { line: 1, column: 5});

        assert_eq!(tokens.get(2).unwrap().kind, TokenKind::Semicolon);
        assert_eq!(tokens.get(2).unwrap().position, Position { line: 1, column: 8});
    }

    #[test]
    fn test_position_14() {
        let tokens = tokenize("  var foo = \"  \"  ;");

        assert_eq!(tokens.get(0).unwrap().position, Position { line: 1, column: 3});

        assert_eq!(tokens.get(1).unwrap().position, Position { line: 1, column: 7});

        assert_eq!(tokens.get(2).unwrap().kind, TokenKind::Equal);
        assert_eq!(tokens.get(2).unwrap().position, Position { line: 1, column: 11});

        assert_eq!(tokens.get(3).unwrap().position, Position { line: 1, column: 13});

        assert_eq!(tokens.get(4).unwrap().kind, TokenKind::Semicolon);
        assert_eq!(tokens.get(4).unwrap().position, Position { line: 1, column: 19});
    }

    #[test]
    fn test_position_15() {
        let tokens = tokenize("\r\n\n\r  var foo = \"  \"  \t;");
        assert_eq!(tokens.get(0).unwrap().position, Position { line: 4, column: 3});

        assert_eq!(tokens.get(1).unwrap().position, Position { line: 4, column: 7});

        assert_eq!(tokens.get(2).unwrap().kind, TokenKind::Equal);
        assert_eq!(tokens.get(2).unwrap().position, Position { line: 4, column: 11});

        assert_eq!(tokens.get(3).unwrap().position, Position { line: 4, column: 13});

        assert_eq!(tokens.get(4).unwrap().kind, TokenKind::Semicolon);
        assert_eq!(tokens.get(4).unwrap().position, Position { line: 4, column: 20});
    }

    #[test]
    fn test_position_16() {
        let tokens = tokenize("class Bar < Foo {\n   do_stuff() {\n\n   print \"Hello!\";\r\n   }\r}");
        assert_eq!(tokens.get(0).unwrap().kind, TokenKind::Class);
        assert_eq!(tokens.get(0).unwrap().position, Position { line: 1, column: 1});

        assert_eq!(tokens.get(1).unwrap().position, Position { line: 1, column: 7});

        assert_eq!(tokens.get(2).unwrap().kind, TokenKind::Less);
        assert_eq!(tokens.get(2).unwrap().position, Position { line: 1, column: 11});

        assert_eq!(tokens.get(3).unwrap().position, Position { line: 1, column: 13});

        assert_eq!(tokens.get(4).unwrap().kind, TokenKind::LeftBrace);
        assert_eq!(tokens.get(4).unwrap().position, Position { line: 1, column: 17});

        assert_eq!(tokens.get(5).unwrap().position, Position { line: 2, column: 4});

        assert_eq!(tokens.get(6).unwrap().kind, TokenKind::LeftParen);
        assert_eq!(tokens.get(6).unwrap().position, Position { line: 2, column: 12});

        assert_eq!(tokens.get(7).unwrap().kind, TokenKind::RightParen);
        assert_eq!(tokens.get(7).unwrap().position, Position { line: 2, column: 13});

        assert_eq!(tokens.get(8).unwrap().kind, TokenKind::LeftBrace);
        assert_eq!(tokens.get(8).unwrap().position, Position { line: 2, column: 15});

        assert_eq!(tokens.get(9).unwrap().position, Position { line: 4, column: 4});

        assert_eq!(tokens.get(10).unwrap().position, Position { line: 4, column: 10});

        assert_eq!(tokens.get(11).unwrap().kind, TokenKind::Semicolon);
        assert_eq!(tokens.get(11).unwrap().position, Position { line: 4, column: 18});

        assert_eq!(tokens.get(12).unwrap().kind, TokenKind::RightBrace);
        assert_eq!(tokens.get(12).unwrap().position, Position { line: 5, column: 4});

        assert_eq!(tokens.get(13).unwrap().kind, TokenKind::RightBrace);
        assert_eq!(tokens.get(13).unwrap().position, Position { line: 6, column: 1});
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
                    let expected_tk_kind = TokenKind::Number(expected[2].parse::<f64>().unwrap());
                    match (&token.kind, &expected_tk_kind) {
                        (TokenKind::Number(number), TokenKind::Number(expected)) => {
                            assert_eq!(number, expected)
                        },
                        _ => {
                            panic!("expected identifier got {}", token.kind)
                        }
                    }
                }
                //strings
                "STRING" => {
                    let expected_tk_kind = TokenKind::String(Rc::new(expected[2].to_string()));
                    match (&token.kind, &expected_tk_kind) {
                        (TokenKind::String(string), TokenKind::String(expected)) => {
                            assert_eq!(string, expected)
                        },
                        _ => {
                            panic!("expected string got {}", token.kind)
                        }
                    }
                },
                "IDENTIFIER" => {
                    let id = string_interner.get_or_intern(expected[1]);
                    let expected_tk_kind = TokenKind::Identifier(id);
                    match (&token.kind, &expected_tk_kind) {
                        (TokenKind::Identifier(identifier), TokenKind::Identifier(expected_identifier)) => {
                            assert_eq!(identifier, expected_identifier)
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
                    assert_eq!(token.kind, TokenKind::False);
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
                    assert_eq!(token.kind, TokenKind::True);
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
