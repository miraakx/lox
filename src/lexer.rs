use std::{collections::{HashMap}, str::{Chars}};

use crate::{LoxError, LoxErrorKind, common::NthPeekable};

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    LeftParen,  RightParen, 
    LeftBrace,  RightBrace,
    
    Comma,      Dot,     Semicolon,
    Minus,      Plus, 
    Slash,      Star,

    Bang,       BangEqual,
    Equal,      EqualEqual,
    Greater,    GreaterEqual,
    Less,       LessEqual,

    Identifier(String), String(String),  Number(f64),

    True,       False,
    If,         Else,
    For,        While,
    And,        Or,
    Class,      Fun,   
    Super,      This,
    Var,        Nil, 
    Print,      Return, 

    UnexpectedToken(char)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub position: Position,
    pub errors: Vec<LoxError>
}

impl Token {

    #[inline]
    fn new(kind: TokenKind, position: Position) -> Token{
        Token{ kind, position, errors: vec!() }
    }

    #[inline]
    fn error(kind: TokenKind, position: Position, error: LoxError) -> Token{
        Token{ kind, position, errors: vec!(error) }
    }

    #[inline]
    fn errors(kind: TokenKind, position: Position, errors: Vec<LoxError>) -> Token{
        Token{ kind, position, errors: errors }
    }

    #[inline]
    fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub column: usize,
    pub line: usize   
}

const LINE_START_INDEX: usize = 0;
const COLUMN_START_INDEX: usize = 0;

struct Scanner<'a> {
    iter: NthPeekable<Chars<'a>, char>,
    position: Position
}

impl <'a> Scanner<'a> {

    fn next(&mut self) -> Option<char> {
        self.iter.next()
    }

    #[inline]
    fn new_line(&mut self) {
        self.position.line = LINE_START_INDEX;
        self.position.column = self.position.column + 1;
    }

    #[inline]
    fn from_str(str: &'a str) -> Scanner<'a> {
        Scanner { iter: NthPeekable::new(str.chars(), 2), position: Position { line: LINE_START_INDEX, column: COLUMN_START_INDEX } }
    }

    #[inline]
    fn peek(&mut self) -> Option<char>{ 
        self.iter.peek().cloned()
    }

    fn peek_nth(&mut self, index: usize) -> Option<char> {
        self.iter.peek_nth(index).cloned()
    }

    #[inline]
    fn is_peek(&mut self, ch: char) -> bool {
        self.peek().map_or(false, |v| v==ch)        
    }

    #[inline]
    fn consume_if_peek_is(&mut self, ch: char) {
        if let Some(next_ch) = self.peek() {
            if next_ch == ch {
                self.next();
            }
        }
    }    
}

pub struct Lexer<'a> {
    keywords_map: HashMap<&'a str, TokenKind>,
    scanner: Scanner<'a>,
    flg_end: bool
}

impl <'a> Lexer<'a> {
    
    pub fn new(code: &'a str) -> Self {

        Lexer {
            keywords_map: HashMap::from([
                            (TRUE,      TokenKind::True),
                            (FALSE,     TokenKind::False),
                            (IF,        TokenKind::If),
                            (ELSE,      TokenKind::Else),
                            (FOR,       TokenKind::For),
                            (WHILE,     TokenKind::While),
                            (OR,        TokenKind::Or),
                            (AND,       TokenKind::And),
                            (CLASS,     TokenKind::Class),
                            (FUN,       TokenKind::Fun),
                            (SUPER,     TokenKind::Super),
                            (THIS,      TokenKind::This),
                            (VAR,       TokenKind::Var),
                            (NIL,       TokenKind::Nil),
                            (PRINT,     TokenKind::Print),
                            (RETURN,    TokenKind::Return)
                        ]),
            scanner: Scanner::from_str(code),
            flg_end: false
        }
        
    }
}

impl <'a> Iterator for Lexer<'a> {
    
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            
            let opt_ch: Option<char> = self.scanner.next();
            
            if opt_ch.is_none() {
                return None;
            }
            
            let ch: char = opt_ch.unwrap();
            
