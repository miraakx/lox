use once_cell::sync::Lazy;
use unique_id::{sequence::SequenceGenerator, Generator};

use crate::{tokens::{Token, TokenKind, TokenSource, consume_if, consume, check, consume_identifier, Identifier, BinaryOperatorKind, UnaryOperatorKind, LogicalOperatorKind, Operator, Position}, error::{LoxError, ParserErrorKind}, value::Value};

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
    Binary  (Box<BinaryExpr>),
    Grouping(Box<Expr>),
    Unary   (Box<UnaryExpr>),
    Literal (Value),
    Variable(Identifier),
    Assign  (Box<AssignExpr>),
    Logical (Box<LogicalExpr>),
    Call    (Box<CallExpr>),
    Get     (Box<GetExpr>),
    Set     (Box<SetExpr>),
    This    (Position)
}

#[derive(Clone, Debug)]
pub struct GetExpr {
    pub expr: Expr,
    pub identifier: Identifier
}

#[derive(Clone, Debug)]
pub struct AssignExpr {
    pub identifier: Identifier,
    pub expr: Expr
}

#[derive(Clone, Debug)]
pub struct UnaryExpr {
    pub operator: Operator<UnaryOperatorKind>,
    pub expr: Expr
}

#[derive(Clone, Debug)]
pub struct BinaryExpr {
    pub left: Expr,
    pub operator: Operator<BinaryOperatorKind>,
    pub right: Expr
}

#[derive(Clone, Debug)]
pub struct LogicalExpr {
    pub left: Expr,
    pub operator: Operator<LogicalOperatorKind>,
    pub right: Expr
}

#[derive(Clone, Debug)]
pub struct SetExpr {
    pub target: Expr,
    pub identifier: Identifier,
    pub value: Expr
}

#[derive(Clone, Debug)]
pub struct CallExpr {
    pub callee: Expr,
    pub arguments: Vec<Expr>,
    pub position: Position
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
                    Ok(Expr::new(ExprKind::Assign(Box::new(AssignExpr { identifier, expr: value }))))
                },
                //Assign a value expression to an instance property
                ExprKind::Get(get_expr) => {
                    Ok(Expr::new(ExprKind::Set(Box::new(SetExpr { target: get_expr.expr, identifier: get_expr.identifier, value: value }))))
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
                expr =  Expr::new(ExprKind::Logical(Box::new(LogicalExpr { left: expr, operator: Operator::<LogicalOperatorKind>::from_token(&operator), right: right })));
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
                expr = Expr::new(ExprKind::Logical(Box::new(LogicalExpr { left: expr, operator: Operator::<LogicalOperatorKind>::from_token(&operator), right: right })));
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
                expr = Expr::new(ExprKind::Binary(Box::new(BinaryExpr { left: expr, operator: Operator::<BinaryOperatorKind>::from_token(&operator), right: right })));
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
                expr = Expr::new(ExprKind::Binary(Box::new(BinaryExpr { left: expr, operator: Operator::<BinaryOperatorKind>::from_token(&operator), right: right })));
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
                expr = Expr::new(ExprKind::Binary(Box::new(BinaryExpr { left: expr, operator: Operator::<BinaryOperatorKind>::from_token(&operator), right: right })));
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
                expr = Expr::new(ExprKind::Binary(Box::new(BinaryExpr { left: expr, operator: Operator::<BinaryOperatorKind>::from_token(&operator), right: right })));
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
            Ok(Expr::new(ExprKind::Unary(Box::new(UnaryExpr { operator: Operator::<UnaryOperatorKind>::from_token(&operator), expr: right }))))
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
                expr = Expr::new(ExprKind::Call(Box::new(CallExpr { callee: expr, arguments: Vec::new(), position: left_paren.position })));
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
            if args.len() < 255 {
                expr = Expr::new(ExprKind::Call(Box::new(CallExpr { callee: expr, arguments: args, position: left_paren.position })));
            } else {
                todo!("Segnalare errore se più di 255 argomenti");
            }
        }
        else if consume_if(token_source, TokenKind::Dot)
        {
            let identifier: crate::tokens::Identifier = consume_identifier(token_source)?;
            expr = Expr::new(ExprKind::Get(Box::new(GetExpr { expr, identifier })));
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
            Ok(Expr::new(ExprKind::Literal(Value::from_token(token))))
        }
        TokenKind::False(_) | TokenKind::True(_) | TokenKind::Number(_) | TokenKind::String(_) => {
            Ok(Expr::new(ExprKind::Literal(Value::from_token(token))))
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