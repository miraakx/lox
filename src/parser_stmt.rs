use crate::error::{LoxError, LoxErrorKind};
use crate::common::Peekable;
use crate::parser_expr::{Expr, expression};
use crate::tokens::{Token, TokenKind, Position, extract_identifier, TokenSource};

#[derive(Clone, Debug)]
pub enum Stmt
{
    Print(Expr),
    ExprStmt(Expr),
    Var(String, Position, Option<Expr>),
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>),
    IfElse(Expr, Box<Stmt>, Box<Stmt>),
    While(Expr, Box<Stmt>),
}

pub fn parse(token_iter: &mut dyn Iterator<Item=Token>) -> Result<Stmt, LoxError>
{
    let mut token_source: TokenSource = Peekable::new(token_iter);
    let mut statements: Vec<Stmt> = vec!();
    loop {
        let token = token_source.peek().unwrap();
        match token.kind {
            TokenKind::EOF => {
                //Se il file è finito non procedo oltre.
                return Ok(Stmt::Block(statements));
            },
            _ => {
                let stmt = declaration(&mut token_source)?;
                statements.push(stmt);
            }
        }
    }
}

fn declaration(token_source: &mut TokenSource)  -> Result<Stmt, LoxError>
{
    let token = token_source.peek().unwrap();
    match token.kind {
        TokenKind::EOF => {
            panic!("EOF inatteso!");
        },
        TokenKind::Var => {
            //consuma il Var appena trovato
            token_source.consume();
            return var_declaration(token_source);
        },
        _ => {
            return statement(token_source);
        }
    }
}

fn var_declaration(token_source: &mut TokenSource)  -> Result<Stmt, LoxError>
{
    let token = token_source.next().unwrap();
    match token.kind {
        TokenKind::EOF => {
            return Err(LoxError::new(LoxErrorKind::UnexpectedEndOfFile, token.position));
        },
        TokenKind::Identifier => {
            if token.value.is_none() {
                panic!("Errore inatteso. Literal value atteso nel token ma non trovato.");
            }
            if token_source.next().unwrap().kind == TokenKind::Equal {
                let expr: Expr = expression(token_source)?;
                expect_token(token_source.next().unwrap(), TokenKind::Semicolon)?;
                let val = extract_identifier(token);
                return Ok(Stmt::Var(val.0, val.1, Some(expr)));
            } else {
                expect_token(token_source.next().unwrap(), TokenKind::Semicolon)?;
                let val = extract_identifier(token);
                return Ok(Stmt::Var(val.0, val.1, None));
            }
        },
        _ => {
            return Err(LoxError::new(LoxErrorKind::VariableNameExpected, token.position));
        }
    }
}

fn statement(token_source: &mut TokenSource) -> Result<Stmt, LoxError>
{
    let token = token_source.peek().unwrap();
    match token.kind {
        TokenKind::EOF => {
            return Err(LoxError::new(LoxErrorKind::UnexpectedEndOfFile, token.position));
        },
        TokenKind::Print => {
            token_source.consume();
            return print_statement(token_source);
        },
        TokenKind::LeftBrace => {
            token_source.consume();
            return block_statement(token_source);
        },
        TokenKind::If => {
            token_source.consume();
            return if_statement(token_source);
        },
        TokenKind::While => {
            token_source.consume();
            return while_statement(token_source);
        }
        _ => {
            return expression_statement(token_source);
        }
    }
}

fn while_statement(token_source: &mut TokenSource) -> Result<Stmt, LoxError>
{
    expect_token(token_source.next().unwrap(), TokenKind::LeftParen)?;
    let expr = expression(token_source)?;
    expect_token(token_source.next().unwrap(), TokenKind::RightParen)?;
    let stmt = statement(token_source)?;
    return Ok(Stmt::While(expr, Box::new(stmt)));
}

fn if_statement(token_source: &mut TokenSource) -> Result<Stmt, LoxError>
{
    expect_token(token_source.next().unwrap(), TokenKind::LeftParen)?;
    let condition = expression(token_source)?;
    expect_token(token_source.next().unwrap(), TokenKind::RightParen)?;
    let then_stmt = statement(token_source)?;
    let token = token_source.peek().unwrap();
    match token.kind {
        TokenKind::EOF => {
            return Ok(Stmt::If(condition, Box::new(then_stmt)));
        },
        TokenKind::Else => {
            token_source.consume();
            let else_stmt = statement(token_source)?;
            return Ok(Stmt::IfElse(condition, Box::new(then_stmt), Box::new(else_stmt)));
        },
        _ => {
            return Ok(Stmt::If(condition, Box::new(then_stmt)));
        }
    }
}

fn block_statement(token_source: &mut TokenSource) -> Result<Stmt, LoxError>
{
    let mut statements: Vec<Stmt> = vec!();
    loop {
        let token = token_source.peek().unwrap();
        match token.kind {
            TokenKind::EOF => {
                return Err(LoxError::new(LoxErrorKind::UnexpectedEndOfFile, token.position));
            },
            TokenKind::RightBrace => {
                token_source.consume();
                return Ok(Stmt::Block(statements));
            },
            _ => {
                statements.push(declaration(token_source)?);
            }
        }
    }
}

fn print_statement(token_source: &mut TokenSource) -> Result<Stmt, LoxError>
{
    let r_expr = expression(token_source);
    if r_expr.is_err() {
        return Err(r_expr.err().unwrap());
    }
    expect_token(token_source.next().unwrap(), TokenKind::Semicolon)?;
    return Ok(Stmt::Print(r_expr.unwrap()));
}

fn expression_statement(token_source: &mut TokenSource) -> Result<Stmt, LoxError>
{
    let r_expr = expression(token_source);
    if r_expr.is_err() {
        return Err(r_expr.err().unwrap());
    }
    expect_token(token_source.next().unwrap(), TokenKind::Semicolon)?;
    return Ok(Stmt::ExprStmt(r_expr.unwrap()));
}



#[inline]
fn expect_token(token: Token, token_kind: TokenKind) -> Result<(),LoxError>
{
    if token_kind == token.kind {
        return Ok(());
    }
    match token.kind {
        TokenKind::EOF => {
            Err(LoxError::new(LoxErrorKind::UnexpectedEndOfFile, token.position))
        },
        _ => {
            Err(LoxError::new(LoxErrorKind::ExpectedToken(token_kind), token.position))
        }
    }
}