            match ch {
                SPACE | TAB => {
                },
                LINE_FEED => {
                    self.scanner.new_line();
                },
                CARRIAGE_RETURN => {
                    self.scanner.new_line();
                    self.scanner.consume_if_peek_is(LINE_FEED);
                },
                LEFT_PAREN => {
                    return Some(Token::new(TokenKind::LeftParen, self.scanner.position));
                },
                RIGHT_PAREN => {
                    return Some(Token::new(TokenKind::RightParen, self.scanner.position));
                },
                LEFT_BRACE => {
                    return Some(Token::new(TokenKind::LeftBrace, self.scanner.position));
                },
                RIGHT_BRACE => {
                    return Some(Token::new(TokenKind::RightBrace, self.scanner.position));
                },
                COMMA => {
                    return Some(Token::new(TokenKind::Comma, self.scanner.position));
                },
                DOT => {
                    return Some(Token::new(TokenKind::Dot, self.scanner.position));
                }, 
                SEMICOLON => {
                    return Some(Token::new(TokenKind::Semicolon, self.scanner.position));
                },
                MINUS => {
                    return Some(Token::new(TokenKind::Minus, self.scanner.position));
                },
                PLUS => {
                    return Some(Token::new(TokenKind::Plus, self.scanner.position));
                },
                STAR => {
                    return Some(Token::new(TokenKind::Star, self.scanner.position));
                },
                EQUAL => {
                    if self.scanner.is_peek(EQUAL) {
                        self.scanner.next();
                        return Some(Token::new(TokenKind::EqualEqual, self.scanner.position));
                    } else {
                        return Some(Token::new(TokenKind::Equal, self.scanner.position));
                    }
                },
                BANG => {
                    if self.scanner.is_peek(EQUAL) {
                        self.scanner.next();
                        return Some(Token::new(TokenKind::BangEqual, self.scanner.position));
                    } else {
                        return Some(Token::new(TokenKind::Bang, self.scanner.position));
                    }
                },
                GREATER => {
                    if self.scanner.is_peek(EQUAL) {
                        self.scanner.next();
                        return Some(Token::new(TokenKind::GreaterEqual, self.scanner.position));
                    } else {
                        return Some(Token::new(TokenKind::Greater, self.scanner.position));
                    }
                },
                LESS => {
                    if self.scanner.is_peek(EQUAL) {
                        self.scanner.next();
                        return Some(Token::new(TokenKind::LessEqual, self.scanner.position));
                    } else {
                        return Some(Token::new(TokenKind::Less, self.scanner.position));
                    }
                },
                SLASH => {
                    if !self.scanner.is_peek(SLASH) {
                        return Some(Token::new(TokenKind::Slash, self.scanner.position));
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
                QUOTE => {
                   return Some(string(&mut self.scanner));          
                },
                ch if is_number(ch) => {
                   return Some(number(ch, &mut self.scanner));
                },
                ch if is_identifier(ch) => {
                   return Some(identifier(ch, &mut self.scanner, &self.keywords_map));
                },
                _ => {
                   return Some(Token::error(TokenKind::UnexpectedToken(ch), self.scanner.position, LoxError::new(LoxErrorKind::UnexpectedToken(ch), self.scanner.position)));
                }

            }
        }
    }
}

pub fn tokenize(code: &str) -> Vec<Token> {
    Lexer::new(code).collect()
}

#[inline]
fn is_identifier_char_allowed(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_' || ch.is_digit(10)
}

