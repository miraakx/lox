use std::rc::Rc;
use crate::{common::Scanner, tokens::*, error::*};

pub struct Lexer<'a>
{
    scanner:       Scanner<'a>,
    error_handler: Box<dyn Fn(LoxError)>,
    end_of_file:   bool
}

impl<'a> Lexer<'a>
{
    pub fn new(code: &'a str) -> Self
    {
        Lexer
        {
           scanner:       Scanner::from_str(code, 2),
           error_handler: Box::new(println_handle_error),
           end_of_file:   false
        }
    }
}

impl<'a> Iterator for Lexer<'a>
{
    type Item = Token;

    fn next(&mut self) -> Option<Token>
    {
        let mut opt_token_kind:  Option<TokenKind>;
        let mut opt_token_value: Option<LiteralValue>;
        let column_start: u32 = self.scanner.column();
        let line_start: u32 = self.scanner.line();
        let index_start: u32 = self.scanner.index();

        loop {

            let opt_ch: Option<char> = self.scanner.next();
            if opt_ch.is_none() {
                if self.end_of_file {
                    return None;
                } else {
                    self.end_of_file = true;
                    return Some(Token{ kind: TokenKind::EOF, value: None, position: Position { line: self.scanner.line(), column: self.scanner.column()}, length: 0 });
                }
            }

            let ch: char = opt_ch.unwrap();

            opt_token_kind = None;
            opt_token_value = None;

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
                        let value: Option<char> = self.scanner.next();
                        if value.is_none() {
                            (self.error_handler)
                            (
                                LoxError
                                {
                                    kind: LoxErrorKind::UnterminatedString,
                                    position: Position
                                    {
                                        line: self.scanner.line(),
                                        column: self.scanner.column()
                                    }
                                }
                            );
                            opt_token_kind = Some(TokenKind::String);
                            opt_token_value = Some(LiteralValue::String(Rc::new(string)));
                            break;
                        }
                        let ch = value.unwrap();
                        match ch {
                            BACK_SLASH => {
                                if let Some(next_ch) = self.scanner.peek() {
                                    match next_ch {
                                        'n' => {
                                            self.scanner.next();
                                            string.push('\n');
                                        },
                                        'r' => {
                                            self.scanner.next();
                                            string.push('\r');
                                        },
                                        't' => {
                                            self.scanner.next();
                                            string.push('\t');
                                        },
                                        '\\' => {
                                            self.scanner.next();
                                            string.push('\\');
                                        },
                                        '0' => {
                                            self.scanner.next();
                                            string.push('\0');
                                        },
                                        '"' => {
                                            self.scanner.next();
                                            string.push('"');
                                        },
                                        _=> {
                                            string.push(ch);
                                            (self.error_handler)(LoxError { kind: LoxErrorKind::InvalidEscapeCharacter, position: Position { line: self.scanner.line(), column: self.scanner.column() }});
                                        }
                                    }
                                }
                            },
                            '"' => {
                                opt_token_kind = Some(TokenKind::String);
                                opt_token_value = Some(LiteralValue::String(Rc::new(string)));
                                break;
                            },
                            _ => {
                                string.push(ch);
                            }
                        }
                    }
                },
                ch if is_number(ch) =>
                {
                    let mut flg_decimal = false;
                    let mut number_string = String::from(ch);
                    loop {
                        let opt_next_ch: Option<char> = self.scanner.peek();

                        if opt_next_ch.is_none() {
                            break;
                        }

                        let next_ch = opt_next_ch.unwrap();
                        let opt_next_next_ch: Option<char> = self.scanner.peek_nth(1);

                        if is_number(next_ch) {
                            number_string.push(self.scanner.next().unwrap());
                        } else if next_ch == '.' && opt_next_next_ch.is_some() && is_number(opt_next_next_ch.unwrap()) && !flg_decimal {
                            flg_decimal = true;
                            number_string.push(self.scanner.next().unwrap());
                            number_string.push(self.scanner.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    let r_number = number_string.parse::<f64>();
                    match r_number {
                        Ok(number) => {
                            opt_token_kind = Some(TokenKind::Number);
                            opt_token_value = Some(LiteralValue::Number(number));
                        }
                        Err(_) => {
                            (self.error_handler)(LoxError { kind: LoxErrorKind::ParseFloatError(number_string), position: Position { line: self.scanner.line(), column: self.scanner.column() } });
                            opt_token_kind = Some(TokenKind::Number);
                            opt_token_value = Some(LiteralValue::Number(f64::NAN));
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
                        opt_token_value = match keyword_token {
                            TokenKind::True  => Some(LiteralValue::Bool(true)),
                            TokenKind::False => Some(LiteralValue::Bool(false)),
                            TokenKind::Nil   => Some(LiteralValue::Nil),
                            _ => None
                        };
                        opt_token_kind = Some(keyword_token);

                    } else {
                        opt_token_kind  = Some(TokenKind::Identifier);
                        opt_token_value = Some(LiteralValue::Identifier(identifier));
                    }
                },
                _ =>
                {
                    (self.error_handler)(LoxError { kind: LoxErrorKind::UnexpectedToken(ch), position: Position { line: self.scanner.line(), column: self.scanner.column() }});
                    opt_token_kind = Some(TokenKind::UnexpectedToken);
                }
            }
            if let Some(token_kind) = opt_token_kind {
                return Some(Token{ kind: token_kind, value: opt_token_value, position: Position { line: line_start, column: column_start }, length: self.scanner.index() - index_start });
            }
        }
    }
}

#[inline(always)]
fn is_identifier(ch: char) -> bool
{
    ch.is_ascii_alphabetic() || ch == '_'
}

#[inline(always)]
fn is_number(ch: char) -> bool
{
    ch.is_ascii_digit()
}

#[inline(always)]
fn is_identifier_char_allowed(ch: char) -> bool
{
    ch.is_ascii_alphabetic() || ch == '_' || ch.is_ascii_digit()
}

#[inline(always)]
fn tokenize(code: &str) -> Vec<Token>
{
    Lexer::new(code).collect()
}

#[test]
fn test_parens()
{
    assert_eq!(tokenize("{").get(0).unwrap().kind, TokenKind::LeftBrace);
    assert_eq!(tokenize("}").get(0).unwrap().kind, TokenKind::RightBrace);
    assert_eq!(tokenize("(").get(0).unwrap().kind, TokenKind::LeftParen);
    assert_eq!(tokenize(")").get(0).unwrap().kind, TokenKind::RightParen);
    let tokens = tokenize("({ })");
    assert_eq!(tokens.get(0).unwrap().kind, TokenKind::LeftParen);
    assert_eq!(tokens.get(1).unwrap().kind, TokenKind::LeftBrace);
    assert_eq!(tokens.get(2).unwrap().kind, TokenKind::RightBrace);
    assert_eq!(tokens.get(3).unwrap().kind, TokenKind::RightParen);
}

#[test]
fn test_equalities()
{
    assert_eq!(tokenize("=").get(0).unwrap().kind, TokenKind::Equal);
    assert_eq!(tokenize("!").get(0).unwrap().kind, TokenKind::Bang);
    assert_eq!(tokenize("==").get(0).unwrap().kind, TokenKind::EqualEqual);
    assert_eq!(tokenize("!=").get(0).unwrap().kind, TokenKind::BangEqual);
    assert_eq!(tokenize(">").get(0).unwrap().kind, TokenKind::Greater);
    assert_eq!(tokenize(">=").get(0).unwrap().kind, TokenKind::GreaterEqual);
    assert_eq!(tokenize("<").get(0).unwrap().kind, TokenKind::Less);
    assert_eq!(tokenize("<=").get(0).unwrap().kind, TokenKind::LessEqual);
    let tokens = tokenize("==!=<=>> =");
    assert_eq!(tokens.get(0).unwrap().kind, TokenKind::EqualEqual);
    assert_eq!(tokens.get(1).unwrap().kind, TokenKind::BangEqual);
    assert_eq!(tokens.get(2).unwrap().kind, TokenKind::LessEqual);
    assert_eq!(tokens.get(3).unwrap().kind, TokenKind::Greater);
    assert_eq!(tokens.get(4).unwrap().kind, TokenKind::Greater);
    assert_eq!(tokens.get(5).unwrap().kind, TokenKind::Equal);
}

#[test]
fn test_numbers()
{
    assert_eq!(*tokenize("10.0245").get(0).unwrap().value.as_ref().unwrap(), LiteralValue::Number(10.0245));
    assert_eq!(*tokenize("0000.0000245").get(0).unwrap().value.as_ref().unwrap(), LiteralValue::Number(0.0000245));
    assert_eq!(*tokenize("0001.. ..").get(0).unwrap().value.as_ref().unwrap(), LiteralValue::Number(1.0));
    assert_eq!(*tokenize("8 .1").get(0).unwrap().value.as_ref().unwrap(), LiteralValue::Number(8.0));
}

#[test]
fn test_strings()
{
    assert_eq!(*tokenize("\"funzionerà? 😀 成\"").get(0).unwrap().value.as_ref().unwrap(), LiteralValue::String(Rc::new("funzionerà? 😀 成".to_owned())));
    assert_eq!(*tokenize("\"\\n \\0 \\r \\t \\\\ \\\"\"").get(0).unwrap().value.as_ref().unwrap(), LiteralValue::String(Rc::new("\n \0 \r \t \\ \"".to_owned())));
    //assert_eq!(tokenize("\"unterminated string").get(0).unwrap()., LoxErrorKind::UnterminatedString);
}

#[test]
fn test_keywords()
{
    assert_eq!(tokenize("true").get(0).unwrap().kind, TokenKind::True);
    assert_eq!(tokenize("false").get(0).unwrap().kind, TokenKind::False);
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

    assert_eq!(tokenize("true!").get(0).unwrap().kind, TokenKind::True);
    assert_eq!(tokenize("false) ").get(0).unwrap().kind, TokenKind::False);
    assert_eq!(tokenize(" if else ").get(0).unwrap().kind, TokenKind::If);
}

#[test]
fn test_identifiers()
{
    assert_eq!(*tokenize("truee").get(0).unwrap().value.as_ref().unwrap(), LiteralValue::Identifier("truee".to_owned()));
    assert_eq!(*tokenize("ffalse").get(0).unwrap().value.as_ref().unwrap(), LiteralValue::Identifier("ffalse".to_owned()));
    assert_eq!(*tokenize("Nil").get(0).unwrap().value.as_ref().unwrap(), LiteralValue::Identifier("Nil".to_owned()));
    assert_eq!(*tokenize("ELSE").get(0).unwrap().value.as_ref().unwrap(), LiteralValue::Identifier("ELSE".to_owned()));
    assert_eq!(*tokenize("whilewhile").get(0).unwrap().value.as_ref().unwrap(), LiteralValue::Identifier("whilewhile".to_owned()));
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
    assert_eq!(tokenize(" \r \t \r\n \n // ////true or false?").get(0), None);
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
fn test_construct()
{
    let tokens = tokenize("fun prova(var1, var2) {return var1+var2;}");
    let mut index: usize = 0;
    assert_eq!(tokens.get(index).unwrap().kind, TokenKind::Fun);
    index = index + 1;
    assert_eq!(*tokens.get(index).unwrap().value.as_ref().unwrap(), LiteralValue::Identifier("prova".to_owned()));
    index = index + 1;
    assert_eq!(tokens.get(index).unwrap().kind, TokenKind::LeftParen);
    index = index + 1;
    assert_eq!(*tokens.get(index).unwrap().value.as_ref().unwrap(), LiteralValue::Identifier("var1".to_owned()));
    index = index + 1;
    assert_eq!(tokens.get(index).unwrap().kind, TokenKind::Comma);
    index = index + 1;
    assert_eq!(*tokens.get(index).unwrap().value.as_ref().unwrap(), LiteralValue::Identifier("var2".to_owned()));
    index = index + 1;
    assert_eq!(tokens.get(index).unwrap().kind, TokenKind::RightParen);
    index = index + 1;
    assert_eq!(tokens.get(index).unwrap().kind, TokenKind::LeftBrace);
    index = index + 1;
    assert_eq!(tokens.get(index).unwrap().kind, TokenKind::Return);
    index = index + 1;
    assert_eq!(*tokens.get(index).unwrap().value.as_ref().unwrap(), LiteralValue::Identifier("var1".to_owned()));
    index = index + 1;
    assert_eq!(tokens.get(index).unwrap().kind, TokenKind::Plus);
    index = index + 1;
    assert_eq!(*tokens.get(index).unwrap().value.as_ref().unwrap(), LiteralValue::Identifier("var2".to_owned()));
    index = index + 1;
    assert_eq!(tokens.get(index).unwrap().kind, TokenKind::Semicolon);
    index = index + 1;
    assert_eq!(tokens.get(index).unwrap().kind, TokenKind::RightBrace);
}
