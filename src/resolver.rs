use std::{cell::RefCell, rc::Rc};

use rustc_hash::FxHashMap;
use string_interner::StringInterner;

use crate::{parser_stmt::{Stmt, FunctionDeclaration}, parser_expr::{Expr, ExprKind}, common::Stack, error::{LoxError, ErrorLogger, ResolverErrorKind}, interpreter::Interpreter, alias::IdentifierSymbol, tokens::{Position, THIS}};

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    stack: Stack<FxHashMap<IdentifierSymbol, bool>>,
    error_logger: Box<dyn ErrorLogger>,
    string_interner: Rc<RefCell<StringInterner>>,
    has_error: bool,
    current_function: FunctionType,
    current_class: ClassType,
    this_symbol: IdentifierSymbol,
    init_symbol: IdentifierSymbol
}

impl <'a> Resolver<'a>
{
    pub fn new(interpreter: &'a mut Interpreter, error_logger: impl ErrorLogger + 'static, string_interner: Rc<RefCell<StringInterner>>) -> Self {
        let this_symbol = string_interner.borrow_mut().get_or_intern_static(THIS);
        let init_symbol = string_interner.borrow_mut().get_or_intern_static("init");
        Resolver {
            stack: Stack::new(),
            interpreter,
            error_logger: Box::new(error_logger),
            string_interner,
            has_error: false,
            current_function: FunctionType::None,
            current_class: ClassType::None,
            this_symbol,
            init_symbol
        }
    }

    fn error(&mut self, err_kind: ResolverErrorKind, position: &Position) {
        self.error_logger.log(LoxError::resolver_error(err_kind, *position));
        self.has_error = true;
    }

    pub fn resolve(&mut self, stmts: &[Stmt]) -> Result<(),()>{
        for stmt in stmts {
            self.resolve_stmt(stmt, self.current_function, self.current_class);
        }
        if self.has_error {
            Err(())
        } else {
            Ok(())
        }
    }

    fn resolve_stmt(&mut self, stmt: &Stmt, function_type: FunctionType, class_type: ClassType)
    {
        let enclosing_function = self.current_function;
        let enclosing_class = self.current_class;
        //> set-current-function
        self.current_function = function_type;
        self.current_class = class_type;
        //< set-current-function
        match stmt
        {
            Stmt::Print(print_expr) =>
            {
                self.resolve_expr(print_expr);
            },
            Stmt::ExprStmt(expr) =>
            {
                self.resolve_expr(expr);
            },
            Stmt::Var(identifier, opt_expr) =>
            {
                match self.declare(identifier.name) {
                    Err(err_kind) => {
                        self.error(err_kind, &identifier.position);
                    },
                    _ => {}
                }
                if let Some(expr) = opt_expr {
                    self.resolve_expr(&expr);
                }
                self.define(identifier.name)
            },
            Stmt::Block(stmt_list) =>
            {
                self.begin_scope();
                for stmt in stmt_list {
                    self.resolve_stmt(stmt, self.current_function, self.current_class);
                }
                self.end_scope();
            },
            Stmt::If(expr, then_stmt) =>
            {
                self.resolve_expr(expr);
                self.resolve_stmt(then_stmt, self.current_function, self.current_class);
            },
            Stmt::IfElse(expr, then_stmt, else_stmt) =>
            {
                self.resolve_expr(expr);
                self.resolve_stmt(&then_stmt, self.current_function, self.current_class);
                self.resolve_stmt(&else_stmt, self.current_function, self.current_class);
            },
            Stmt::While(condition, body) =>
            {
                self.resolve_expr(condition);
                self.resolve_stmt(body, self.current_function, self.current_class);
            },
            Stmt::For(opt_initializer, opt_condition, opt_increment, body) =>
            {
                if let Some(initializer) = opt_initializer.as_ref() {
                    self.resolve_stmt(initializer, self.current_function, self.current_class);
                }
                if let Some(condition) = opt_condition {
                    self.resolve_expr(condition);
                }
                if let Some(increment) = opt_increment {
                    self.resolve_expr(increment);
                }
                self.resolve_stmt(body, self.current_function, self.current_class);
            },
            Stmt::Break     => { /*do nothing*/ },
            Stmt::Continue  => { /*do nothing*/ },
            Stmt::FunctionDeclaration(func_decl) =>
            {
                self.resolve_function(func_decl, FunctionType::Function, self.current_class);
            },
            Stmt::Return(return_token, opt_expr) =>
            {
                match self.current_function {
                    FunctionType::None => {
                        self.error(ResolverErrorKind::ReturnFromTopLevelCode, &return_token.position)
                    },
                    FunctionType::Initializer => {
                        if opt_expr.is_some() {
                            self.error(ResolverErrorKind::ReturnFromInitializer, &return_token.position)
                        }
                    },
                    _ => {
                        if let Some(expr) = opt_expr {
                            self.resolve_expr(expr);
                        }
                    }
                }
            },
            Stmt::ClassDeclaration(class_declaration) =>
            {
                //resolve class name
                match self.declare(class_declaration.identifier.name) {
                    Err(err_kind) => {
                        self.error(err_kind, &class_declaration.identifier.position);
                    },
                    _ => {}
                }
                self.define(class_declaration.identifier.name);

                //>start THIS scope wrapping around methods declarations
                self.begin_scope();

                self.define(self.this_symbol);

                //>resolve methods
                let methods = &class_declaration.methods;
                for (_, rc_method) in methods.into_iter() {
                    let function_type;
                    if rc_method.identifier.name == self.init_symbol {
                        function_type = FunctionType::Initializer;
                    } else {
                        function_type = FunctionType::Method;
                    }
                    self.resolve_function(rc_method, function_type, ClassType::Class);
                }
                //<resolve methods

                self.end_scope();
                //<end THIS scope wrapping around methods declarations
            },
        }
        //> restore-current-function
        self.current_function = enclosing_function;
        self.current_class = enclosing_class;
        //< restore-current-function
    }

