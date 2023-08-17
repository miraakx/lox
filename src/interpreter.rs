use std::{fmt::Display, rc::Rc};

use crate::{parser_stmt::Stmt, tokens::TokenKind, environment::Environment, error::LoxError, parser_expr::Expr};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(Rc<String>), Number(f64), Bool(bool), Nil
}

impl Display for Value
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Value::String(str) => { write!(f, "{}", str ) },
            Value::Number(num) => { write!(f, "{}", num ) },
            Value::Bool(bool) => { write!(f, "{}", bool) },
            Value::Nil => { write!(f, "nil") },
        }
    }
}

pub enum State {
    Normal,
    Break,
    Continue,
}

pub struct Interpreter
{
    env: Environment
}

impl Interpreter
{
    #[inline]
    pub fn new() -> Self
    {
        Interpreter{
            env: Environment::new()
        }
    }

    #[inline]
    pub fn execute(&mut self, stmt: &Stmt) -> Result<State, LoxError>
    {
        match stmt
        {
            Stmt::Print(expr) =>
            {
                let value = self.evaluate(expr)?;
                println!("{}", value);
                return Ok(State::Normal);
            },
            Stmt::ExprStmt(expr) =>
            {
                let _ = self.evaluate(expr)?;
                return Ok(State::Normal);
            }
            Stmt::Var(variable, _, opt_expr) =>
            {
                match opt_expr
                {
                    Some(expr) =>
                    {
                        let value = self.evaluate(expr)?;
                        self.env.define(variable.to_owned(), value);
                    },
                    None =>
                    {
                        self.env.define(variable.to_owned(), Value::Nil);
                    },
                }
                return Ok(State::Normal);
            }
            Stmt::Block(statements) =>
            {
                self.env.new_scope();
                for stmt in statements
                {
                    match self.execute(stmt)? {
                        State::Normal => {
                            continue;
                        },
                        State::Break => {
                            return Ok(State::Break);
                        },
                        State::Continue => {
                            return Ok(State::Continue);
                        },
                    };
                }
                self.env.remove_scope();
                return Ok(State::Normal);
            },
            Stmt::If(condition, then_stmt) =>
            {
                let condition_value = self.evaluate(condition)?;
                if is_truthy(&condition_value) {
                    return self.execute(then_stmt);
                } else {
                    return Ok(State::Normal);
                }
            },
            Stmt::IfElse(condition, then_stmt, else_stmt) =>
            {
                let condition_value = self.evaluate(condition)?;
                if is_truthy(&condition_value) {
                    return self.execute(then_stmt);
                } else {
                    return self.execute(else_stmt);
                }
            },
            Stmt::While(condition, body) =>
            {
                while is_truthy(&self.evaluate(condition)?)
                {
                    match self.execute(body.as_ref())?
                    {
                        State::Normal  | State::Continue =>
                        {
                            continue;
                        },
                        State::Break =>
                        {
                            break;
                        },
                    }
                }
                return Ok(State::Normal);
            },
            Stmt::Break => {
                return Ok(State::Break);
            },
            Stmt::Continue => {
                return Ok(State::Continue);
            },
            Stmt::For(opt_initializer, opt_condition, opt_increment, body) =>
            {
                self.env.new_scope();

                if let Some(initializer) = opt_initializer.as_ref() {
                    self.execute(initializer)?;
                }

                while is_truthy(&self.evaluate_or(opt_condition, Value::Bool(true))?)
                {
                    match self.execute(body)?
                    {
                        State::Normal | State::Continue =>
                        {
                            self.evaluate_or(opt_increment, Value::Bool(true))?;
                            continue;
                        },
                        State::Break =>
                        {
                            break;
                        },
                    }
                }

                self.env.remove_scope();

                return Ok(State::Normal);
            },
        }
    }

