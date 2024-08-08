
use std::fmt;

use string_interner::Symbol;

use crate::{alias::IdentifierSymbol, common::Peekable, error::{InternalErrorKind, LoxError, ParserErrorKind}, value::Value};

#[derive(Clone, Debug)]
pub struct Token
{
    pub kind:     TokenKind,
    pub position: Position
}

#[derive(Clone, Debug, PartialEq)]
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
    Eof
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::LeftParen        => { write!(f, "LeftParen") },
            TokenKind::RightParen       => { write!(f, "RightParen") },
            TokenKind::LeftBrace        => { write!(f, "LeftBrace") },
            TokenKind::RightBrace       => { write!(f, "RightBrace") },
            TokenKind::Comma            => { write!(f, "Comma") },
            TokenKind::Dot              => { write!(f, "Dot") },
            TokenKind::Semicolon        => { write!(f, "Semicolon") },
            TokenKind::Minus            => { write!(f, "Minus") },
            TokenKind::Plus             => { write!(f, "Plus") },
            TokenKind::Slash            => { write!(f, "Slash") },
            TokenKind::Star             => { write!(f, "Star") },
            TokenKind::Bang             => { write!(f, "Bang") },
            TokenKind::BangEqual        => { write!(f, "BangEqual") },
            TokenKind::Equal            => { write!(f, "Equal") },
            TokenKind::EqualEqual       => { write!(f, "EqualEqual") },
            TokenKind::Greater          => { write!(f, "Greater") },
            TokenKind::GreaterEqual     => { write!(f, "GreaterEqual") },
            TokenKind::Less             => { write!(f, "Less") },
            TokenKind::LessEqual        => { write!(f, "LessEqual") },
            TokenKind::If               => { write!(f, "If") },
            TokenKind::Else             => { write!(f, "Else") },
            TokenKind::For              => { write!(f, "For") },
            TokenKind::While            => { write!(f, "While") },
            TokenKind::And              => { write!(f, "And") },
            TokenKind::Or               => { write!(f, "Or") },
            TokenKind::Class            => { write!(f, "Class") },
            TokenKind::Fun              => { write!(f, "Fun") },
            TokenKind::Super            => { write!(f, "Super") },
            TokenKind::This             => { write!(f, "This") },
            TokenKind::Var              => { write!(f, "Var") },
            TokenKind::Nil              => { write!(f, "Nil") },
            TokenKind::Print            => { write!(f, "Print") },
            TokenKind::Return           => { write!(f, "Return") },
            TokenKind::True(_)          => { write!(f, "True") },
            TokenKind::False(_)         => { write!(f, "False") },
            TokenKind::String(_)        => { write!(f, "String") },
            TokenKind::Number(_)        => { write!(f, "Number") },
            TokenKind::Identifier(id)   => { write!(f, "{}", id) },
            TokenKind::Break            => { write!(f, "Break") },
            TokenKind::Continue         => { write!(f, "Continue") },
            TokenKind::UnexpectedToken  => { write!(f, "UnexpectedToken") },
            TokenKind::Eof              => { write!(f, "EndOfFile") },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Identifier {
    pub name: IdentifierSymbol,
    pub position: Position
}

impl fmt::Display for Identifier {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Identifier (Symbol={})", self.name.to_usize())
    }
}

#[derive(Clone, Debug)]
pub struct Operator<Kind>
{
    pub kind: Kind,
    pub position: Position
}

#[derive(Clone, Debug)]
pub enum BinaryOperatorKind
{
    Minus,           Plus,
    Slash,           Star,
    BangEqual,       EqualEqual,
    Greater,         GreaterEqual,
    Less,            LessEqual,
}

