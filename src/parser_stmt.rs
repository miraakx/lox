use std::rc::Rc;

use rustc_hash::FxHashMap;
use string_interner::StringInterner;

use crate::alias::IdentifierSymbol;
use crate::error::{LoxError, ParserErrorKind, ErrorLogger, ConsoleErrorLogger};
use crate::common::Peekable;
use crate::lexer::Lexer;
use crate::parser_expr::{Expr, expression};
use crate::tokens::{TokenKind, TokenSource, consume, check, consume_if, check_end_of_file, is_at_end, consume_identifier, Identifier, Position};

#[derive(Clone, Debug)]
pub enum Stmt
{
    Expr    (Expr),
    Var     (Identifier, Option<Expr>),
    Block   (Vec<Stmt>),
    If      (Expr, Box<Stmt>),
    IfElse  (Expr, Box<Stmt>, Box<Stmt>),
    While   (Expr, Box<Stmt>),
    For     (Box<Option<Stmt>>, Option<Expr>, Option<Expr>, Box<Stmt>),
    Return  (Option<Expr>, Position),
    Break,
    Continue,
    FunctionDeclaration (Rc<FunctionDeclaration>),
    ClassDeclaration    (Rc<ClassDeclaration>),
    Print   (Expr),

}

pub struct Parser
{
    in_loop: u32,
    error_logger: Box<dyn ErrorLogger>
}

impl Parser
{
    pub fn new(error_logger: impl ErrorLogger + 'static) -> Self {
        Self { in_loop: 0, error_logger: Box::new(error_logger) }
    }

