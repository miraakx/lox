
use std::{fmt, rc::Rc};

use crate::{common::Peekable, error::{ParserErrorKind, LoxError}, alias::IdentifierSymbol, value::Value};

#[derive(Clone, Debug)]
pub struct Token
{
    pub kind:     TokenKind,
    pub position: Position,
    pub length:   u32,
}

#[derive(Clone, Debug)]
pub enum TokenKind
{
    LeftParen,       RightParen,
    LeftBrace,       RightBrace,
    //operators
    Comma,           Dot,           Semicolon,
    Minus,           Plus,
    Slash,           Star,
    Bang,            BangEqual,
    Equal,           EqualEqual,
    Greater,         GreaterEqual,
    Less,            LessEqual,

    If,              Else,
    For,             While,
    And,             Or,
    Class,           Fun,
    Super,           This,
    Var,             Nil,
    Print,           Return,
    True(Value),     False(Value),
    String(Value),   Number(Value),  Identifier(Identifier),
    Break,           Continue,
    UnexpectedToken,
    EOF
}

#[derive(Clone, Debug)]
pub struct Identifier {
    pub name: IdentifierSymbol,
    pub position: Position
}

#[derive(Clone, Debug)]
pub struct Operator {
    pub kind: OperatorKind,
    pub position: Position
}

#[derive(Clone, Debug)]
pub enum OperatorKind {
    Minus,           Plus,
    Slash,           Star,
    Bang,            BangEqual,
    Equal,           EqualEqual,
    Greater,         GreaterEqual,
    Less,            LessEqual,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralValue
{
    String(Rc<String>),
    Number(f64),
    Bool(bool),
    Nil,
    Identifier(IdentifierSymbol)
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

const TRUE:     &str = "true";
const FALSE:    &str = "false";
const IF:       &str = "if";
const ELSE:     &str = "else";
const FOR:      &str = "for";
const WHILE:    &str = "while";
const OR:       &str = "or";
const AND:      &str = "and";
const CLASS:    &str = "class";
const FUN:      &str = "fun";
const SUPER:    &str = "super";
pub const THIS:     &str = "this";
const VAR:      &str = "var";
const NIL:      &str = "nil";
const PRINT:    &str = "print";
const RETURN:   &str = "return";
const BREAK:    &str = "break";
const CONTINUE: &str = "continue";

pub fn find_keyword(str: &str) -> Option<TokenKind>
{
    const TOKEN_FALSE: TokenKind = TokenKind::False(Value::Bool(false));
    const TOKEN_TRUE:  TokenKind = TokenKind::True(Value::Bool(true));
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

pub type TokenSource<'a> = Peekable<&'a mut dyn Iterator<Item=Token>, Token>;

impl<'a> TokenSource<'a>
{

    pub fn consume(&mut self) {
        self.next();
    }
}

fn compare(str: &str, keyword: &str, token_kind: TokenKind) -> Option<TokenKind> {
    if str.len() == keyword.len() && str.eq(keyword) {
        Some(token_kind)
    } else {
        None
    }
}

pub fn consume(token_source: &mut TokenSource, token_kind: TokenKind) -> Result<Token,LoxError>
{
    let token = token_source.next().unwrap();
    if std::mem::discriminant(&token.kind) == std::mem::discriminant(&token_kind) {
        Ok(token)
    } else if std::mem::discriminant(&token.kind) == std::mem::discriminant(&TokenKind::EOF) {
        Err(LoxError::parser_error(ParserErrorKind::UnexpectedEndOfFile, token.position))
    } else {
        Err(LoxError::parser_error(ParserErrorKind::ExpectedToken, token.position))
    }
}

pub fn consume_identifier(token_source: &mut TokenSource) -> Result<Identifier,LoxError>
{
    let token = token_source.next().unwrap();
    match token.kind {
        TokenKind::Identifier(identifier) => {
            Ok(identifier)
        },
        TokenKind::EOF => {
            Err(LoxError::parser_error(ParserErrorKind::UnexpectedEndOfFile, token.position))
        },
        _ => {
            Err(LoxError::parser_error(ParserErrorKind::ExpectedToken, token.position))
        }
    }
}

pub fn check(token_source: &mut TokenSource, token_kind: TokenKind) -> bool {
    std::mem::discriminant(&token_source.peek().unwrap().kind) == std::mem::discriminant(&token_kind)
}

pub fn is_at_end(token_source: &mut TokenSource) -> bool {
    check(token_source, TokenKind::EOF)
}

pub fn consume_if(token_source: &mut TokenSource, token_kind: TokenKind) -> bool {
    let token = token_source.peek().unwrap();
    if std::mem::discriminant(&token.kind) == std::mem::discriminant(&token_kind) {
        token_source.consume();
            return true;
    }
    return false;
}

pub fn check_end_of_file(token_source: &mut TokenSource) -> Result<(),LoxError> {
    let peek = token_source.peek().unwrap();
    match peek.kind {
        TokenKind::EOF => {
            Err(LoxError::parser_error(ParserErrorKind::UnexpectedEndOfFile, peek.position))
        },
        _ => {
            Ok(())
        }
    }
}