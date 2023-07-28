
use std::{collections::HashMap, fmt};

pub const SPACE:            char = ' ';
pub const TAB:              char = '\t';
pub const CARRIAGE_RETURN:  char = '\r';
pub const LINE_FEED:        char = '\n';
pub const LEFT_PAREN:       char = '(';
pub const RIGHT_PAREN:      char = ')';
pub const LEFT_BRACE:       char = '{';
pub const RIGHT_BRACE:      char = '}';
pub const COMMA:            char = ',';
pub const DOT:              char = '.';
pub const SEMICOLON:        char = ';';
pub const MINUS:            char = '-';
pub const PLUS:             char = '+';
pub const STAR:             char = '*';
pub const BANG:             char = '!';
pub const EQUAL:            char = '=';
pub const LESS:             char = '<';
pub const GREATER:          char = '>';
pub const SLASH:            char = '/';
pub const BACK_SLASH:       char = '\\';
pub const QUOTE:            char = '"';
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

#[derive(Copy, Clone, Debug, PartialEq)]
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
    True,       False,
    If,         Else,
    For,        While,
    And,        Or,
    Class,      Fun,   
    Super,      This,
    Var,        Nil, 
    Print,      Return, 
    String,     Number,  Identifier,
    UnexpectedToken,
    EOF
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String(String),  Number(f64), Bool(bool), Nil, Identifier(String), 
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub line: u32,
    pub column: u32
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line: {}, column: {}.", self.line, self.column)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind:     TokenKind,
    pub value:    Option<Literal>,
    pub position: Position
}

pub fn keyword_map<'a>() -> HashMap<&'a str, TokenKind>{
    HashMap::from(
        [
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
        ]
    )
}