    fn resolve_function(&mut self, func_decl: &Rc<FunctionDeclaration>, function_type: FunctionType, class_type: ClassType) {
        match self.declare(func_decl.identifier.name) {
            Err(err_kind) => {
                self.error(err_kind, &func_decl.identifier.position);
            },
            _ => {}
        }
        self.define(func_decl.identifier.name);
        self.begin_scope();
        for param in &func_decl.parameters {
            match self.declare(param.identifier.name) {
                Err(err_kind) => {
                    self.error(err_kind, &param.identifier.position);
                },
                _ => {}
            }
            self.define(param.identifier.name);
        }
        //>Inside a function stmt set current function to FunctionType::Function
        self.resolve_stmt(&func_decl.body, function_type, class_type);
        //<Inside a function stmt set current function to FunctionType::Function
        self.end_scope();
    }

    fn resolve_expr(&mut self, expr: &Expr)
    {
        match &expr.kind
        {
            ExprKind::Binary(expr_left, _, expr_right) =>
            {
                self.resolve_expr(expr_left);
                self.resolve_expr(expr_right);
            },
            ExprKind::Grouping(expr) =>
            {
                self.resolve_expr(expr);
            },
            ExprKind::Unary(_, expr) =>
            {
                self.resolve_expr(expr);
            },
            ExprKind::Literal(_) =>
            {
                /*do nothing*/
            },
            ExprKind::Variable(identifier) =>
            {
                if !self.stack.is_empty() {
                    let opt_bool =self.stack.peek().unwrap().get(&identifier.name);
                    if opt_bool.is_none() || *opt_bool.unwrap() == false {
                        LoxError::resolver_error(crate::error::ResolverErrorKind::LocalVariableNotFound(identifier.name, Rc::clone(&self.string_interner)), identifier.position);
                    }
                }
                self.resolve_local(expr, identifier.name);
            },
            ExprKind::Assign(identifier, expr) =>
            {
                self.resolve_expr(expr);
                self.resolve_local(expr, identifier.name);
            },
            ExprKind::Logical(expr_left, _, expr_right) =>
            {
                self.resolve_expr(expr_left);
                self.resolve_expr(expr_right);
            },
            ExprKind::Call(expr, opt_args, _) =>
            {
                self.resolve_expr(expr);
                if let Some(args) = opt_args {
                    for arg in args {
                        self.resolve_expr(arg);
                    }
                }
            },
            ExprKind::Get(expr, _) => {
                self.resolve_expr(expr);
            },
            ExprKind::Set(object, _, value) => {
                self.resolve_expr(object);
                self.resolve_expr(value);
            },
            ExprKind::This(token) => {
                match self.current_class {
                    ClassType::None => {
                        self.error(ResolverErrorKind::InvalidThisUsage, &token.position)
                    },
                    _ => {
                        self.resolve_local(expr, self.this_symbol);
                    }
                }
            },
        }
    }

    fn begin_scope(&mut self)
    {
        self.stack.push(FxHashMap::default());
    }

    fn end_scope(&mut self)
    {
        self.stack.pop();
    }

    fn declare(&mut self, identifier: IdentifierSymbol) -> Result<(), ResolverErrorKind>
    {
        if let Some(scope) = self.stack.peek_mut()
        {
            if scope.contains_key(&identifier) {
                return Err(ResolverErrorKind::VariableAlreadyExists(identifier, Rc::clone(&self.string_interner)));
            }
            scope.insert(identifier, false);
        }
        return Ok(());
    }

    fn define(&mut self, identifier: IdentifierSymbol)
    {
        if let Some(last) = self.stack.peek_mut() {
            last.insert(identifier, true);
        }
    }

    fn resolve_local(&mut self, expr: &Expr, identifier: IdentifierSymbol)
    {
        for (index, scope) in self.stack.iter().enumerate().rev() {
            if scope.contains_key(&identifier) {
                self.interpreter.resolve(expr.id, index);
            }
        }
    }
}

#[derive(Clone, Debug, Copy)]
enum FunctionType {
    None, Function, Method, Initializer
}

#[derive(Clone, Debug, Copy)]
enum ClassType {
    None, Class
}