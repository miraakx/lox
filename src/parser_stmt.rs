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
    Loop(Box<Stmt>),
    Break,
}

pub struct Parser {
    in_loop: u32
}

impl Parser {

    pub fn new() -> Self {
        Parser { in_loop: 0 }
    }

    pub fn parse(&mut self, token_iter: &mut dyn Iterator<Item=Token>) -> Result<Stmt, LoxError>
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
                    let stmt = self.declaration(&mut token_source)?;
                    statements.push(stmt);
                }
            }
        }
    }

    fn declaration(&mut self, token_source: &mut TokenSource)  -> Result<Stmt, LoxError>
    {
        let token = token_source.peek().unwrap();
        match token.kind {
            TokenKind::EOF => {
                panic!("EOF inatteso!");
            },
            TokenKind::Var => {
                //consuma il Var appena trovato
                token_source.consume();
                return self.var_declaration(token_source);
            },
            _ => {
                return self.statement(token_source);
            }
        }
    }

    fn var_declaration(&mut self, token_source: &mut TokenSource)  -> Result<Stmt, LoxError>
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

    fn statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        let token = token_source.peek().unwrap();
        match token.kind {
            TokenKind::EOF => {
                return Err(LoxError::new(LoxErrorKind::UnexpectedEndOfFile, token.position));
            },
            TokenKind::Print => {
                token_source.consume();
                return self.print_statement(token_source);
            },
            TokenKind::LeftBrace => {
                token_source.consume();
                return self.block_statement(token_source);
            },
            TokenKind::If => {
                token_source.consume();
                return self.if_statement(token_source);
            },
            TokenKind::While => {
                self.in_loop = self.in_loop + 1;
                token_source.consume();
                let while_stmt = self.while_statement(token_source);
                self.in_loop = self.in_loop - 1;
                return while_stmt;
            },
            TokenKind::For => {
                self.in_loop = self.in_loop + 1;
                token_source.consume();
                let for_stmt = self.for_statement(token_source);
                self.in_loop = self.in_loop - 1;
                return for_stmt;
            },
            TokenKind::Break => {
                if self.in_loop < 1 {
                    return Err(LoxError { kind: LoxErrorKind::BreakOutsideLoop, position: token.position })
                }
                token_source.consume();
                return self.break_statement(token_source);
            },
            _ => {
                return self.expression_statement(token_source);
            }
        }
    }

    fn break_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        expect_token(token_source.next().unwrap(), TokenKind::Semicolon)?;
        return Ok(Stmt::Break);
    }

    fn for_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        expect_token(token_source.next().unwrap(), TokenKind::LeftParen)?;

        //Parsing ...
        //parse initializer
        let token = token_source.peek().unwrap();
        let opt_initializer: Option<Stmt> = match token.kind {
            TokenKind::EOF => {
                return Err(LoxError::new(LoxErrorKind::UnexpectedEndOfFile, token.position));
            },
            TokenKind::Semicolon => {
                //consume semicolon
                token_source.consume();
                None
            },
            TokenKind::Var => {
                //consume 'var'
                token_source.consume();
                Some(self.var_declaration(token_source)?)
            },
            _ => {
                Some(self.expression_statement(token_source)?)
            }
        };
        //semicolon already consumed by var_declaration or expression_statement

        //parse condition
        let token = token_source.peek().unwrap();
        let opt_condition = match token.kind {
            TokenKind::EOF => {
                return Err(LoxError::new(LoxErrorKind::UnexpectedEndOfFile, token.position));
            },
            TokenKind::Semicolon => {
                None
            },
            _ => {
                Some(expression(token_source)?)
            }
        };
        //consume semicolon
        token_source.consume();

        //parse increment
        let token = token_source.peek().unwrap();
        let opt_increment = match token.kind {
            TokenKind::EOF => {
                return Err(LoxError::new(LoxErrorKind::UnexpectedEndOfFile, token.position));
            },
            TokenKind::RightParen => {
                None
            },
            _ => {
                Some(expression(token_source)?)
            }
        };
        //consume right paren
        token_source.consume();

        //parse body
        let mut body = self.statement(token_source)?;

        //Desugaring ...
        // {
        //    initializer;
        //    while ( condition )
        //    {
        //       body;
        //       increment;
        //    }
        // }
        //desugar increment
        if let Some(increment) = opt_increment {
            body = Stmt::Block(vec!(body, Stmt::ExprStmt(increment)));
        }

        //desugar condition
        match opt_condition
        {
            Some(condition) => {
                body = Stmt::While(condition, Box::new(body));
            },
            None => {
                body = Stmt::Loop(Box::new(body));
            }
        }

        //desugar initializer
        if let Some(initializer) = opt_initializer {
            body = Stmt::Block(vec!(initializer, body));
        }

        return Ok(body);
    }

    fn while_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        expect_token(token_source.next().unwrap(), TokenKind::LeftParen)?;
        let expr = expression(token_source)?;
        expect_token(token_source.next().unwrap(), TokenKind::RightParen)?;
        let stmt = self.statement(token_source)?;
        return Ok(Stmt::While(expr, Box::new(stmt)));
    }

    fn if_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        expect_token(token_source.next().unwrap(), TokenKind::LeftParen)?;
        let condition = expression(token_source)?;
        expect_token(token_source.next().unwrap(), TokenKind::RightParen)?;
        let then_stmt = self.statement(token_source)?;
        let token = token_source.peek().unwrap();
        match token.kind {
            TokenKind::EOF => {
                return Ok(Stmt::If(condition, Box::new(then_stmt)));
            },
            TokenKind::Else => {
                token_source.consume();
                let else_stmt = self.statement(token_source)?;
                return Ok(Stmt::IfElse(condition, Box::new(then_stmt), Box::new(else_stmt)));
            },
            _ => {
                return Ok(Stmt::If(condition, Box::new(then_stmt)));
            }
        }
    }

    fn block_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
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
                    statements.push(self.declaration(token_source)?);
                }
            }
        }
    }

    fn print_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        let r_expr = expression(token_source);
        if r_expr.is_err() {
            return Err(r_expr.err().unwrap());
        }
        expect_token(token_source.next().unwrap(), TokenKind::Semicolon)?;
        return Ok(Stmt::Print(r_expr.unwrap()));
    }

    fn expression_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        let r_expr = expression(token_source);
        if r_expr.is_err() {
            return Err(r_expr.err().unwrap());
        }
        expect_token(token_source.next().unwrap(), TokenKind::Semicolon)?;
        return Ok(Stmt::ExprStmt(r_expr.unwrap()));
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