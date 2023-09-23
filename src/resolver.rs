use std::collections::HashMap;

use crate::{parser_stmt::Stmt, parser_expr::Expr, common::Stack, tokens::Token, error::LoxError};

struct Resolver {
    stack: Stack<HashMap<String, bool>>
}

impl Resolver
{
    fn new() -> Self {
        Resolver { stack: Stack::new() }
    }

    fn resolve(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Print(_) => todo!(),
            Stmt::ExprStmt(_) => todo!(),
            Stmt::Var(variable, pos, opt_expr) => {
                self.declare(&variable);
                if let Some(expr) = opt_expr {
                    self.resolve_expr(&expr);
                }
                self.define(&variable)
            },
            Stmt::Block(stmt_list) => {
                self.begin_scope();
                self.resolve_block(stmt_list);
                self.end_scope();
            },
            Stmt::If(_, _) => todo!(),
            Stmt::IfElse(_, _, _) => todo!(),
            Stmt::While(_, _) => todo!(),
            Stmt::For(_, _, _, _) => todo!(),
            Stmt::Break => todo!(),
            Stmt::Continue => todo!(),
            Stmt::Function(_) => todo!(),
            Stmt::Return(_, _) => todo!(),
        }
    }

    fn resolve_block(&mut self, stmt_list: Vec<Stmt>) {
        for stmt in stmt_list {
            self.resolve(stmt);
        }
    }

    #[inline]
    fn begin_scope(&mut self) {
        self.stack.push(HashMap::new());
    }

    #[inline]
    fn end_scope(&mut self) {
        self.stack.pop();
    }

    fn resolve_expr(&self, expr: &Expr) {
        match expr {
            Expr::Binary(_, _, _) => todo!(),
            Expr::Grouping(_) => todo!(),
            Expr::Unary(_, _) => todo!(),
            Expr::Literal(_) => todo!(),
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
            Expr::Assign(_, _, _) => todo!(),
            Expr::Logical(_, _, _) => todo!(),
            Expr::Call(_, _, _) => todo!(),
        }
    }

    fn declare(&mut self, token: &String) {
        if let Some(last) = self.stack.peek_mut() {
            last.insert(token.clone(), false);
        }
    }

    fn define(&mut self, token: &String) {
        if let Some(last) = self.stack.peek_mut() {
            last.insert(token.clone(), true);
        }
    }

    fn resolve_local(&self, expr: &Expr, name: &String) {
        for scope in self.stack.iter().rev() {

        }
    }

}
