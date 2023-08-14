use std::{fmt::Display, rc::Rc};

use crate::{parser::{Expr, Stmt}, tokens::{TokenKind, extract_identifier, Position}, environment::Environment, error::LoxError};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(Rc<String>),  Number(f64), Bool(bool), Nil
}

impl Display for Value {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(str) => { write!(f, "{}", str ) },
            Value::Number(num)    => { write!(f, "{}", num ) },
            Value::Bool(bool)    => { write!(f, "{}", bool) },
            Value::Nil                  => { write!(f, "nil"     ) },
        }
    }
}

pub struct Interpreter {
    env: Environment
}

impl Interpreter {

    pub fn new() -> Self {
        Interpreter{
            env: Environment::new()
        }
    }

    pub fn new_scope(&mut self) {
        self.env.new_scope();
    }

    pub fn remove_scope(&mut self) {
        self.env.remove_scope();
    }

    pub fn interpret(&mut self, stmt: Stmt) -> Result<(), LoxError>{
        match stmt {
            Stmt::Print(expr) => {
                let value = evaluate_expr(expr, &mut self.env)?;
                println!("{}", value);
            },
            Stmt::ExprStmt(expr) => {
                let _ = evaluate_expr(expr, &mut self.env)?;
            }
            Stmt::Var(variable, _, opt_expr) => {
                match opt_expr {
                    Some(expr) => {
                        let value = evaluate_expr(expr, &mut self.env)?;
                        self.env.define(variable, value);
                    },
                    None => {
                        self.env.define(variable, Value::Nil);
                    },
                }
            }
        }
        Ok(())
    }

}

fn evaluate_expr(expr: Expr, environment: &mut Environment) -> Result<Value, LoxError> {
    match expr {
        Expr::Literal(token) => {
            if let Some(value) = token.value {
                match value {
                    crate::tokens::LiteralValue::String(val) => {
                        return Ok(Value::String(Rc::new(val)));
                    }
                    crate::tokens::LiteralValue::Number(val) => {
                        return Ok(Value::Number(val));
                    },
                    crate::tokens::LiteralValue::Bool(val) => {
                        return Ok(Value::Bool(val));
                    },
                    crate::tokens::LiteralValue::Nil => {
                        return Ok(Value::Nil);
                    },
                    crate::tokens::LiteralValue::Identifier(_) => {
                        panic!("unexpected state");
                    },
                }
            } else {
                panic!("unsupported token!");
            }
        },
        Expr::Unary(token, right) => {
            let val_right: Value = evaluate_expr(*right, environment)?;
            match token.kind {
                TokenKind::Minus => {
                    match val_right {
                        Value::Number(num) => {
                            return Ok(Value::Number(-num));
                        },
                        _ => {
                            panic!("unsupported unary expression!");
                        }
                    }
                },
                TokenKind::Bang => {
                    Ok(Value::Bool(!is_truthy(val_right)))
                },
                _ => {
                    panic!("invalid token type for unary expression!");
                }
            }
        },
        Expr::Grouping(expr) => {
            evaluate_expr(*expr, environment)
        },
        Expr::Binary(left, token, right) => {
            let val_left:  Value = evaluate_expr(*left, environment)?;
            let val_right: Value = evaluate_expr(*right, environment)?;
            match token.kind {
                TokenKind::Minus => {
                    match (val_left, val_right) {
                        (Value::Number(num_left), Value::Number(num_right)) => {
                            return Ok(Value::Number(num_left - num_right));
                        },
                        _ => {
                            panic!("both expressions side are not of the same type!");
                        }
                    }
                },
                TokenKind::Plus => {
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
                TokenKind::Slash => {
                    match (val_left, val_right) {
                        (Value::Number(num_left), Value::Number(num_right)) => {
                            return Ok(Value::Number(num_left / num_right));
                        },
                        _ => {
                            panic!("both expressions side are not of the same type!");
                        }
                    }
                },
                TokenKind::Star => {
                    match (val_left, val_right) {
                        (Value::Number(num_left), Value::Number(num_right)) => {
                            return Ok(Value::Number(num_left * num_right));
                        },
                        _ => {
                            panic!("both expressions side are not of the same type!");
                        }
                    }
                },
                TokenKind::Greater => {
                    match (val_left, val_right) {
                        (Value::Number(num_left), Value::Number(num_right)) => {
                            return Ok(Value::Bool(num_left > num_right));
                        },
                        _ => {
                            panic!("both expressions side are not of the same type!");
                        }
                    }
                },
                TokenKind::GreaterEqual => {
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
                TokenKind::LessEqual => {
                    match (val_left, val_right) {
                        (Value::Number(num_left), Value::Number(num_right)) => {
                            return Ok(Value::Bool(num_left <= num_right));
                        },
                        _ => {
                            panic!("both expressions side are not of the same type!");
                        }
                    }
                },
                TokenKind::EqualEqual => {
                    return Ok(Value::Bool(is_equal(val_left, val_right)));
                },
                TokenKind::BangEqual => {
                    return Ok(Value::Bool(!is_equal(val_left, val_right)));
                },
                _ => {
                    panic!("invalid token type for binary expression between numbers!");
                }
            }
        }
        Expr::Variable(variable, position) => {
            environment.get(&variable, position)
        },
        Expr::Assign(token, expr) => {
            let value: Value = evaluate_expr(*expr, environment)?;
            let tup_identifier: (String, Position) = extract_identifier(token);
            let result: Value = environment.assign(tup_identifier.0, value, tup_identifier.1)?;
            return Ok(result);
        },
    }
}

#[inline]
fn is_equal(val_left: Value, val_right: Value) -> bool {
    match (val_left, val_right) {
        (Value::Bool(left),     Value::Bool(right))             => left == right,
        (Value::Number(left),    Value::Number(right))            => left == right,
        (Value::String(left), Value::String(right)) => left == right,
        (Value::Nil,                  Value::Nil)                           => true,
        _                                                                   => false
    }
}

#[inline]
fn is_truthy(value: Value) -> bool {
    match value {
        Value::String(_)           => true,
        Value::Number(_)           => true,
        Value::Bool(boolean) => boolean,
        Value::Nil                 => false,
    }
}