impl Operator<BinaryOperatorKind>
{
    pub fn from_token(token: &Token) -> Self
    {
        let bonary_op_kind = match token.kind {
            TokenKind::Minus        => BinaryOperatorKind::Minus,
            TokenKind::Plus         => BinaryOperatorKind::Plus,
            TokenKind::Slash        => BinaryOperatorKind::Slash,
            TokenKind::Star         => BinaryOperatorKind::Star,
            TokenKind::BangEqual    => BinaryOperatorKind::BangEqual,
            TokenKind::EqualEqual   => BinaryOperatorKind::EqualEqual,
            TokenKind::Greater      => BinaryOperatorKind::Greater,
            TokenKind::GreaterEqual => BinaryOperatorKind::GreaterEqual,
            TokenKind::Less         => BinaryOperatorKind::Less,
            TokenKind::LessEqual    => BinaryOperatorKind::LessEqual,
            _ => {
                panic!("Internal error, unexpecter operator type");
            }
        };
        Self { kind: bonary_op_kind, position: token.position }
    }
}


#[derive(Clone, Debug)]
pub enum UnaryOperatorKind
{
    Bang, Minus,
}

impl Operator<UnaryOperatorKind>
{
    pub fn from_token(token: &Token) -> Self
    {
        let bonary_op_kind = match token.kind {
            TokenKind::Bang  => UnaryOperatorKind::Bang,
            TokenKind::Minus => UnaryOperatorKind::Minus,
            _ => {
                panic!("Internal error, unexpecter operator type");
            }
        };
        Self { kind: bonary_op_kind, position: token.position }
    }
}

#[derive(Clone, Debug)]
pub enum LogicalOperatorKind
{
    And, Or,
}

impl Operator<LogicalOperatorKind>
{
    pub fn from_token(token: &Token) -> Self
    {
        let bonary_op_kind = match token.kind {
            TokenKind::And  => LogicalOperatorKind::And,
            TokenKind::Or => LogicalOperatorKind::Or,
            _ => {
                panic!("Internal error, unexpecter operator type");
            }
        };
        Self { kind: bonary_op_kind, position: token.position }
    }
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
pub const THIS: &str = "this";
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

pub fn consume(token_source: &mut TokenSource, token_kind: TokenKind, message: &str) -> Result<Token,LoxError>
{
    let token = token_source.peek().unwrap();
    let is_token_kind =
    if std::mem::discriminant(&token.kind) == std::mem::discriminant(&token_kind) {
        true
    } else {
        return Err(LoxError::parser_error(ParserErrorKind::ExpectedToken(message.to_string()), token.position));
    };
    if is_token_kind {
        let token = token_source.next().unwrap();
        return Ok(token);
    }

    Err(LoxError::parser_error(ParserErrorKind::ExpectedToken(message.to_string()), token.position))

}

pub fn consume_identifier(token_source: &mut TokenSource, message: &str) -> Result<Identifier, LoxError>
{
    let mut is_identifier = false;
    let position;

    match token_source.peek()
    {
        Some(token) => {
            position = token.position;
            if let TokenKind::Identifier(_) = &token.kind {
                is_identifier = true
            }
        },
        None => {
            return Err(LoxError::internal_error(InternalErrorKind::ExpectToken));
        },
    }

    if is_identifier
    {
        match token_source.next().unwrap().kind
        {
            TokenKind::Identifier(identifier) =>
            {
                return Ok(identifier);
            },
            _ => {
                return Err(LoxError::internal_error(InternalErrorKind::ExpectToken));
            }
        }
    }

    Err(LoxError::parser_error(ParserErrorKind::ExpectedIdentifier(message.to_string()), position))

}

#[inline]
pub fn check(token_source: &mut TokenSource, token_kind: TokenKind) -> bool {
    check_token(token_source.peek().unwrap(), token_kind)
}

#[inline]
pub fn check_token(token: &Token, token_kind: TokenKind) -> bool {
    std::mem::discriminant(&token.kind) == std::mem::discriminant(&token_kind)
}

#[inline]
pub fn is_at_end(token_source: &mut TokenSource) -> bool {
    check(token_source, TokenKind::Eof)
}

pub fn consume_if(token_source: &mut TokenSource, token_kind: TokenKind) -> bool {
    let token = token_source.peek().unwrap();
    if std::mem::discriminant(&token.kind) == std::mem::discriminant(&token_kind) {
        token_source.consume();
        true
    } else {
        false
    }
}