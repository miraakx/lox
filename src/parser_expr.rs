use once_cell::sync::Lazy;
use unique_id::{sequence::SequenceGenerator, Generator};

use crate::{tokens::{Token, TokenKind, TokenSource, consume_if, consume, check, consume_identifier, Identifier, BinaryOperatorKind, UnaryOperatorKind, LogicalOperatorKind, Operator, Literal, Position}, error::{LoxError, ParserErrorKind}};

static ID_GENERATOR: Lazy<SequenceGenerator> = Lazy::new(SequenceGenerator::default);

#[derive(Clone, Debug)]
pub struct Expr
{
    pub id: i64,
    pub kind: ExprKind
}

impl Expr
{
    fn new(kind: ExprKind) -> Self {
        Self { id: ID_GENERATOR.next_id(), kind }
    }
}

#[derive(Clone, Debug)]
pub enum ExprKind
{
    Binary  (Box<Expr>, Operator<BinaryOperatorKind>, Box<Expr>),
    Grouping(Box<Expr>),
    Unary   (Operator<UnaryOperatorKind>, Box<Expr>),
    Literal (Literal),
    Variable(Identifier),
    Assign  (Identifier, Box<Expr>),
    Logical (Box<Expr>, Operator<LogicalOperatorKind>, Box<Expr>),
    Call    (Box<Expr>, Vec<Expr>, Position),
    Get     (Box<Expr>, Identifier),
    Set     (Box<Expr>, Identifier, Box<Expr>),
    This    (Position)
}


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
        TokenKind::Eof => {
            Err(LoxError::parser_error(ParserErrorKind::UnexpectedEndOfFile, position))
        },
        TokenKind::Equal => {
            token_source.consume();
            let value: Expr = assignment(token_source)?;

            match expr.kind {
                ExprKind::Variable(identifier) => {
                    Ok(Expr::new(ExprKind::Assign(identifier, Box::new(value))))
                },
                //Assign a value expression to an instance property
                ExprKind::Get(expr, identifier) => {
                    Ok(Expr::new(ExprKind::Set(expr, identifier, Box::new(value))))
                }
                _ => {
                    Err(LoxError::parser_error(ParserErrorKind::UnexpectedEndOfFile, position))
                }
            }
        },
        _ => {
            Ok(expr)
        }
    }
}