    #[inline]
    fn evaluate_or(&mut self, opt_expr: &Option<Expr>, or_value: Value) ->  Result<Value, LoxError> {
        match opt_expr {
            Some(expr) => {
                return self.evaluate(expr);
            },
            None => {
                return Ok(or_value);
            },
        };
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, LoxError>
    {
        match expr {
            Expr::Literal(token) =>
            {
                if let Some(value) = &token.value {
                    match value {
                        crate::tokens::LiteralValue::String(val) =>
                        {
                            return Ok(Value::String(val.clone()));
                        }
                        crate::tokens::LiteralValue::Number(val) =>
                        {
                            return Ok(Value::Number(*val));
                        },
                        crate::tokens::LiteralValue::Bool(val) =>
                        {
                            return Ok(Value::Bool(*val));
                        },
                        crate::tokens::LiteralValue::Nil =>
                        {
                            return Ok(Value::Nil);
                        },
                        crate::tokens::LiteralValue::Identifier(_) =>
                        {
                            panic!("unexpected state");
                        },
                    }
                } else {
                    panic!("unsupported token!");
                }
            },
            Expr::Unary(token, right) =>
            {
                let val_right: Value = self.evaluate(right.as_ref())?;
                match token.kind
                {
                    TokenKind::Minus =>
                    {
                        match val_right
                        {
                            Value::Number(num) =>
                            {
                                return Ok(Value::Number(-num));
                            },
                            _ => {
                                panic!("unsupported unary expression!");
                            }
                        }
                    },
                    TokenKind::Bang =>
                    {
                        Ok(Value::Bool(!is_truthy(&val_right)))
                    },
                    _ => {
                        panic!("invalid token type for unary expression!");
                    }
                }
            },
            Expr::Grouping(expr) =>
            {
                self.evaluate(expr.as_ref())
            },
            Expr::Binary(left, token, right) =>
            {
                let val_left:  Value = self.evaluate(left.as_ref())?;
                let val_right: Value = self.evaluate(right.as_ref())?;
                match token.kind {
                    TokenKind::Minus =>
                    {
                        match (val_left, val_right)
                        {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                return Ok(Value::Number(num_left - num_right));
                            },
                            _ => {
                                panic!("both expressions side are not of the same type!");
                            }
                        }
                    },
                    TokenKind::Plus =>
                    {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                return Ok(Value::Number(num_left + num_right));
                            },
                            (Value::String(str_left), Value::String(str_right)) => {
                                let mut result = (*str_left).clone();
                                result.push_str(&str_right);
                                return Ok(Value::String(Rc::new(result)));
                            },
                            _ => {
                                panic!("both expressions side are not of the same type!");
                            }
                        }
                    },
                    TokenKind::Slash =>
                    {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                return Ok(Value::Number(num_left / num_right));
                            },
                            _ => {
                                panic!("both expressions side are not of the same type!");
                            }
                        }
                    },
                    TokenKind::Star =>
                    {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                return Ok(Value::Number(num_left * num_right));
                            },
                            _ => {
                                panic!("both expressions side are not of the same type!");
                            }
                        }
                    },
                    TokenKind::Greater =>
                    {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                return Ok(Value::Bool(num_left > num_right));
                            },
                            _ => {
                                panic!("both expressions side are not of the same type!");
                            }
                        }
                    },
                    TokenKind::GreaterEqual =>
                    {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                return Ok(Value::Bool(num_left >= num_right));
                            },
                            _ => {
                                panic!("both expressions side are not of the same type!");
                            }
                        }
                    },
                    TokenKind::Less => {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                return Ok(Value::Bool(num_left < num_right));
                            },
                            _ => {
                                panic!("both expressions side are not of the same type!");
                            }
                        }
                    },
                    TokenKind::LessEqual =>
                    {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                return Ok(Value::Bool(num_left <= num_right));
                            },
                            _ => {
                                panic!("both expressions side are not of the same type!");
                            }
                        }
                    },
                    TokenKind::EqualEqual =>
                    {
                        return Ok(Value::Bool(is_equal(val_left, val_right)));
                    },
                    TokenKind::BangEqual =>
                    {
                        return Ok(Value::Bool(!is_equal(val_left, val_right)));
                    },
                    _ => {
                        panic!("invalid token type for binary expression between numbers!");
                    }
                }
            }
            Expr::Variable(variable, position) =>
            {
                self.env.get(&variable, *position)
            },
            Expr::Assign(name, expr, pos) =>
            {
                let value: Value = self.evaluate(expr.as_ref())?;
                let result: Value = self.env.assign(name.to_owned(), value, *pos)?;
                return Ok(result);
            },
            Expr::Logical(left, token, right) => {
                let val_left:  Value = self.evaluate(left.as_ref())?;
                match token.kind {
                    TokenKind::Or => {
                        if is_truthy(&val_left) {
                            return Ok(val_left);
                        } else {
                            return self.evaluate(right.as_ref());
                        }
                    },
                    TokenKind::And => {
                        if !is_truthy(&val_left) {
                            return Ok(val_left);
                        } else {
                            return self.evaluate(right.as_ref());
                        }
                    },
                    _ => {
                        panic!("invalid operator type for logical expression!");
                    }
                }
            },
        }
    }

}

#[inline]
fn is_equal(val_left: Value, val_right: Value) -> bool
{
    match (val_left, val_right)
    {
        (Value::Bool(left),     Value::Bool(right))             => left == right,
        (Value::Number(left),    Value::Number(right))            => left == right,
        (Value::String(left), Value::String(right)) => left == right,
        (Value::Nil,                  Value::Nil)                           => true,
        _                                                                   => false
    }
}

#[inline]
fn is_truthy(value: &Value) -> bool
{
    match value
    {
        Value::String(_)           => true,
        Value::Number(_)           => true,
        Value::Bool(boolean) => *boolean,
        Value::Nil                 => false,
    }
}
