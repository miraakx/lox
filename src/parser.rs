use crate::error::{LoxError, LoxErrorKind};
use crate::common::Peekable;
use crate::tokens::{Token, TokenKind, Position, LiteralValue, extract_identifier};

pub enum Expr
{
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Unary(Token, Box<Expr>),
    Literal(Token),
    Variable(String, Position),
    Assign(Token, Box<Expr>)
}

pub enum Stmt
{
    Print(Expr),
    ExprStmt(Expr),
    Var(String, Position, Option<Expr>),
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>),
    IfElse(Expr, Box<Stmt>, Box<Stmt>)
}

struct EOF;

type TokenSource<'a> = Peekable<&'a mut dyn Iterator<Item=Token>, Token>;

impl<'a> TokenSource<'a>
{
    #[inline(always)]
    fn consume(&mut self) {
        self.next();
    }
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
        _ => {
            return expression_statement(token_source);
        }
    }
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

#[inline(always)]
fn expression(token_source: &mut TokenSource) -> Result<Expr,LoxError>
{
    assignment(token_source)
}

fn assignment(token_source: &mut TokenSource) -> Result<Expr,LoxError>
{
    let expr = equality(token_source)?;
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

            match expr {
                Expr::Variable(name, pos) => {
                    return Ok(Expr::Assign(Token { kind: TokenKind::Identifier, position: pos, value: Some(LiteralValue::Identifier(name)) }, Box::new(value)));
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
                expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
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
                expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
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
            TokenKind::Minus|TokenKind::Plus => {
                let operator = token_source.next().unwrap();
                let right: Expr = factor(token_source)?;
                expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
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
                expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
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
            Ok(Expr::Unary(operator, Box::new(right)))
        },
        _ => {
            primary(token_source)
        }
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
            Ok(Expr::Literal(token_source.next().unwrap()))
        },
        TokenKind::Identifier => {
            let val = extract_identifier(token_source.next().unwrap());
            Ok(Expr::Variable(val.0, val.1))
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
            return Ok(Expr::Grouping(Box::new(expr)));
        },
        found => {
            Err(LoxError::new(LoxErrorKind::LiteralExpected(*found), position))
        }
    }
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