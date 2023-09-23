use std::collections::HashMap;

use crate::{parser_stmt::Stmt, parser_expr::Expr, common::Stack, error::LoxError};

struct Resolver {
    stack: Stack<HashMap<String, bool>>
}

impl Resolver
{
    fn new() -> Self {
        Resolver { stack: Stack::new() }
    }

    fn resolve(&mut self, stmt: &Stmt)
    {
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
            Stmt::Var(variable, _, opt_expr) =>
            {
                self.declare(&variable);
                if let Some(expr) = opt_expr {
                    self.resolve_expr(&expr);
                }
                self.define(&variable)
            },
            Stmt::Block(stmt_list) =>
            {
                self.begin_scope();
                self.resolve_block(stmt_list);
                self.end_scope();
            },
            Stmt::If(expr, then_stmt) =>
            {
                self.resolve_expr(expr);
                self.resolve(then_stmt);
            },
            Stmt::IfElse(expr, then_stmt, else_stmt) =>
            {
                self.resolve_expr(expr);
                self.resolve(&then_stmt);
                self.resolve(&else_stmt);
            },
            Stmt::While(condition, body) =>
            {
                self.resolve_expr(condition);
                self.resolve(body);
            },
            Stmt::For(opt_initializer, opt_condition, opt_increment, body) =>
            {
                if let Some(initializer) = opt_initializer.as_ref() {
                    self.resolve(initializer);
                }
                if let Some(condition) = opt_condition {
                    self.resolve_expr(condition);
                }
                if let Some(increment) = opt_increment {
                    self.resolve_expr(increment);
                }
                self.resolve(body);
            },
            Stmt::Break     => { /*do nothing*/ },
            Stmt::Continue  => { /*do nothing*/ },
            Stmt::Function(func_decl) =>
            {
                let name = &func_decl.name.get_identifier();
                self.declare(name);
                self.define(name);
                self.begin_scope();
                for param in &func_decl.parameters {
                    self.declare(&param.get_identifier());
                    self.define(&param.get_identifier());
                }
                self.resolve(&func_decl.body);
            },
            Stmt::Return(_, opt_expr) =>
            {
                if let Some(expr) = opt_expr {
                    self.resolve_expr(expr);
                }
            },
        }
    }

    fn resolve_expr(&self, expr: &Expr)
    {
        match expr
        {
            Expr::Binary(expr_left, _, expr_right) =>
            {
                self.resolve_expr(expr_left);
                self.resolve_expr(expr_right);
            },
            Expr::Grouping(expr) =>
            {
                self.resolve_expr(expr);
            },
            Expr::Unary(_, expr) =>
            {
                self.resolve_expr(expr);
            },
            Expr::Literal(_) =>
            {
                /*do nothing*/
            },
            Expr::Variable(name, pos) =>
            {
                if !self.stack.is_empty() {
                    let opt_bool =self.stack.peek().unwrap().get(name);
                    if opt_bool.is_none() || *opt_bool.unwrap() == false {
                        LoxError::new(crate::error::LoxErrorKind::ResolverLocalVariableNotFound(name.to_owned()), *pos);
                    }
                }
                self.resolve_local(expr, name);
            },
            Expr::Assign(name, expr, _) =>
            {
                self.resolve_expr(expr);
                self.resolve_local(expr, name);
            },
            Expr::Logical(expr_left, _, expr_right) =>
            {
                self.resolve_expr(expr_left);
                self.resolve_expr(expr_right);
            },
            Expr::Call(expr, opt_args, _) =>
            {
                self.resolve_expr(expr);
                if let Some(args) = opt_args {
                    for arg in args {
                        self.resolve_expr(arg);
                    }
                }
            },
        }
    }

    fn resolve_block(&mut self, stmt_list: &Vec<Stmt>)
    {
        for stmt in stmt_list {
            self.resolve(stmt);
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

    fn declare(&mut self, token: &String)
    {
        if let Some(last) = self.stack.peek_mut()
        {
            last.insert(token.clone(), false);
        }
    }

    fn define(&mut self, token: &String)
    {
        if let Some(last) = self.stack.peek_mut() {
            last.insert(token.clone(), true);
        }
    }

    fn resolve_local(&self, expr: &Expr, name: &String)
    {
        for scope in self.stack.iter().rev() {
            if scope.contains_key(name) {
                todo!();
                return;
            }
        }
    }
}