fn or(token_source: &mut TokenSource) -> Result<Expr,LoxError>
{
    let mut expr = and(token_source)?;
    loop {
        let peek_token = token_source.peek().unwrap();
        match &peek_token.kind {
            TokenKind::Eof => {
                return Err(LoxError::parser_error(ParserErrorKind::UnexpectedEndOfFile, peek_token.position));
            },
            TokenKind::Or => {
                let operator = token_source.next().unwrap();
                let right: Expr = and(token_source)?;
                expr =  Expr::new(ExprKind::Logical(Box::new(expr), Operator::<LogicalOperatorKind>::from_token(&operator), Box::new(right)));
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
            TokenKind::Eof => {
                return Err(LoxError::parser_error(ParserErrorKind::UnexpectedEndOfFile, peek_token.position));
            },
            TokenKind::And => {
                let operator = token_source.next().unwrap();
                let right: Expr = equality(token_source)?;
                expr = Expr::new(ExprKind::Logical(Box::new(expr), Operator::<LogicalOperatorKind>::from_token(&operator), Box::new(right)));
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
            TokenKind::Eof => {
                return Err(LoxError::parser_error(ParserErrorKind::UnexpectedEndOfFile, peek_token.position));
            },
            TokenKind::BangEqual|TokenKind::EqualEqual => {
                let operator: Token = token_source.next().unwrap();
                let right: Expr = comparison(token_source)?;
                expr = Expr::new(ExprKind::Binary(Box::new(expr), Operator::<BinaryOperatorKind>::from_token(&operator), Box::new(right)));
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
            TokenKind::Eof => {
                return Err(LoxError::parser_error(ParserErrorKind::UnexpectedEndOfFile, peek_token.position));
            },
            TokenKind::Greater | TokenKind::GreaterEqual | TokenKind::Less | TokenKind::LessEqual => {
                let operator = token_source.next().unwrap();
                let right: Expr = term(token_source)?;
                expr = Expr::new(ExprKind::Binary(Box::new(expr), Operator::<BinaryOperatorKind>::from_token(&operator), Box::new(right)));
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
            TokenKind::Eof => {
                return Err(LoxError::parser_error(ParserErrorKind::UnexpectedEndOfFile, peek_token.position));
            },
            TokenKind::Minus | TokenKind::Plus => {
                let operator = token_source.next().unwrap();
                let right: Expr = factor(token_source)?;
                expr = Expr::new(ExprKind::Binary(Box::new(expr), Operator::<BinaryOperatorKind>::from_token(&operator), Box::new(right)));
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
            TokenKind::Eof => {
                return Err(LoxError::parser_error(ParserErrorKind::UnexpectedEndOfFile, peek_token.position));
            },
            TokenKind::Slash | TokenKind::Star => {
                let operator: Token = token_source.next().unwrap();
                let right = unary(token_source)?;
                expr = Expr::new(ExprKind::Binary(Box::new(expr), Operator::<BinaryOperatorKind>::from_token(&operator), Box::new(right)));
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
        TokenKind::Eof => {
            Err(LoxError::parser_error(ParserErrorKind::UnexpectedEndOfFile, peek_token.position))
        },
        TokenKind::Bang | TokenKind::Minus => {
            let operator: Token = token_source.next().unwrap();
            let right:    Expr = unary(token_source)?;
            Ok(Expr::new(ExprKind::Unary(Operator::<UnaryOperatorKind>::from_token(&operator), Box::new(right))))
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
        if !check(token_source, TokenKind::LeftParen) && !check(token_source, TokenKind::Dot) {
            return Ok(expr);
        }
        if check(token_source, TokenKind::LeftParen)
        {
            let left_paren = token_source.next().unwrap();
            if consume_if(token_source, TokenKind::RightParen) {
                expr = Expr::new(ExprKind::Call(Box::new(expr), Vec::new(), left_paren.position));
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
            expr = Expr::new(ExprKind::Call(Box::new(expr), args, left_paren.position));
        }
        else if consume_if(token_source, TokenKind::Dot)
        {
            let identifier: crate::tokens::Identifier = consume_identifier(token_source)?;
            expr = Expr::new(ExprKind::Get(Box::new(expr), identifier));
        }
    }
}

fn primary(token_source: &mut TokenSource) -> Result<Expr, LoxError>
{
    let token = token_source.next().unwrap();
    let position = token.position;

    match &token.kind {
        TokenKind::Eof => {
            Err(LoxError::parser_error(ParserErrorKind::UnexpectedEndOfFile, position))
        },
        TokenKind::Nil => {
            Ok(Expr::new(ExprKind::Literal(Literal::from_token(&token))))
        }
        TokenKind::False(_) | TokenKind::True(_) | TokenKind::Number(_) | TokenKind::String(_) => {
            Ok(Expr::new(ExprKind::Literal(Literal::from_token(&token))))
        },
        TokenKind::Identifier(identifier) => {
             Ok(Expr::new(ExprKind::Variable(identifier.clone())))
        },
        TokenKind::LeftParen => {
            //consuma la parentesi appena trovata
            token_source.consume();
            let expr: Expr = expression(token_source)?;
            match token_source.next().unwrap().kind {
                TokenKind::Eof => {
                    return Err(LoxError::parser_error(ParserErrorKind::UnexpectedEndOfFile, position));
                },
                TokenKind::RightParen => {
                    //consume right parenthesis if it's present
                },
                _ => {
                    return Err(LoxError::parser_error(ParserErrorKind::MissingClosingParenthesis, position));
                }
            }
            Ok(Expr::new(ExprKind::Grouping(Box::new(expr))))
        },
        TokenKind::This => {
            let token = token_source.next().unwrap();
            Ok(Expr::new(ExprKind::This(token.position)))
        },
        _ => {
            Err(LoxError::parser_error(ParserErrorKind::LiteralExpected, position))
        }
    }
}