#[inline]
fn is_identifier(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn identifier(first_char: char, scanner: &mut Scanner, keywords_map: &HashMap<&str, TokenKind>) -> Token {

    let mut identifier = String::from(first_char);
    let position_clone = scanner.position.clone();

    loop {

        let opt_next_ch: Option<char> = scanner.peek();

        if opt_next_ch.is_none() {
            break;
        } 

        let next_ch = opt_next_ch.unwrap();

        if !is_identifier_char_allowed(next_ch) {
            break;
        }

        identifier.push(scanner.next().unwrap());

    }
    if let Some(keyword_token) = keywords_map.get(identifier.as_str()) {
        Token::new(keyword_token.clone(), position_clone)
    } else {
        Token::new(TokenKind::Identifier(identifier), position_clone)
    }

}

#[inline]
fn is_number(ch: char) -> bool {
    ch.is_digit(10)
}

fn number(first_digit: char, scanner: &mut Scanner) -> Token {

    let mut flg_decimal = false;
    let mut number_string = String::from(first_digit);
    let position_clone = scanner.position.clone();
    
    loop {

        let opt_next_ch: Option<char> = scanner.peek();
        
        if opt_next_ch.is_none() {
            break;
        }

        let next_ch = opt_next_ch.unwrap();
        let opt_next_next_ch: Option<char> = scanner.peek_nth(1);

        if is_number(next_ch) {
            number_string.push(scanner.next().unwrap());
        } else if next_ch == '.' && opt_next_next_ch.is_some() && is_number(opt_next_next_ch.unwrap()) && !flg_decimal {
            flg_decimal = true;
            number_string.push(scanner.next().unwrap());
            number_string.push(scanner.next().unwrap());
        } else {
            break;
        }

    }
    let r_number = number_string.parse::<f64>();
    match r_number {
        Ok(number) => {
            Token::new(TokenKind::Number(number), position_clone)
        }
        Err(_) => {
            Token::error(TokenKind::Number(0.0), position_clone, LoxError::new(LoxErrorKind::ParseFloatError(number_string), position_clone))
        }
    }
    
}

fn string(scanner: &mut Scanner) -> Token {
    
    let mut string = String::new();
    let mut errors: Vec<LoxError> = vec!();
    let position_clone = scanner.position.clone();

    loop {

        let value: Option<char> = scanner.next();

        if value.is_none() {
            errors.push(LoxError::new(LoxErrorKind::UnterminatedString, scanner.position));
            return Token::errors(TokenKind::String(string), scanner.position, errors);
        }
        
        let ch = value.unwrap();
        
        match ch {
            BACK_SLASH => {
                if let Some(next_ch) = scanner.peek() {
                    match next_ch {
                        'n' => { 
                            scanner.next();
                            string.push('\n');
                        },
                        'r' => { 
                            scanner.next();
                            string.push('\r');
                        },
                        't' => { 
                            scanner.next();
                            string.push('\t');
                        },
                        '\\' => { 
                            scanner.next();
                            string.push('\\');
                        },
                        '0' => { 
                            scanner.next();
                            string.push('\0');
                        },
                        '"' => { 
                            scanner.next();
                            string.push('"');
                        },
                        _=> {
                            string.push(ch);
                            errors.push(LoxError::new(LoxErrorKind::InvalidEscapeCharacter, scanner.position));
                        }
                    }
                }                    
            },
            '"' => {
                return Token::new(TokenKind::String(string), position_clone);
            },
            _ => {
                string.push(ch);
            }
        }
    }
}

const SPACE:            char = ' ';
const TAB:              char = '\t';

const CARRIAGE_RETURN:  char = '\r';
const LINE_FEED:        char = '\n';

const LEFT_PAREN:       char = '(';
const RIGHT_PAREN:      char = ')';

const LEFT_BRACE:       char = '{';
const RIGHT_BRACE:      char = '}';

const COMMA:            char = ',';
const DOT:              char = '.';
const SEMICOLON:        char = ';';

const MINUS:            char = '-';
const PLUS:             char = '+';
const STAR:             char = '*';

const BANG:             char = '!';
const EQUAL:            char = '=';
const LESS:             char = '<';
const GREATER:          char = '>';

const SLASH:            char = '/';
const BACK_SLASH:       char = '\\';

const QUOTE:            char = '"';

const TRUE:            &str = "true";
const FALSE:           &str = "false";
const IF:              &str = "if";
const ELSE:            &str = "else";
const FOR:             &str = "for";      
const WHILE:           &str = "while"; 
const OR:              &str = "or"; 
const AND:             &str = "and"; 
const CLASS:           &str = "class"; 
const FUN:             &str = "fun"; 
const SUPER:           &str = "super"; 
const THIS:            &str = "this"; 
const VAR:             &str = "var"; 
const NIL:             &str = "nil"; 
const PRINT:           &str = "print"; 
const RETURN:          &str = "return"; 

#[test]
fn test_parens() {
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
fn test_equalities() {
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
fn test_numbers() {
    assert_eq!(tokenize("10.0245").get(0).unwrap().kind, TokenKind::Number(10.0245));
    assert_eq!(tokenize("0000.0000245").get(0).unwrap().kind, TokenKind::Number(0.0000245));
    assert_eq!(tokenize("0001.. ..").get(0).unwrap().kind, TokenKind::Number(1.0));
    assert_eq!(tokenize("8 .1").get(0).unwrap().kind, TokenKind::Number(8.0));
}

#[test]
fn test_strings() {
    assert_eq!(tokenize("\"funzionerà? 😀 成\"").get(0).unwrap().kind, TokenKind::String("funzionerà? 😀 成".to_owned()));
    assert_eq!(tokenize("\"\\n \\0 \\r \\t \\\\ \\\"\"").get(0).unwrap().kind, TokenKind::String("\n \0 \r \t \\ \"".to_owned()));
    assert_eq!(tokenize("\"unterminated string").get(0).unwrap().errors.get(0).unwrap().kind, LoxErrorKind::UnterminatedString);
}

#[test]
fn test_keywords() {
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
fn test_identifiers() {
    assert_eq!(tokenize("truee").get(0).unwrap().kind, TokenKind::Identifier("truee".to_owned()));
    assert_eq!(tokenize("ffalse").get(0).unwrap().kind, TokenKind::Identifier("ffalse".to_owned()));
    assert_eq!(tokenize("Nil").get(0).unwrap().kind, TokenKind::Identifier("Nil".to_owned()));
    assert_eq!(tokenize("ELSE").get(0).unwrap().kind, TokenKind::Identifier("ELSE".to_owned()));
    assert_eq!(tokenize("whilewhile").get(0).unwrap().kind, TokenKind::Identifier("whilewhile".to_owned()));
}

#[test]
fn test_others() {
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
fn test_unexpected_tokens() {
    assert_eq!(tokenize(":").get(0).unwrap().kind, TokenKind::UnexpectedToken(':'));
    assert_eq!(tokenize("&").get(0).unwrap().kind, TokenKind::UnexpectedToken('&'));
    assert_eq!(tokenize("&&").get(0).unwrap().kind, TokenKind::UnexpectedToken('&'));
    assert_eq!(tokenize("|").get(0).unwrap().kind, TokenKind::UnexpectedToken('|'));
    assert_eq!(tokenize("||").get(0).unwrap().kind, TokenKind::UnexpectedToken('|'));
}

#[test]
fn test_construct() {
    
    let tokens = tokenize("fun prova(var1, var2) {return var1+var2;}");
    let mut index: usize = 0;
    assert_eq!(tokens.get(index).unwrap().kind, TokenKind::Fun);
    index = index + 1;
    assert_eq!(tokens.get(index).unwrap().kind, TokenKind::Identifier("prova".to_owned()));
    index = index + 1;
    assert_eq!(tokens.get(index).unwrap().kind, TokenKind::LeftParen);
    index = index + 1;
    assert_eq!(tokens.get(index).unwrap().kind, TokenKind::Identifier("var1".to_owned()));
    index = index + 1;
    assert_eq!(tokens.get(index).unwrap().kind, TokenKind::Comma);
    index = index + 1;
    assert_eq!(tokens.get(index).unwrap().kind, TokenKind::Identifier("var2".to_owned()));
    index = index + 1;
    assert_eq!(tokens.get(index).unwrap().kind, TokenKind::RightParen);
    index = index + 1;
    assert_eq!(tokens.get(index).unwrap().kind, TokenKind::LeftBrace);
    index = index + 1;
    assert_eq!(tokens.get(index).unwrap().kind, TokenKind::Return);
    index = index + 1;
    assert_eq!(tokens.get(index).unwrap().kind, TokenKind::Identifier("var1".to_owned()));
    index = index + 1;
    assert_eq!(tokens.get(index).unwrap().kind, TokenKind::Plus);
    index = index + 1;
    assert_eq!(tokens.get(index).unwrap().kind, TokenKind::Identifier("var2".to_owned()));
    index = index + 1;
    assert_eq!(tokens.get(index).unwrap().kind, TokenKind::Semicolon);
    index = index + 1;
    assert_eq!(tokens.get(index).unwrap().kind, TokenKind::RightBrace);
}