    fn synchronize(&mut self, token_source: &mut TokenSource) {
        loop {
            let peek = token_source.peek().unwrap();
            match peek.kind {
                TokenKind::Class | TokenKind::Fun    |
                TokenKind::Var   | TokenKind::For    |
                TokenKind::If    | TokenKind::While  |
                TokenKind::Print | TokenKind::Return |
                TokenKind::Eof =>
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

    pub fn parse(&mut self, code: &str) -> Result<(Vec<Stmt>, StringInterner), ()>
    {
        let mut statements: Vec<Stmt> = vec![];
        let mut interner  : StringInterner = StringInterner::default();

        let mut is_error  : bool      = false;

        let mut lexer       : Lexer<'_>      = Lexer::new(code, &mut interner, ConsoleErrorLogger{});
        let mut token_source: TokenSource    = Peekable::new(&mut lexer);

        loop {
            if is_at_end(&mut token_source) {
                if is_error {
                    return Err(());
                } else {
                    return Ok((statements, interner));
                }
            }
            let result = self.declaration(&mut token_source);
            match result {
                Ok(stmt) => {
                    statements.push(stmt);
                }
                Err(err) => {
                    is_error = true;
                    self.error_logger.log(err);
                    self.synchronize(&mut token_source);
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

    fn class_declaration(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        let identifier = consume_identifier(token_source)?;
        let mut class_declaration = ClassDeclaration::new(identifier);
        consume(token_source, TokenKind::LeftBrace)?;

        while !check(token_source, TokenKind::RightBrace) && !is_at_end(token_source) {
            let method_declaration = self.create_fun_declaration(token_source)?;
            class_declaration.insert_method(method_declaration.identifier.name, method_declaration);
        }
        consume(token_source, TokenKind::RightBrace)?;
        Ok(Stmt::ClassDeclaration(Rc::new(class_declaration)))
    }

    fn fun_declaration(&mut self, token_source: &mut TokenSource)  -> Result<Stmt, LoxError>
    {
        Ok(Stmt::FunctionDeclaration(Rc::new(self.create_fun_declaration(token_source)?)))
    }

    fn create_fun_declaration(&mut self, token_source: &mut TokenSource)  -> Result<FunctionDeclaration, LoxError>
    {
        let identifier = consume_identifier(token_source)?;
        consume(token_source, TokenKind::LeftParen)?;
        let mut args: Vec<Identifier> = vec![];
        if !check(token_source, TokenKind::RightParen)
        {
            loop
            {
                args.push(consume_identifier(token_source)?);

                if !consume_if(token_source, TokenKind::Comma) {
                    break;
                }
            }
        }
        consume(token_source, TokenKind::RightParen)?;
        consume(token_source, TokenKind::LeftBrace)?;
        let body = self.block_statement(token_source)?;
        let declaration = FunctionDeclaration::new(identifier, args, body);
        Ok(declaration)
    }

    fn var_declaration(&mut self, token_source: &mut TokenSource)  -> Result<Stmt, LoxError>
    {
        let identifier = consume_identifier(token_source)?;
        if consume_if(token_source, TokenKind::Equal) {
            let expr: Expr = expression(token_source)?;
            consume(token_source, TokenKind::Semicolon)?;
            Ok(Stmt::Var(identifier, Some(expr)))
        } else {
            consume(token_source, TokenKind::Semicolon)?;
            Ok(Stmt::Var(identifier, None))
        }
    }

    fn statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        let token = token_source.peek().unwrap();
        match token.kind {
            TokenKind::Eof => {
                Err(LoxError::parser_error(ParserErrorKind::UnexpectedEndOfFile, token.position))
            },
            TokenKind::Print => {
                token_source.consume();
                self.print_statement(token_source)
            },
            TokenKind::LeftBrace => {
                token_source.consume();
                self.block_statement(token_source)
            },
            TokenKind::If => {
                token_source.consume();
                self.if_statement(token_source)
            },
            TokenKind::While => {
                self.in_loop += 1;
                token_source.consume();
                let while_stmt = self.while_statement(token_source);
                self.in_loop -= 1;
                while_stmt
            },
            TokenKind::For => {
                self.in_loop += 1;
                token_source.consume();
                let for_stmt = self.for_statement(token_source);
                self.in_loop -= 1;
                for_stmt
            },
            TokenKind::Break => {
                if self.in_loop < 1 {
                    return Err(LoxError::parser_error(ParserErrorKind::BreakOutsideLoop, token.position));
                }
                token_source.consume();
                self.break_statement(token_source)
            },
            TokenKind::Continue => {
                if self.in_loop < 1 {
                    return Err(LoxError::parser_error(ParserErrorKind::BreakOutsideLoop, token.position))
                }
                token_source.consume();
                self.continue_statement(token_source)
            },
            TokenKind::Return => {
                self.return_statement(token_source)
            },
            _ => {
                self.expression_statement(token_source)
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
        Ok(Stmt::Return(expr, return_token.position))
    }

    fn continue_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        consume(token_source, TokenKind::Semicolon)?;
        Ok(Stmt::Continue)
    }

    fn break_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        consume(token_source, TokenKind::Semicolon)?;
        Ok(Stmt::Break)
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

        Ok(Stmt::For(Box::new(opt_initializer), opt_condition, opt_increment, Box::new(body)))

    }

    fn while_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        consume(token_source, TokenKind::LeftParen)?;
        let expr = expression(token_source)?;
        consume(token_source, TokenKind::RightParen)?;
        let stmt = self.statement(token_source)?;
        Ok(Stmt::While(expr, Box::new(stmt)))
    }

    fn if_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        consume(token_source, TokenKind::LeftParen)?;
        let condition = expression(token_source)?;
        consume(token_source, TokenKind::RightParen)?;
        let then_stmt = self.statement(token_source)?;

        check_end_of_file(token_source)?;

        if consume_if(token_source, TokenKind::Else) {
            Ok(Stmt::IfElse(condition, Box::new(then_stmt), Box::new(self.statement(token_source)?)))
        } else {
            Ok(Stmt::If(condition, Box::new(then_stmt)))
        }
    }

    fn block_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        let mut statements: Vec<Stmt> = vec![];
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
        Ok(Stmt::Print(expr))
    }

    fn expression_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        let expr = expression(token_source)?;
        consume(token_source, TokenKind::Semicolon)?;
        Ok(Stmt::Expr(expr))
    }
}

#[derive(Clone, Debug)]
pub struct FunctionDeclaration
{
    pub identifier: Identifier,
    pub parameters: Vec<IdentifierSymbol>,
    pub positions: Vec<Position>,
    pub body: Stmt
}

impl FunctionDeclaration
{
    pub fn new(identifier: Identifier, parameters: Vec<Identifier>, body: Stmt) -> Self
    {
        Self
        {
            identifier,
            parameters: parameters.iter().map(|p| p.name).collect(),
            positions: parameters.iter().map(|p| p.position).collect(),
            body
        }
    }
}

#[derive(Clone, Debug)]
pub struct ClassDeclaration
{
    pub identifier: Identifier,
    pub methods: FxHashMap<IdentifierSymbol, Rc<FunctionDeclaration>>
}

impl ClassDeclaration
{
    fn new(identifier: Identifier) -> Self
    {
        Self {identifier, methods: FxHashMap::default()}
    }

    fn insert_method(&mut self, name: IdentifierSymbol, method_declaration: FunctionDeclaration)
    {
        self.methods.insert(name, Rc::new(method_declaration));
    }
}