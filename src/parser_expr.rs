use once_cell::sync::Lazy;
use unique_id::{sequence::SequenceGenerator, Generator};

use crate::{tokens::{Token, Position, TokenKind, TokenSource, consume_if, consume, check}, error::{LoxError, LoxErrorKind}};

static ID_GENERATOR: Lazy<SequenceGenerator> = Lazy::new(||SequenceGenerator::default());

#[derive(Clone, Debug)]
pub struct Expr {
    pub id: i64,
    pub kind: ExprKind
}

impl Expr {
    fn new(kind: ExprKind) -> Self {
        Expr { id: ID_GENERATOR.next_id(), kind }
    }
}

#[derive(Clone, Debug)]
pub enum ExprKind
{
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Unary(Token, Box<Expr>),
    Literal(Token),
    Variable(String, Position),
    Assign(String, Box<Expr>, Position),
    Logical(Box<Expr>, Token, Box<Expr>),
    Call(Box<Expr>, Option<Vec<Expr>>, Token),
}



#[inline(always)]
pub fn expression(token_source: &mut TokenSource) -> Result<Expr,LoxError>
{
    assignment(token_source)
}

fn assignment(token_source: &mut TokenSource) -> Result<Expr,LoxError>
{
    let expr = or(token_source)?;

    let peek_token = token_source.peek().unwrap();
    //Copy position to evade borrow checker
    let position = peek_token.position;

    match peek_token.kind {
        TokenKind::EOF => {
            return Err(LoxError::new(LoxErrorKind::UnexpectedEndOfFile, position));
        },
        TokenKind::Equal => {
            token_source.consume();
            let value: Expr = assignment(token_source)?;

            match expr.kind {
                ExprKind::Variable(name, pos) => {
                    return Ok(Expr::new(ExprKind::Assign(name, Box::new(value), pos)));
                },
                _ => {
                    return Err(LoxError::new(LoxErrorKind::UnexpectedEndOfFile, position));
                }
            }
        },
        _ => {
            return Ok(expr);
        }
    }
}

fn or(token_source: &mut TokenSource) -> Result<Expr,LoxError>
{
    let mut expr = and(token_source)?;
    loop {
        let peek_token = token_source.peek().unwrap();
        match &peek_token.kind {
            TokenKind::EOF => {
                return Err(LoxError::new(LoxErrorKind::UnexpectedEndOfFile, peek_token.position));
            },
            TokenKind::Or => {
                let operator = token_source.next().unwrap();
                let right: Expr = and(token_source)?;
                expr =  Expr::new(ExprKind::Logical(Box::new(expr), operator, Box::new(right)));
            },
            _ => {
                return Ok(expr);
            }
        }
    }
}

fn and(token_source: &mut TokenSource) -> Result<Expr,LoxError>
{
    let mut expr = equality(token_source)?;
    loop {
        let peek_token = token_source.peek().unwrap();
        match &peek_token.kind {
            TokenKind::EOF => {
                return Err(LoxError::new(LoxErrorKind::UnexpectedEndOfFile, peek_token.position));
            },
            TokenKind::And => {
                let operator = token_source.next().unwrap();
                let right: Expr = equality(token_source)?;
                expr = Expr::new(ExprKind::Logical(Box::new(expr), operator, Box::new(right)));
            },
            _ => {
                return Ok(expr);
            }
        }
    }
}

fn equality(token_source: &mut TokenSource) -> Result<Expr,LoxError>
{
    let mut expr = comparison(token_source)?;
    loop {
        let peek_token = token_source.peek().unwrap();
        match &peek_token.kind {
            TokenKind::EOF => {
                return Err(LoxError::new(LoxErrorKind::UnexpectedEndOfFile, peek_token.position));
            },
            TokenKind::BangEqual|TokenKind::EqualEqual => {
                let operator: Token = token_source.next().unwrap();
                let right: Expr = comparison(token_source)?;
                expr = Expr::new(ExprKind::Binary(Box::new(expr), operator, Box::new(right)));
            },
            _ => {
                return Ok(expr);
            }
        }
    }
}

fn comparison(token_source: &mut TokenSource) -> Result<Expr,LoxError>
{
    let mut expr = term(token_source)?;
    loop {
        let peek_token = token_source.peek().unwrap();
        match &peek_token.kind {
            TokenKind::EOF => {
                return Err(LoxError::new(LoxErrorKind::UnexpectedEndOfFile, peek_token.position));
            },
            TokenKind::Greater | TokenKind::GreaterEqual | TokenKind::Less | TokenKind::LessEqual => {
                let operator = token_source.next().unwrap();
                let right: Expr = term(token_source)?;
                expr = Expr::new(ExprKind::Binary(Box::new(expr), operator, Box::new(right)));
            },
            _ => {
                return Ok(expr);
            }
        }
    }
}

