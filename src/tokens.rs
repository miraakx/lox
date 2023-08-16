
use std::{fmt, rc::Rc};

use crate::common::Peekable;

pub const SPACE:           char = ' ';
pub const TAB:             char = '\t';
pub const CARRIAGE_RETURN: char = '\r';
pub const LINE_FEED:       char = '\n';
pub const LEFT_PAREN:      char = '(';
pub const RIGHT_PAREN:     char = ')';
pub const LEFT_BRACE:      char = '{';
pub const RIGHT_BRACE:     char = '}';
pub const COMMA:           char = ',';
pub const DOT:             char = '.';
pub const SEMICOLON:       char = ';';
pub const MINUS:           char = '-';
pub const PLUS:            char = '+';
pub const STAR:            char = '*';
pub const BANG:            char = '!';
pub const EQUAL:           char = '=';
pub const LESS:            char = '<';
pub const GREATER:         char = '>';
pub const SLASH:           char = '/';
pub const BACK_SLASH:      char = '\\';
pub const QUOTE:           char = '"';

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenKind
{
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
pub enum LiteralValue
{
    String(Rc<String>),
    Number(f64),
    Bool(bool),
    Nil,
    Identifier(String)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Position
{
    pub line: u32,
    pub column: u32
}

impl fmt::Display for Position
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "line: {}, column: {}.", self.line, self.column)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token
{
    pub kind:     TokenKind,
    pub value:    Option<LiteralValue>,
    pub position: Position,
    pub length:   u32,
}

pub type TokenSource<'a> = Peekable<&'a mut dyn Iterator<Item=Token>, Token>;

impl<'a> TokenSource<'a>
{
    #[inline(always)]
    pub fn consume(&mut self) {
        self.next();
    }
}

const TRUE:   &str = "true";
const FALSE:  &str = "false";
const IF:     &str = "if";
const ELSE:   &str = "else";
const FOR:    &str = "for";
const WHILE:  &str = "while";
const OR:     &str = "or";
const AND:    &str = "and";
const CLASS:  &str = "class";
const FUN:    &str = "fun";
const SUPER:  &str = "super";
const THIS:   &str = "this";
const VAR:    &str = "var";
const NIL:    &str = "nil";
const PRINT:  &str = "print";
const RETURN: &str = "return";

pub fn find_keyword(str: &str) -> Option<TokenKind>
{
    let mut chars = str.chars();
    match chars.next()?
    {
        'f' =>
        {
            match chars.next()?
            {
                'a' => { return compare(str, FALSE, TokenKind::False); },
                'o' => { return compare(str, FOR,   TokenKind::For  ); },
                'u' => { return compare(str, FUN,   TokenKind::Fun  ); },
                _ =>   { return None; }
            };
        },
        't' =>
        {
            match chars.next()?
            {
                'h' => { return compare(str, THIS, TokenKind::This); },
                'r' => { return compare(str, TRUE, TokenKind::True); },
                _ =>   { return None; }
            };
        },
        'v' => { return compare(str, VAR,    TokenKind::Var   ); },
        'a' => { return compare(str, AND,    TokenKind::And   ); },
        'c' => { return compare(str, CLASS,  TokenKind::Class ); },
        'e' => { return compare(str, ELSE,   TokenKind::Else  ); },
        'i' => { return compare(str, IF,     TokenKind::If    ); },
        'n' => { return compare(str, NIL,    TokenKind::Nil   ); },
        'o' => { return compare(str, OR,     TokenKind::Or    ); },
        'p' => { return compare(str, PRINT,  TokenKind::Print ); },
        'r' => { return compare(str, RETURN, TokenKind::Return); },
        's' => { return compare(str, SUPER,  TokenKind::Super ); },
        'w' => { return compare(str, WHILE,  TokenKind::While ); },
        _ => { return None; }
    };
}

#[inline(always)]
fn compare(str: &str, keyword: &str, token_kind: TokenKind) -> Option<TokenKind> {
    if str.len() == keyword.len() && str.eq(keyword) {
        Some(token_kind)
    } else {
        None
    }
}

#[inline]
pub fn extract_identifier(token: Token) -> (String, Position)
{
    if let Some(value) = token.value {
        match value
        {
            crate::tokens::LiteralValue::Identifier(identifier) => {
                return (identifier, token.position);
            },
            _ => {
                panic!();
            }
        }

    } else {
        panic!();
    }
}
