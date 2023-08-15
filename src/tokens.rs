
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
    let ch = str.chars().nth(0).unwrap();
    return match ch
    {
        'a' => { if str.eq(AND  ) { Some(TokenKind::And   ) } else { None } },
        'c' => { if str.eq(CLASS) { Some(TokenKind::Class ) } else { None } },
        'e' => { if str.eq(ELSE ) { Some(TokenKind::Else  ) } else { None } },
        'f' =>
        {
            let ch = str.chars().nth(1).unwrap();
            return match ch
            {
                'a' => { if str.eq(FALSE) { Some(TokenKind::False) } else { None } },
                'o' => { if str.eq(FOR  ) { Some(TokenKind::For  ) } else { None } },
                'u' => { if str.eq(FUN  ) { Some(TokenKind::Fun  ) } else { None } },
                _ =>   { None }
            }
        },
        'i' => { if str.eq(IF    ) { Some(TokenKind::If    ) } else { None } },
        'n' => { if str.eq(NIL   ) { Some(TokenKind::Nil   ) } else { None } },
        'o' => { if str.eq(OR    ) { Some(TokenKind::Or    ) } else { None } },
        'p' => { if str.eq(PRINT ) { Some(TokenKind::Print ) } else { None } },
        'r' => { if str.eq(RETURN) { Some(TokenKind::Return) } else { None } },
        's' => { if str.eq(SUPER ) { Some(TokenKind::Super ) } else { None } },
        't' =>
        {
            let ch = str.chars().nth(1).unwrap();
            return match ch
            {
                'h' => { if str.eq(THIS) { Some(TokenKind::This) } else { None } },
                'r' => { if str.eq(TRUE) { Some(TokenKind::True) } else { None } },
                _ =>   { None }
            }
        },
        'v' => { if str.eq(VAR  ) { Some(TokenKind::Var   ) } else { None } },
        'w' => { if str.eq(WHILE) { Some(TokenKind::While ) } else { None } },
        _ => { None }
    };
}

#[inline(always)]
fn check_keyword(str: &str, keyword: &str) -> bool
{
    return keyword.eq(str);
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
