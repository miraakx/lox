
use std::{fmt, rc::Rc};

use string_interner::Symbol;

use crate::{alias::IdentifierSymbol, utils::utils::Peekable};

use super::position::Position;

#[derive(Clone, Debug)]
pub struct Token
{
    pub kind:     TokenKind,
    pub position: Position
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind
{
    LeftParen,          RightParen,
    LeftBrace,          RightBrace,
    Comma,              Dot,
    Semicolon,
    Minus,              Plus,
    Slash,              Star,
    Bang,               BangEqual,
    Equal,              EqualEqual,
    Greater,            GreaterEqual,
    Less,               LessEqual,
    If,                 Else,
    For,                While,
    And,                Or,
    Class,              Fun,
    Super,              This,
    Var,                Nil,
    Print,              Return,
    True,               False,
    String(Rc<String>), Number(f64),  Identifier(IdentifierSymbol),
    Break,              Continue,
    UnexpectedToken,
    Eof
}

impl fmt::Display for TokenKind
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
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
            TokenKind::True             => { write!(f, "True") },
            TokenKind::False            => { write!(f, "False") },
            TokenKind::String(_)        => { write!(f, "String") },
            TokenKind::Number(_)        => { write!(f, "Number") },
            TokenKind::Identifier(id)   => { write!(f, "{}", id.to_usize()) },
            TokenKind::Break            => { write!(f, "Break") },
            TokenKind::Continue         => { write!(f, "Continue") },
            TokenKind::UnexpectedToken  => { write!(f, "UnexpectedToken") },
            TokenKind::Eof              => { write!(f, "EndOfFile") },
        }
    }
}

pub type TokenSource<'a> = Peekable<&'a mut dyn Iterator<Item=Token>, Token>;

impl<'a> TokenSource<'a>
{
    pub fn consume(&mut self)
    {
        self.next();
    }
}
