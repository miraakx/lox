use std::{collections::HashMap, cell::RefCell, rc::Rc};

use string_interner::StringInterner;

use crate::{parser_stmt::Stmt, parser_expr::{Expr, ExprKind}, common::Stack, error::{LoxError, ErrorLogger, ResolverErrorKind}, interpreter::Interpreter, alias::Identifier, tokens::Position};

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    stack: Stack<HashMap<Identifier, bool>>,
    error_logger: Box<dyn ErrorLogger>,
    string_interner: Rc<RefCell<StringInterner>>,
    has_error: bool,
    current_function: FunctionType
}

impl <'a> Resolver<'a>
{
    #[inline]
    pub fn new(interpreter: &'a mut Interpreter, error_logger: impl ErrorLogger + 'static, string_interner: Rc<RefCell<StringInterner>>) -> Self {
        Resolver { stack: Stack::new(), interpreter, error_logger: Box::new(error_logger), string_interner, has_error: false, current_function: FunctionType::None }
    }

    fn error(&mut self, err_kind: ResolverErrorKind, position: &Position) {
        self.error_logger.log(LoxError::resolver_error(err_kind, *position));
        self.has_error = true;
    }

    #[inline]
    pub fn resolve(&mut self, stmts: &[Stmt]) -> Result<(),()>{
        for stmt in stmts {
            self.resolve_stmt(stmt, self.current_function);
        }
        if self.has_error {
            Err(())
        } else {
            Ok(())
        }
    }

    fn resolve_stmt(&mut self, stmt: &Stmt, function_type: FunctionType)
    {
        let enclosing_function = self.current_function;
        //> set-current-function
        self.current_function = function_type;
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
            Stmt::Var(variable, position, opt_expr) =>
            {
                match self.declare(*variable) {
                    Err(err_kind) => {
                        self.error(err_kind, position);
                    },
                    _ => {}
                }
                if let Some(expr) = opt_expr {
                    self.resolve_expr(&expr);
                }
                self.define(*variable)
            },
            Stmt::Block(stmt_list) =>
            {
                self.begin_scope();
                for stmt in stmt_list {
                    self.resolve_stmt(stmt, self.current_function);
                }
                self.end_scope();
            },
            Stmt::If(expr, then_stmt) =>
            {
                self.resolve_expr(expr);
                self.resolve_stmt(then_stmt, self.current_function);
            },
            Stmt::IfElse(expr, then_stmt, else_stmt) =>
            {
                self.resolve_expr(expr);
                self.resolve_stmt(&then_stmt, self.current_function);
                self.resolve_stmt(&else_stmt, self.current_function);
            },
            Stmt::While(condition, body) =>
            {
                self.resolve_expr(condition);
                self.resolve_stmt(body, self.current_function);
            },
            Stmt::For(opt_initializer, opt_condition, opt_increment, body) =>
            {
                if let Some(initializer) = opt_initializer.as_ref() {
                    self.resolve_stmt(initializer, self.current_function);
                }
                if let Some(condition) = opt_condition {
                    self.resolve_expr(condition);
                }
                if let Some(increment) = opt_increment {
                    self.resolve_expr(increment);
                }
                self.resolve_stmt(body, self.current_function);
            },
            Stmt::Break     => { /*do nothing*/ },
            Stmt::Continue  => { /*do nothing*/ },
            Stmt::FunctionDeclaration(func_decl) =>
            {
                let name = &func_decl.name.get_identifier();
                match self.declare(*name) {
                    Err(err_kind) => {
                        self.error(err_kind, &func_decl.name.position);
                    },
                    _ => {}
                }
                self.define(*name);
                self.begin_scope();
                for param in &func_decl.parameters {
                    let name = &param.get_identifier();
                    match self.declare(*name) {
                        Err(err_kind) => {
                            self.error(err_kind, &param.position);
                        },
                        _ => {}
                    }
                    self.define(*name);
                }
                //>Inside a function stmt set current function to FunctionType::Function
                self.resolve_stmt(&func_decl.body, FunctionType::Function);
                //<Inside a function stmt set current function to FunctionType::Function
            },
            Stmt::Return(return_token, opt_expr) =>
            {
                match self.current_function {
                    FunctionType::None => {
                        self.error(ResolverErrorKind::ReturnFromTopLevelCode, &return_token.position)
                    },
                    _ => {}
                }
                if let Some(expr) = opt_expr {
                    self.resolve_expr(expr);
                }
            },
            Stmt::ClassDeclaration(class_declaration) => {

                match self.declare(class_declaration.name.get_identifier()) {
                    Err(err_kind) => {
                        self.error(err_kind, &class_declaration.name.position);
                    },
                    _ => {}
                }
                self.define(class_declaration.name.get_identifier());
            },
        }
        //> restore-current-function
        self.current_function = enclosing_function;
        //< restore-current-function
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
            ExprKind::Variable(name, pos) =>
            {
                if !self.stack.is_empty() {
                    let opt_bool =self.stack.peek().unwrap().get(name);
                    if opt_bool.is_none() || *opt_bool.unwrap() == false {
                        LoxError::resolver_error(crate::error::ResolverErrorKind::LocalVariableNotFound(*name, self.string_interner.clone()), *pos);
                    }
                }
                self.resolve_local(expr, *name);
            },
            ExprKind::Assign(name, expr, _) =>
            {
                self.resolve_expr(expr);
                self.resolve_local(expr, *name);
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
        }
    }

    #[inline]
    fn begin_scope(&mut self)
    {
        self.stack.push(HashMap::new());
    }

    #[inline]
    fn end_scope(&mut self)
    {
        self.stack.pop();
    }

    fn declare(&mut self, identifier: Identifier) -> Result<(), ResolverErrorKind>
    {
        if let Some(scope) = self.stack.peek_mut()
        {
            if scope.contains_key(&identifier) {
                return Err(ResolverErrorKind::VariableAlreadyExists(identifier, self.string_interner.clone()));
            }
            scope.insert(identifier, false);
        }
        return Ok(());
    }

    fn define(&mut self, identifier: Identifier)
    {
        if let Some(last) = self.stack.peek_mut() {
            last.insert(identifier, true);
        }
    }

    fn resolve_local(&mut self, expr: &Expr, identifier: Identifier)
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
    None, Function, Class
}