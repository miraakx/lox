use std::rc::Rc;

use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;
use string_interner::StringInterner;
use unique_id::sequence::SequenceGenerator;

use crate::alias::IdentifierSymbol;
use crate::error::{LoxError, ParserErrorKind, ErrorLogger, ConsoleErrorLogger, ExecutionResult};
use crate::common::Peekable;
use crate::lexer::Lexer;
use crate::tokens::{check, check_end_of_file, consume, consume_identifier, consume_if, is_at_end, BinaryOperatorKind, Identifier, LogicalOperatorKind, Operator, Position, Token, TokenKind, TokenSource, UnaryOperatorKind};
use crate::value::Value;
use unique_id::Generator;

static ID_GENERATOR: Lazy<SequenceGenerator> = Lazy::new(SequenceGenerator::default);

#[derive(Clone, Debug)]
pub struct Expr
{
    pub id: i64,
    pub kind: ExprKind
}

impl Expr
{
    pub fn new(kind: ExprKind) -> Self {
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
    This    (Position),
    Super   (Identifier)
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

#[derive(Clone, Debug)]
pub enum Stmt
{
    Expr    (Expr),
    Var     (Identifier, Option<Expr>),
    Block   (Vec<Stmt>),
    If      (Box<IfStmt>),
    IfElse  (Box<IfElseStmt>),
    While   (Box<WhileStmt>),
    Return  (Option<Expr>, Position),
    Break,
    Continue,
    FunctionDeclaration (Rc<FunctionDeclaration>),
    ClassDeclaration    (Rc<ClassDeclaration>),
    Print   (Expr),
}

#[derive(Clone, Debug)]
pub struct IfElseStmt {
    pub condition: Expr,
    pub then_stmt: Stmt,
    pub else_stmt: Stmt
}

#[derive(Clone, Debug)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_stmt: Stmt
}

#[derive(Clone, Debug)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: Stmt
}

#[derive(Clone, Debug)]
pub struct FunctionDeclaration
{
    pub identifier: Identifier,
    pub parameters: Vec<IdentifierSymbol>,
    pub positions: Vec<Position>,
    //Attenzione! non puo' essere uno Stmt altrimenti i parametri della funzione verrebbero definiti in uno scope esterno rispetto al body e l'utente potrebbe ridefinirli nel body!
    pub body: Vec<Stmt>,
    pub is_initializer: bool
}

impl FunctionDeclaration
{
    pub fn new(identifier: Identifier, parameters: Vec<Identifier>, body: Vec<Stmt>, is_initializer: bool) -> Self
    {
        Self
        {
            identifier,
            parameters: parameters.iter().map(|p| p.name).collect(),
            positions: parameters.iter().map(|p| p.position).collect(),
            body,
            is_initializer
        }
    }
}

#[derive(Clone, Debug)]
pub struct ClassDeclaration
{
    pub identifier: Identifier,
    pub methods: FxHashMap<IdentifierSymbol, Rc<FunctionDeclaration>>,
    pub superclass_expr: Option<Expr>
}

impl ClassDeclaration
{
    fn new(identifier: Identifier, superclass_expr: Option<Expr>) -> Self
    {
        Self {
            identifier,
            methods: FxHashMap::default(),
            superclass_expr
        }
    }
}

pub struct Parser
{
    in_loop: u32,
    error_logger: Box<dyn ErrorLogger>,
    init_symbol: IdentifierSymbol
}