fn term(token_source: &mut TokenSource) -> Result<Expr,LoxError>
{
    let mut expr: Expr = factor(token_source)?;
    loop {
        let peek_token = token_source.peek().unwrap();
        match &peek_token.kind {
            TokenKind::EOF => {
                return Err(LoxError::new(LoxErrorKind::UnexpectedEndOfFile, peek_token.position));
            },
            TokenKind::Minus | TokenKind::Plus => {
                let operator = token_source.next().unwrap();
                let right: Expr = factor(token_source)?;
                expr = Expr::new(ExprKind::Binary(Box::new(expr), operator, Box::new(right)));
            },
            _ => {
                return Ok(expr);
            }
        }
    }
}

fn factor(token_source: &mut TokenSource) -> Result<Expr, LoxError>
{
    let mut expr = unary(token_source)?;
    loop {
        let peek_token: &Token = token_source.peek().unwrap();
        match &peek_token.kind {
            TokenKind::EOF => {
                return Err(LoxError::new(LoxErrorKind::UnexpectedEndOfFile, peek_token.position));
            },
            TokenKind::Slash | TokenKind::Star => {
                let operator: Token = token_source.next().unwrap();
                let right = unary(token_source)?;
                expr = Expr::new(ExprKind::Binary(Box::new(expr), operator, Box::new(right)));
            },
            _ => {
                return Ok(expr);
            }
        }
    }
}

fn unary(token_source: &mut TokenSource) -> Result<Expr, LoxError>
{
    let peek_token = token_source.peek().unwrap();
    match &peek_token.kind {
        TokenKind::EOF => {
            Err(LoxError::new(LoxErrorKind::UnexpectedEndOfFile, peek_token.position))
        },
        TokenKind::Bang | TokenKind::Minus => {
            let operator: Token = token_source.next().unwrap();
            let right:    Expr = unary(token_source)?;
            Ok(Expr::new(ExprKind::Unary(operator, Box::new(right))))
        },
        _ => {
            call(token_source)
        }
    }
}

fn call(token_source: &mut TokenSource) -> Result<Expr, LoxError>
{
    let mut expr = primary(token_source)?;
    loop {
        if !check(token_source, TokenKind::LeftParen) {
            return Ok(expr);
        }
        let left_paren = token_source.next().unwrap();
        if consume_if(token_source, TokenKind::RightParen) {
            expr = Expr::new(ExprKind::Call(Box::new(expr), None, left_paren));
            continue;
        }
        let mut args: Vec<Expr> = Vec::new();
        loop {
            args.push(expression(token_source)?);
            if !consume_if(token_source, TokenKind::Comma) {
               break;
            }
        }
        consume(token_source, TokenKind::RightParen)?;
        if args.len() >= 255 {
            todo!("Segnalare errore se più di 255 argomenti");
        }
        expr = Expr::new(ExprKind::Call(Box::new(expr), Some(args), left_paren));
    }
}

fn primary(token_source: &mut TokenSource) -> Result<Expr, LoxError>
{
    let token = token_source.peek().unwrap();
    let position = token.position;

    match &token.kind {
        TokenKind::EOF => {
            Err(LoxError::new(LoxErrorKind::UnexpectedEndOfFile, position))
        },
        TokenKind::False|TokenKind::True|TokenKind::Nil|TokenKind::Number|TokenKind::String => {
            Ok(Expr::new(ExprKind::Literal(token_source.next().unwrap())))
        },
        TokenKind::Identifier => {
            let val = token_source.next().unwrap().get_identifier_and_position();
            Ok(Expr::new(ExprKind::Variable(val.0, val.1)))
        },
        TokenKind::LeftParen => {
            //consuma la parentesi appena trovata
            token_source.consume();
            let expr: Expr = expression(token_source)?;
            match token_source.next().unwrap().kind {
                TokenKind::EOF => {
                    return Err(LoxError::new(LoxErrorKind::UnexpectedEndOfFile, position));
                },
                TokenKind::RightParen => {
                    //consume right parenthesis if it's present
                },
                _ => {
                    return Err(LoxError::new(LoxErrorKind::MissingClosingParenthesis, position));
                }
            }
            return Ok(Expr::new(ExprKind::Grouping(Box::new(expr))));
        },
        found => {
            Err(LoxError::new(LoxErrorKind::LiteralExpected(*found), position))
        }
    }
}