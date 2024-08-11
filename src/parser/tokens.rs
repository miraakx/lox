
use std::{fmt, rc::Rc};

use string_interner::Symbol;

use crate::{alias::IdentifierSymbol, error::{InternalErrorKind, LoxError, ParserErrorKind}, utils::utils::Peekable};

use super::{position::Position, types::Identifier};

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

    /// Expect and consume the following token only if it matches the supplied `TokenKind`.
    ///
    /// Return the given error message if not found as expected.
    pub fn consume_or_error(&mut self, token_kind: TokenKind, message: &str) -> Result<Token,LoxError>
    {
        let token = self.peek().unwrap();
        let is_token_kind =
        if std::mem::discriminant(&token.kind) == std::mem::discriminant(&token_kind) {
            true
        } else {
            return Err(LoxError::parser_error(ParserErrorKind::ExpectedToken(message.to_string()), token.position));
        };
        if is_token_kind {
            let token = self.next().unwrap();
            return Ok(token);
        }

        Err(LoxError::parser_error(ParserErrorKind::ExpectedToken(message.to_string()), token.position))

    }

    /// Expect and consume an identifier `TokenKind::Identifier`.
    ///
    /// If an dentifier is not found as expected returns an error of type `ParserErrorKind::ExpectedIdentifier` with the supplied error message.
    pub fn consume_identifier(&mut self, message: &str) -> Result<Identifier, LoxError>
    {
        let mut is_identifier = false;
        let position;

        match self.peek()
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
            let token = self.next().unwrap();
            match &token.kind
            {
                TokenKind::Identifier(identifier) =>
                {
                    return Ok(Identifier {name: *identifier, position: token.position});
                },
                _ => {
                    return Err(LoxError::internal_error(InternalErrorKind::ExpectToken));
                }
            }
        }

        Err(LoxError::parser_error(ParserErrorKind::ExpectedIdentifier(message.to_string()), position))

    }

    /// Peek the next token and check if it matches with the supplied `TokenKind`.
    ///
    /// Returns true or false.
    #[inline]
    pub fn check(&mut self, token_kind: TokenKind) -> bool {
        if let Some(peek) = self.peek() {
            std::mem::discriminant(&peek.kind,) == std::mem::discriminant(&token_kind)
        } else {
            false
        }
    }

    /// Check if the next token is a `TokenKind::Eof` (end of file token).
    ///
    /// Returns true or false.
    #[inline]
    pub fn is_at_end(&mut self) -> bool {
        self.check(TokenKind::Eof)
    }

    /// If the next token matches the supplied `TokenKind` consumes it and returns true. Otherwise returns false and leave the token unconsumed.
    pub fn consume_if(&mut self, token_kind: TokenKind) -> bool {
        let token = self.peek().unwrap();
        if std::mem::discriminant(&token.kind) == std::mem::discriminant(&token_kind) {
            self.consume();
            true
        } else {
            false
        }
    }

}