impl Parser
{
    pub fn new(error_logger: impl ErrorLogger + 'static, init_symbol: IdentifierSymbol) -> Self {
        Self { in_loop: 0, error_logger: Box::new(error_logger), init_symbol }
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

    pub fn parse(&mut self, code: &str, interner: &mut StringInterner) -> Result<Vec<Stmt>, ExecutionResult>
    {
        let mut statements: Vec<Stmt> = vec![];

        let mut is_error  : bool      = false;

        let mut lexer       : Lexer<'_>      = Lexer::new(code, interner, ConsoleErrorLogger{});
        let mut token_source: TokenSource    = Peekable::new(&mut lexer);

        loop {
            if is_at_end(&mut token_source) {
                if is_error {
                    return Err(ExecutionResult::ParserError);
                } else {
                    return Ok(statements);
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
        let class_name = consume_identifier(token_source)?;
        let mut class_stmt;

        //Check if a superclass is present (superclass are declared with a 'less then' sign after the class name: class Klass < Super {} )
        if check(token_source, TokenKind::Less)
        {
            token_source.consume();
            let superclass_name = consume_identifier(token_source)?;
            let superclass_expr = Expr::new(ExprKind::Variable(superclass_name));
            class_stmt = ClassDeclaration::new(class_name, Some(superclass_expr));
        }
        else
        {
            class_stmt = ClassDeclaration::new(class_name, None);
        }
        consume(token_source, TokenKind::LeftBrace)?;
        let mut methods: FxHashMap<IdentifierSymbol, Rc<FunctionDeclaration>> = FxHashMap::default();
        //Declares all the methods found in the class (properties are not declared).
        while !check(token_source, TokenKind::RightBrace) && !is_at_end(token_source)
        {
            let method_declaration = self.create_fun_declaration(token_source, true)?;
            methods.insert(method_declaration.identifier.name, Rc::new(method_declaration));
        }
        class_stmt.methods = methods;
        consume(token_source, TokenKind::RightBrace)?;

        Ok(Stmt::ClassDeclaration(Rc::new(class_stmt)))
    }

    fn fun_declaration(&mut self, token_source: &mut TokenSource)  -> Result<Stmt, LoxError>
    {
        Ok(Stmt::FunctionDeclaration(Rc::new(self.create_fun_declaration(token_source, false)?)))
    }

    fn create_fun_declaration(&mut self, token_source: &mut TokenSource, is_method: bool)  -> Result<FunctionDeclaration, LoxError>
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
        let right_paren_position = consume(token_source, TokenKind::RightParen)?.position;
        consume(token_source, TokenKind::LeftBrace)?;
        let body = self.block_statement(token_source)?;
        if args.len() > 255 {
            return Err(LoxError::parser_error(ParserErrorKind::TooManyParameters, right_paren_position));
        }
        let stmts = match body {
            Stmt::Block(stmts) => {
                stmts
            },
            _ => {
                return Err(LoxError::parser_error(ParserErrorKind::ExpectedBlock, right_paren_position));
            }
        };
        let mut is_initializer = false;
        if is_method && identifier.name == self.init_symbol {
            is_initializer = true;
        }
        let declaration = FunctionDeclaration::new(identifier, args, stmts, is_initializer);
        Ok(declaration)
    }

    fn var_declaration(&mut self, token_source: &mut TokenSource)  -> Result<Stmt, LoxError>
    {
        let identifier = consume_identifier(token_source)?;
        if consume_if(token_source, TokenKind::Equal) {
            let expr: Expr = self.expression(token_source)?;
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
            Some(self.expression(token_source)?)
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
                consume(token_source, TokenKind::Semicolon)?;
                None
            };

        //parse condition
        let opt_condition =
            if !check(token_source, TokenKind::Semicolon) {
                Some(self.expression(token_source)?)
            } else {
                None
            };
        consume(token_source, TokenKind::Semicolon)?;

        //parse increment
        let opt_increment =
            if !check(token_source, TokenKind::RightParen) {
                Some(self.expression(token_source)?)
            } else {
                None
            };
        consume(token_source, TokenKind::RightParen)?;

        //parse body
        let body = self.statement(token_source)?;

        //desugaring phase
        let body_plus_increment =
            if opt_increment.is_some() {
                Stmt::Block(vec![body, Stmt::Expr(opt_increment.unwrap())])
            } else {
                body
            };

        let condition_expr =
            match opt_condition {
                Some(condition_expr) => condition_expr,
                None => {
                    Expr::new(ExprKind::Literal(crate::value::Value::Bool(true)))
                },
            };

        let while_stmt = Stmt::While(Box::new(WhileStmt {condition: condition_expr, body: body_plus_increment })) ;

        let  initializer_plus_while =
            if opt_initializer.is_some() {
                Stmt::Block(vec![opt_initializer.unwrap(), while_stmt])
            } else {
                while_stmt
            };

        Ok(initializer_plus_while)

    }

    fn while_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        consume(token_source, TokenKind::LeftParen)?;
        let expr = self.expression(token_source)?;
        consume(token_source, TokenKind::RightParen)?;
        let stmt = self.statement(token_source)?;
        Ok(Stmt::While(Box::new(WhileStmt { condition: expr, body: stmt })))
    }

    fn if_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        consume(token_source, TokenKind::LeftParen)?;
        let condition = self.expression(token_source)?;
        consume(token_source, TokenKind::RightParen)?;
        let then_stmt = self.statement(token_source)?;

        if consume_if(token_source, TokenKind::Else) {
            Ok(Stmt::IfElse(Box::new(IfElseStmt { condition, then_stmt, else_stmt: self.statement(token_source)? })))
        } else {
            Ok(Stmt::If(Box::new(IfStmt { condition, then_stmt })))
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
        let expr = self.expression(token_source)?;
        consume(token_source, TokenKind::Semicolon)?;
        Ok(Stmt::Print(expr))
    }

    fn expression_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        let expr = self.expression(token_source)?;
        consume(token_source, TokenKind::Semicolon)?;
        Ok(Stmt::Expr(expr))
    }

    pub fn expression(&mut self, token_source: &mut TokenSource) -> Result<Expr,LoxError>
    {
        self.assignment(token_source)
    }

    fn assignment(&mut self, token_source: &mut TokenSource) -> Result<Expr,LoxError>
    {
        let expr = self.or(token_source)?;

        let peek_token = token_source.peek().unwrap();
        //Copy position to evade borrow checker
        let position = peek_token.position;

        match peek_token.kind {
            TokenKind::Eof => {
                Err(LoxError::parser_error(ParserErrorKind::UnexpectedEndOfFile, position))
            },
            TokenKind::Equal => {
                token_source.consume();
                let value: Expr = self.assignment(token_source)?;

                match expr.kind {
                    ExprKind::Variable(identifier) => {
                        Ok(Expr::new(ExprKind::Assign(Box::new(AssignExpr { identifier, expr: value }))))
                    },
                    //Assign a value expression to an instance property
                    ExprKind::Get(get_expr) => {
                        Ok(Expr::new(ExprKind::Set(Box::new(SetExpr { target: get_expr.expr, identifier: get_expr.identifier, value }))))
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

    fn or(&mut self, token_source: &mut TokenSource) -> Result<Expr,LoxError>
    {
        let mut expr = self.and(token_source)?;
        loop {
            let peek_token = token_source.peek().unwrap();
            match &peek_token.kind {
                TokenKind::Eof => {
                    return Err(LoxError::parser_error(ParserErrorKind::UnexpectedEndOfFile, peek_token.position));
                },
                TokenKind::Or => {
                    let operator = token_source.next().unwrap();
                    let right: Expr = self.and(token_source)?;
                    expr =  Expr::new(ExprKind::Logical(Box::new(LogicalExpr { left: expr, operator: Operator::<LogicalOperatorKind>::from_token(&operator), right })));
                },
                _ => {
                    return Ok(expr);
                }
            }
        }
    }

    fn and(&mut self, token_source: &mut TokenSource) -> Result<Expr,LoxError>
    {
        let mut expr = self.equality(token_source)?;
        loop {
            let peek_token = token_source.peek().unwrap();
            match &peek_token.kind {
                TokenKind::Eof => {
                    return Err(LoxError::parser_error(ParserErrorKind::UnexpectedEndOfFile, peek_token.position));
                },
                TokenKind::And => {
                    let operator = token_source.next().unwrap();
                    let right: Expr = self.equality(token_source)?;
                    expr = Expr::new(ExprKind::Logical(Box::new(LogicalExpr { left: expr, operator: Operator::<LogicalOperatorKind>::from_token(&operator), right })));
                },
                _ => {
                    return Ok(expr);
                }
            }
        }
    }

    fn equality(&mut self, token_source: &mut TokenSource) -> Result<Expr,LoxError>
    {
        let mut expr = self.comparison(token_source)?;
        loop {
            let peek_token = token_source.peek().unwrap();
            match &peek_token.kind {
                TokenKind::Eof => {
                    return Err(LoxError::parser_error(ParserErrorKind::UnexpectedEndOfFile, peek_token.position));
                },
                TokenKind::BangEqual|TokenKind::EqualEqual => {
                    let operator: Token = token_source.next().unwrap();
                    let right: Expr = self.comparison(token_source)?;
                    expr = Expr::new(ExprKind::Binary(Box::new(BinaryExpr { left: expr, operator: Operator::<BinaryOperatorKind>::from_token(&operator), right })));
                },
                _ => {
                    return Ok(expr);
                }
            }
        }
    }

    fn comparison(&mut self, token_source: &mut TokenSource) -> Result<Expr,LoxError>
    {
        let mut expr = self.term(token_source)?;
        loop {
            let peek_token = token_source.peek().unwrap();
            match &peek_token.kind {
                TokenKind::Eof => {
                    return Err(LoxError::parser_error(ParserErrorKind::UnexpectedEndOfFile, peek_token.position));
                },
                TokenKind::Greater | TokenKind::GreaterEqual | TokenKind::Less | TokenKind::LessEqual => {
                    let operator = token_source.next().unwrap();
                    let right: Expr = self.term(token_source)?;
                    expr = Expr::new(ExprKind::Binary(Box::new(BinaryExpr { left: expr, operator: Operator::<BinaryOperatorKind>::from_token(&operator), right })));
                },
                _ => {
                    return Ok(expr);
                }
            }
        }
    }

    fn term(&mut self, token_source: &mut TokenSource) -> Result<Expr,LoxError>
    {
        let mut expr: Expr = self.factor(token_source)?;
        loop {
            let peek_token = token_source.peek().unwrap();
            match &peek_token.kind {
                TokenKind::Eof => {
                    return Err(LoxError::parser_error(ParserErrorKind::UnexpectedEndOfFile, peek_token.position));
                },
                TokenKind::Minus | TokenKind::Plus => {
                    let operator = token_source.next().unwrap();
                    let right: Expr = self.factor(token_source)?;
                    expr = Expr::new(ExprKind::Binary(Box::new(BinaryExpr { left: expr, operator: Operator::<BinaryOperatorKind>::from_token(&operator), right })));
                },
                _ => {
                    return Ok(expr);
                }
            }
        }
    }

    fn factor(&mut self, token_source: &mut TokenSource) -> Result<Expr, LoxError>
    {
        let mut expr = self.unary(token_source)?;
        loop {
            let peek_token: &Token = token_source.peek().unwrap();
            match &peek_token.kind {
                TokenKind::Eof => {
                    return Err(LoxError::parser_error(ParserErrorKind::UnexpectedEndOfFile, peek_token.position));
                },
                TokenKind::Slash | TokenKind::Star => {
                    let operator: Token = token_source.next().unwrap();
                    let right = self.unary(token_source)?;
                    expr = Expr::new(ExprKind::Binary(Box::new(BinaryExpr { left: expr, operator: Operator::<BinaryOperatorKind>::from_token(&operator), right })));
                },
                _ => {
                    return Ok(expr);
                }
            }
        }
    }

    fn unary(&mut self, token_source: &mut TokenSource) -> Result<Expr, LoxError>
    {
        let peek_token = token_source.peek().unwrap();
        match &peek_token.kind {
            TokenKind::Eof => {
                Err(LoxError::parser_error(ParserErrorKind::UnexpectedEndOfFile, peek_token.position))
            },
            TokenKind::Bang | TokenKind::Minus => {
                let operator: Token = token_source.next().unwrap();
                let right:    Expr = self.unary(token_source)?;
                Ok(Expr::new(ExprKind::Unary(Box::new(UnaryExpr { operator: Operator::<UnaryOperatorKind>::from_token(&operator), expr: right }))))
            },
            _ => {
                self.call(token_source)
            }
        }
    }

    fn call(&mut self, token_source: &mut TokenSource) -> Result<Expr, LoxError>
    {
        let mut expr = self.primary(token_source)?;
        loop {
            let token = token_source.peek().unwrap();
            //println!("call {}", token.kind);
            match token.kind {
                TokenKind::LeftParen => {
                    let left_paren = token_source.next().unwrap();
                    if consume_if(token_source, TokenKind::RightParen) {
                        expr = Expr::new(ExprKind::Call(Box::new(CallExpr { callee: expr, arguments: Vec::new(), position: left_paren.position })));
                        continue;
                    }
                    let mut args: Vec<Expr> = Vec::new();
                    loop {
                        args.push(self.expression(token_source)?);
                        if !consume_if(token_source, TokenKind::Comma) {
                            break;
                        }
                    }
                    consume(token_source, TokenKind::RightParen)?;
                    if args.len() < 255 {
                        expr = Expr::new(ExprKind::Call(Box::new(CallExpr { callee: expr, arguments: args, position: left_paren.position })));
                    } else {
                        return Err(LoxError::parser_error(ParserErrorKind::TooManyArguments, left_paren.position));
                    }
                },
                TokenKind::Dot => {
                    token_source.consume();
                    let identifier: crate::tokens::Identifier = consume_identifier(token_source)?;
                    expr = Expr::new(ExprKind::Get(Box::new(GetExpr { expr, identifier })));
                },
                _ => {
                    return Ok(expr);
                }
            }
        }
    }

    fn primary(&mut self, token_source: &mut TokenSource) -> Result<Expr, LoxError>
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
                let expr: Expr = self.expression(token_source)?;
                let token_kind = token_source.next().unwrap().kind;
                match token_kind{
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
                Ok(Expr::new(ExprKind::This(token.position)))
            },
            TokenKind::Super => {
                consume(token_source, TokenKind::Dot)?;
                let identifier: Identifier = consume_identifier(token_source)?;
                Ok(Expr::new(ExprKind::Super(identifier)))
            },
            _ => {
                Err(LoxError::parser_error(ParserErrorKind::ExpectedLiteral(token.kind), position))
            }
        }
    }
}