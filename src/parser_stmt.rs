use std::rc::Rc;

use crate::error::{LoxError, ParserErrorKind, ErrorLogger};
use crate::common::Peekable;
use crate::parser_expr::{Expr, expression};
use crate::tokens::{Token, TokenKind, Position, TokenSource, consume, check, consume_if, check_end_of_file, is_at_end};

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
    For(Box<Option<Stmt>>, Option<Expr>, Option<Expr>, Box<Stmt>),
    Break, Continue,
    FunctionDeclaration(Rc::<FunctionDeclaration>),
    Return(Token, Option<Expr>),
    ClassDeclaration(Rc<ClassDeclaration>)
}

pub struct Parser {
    in_loop: u32,
    error_logger: Box<dyn ErrorLogger>
}

impl Parser {

    pub fn new(error_logger: impl ErrorLogger + 'static) -> Self {
        Parser { in_loop: 0, error_logger: Box::new(error_logger) }
    }

    fn synchronize(&mut self, token_source: &mut TokenSource) {
        loop {
            let peek = token_source.peek().unwrap();
            match peek.kind {
                TokenKind::Class | TokenKind::Fun    |
                TokenKind::Var   | TokenKind::For    |
                TokenKind::If    | TokenKind::While  |
                TokenKind::Print | TokenKind::Return |
                TokenKind::EOF =>
                {
                    return;
                },
                TokenKind::Semicolon =>
                {
                    token_source.consume();
                    return;
                },
                _ => {
                    token_source.consume();
                }
            }
        }
    }

    pub fn parse(&mut self, token_iter: &mut dyn Iterator<Item=Token>) -> Result<Vec<Stmt>, LoxError>
    {
        let token_source: &mut TokenSource = &mut Peekable::new(token_iter);
        let mut statements: Vec<Stmt> = vec!();
        loop {
            if is_at_end(token_source) {
                return Ok(statements);
            }
            let result = self.declaration(token_source);
            match result {
                Ok(stmt) => {
                    statements.push(stmt);
                }
                Err(err) => {
                    self.error_logger.log(err);
                    self.synchronize(token_source);
                }
            }
        }
    }

    fn declaration(&mut self, token_source: &mut TokenSource)  -> Result<Stmt, LoxError>
    {
        if consume_if(token_source, TokenKind::Var)
        {
            self.var_declaration(token_source)
        }
        else if consume_if(token_source, TokenKind::Fun)
        {
            self.fun_declaration(token_source)
        }
        else if consume_if(token_source, TokenKind::Class)
        {
            self.class_declaration(token_source)
        }
        else
        {
            self.statement(token_source)
        }
    }

