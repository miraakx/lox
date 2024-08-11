use std::rc::Rc;

use rustc_hash::FxHashMap;
use string_interner::StringInterner;

use crate::alias::IdentifierSymbol;
use crate::error::{ConsoleErrorLogger, ErrorLogger, ExecutionResult, InternalErrorKind, LoxError, ParserErrorKind};
use crate::utils::peekable_iterator::Peekable;

use super::lexer::Lexer;
use super::tokens::{Token, TokenKind, TokenSource};
use super::types::{AssignExpr, BinaryExpr, BinaryOperatorKind, CallExpr, ClassDeclaration, Expr, ExprKind, FunctionDeclaration, GetExpr, Identifier, IfElseStmt, IfStmt, Literal, LogicalExpr, LogicalOperatorKind, Operator, SetExpr, Stmt, UnaryExpr, UnaryOperatorKind, WhileStmt};

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

    /// Handles syntactical errors when the parser meets one.
    ///
    /// If a syntactical error is detected, this method skip ahead and discard all the subsequent tokens
    /// until the start of a new statement is found.
    ///
    fn synchronize(&mut self, token_source: &mut TokenSource)
    {
        while !token_source.check(TokenKind::Eof)
        {
            match token_source.peek().unwrap().kind {
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

    /// Parses the source code using a "recursive descent" parsing alghoritm.
    ///
    /// Returns a `Vec` of `Stmt`. There are various kind of `Stmt` variants (eg. `Var`, `Block`, `If`, `Expr` etc.).
    ///
    /// Grammar:
    /// program     -> declaration* EOF ;
    ///
    pub fn parse(&mut self, code: &str, interner: &mut StringInterner) -> Result<Vec<Stmt>, ExecutionResult>
    {
        let mut statements: Vec<Stmt> = vec![];

        let mut is_error  : bool      = false;

        let mut lexer       : Lexer<'_>      = Lexer::new(code, interner, ConsoleErrorLogger{});
        let mut token_source: TokenSource    = Peekable::new(&mut lexer);

        loop {
            if token_source.is_at_end() {
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

                    // In case of a syntactical error call `synchronize` to skip to the next statement to avoids spitting out gibberish error messages.
                    self.synchronize(&mut token_source);
                }
            }
        }
    }

    // ---------------------------------------------------
    // The following methods are used to parse statements.
    // ---------------------------------------------------

    /// Searches for tokens matching the grammar rule "declaration" (keywords `var`, `fun` and `class`).
    ///
    /// If a declaration is not found the function looks for other kind of statements eg. 'print', blocks, 'if' etc.
    ///
    /// Grammar:
    /// declaration -> classDecl | funDecl | varDecl | statement ;
    ///
    fn declaration(&mut self, token_source: &mut TokenSource)  -> Result<Stmt, LoxError>
    {
        if token_source.consume_if(TokenKind::Var)
        {
            self.var_declaration(token_source)
        }
        else if token_source.consume_if(TokenKind::Fun)
        {
            self.fun_declaration(token_source)
        }
        else if token_source.consume_if(TokenKind::Class)
        {
            self.class_declaration(token_source)
        }
        else
        {
            self.statement(token_source)
        }
    }

    /// Parses a class declaration.
    ///
    /// Grammar:
    /// classDecl   -> "class" IDENTIFIER ( "<" IDENTIFIER )? "{" function* "}" ;
    ///
    /// function    -> IDENTIFIER "(" parameters? ")" block ;
    /// parameters  -> IDENTIFIER ( "," IDENTIFIER )* ;
    ///
    fn class_declaration(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        let class_name = token_source.consume_identifier("Expect class name.")?;
        let mut class_stmt;

        //Check if a superclass is present (superclass are declared with a 'less then' sign after the class name: class Klass < Super {} )
        if token_source.check(TokenKind::Less)
        {
            token_source.consume();
            let superclass_name = token_source.consume_identifier("Expect superclass name")?;
            let superclass_expr = Expr::new(ExprKind::Variable(superclass_name));
            class_stmt = ClassDeclaration::new(class_name, Some(superclass_expr));
        }
        else
        {
            class_stmt = ClassDeclaration::new(class_name, None);
        }
        token_source.consume_or_error(TokenKind::LeftBrace, "Expect '{' before class body.")?;
        let mut methods: FxHashMap<IdentifierSymbol, Rc<FunctionDeclaration>> = FxHashMap::default();
        //Declares all the methods found in the class (properties are not declared).
        while !token_source.check(TokenKind::RightBrace) && !token_source.is_at_end()
        {
            let method_declaration = self.create_fun_declaration(token_source, true)?;
            methods.insert(method_declaration.identifier.name, Rc::new(method_declaration));
        }
        class_stmt.methods = methods;
        token_source.consume_or_error(TokenKind::RightBrace, "Expect '}' after class body.")?;

        Ok(Stmt::ClassDeclaration(Rc::new(class_stmt)))
    }

    /// Parses a function declaration (eg.fun foo(param1, param2, ...) { ... }).
    ///
    /// Grammar:
    /// funDecl     -> "fun" function ;
    ///
    /// (same as above)
    /// function    -> IDENTIFIER "(" parameters? ")" block ;
    /// parameters  -> IDENTIFIER ( "," IDENTIFIER )* ;
    ///
    fn fun_declaration(&mut self, token_source: &mut TokenSource)  -> Result<Stmt, LoxError>
    {
        Ok(Stmt::FunctionDeclaration(Rc::new(self.create_fun_declaration(token_source, false)?)))
    }

    /// Common method used to parse both a function declaration or a class method. Used in `fun_declaration` and in `class_declaration`.
    ///
    /// Returns a `FunctionDeclaration` struct.
    fn create_fun_declaration(&mut self, token_source: &mut TokenSource, is_method: bool)  -> Result<FunctionDeclaration, LoxError>
    {
        let kind: &str = if is_method { "method" } else { "function" };
        let identifier = token_source.consume_identifier(format!("Expect {} name.", kind).as_str())?;
        token_source.consume_or_error(TokenKind::LeftParen, format!("Expect '(' after {} name.", kind).as_str())?;
        let mut args: Vec<Identifier> = vec![];
        if !token_source.check(TokenKind::RightParen)
        {
            loop
            {
                args.push(token_source.consume_identifier("Expect parameter name.")?);

                if !token_source.consume_if(TokenKind::Comma) {
                    break;
                }
            }
        }
        let right_paren_position = token_source.consume_or_error(TokenKind::RightParen, "Expect ')' after parameters.")?.position;
        token_source.consume_or_error(TokenKind::LeftBrace, format!("Expect '{{' before {} body.", kind).as_str())?;
        let body: Stmt = self.block_statement(token_source)?;
        if args.len() > 255 {
            return Err(LoxError::parser_error(ParserErrorKind::TooManyParameters, right_paren_position));
        }
        let stmts = match body {
            Stmt::Block(stmts) => {
                stmts
            },
            _ => {
                return Err(LoxError::internal_error(InternalErrorKind::ExpectedBlock));
            }
        };
        let mut is_initializer = false;
        if is_method && identifier.name == self.init_symbol {
            is_initializer = true;
        }
        let declaration = FunctionDeclaration::new(identifier, args, stmts, is_initializer);
        Ok(declaration)
    }

    /// Parses a variable declaration (eg.var foo = something;).
    ///
    /// Returns: Stmt::Var
    ///
    /// Grammar:
    /// varDecl     -> "var" IDENTIFIER ( "=" expression )? ";" ;
    ///
    fn var_declaration(&mut self, token_source: &mut TokenSource)  -> Result<Stmt, LoxError>
    {
        let identifier = token_source.consume_identifier("Expect variable name.")?;
        if token_source.consume_if(TokenKind::Equal) {
            let expr: Expr = self.expression(token_source)?;
            token_source.consume_or_error(TokenKind::Semicolon, "Expect ';' after variable declaration.")?;
            Ok(Stmt::Var(identifier, Some(expr)))
        } else {
            token_source.consume_or_error(TokenKind::Semicolon, "Expect ';' after variable declaration.")?;
            Ok(Stmt::Var(identifier, None))
        }
    }

    /// Parses a statement.
    ///
    /// Grammar:
    /// statement   -> exprStmt | forStmt | ifStmt | printStmt | returnStmt | whileStmt | block ;
    ///
    fn statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        let token = token_source.peek().unwrap();
        match token.kind {
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
                    return Err(LoxError::parser_error(ParserErrorKind::ContinueOutsideLoop, token.position))
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

    /// Parses a return statement.
    ///
    /// Grammar:
    /// returnStmt  -> "return" expression? ";" ;
    ///
    fn return_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        let return_token = token_source.consume_or_error(TokenKind::Return, "Internal error: Expect return statement." )?;
        let expr = if !token_source.check(TokenKind::Semicolon) {
            Some(self.expression(token_source)?)
        } else {
            None
        };
        token_source.consume_or_error(TokenKind::Semicolon, "Expect ';' after return value.")?;
        Ok(Stmt::Return(expr, return_token.position))
    }

    /// Parses a continue statement.
    ///
    /// WARNING: NOT IN THE ORIGINAL GRAMMAR
    ///
    /// Grammar:
    /// continueStmt  -> "continue" ";" ;
    ///
    fn continue_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        token_source.consume_or_error(TokenKind::Semicolon, "Expect ';' after 'continue'.")?;
        Ok(Stmt::Continue)
    }

    /// Parses a break statement.
    ///
    /// WARNING: NOT IN THE ORIGINAL GRAMMAR
    ///
    /// Grammar:
    /// continueStmt  -> "break" ";" ;
    ///
    fn break_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        token_source.consume_or_error(TokenKind::Semicolon, "Expect ';' after 'break'.")?;
        Ok(Stmt::Break)
    }

    /// Parses a `for` statement.
    /// This method desugars a C style for loop statement into a while loop with an initializer (optional), a condition (optional), a body (mandatory) and an increment (optional).
    ///
    /// Grammar:
    /// forStmt     -> "for" "(" ( varDecl | exprStmt | ";" ) expression? ";" expression? ")" statement ;
    ///
    fn for_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        //consume left paren first
        token_source.consume_or_error(TokenKind::LeftParen, "Expect '(' after 'for'.")?;

        //parse initializer
        let opt_initializer =
            if !token_source.check(TokenKind::Semicolon) {
                if token_source.consume_if(TokenKind::Var) {
                    Some(self.var_declaration(token_source)?)
                } else {
                    Some(self.expression_statement(token_source)?)
                }
            } else {
                token_source.consume_or_error(TokenKind::Semicolon, "Expect ';' after loop initializer.")?;
                None
            };

        //parse condition
        let condition_expr =
            if !token_source.check(TokenKind::Semicolon) {
                self.expression(token_source)?
            } else {
                Expr::new(ExprKind::Literal(Literal::True(token_source.peek().unwrap().position)))
            };
        token_source.consume_or_error(TokenKind::Semicolon, "Expect ';' after loop condition.")?;

        //parse increment
        let opt_increment =
            if !token_source.check(TokenKind::RightParen) {
                Some(self.expression(token_source)?)
            } else {
                None
            };
        token_source.consume_or_error(TokenKind::RightParen, "Expect ')' after for clauses.")?;

        //parse body
        let body = self.statement(token_source)?;

        //desugaring phase
        let body_plus_increment =
            if opt_increment.is_some() {
                Stmt::Block(vec![body, Stmt::Expr(opt_increment.unwrap())])
            } else {
                body
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

    /// Parses a `while` statement.
    ///
    /// Grammar:
    /// whileStmt   -> "while" "(" expression ")" statement ;
    ///
    fn while_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        token_source.consume_or_error(TokenKind::LeftParen, "Expect '(' after 'while'.")?;
        let expr = self.expression(token_source)?;
        token_source.consume_or_error(TokenKind::RightParen, "Expect ')' after while condition.")?;
        let stmt = self.statement(token_source)?;
        Ok(Stmt::While(Box::new(WhileStmt { condition: expr, body: stmt })))
    }

    /// Parses an `if` statement.
    ///
    /// Grammar:
    /// ifStmt  -> "if" "(" expression ")" statement ( "else" statement )? ;
    ///
    fn if_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        token_source.consume_or_error(TokenKind::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression(token_source)?;
        token_source.consume_or_error(TokenKind::RightParen, "Expect ')' after if condition.")?;
        let then_stmt = self.statement(token_source)?;

        if token_source.consume_if(TokenKind::Else) {
            Ok(Stmt::IfElse(Box::new(IfElseStmt { condition, then_stmt, else_stmt: self.statement(token_source)? })))
        } else {
            Ok(Stmt::If(Box::new(IfStmt { condition, then_stmt })))
        }
    }

    /// Parses a `block` statement.
    ///
    /// Grammar:
    /// block   -> "{" declaration* "}" ;
    ///
    fn block_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        let mut statements: Vec<Stmt> = vec![];
        while !token_source.check(TokenKind::RightBrace) && !token_source.is_at_end() {
            statements.push(self.declaration(token_source)?);
        }
        token_source.consume_or_error(TokenKind::RightBrace, "Expect '}' after block.")?;
        Ok(Stmt::Block(statements))
    }

    /// Parses a `print` statement.
    ///
    /// Grammar:
    /// printStmt   -> "print" expression ";" ;
    ///
    fn print_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        let expr = self.expression(token_source)?;
        token_source.consume_or_error(TokenKind::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(expr))
    }

    /// Parses an expression statement statement (an expression used in a place where a statement is expected).
    ///
    /// Grammar:
    /// exprStmt    -> expression ";" ;
    ///
    fn expression_statement(&mut self, token_source: &mut TokenSource) -> Result<Stmt, LoxError>
    {
        let expr = self.expression(token_source)?;
        token_source.consume_or_error(TokenKind::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Expr(expr))
    }

    // ----------------------------------------------------
    // The following methods are used to parse expressions.
    // ----------------------------------------------------

    /// Parses an expression. This is the root method for parsing expressions.
    ///
    /// Grammar:
    /// expression  -> assignment ;
    ///
    fn expression(&mut self, token_source: &mut TokenSource) -> Result<Expr,LoxError>
    {
        self.assignment(token_source)
    }

    /// Parses an assignment expression. Either an assignment to a variable or an attribute.
    ///
    /// Grammar:
    /// assignment  -> ( call "." )? IDENTIFIER "=" assignment | logic_or ;
    ///
    fn assignment(&mut self, token_source: &mut TokenSource) -> Result<Expr,LoxError>
    {
        let expr = self.or(token_source)?;

        let peek_token = token_source.peek().unwrap();
        //Copy position to evade borrow checker
        let position = peek_token.position;

        match peek_token.kind {
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
                        Err(LoxError::parser_error(ParserErrorKind::InvalidAssignmentTarget, position))
                    }
                }
            },
            _ => {
                Ok(expr)
            }
        }
    }

    /// Parses an `or` expression.
    ///
    /// Grammar:
    /// logic_or    -> logic_and ( "or" logic_and )* ;
    ///
    fn or(&mut self, token_source: &mut TokenSource) -> Result<Expr,LoxError>
    {
        let mut expr = self.and(token_source)?;
        loop {
            let peek_token = token_source.peek().unwrap();
            match &peek_token.kind {
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

    /// Parses an `and` expression.
    ///
    /// Grammar:
    /// logic_and   -> equality ( "and" equality )* ;
    ///
    fn and(&mut self, token_source: &mut TokenSource) -> Result<Expr,LoxError>
    {
        let mut expr = self.equality(token_source)?;
        loop {
            let peek_token = token_source.peek().unwrap();
            match &peek_token.kind {
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

    /// Parses an `==` expression.
    ///
    /// Grammar:
    /// equality    -> comparison ( ( "!=" | "==" ) comparison )* ;
    ///
    fn equality(&mut self, token_source: &mut TokenSource) -> Result<Expr,LoxError>
    {
        let mut expr = self.comparison(token_source)?;
        loop {
            let peek_token = token_source.peek().unwrap();
            match &peek_token.kind {
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

    /// Parses a `>`, `<`, `<=`, `>=` expression.
    ///
    /// Grammar:
    /// comparison  -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    ///
    fn comparison(&mut self, token_source: &mut TokenSource) -> Result<Expr,LoxError>
    {
        let mut expr = self.term(token_source)?;
        loop {
            let peek_token = token_source.peek().unwrap();
            match &peek_token.kind {
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

    /// Parses a term expression (`+`, `-`).
    ///
    /// Grammar:
    /// term    -> factor ( ( "-" | "+" ) factor )* ;
    ///
    fn term(&mut self, token_source: &mut TokenSource) -> Result<Expr,LoxError>
    {
        let mut expr: Expr = self.factor(token_source)?;
        loop {
            let peek_token = token_source.peek().unwrap();
            match &peek_token.kind {
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

    /// Parses a factor expression (`/`, `*`).
    ///
    /// Grammar:
    /// factor  -> unary ( ( "/" | "*" ) unary )* ;
    ///
    fn factor(&mut self, token_source: &mut TokenSource) -> Result<Expr, LoxError>
    {
        let mut expr = self.unary(token_source)?;
        loop {
            let peek_token: &Token = token_source.peek().unwrap();
            match &peek_token.kind {
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

    /// Parses a unary expression (`!`, `-`).
    ///
    /// Grammar:
    /// unary   -> ( "!" | "-" ) unary | call ;
    ///
    fn unary(&mut self, token_source: &mut TokenSource) -> Result<Expr, LoxError>
    {
        let peek_token = token_source.peek().unwrap();
        match &peek_token.kind {
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

    /// Parses a call expression `(`.
    ///
    /// Grammar:
    /// call        -> primary ( "(" arguments? ")" | "." IDENTIFIER )* ;
    /// arguments   -> expression ( "," expression )* ;
    ///
    fn call(&mut self, token_source: &mut TokenSource) -> Result<Expr, LoxError>
    {
        let mut expr = self.primary(token_source)?;
        loop {
            let token = token_source.peek().unwrap();
            //println!("call {}", token.kind);
            match token.kind {
                TokenKind::LeftParen => {
                    let left_paren = token_source.next().unwrap();
                    if token_source.consume_if(TokenKind::RightParen) {
                        expr = Expr::new(ExprKind::Call(Box::new(CallExpr { callee: expr, arguments: Vec::new(), position: left_paren.position })));
                        continue;
                    }
                    let mut args: Vec<Expr> = Vec::new();
                    loop {
                        args.push(self.expression(token_source)?);
                        if !token_source.consume_if(TokenKind::Comma) {
                            break;
                        }
                    }
                    token_source.consume_or_error(TokenKind::RightParen, "Expect ')' after arguments.")?;
                    if args.len() < 255 {
                        expr = Expr::new(ExprKind::Call(Box::new(CallExpr { callee: expr, arguments: args, position: left_paren.position })));
                    } else {
                        return Err(LoxError::parser_error(ParserErrorKind::TooManyArguments, left_paren.position));
                    }
                },
                TokenKind::Dot => {
                    token_source.consume();
                    let identifier: Identifier = token_source.consume_identifier("Expect property name after '.'.")?;
                    expr = Expr::new(ExprKind::Get(Box::new(GetExpr { expr, identifier })));
                },
                _ => {
                    return Ok(expr);
                }
            }
        }
    }

    /// Parses a primary expression `(`.
    ///
    /// Grammar:
    /// primary -> "true" | "false" | "nil" | "this" | NUMBER | STRING | IDENTIFIER | "(" expression ")" | "super" "." IDENTIFIER ;
    ///
    fn primary(&mut self, token_source: &mut TokenSource) -> Result<Expr, LoxError>
    {
        if token_source.is_at_end() {
            return Err(LoxError::parser_error(ParserErrorKind::ExpectedExpression, token_source.peek().unwrap().position));
        }

        let token = token_source.next().unwrap();
        let position = token.position;

        match &token.kind {
            TokenKind::Nil => {
                Ok(Expr::new(ExprKind::Literal(Literal::Nil(token.position))))
            }
            TokenKind::False  => {
                Ok(Expr::new(ExprKind::Literal(Literal::False(token.position))))
            },
            TokenKind::True  => {
                Ok(Expr::new(ExprKind::Literal(Literal::True(token.position))))
            },
            TokenKind::Number(number)  => {
                Ok(Expr::new(ExprKind::Literal(Literal::Number(*number, token.position))))
            }
            TokenKind::String(string) => {
                Ok(Expr::new(ExprKind::Literal(Literal::String(Rc::clone(string), token.position))))
            }
            TokenKind::Identifier(identifier) => {
                Ok(Expr::new(ExprKind::Variable(Identifier {name: *identifier, position: token.position})))
            },
            TokenKind::LeftParen => {
                let expr: Expr = self.expression(token_source)?;
                token_source.consume_or_error(TokenKind::RightParen, "Expect ')' after expression.")?;
                Ok(Expr::new(ExprKind::Grouping(Box::new(expr))))
            },
            TokenKind::This => {
                Ok(Expr::new(ExprKind::This(token.position)))
            },
            TokenKind::Super => {
                token_source.consume_or_error(TokenKind::Dot, "Expect '.' after 'super'.")?;
                let identifier: Identifier = token_source.consume_identifier("Expect superclass method name.")?;
                Ok(Expr::new(ExprKind::Super(identifier)))
            },
            _ => {
                Err(LoxError::parser_error(ParserErrorKind::ExpectedExpression, position))
            }
        }
    }
}