    fn class_declaration(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError> {
        let name = consume(token_source, TokenKind::Identifier)?;
        consume(token_source, TokenKind::LeftBrace)?;
        let mut methods = vec!();
        while !check(token_source, TokenKind::RightBrace) && !is_at_end(token_source) {
            methods.push(self.create_fun_declaration(token_source)?);
        }
        consume(token_source, TokenKind::RightBrace)?;
        return Ok(Stmt::ClassDeclaration(Rc::new(ClassDeclaration{ name, methods })));
    }

    fn fun_declaration(&mut self, token_source: &mut TokenSource)  -> Result<Stmt, LoxError>
    {
        return Ok(Stmt::FunctionDeclaration(Rc::new(self.create_fun_declaration(token_source)?)));
    }

    fn create_fun_declaration(&mut self, token_source: &mut TokenSource)  -> Result<FunctionDeclaration, LoxError>
    {
        let identifier = consume(token_source, TokenKind::Identifier)?;
        consume(token_source, TokenKind::LeftParen)?;
        let mut args: Vec<Token> = vec!();
        if !check(token_source, TokenKind::RightParen) {
            loop {
                args.push(consume(token_source, TokenKind::Identifier)?);
                if !consume_if(token_source, TokenKind::Comma) {
                    break;
                }
            }
        }
        consume(token_source, TokenKind::RightParen)?;
        consume(token_source, TokenKind::LeftBrace)?;
        let body = self.block_statement(token_source)?;
        let declaration = FunctionDeclaration { name: identifier, parameters: args, body: body };
        return Ok(declaration);
    }

    fn var_declaration(&mut self, token_source: &mut TokenSource)  -> Result<Stmt, LoxError>
    {
        let identifier = consume(token_source, TokenKind::Identifier)?;
        if consume_if(token_source, TokenKind::Equal) {
            let expr: Expr = expression(token_source)?;
            consume(token_source, TokenKind::Semicolon)?;
            let val = identifier.get_identifier_and_position();
            return Ok(Stmt::Var(val.0, val.1, Some(expr)));
        } else {
            consume(token_source, TokenKind::Semicolon)?;
            let val = identifier.get_identifier_and_position();
            return Ok(Stmt::Var(val.0, val.1, None));
        }
    }

    fn statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        let token = token_source.peek().unwrap();
        match token.kind {
            TokenKind::EOF => {
                return Err(LoxError::parser_error(ParserErrorKind::UnexpectedEndOfFile, token.position));
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
                    return Err(LoxError::parser_error(ParserErrorKind::BreakOutsideLoop, token.position))
                }
                token_source.consume();
                return self.break_statement(token_source);
            },
            TokenKind::Continue => {
                if self.in_loop < 1 {
                    return Err(LoxError::parser_error(ParserErrorKind::BreakOutsideLoop, token.position))
                }
                token_source.consume();
                return self.continue_statement(token_source);
            },
            TokenKind::Return => {
                return self.return_statement(token_source);
            },
            _ => {
                return self.expression_statement(token_source);
            }
        }
    }

    fn return_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        let return_token = consume(token_source, TokenKind::Return)?;
        let expr = if !check(token_source, TokenKind::Semicolon) {
            Some(expression(token_source)?)
        } else {
            None
        };
        consume(token_source, TokenKind::Semicolon)?;
        return Ok(Stmt::Return(return_token, expr));
    }

    fn continue_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        consume(token_source, TokenKind::Semicolon)?;
        return Ok(Stmt::Continue);
    }

    fn break_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        consume(token_source, TokenKind::Semicolon)?;
        return Ok(Stmt::Break);
    }

    fn for_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        //consume left paren first
        consume(token_source, TokenKind::LeftParen)?;

        //parse initializer
        let opt_initializer =
        if !check(token_source, TokenKind::Semicolon) {
            if consume_if(token_source, TokenKind::Var) {
                Some(self.var_declaration(token_source)?)
            } else {
                Some(self.expression_statement(token_source)?)
            }
        } else {
           None
        };
        //consume(token_source, TokenKind::Semicolon)?;

        //parse condition
        let opt_condition = if !check(token_source, TokenKind::Semicolon) { Some(expression(token_source)?) } else { None };
        consume(token_source, TokenKind::Semicolon)?;

        //parse increment
        let opt_increment = if !check(token_source, TokenKind::RightParen) { Some(expression(token_source)?) } else { None };
        consume(token_source, TokenKind::RightParen)?;

        //parse body
        let body = self.statement(token_source)?;

        return Ok(Stmt::For(Box::new(opt_initializer), opt_condition, opt_increment, Box::new(body)));

    }

    fn while_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        consume(token_source, TokenKind::LeftParen)?;
        let expr = expression(token_source)?;
        consume(token_source, TokenKind::RightParen)?;
        let stmt = self.statement(token_source)?;
        return Ok(Stmt::While(expr, Box::new(stmt)));
    }

    fn if_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        consume(token_source, TokenKind::LeftParen)?;
        let condition = expression(token_source)?;
        consume(token_source, TokenKind::RightParen)?;
        let then_stmt = self.statement(token_source)?;

        check_end_of_file(token_source)?;

        return if consume_if(token_source, TokenKind::Else) {
            Ok(Stmt::IfElse(condition, Box::new(then_stmt), Box::new(self.statement(token_source)?)))
        } else {
            Ok(Stmt::If(condition, Box::new(then_stmt)))
        };
    }

    fn block_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        let mut statements: Vec<Stmt> = vec!();
        loop {
            check_end_of_file(token_source)?;
            if consume_if(token_source, TokenKind::RightBrace) {
                return Ok(Stmt::Block(statements));
            }
            statements.push(self.declaration(token_source)?);
        }
    }

    fn print_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        let expr = expression(token_source)?;
        consume(token_source, TokenKind::Semicolon)?;
        return Ok(Stmt::Print(expr));
    }

    fn expression_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        let expr = expression(token_source)?;
        consume(token_source, TokenKind::Semicolon)?;
        return Ok(Stmt::ExprStmt(expr));
    }
}

#[derive(Clone, Debug)]
pub struct FunctionDeclaration {
    pub name: Token,
    pub parameters: Vec<Token>,
    pub body: Stmt
}

#[derive(Clone, Debug)]
pub struct ClassDeclaration {
    pub name: Token,
    pub methods: Vec<FunctionDeclaration>,
}

impl ClassDeclaration {
    pub fn get_name(&self) -> String {
        self.name.get_identifier()
    }
}
