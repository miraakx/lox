use crate::{parser::Expr, tokens::TokenKind};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),  Number(f64), Bool(bool), Nil
}

pub fn interpret(expr: Expr) -> Value {
    evaluate(expr)
}

fn evaluate(expr: Expr) -> Value {
    match expr {
        Expr::Literal(token) => {
            if let Some(value) = token.value {
                match value {
                    crate::tokens::Literal::Identifier(_) => {
                        todo!();
                    },
                    crate::tokens::Literal::String(val) => {
                        return Value::String(val);
                    }
                    crate::tokens::Literal::Number(val) => {
                        return Value::Number(val);
                    },
                    crate::tokens::Literal::Bool(val) => {
                        return Value::Bool(val);
                    },
                    crate::tokens::Literal::Nil => {
                        return Value::Nil;
                    }
                }
            } else {
                panic!("unsupported token!");
            }
        },
        Expr::Unary(token, right) => {
            let val_right: Value = evaluate(*right);
            match token.kind {
                TokenKind::Minus => {
                    match val_right {
                        Value::Number(num) => {
                            return Value::Number(-num);
                        },
                        _ => {
                            panic!("unsupported unary expression!");
                        }
                    }
                },
                TokenKind::Bang => {
                    Value::Bool(!is_truthy(val_right))
                },
                _ => {
                    panic!("invalid token type for unary expression!");
                }
            }
        },
        Expr::Grouping(expr) => { 
            evaluate(*expr) 
        },
        Expr::Binary(left, token, right) => {
            let val_left:  Value = evaluate(*left);
            let val_right: Value = evaluate(*right);
            match token.kind {
                TokenKind::Minus => {
                    match (val_left, val_right) {
                        (Value::Number(num_left), Value::Number(num_right)) => {
                            return Value::Number(num_left - num_right);
                        },
                        _ => {
                            panic!("both expressions side are not of the same type!");
                        }
                    }
                },
                TokenKind::Plus => {
                    match (val_left, val_right) {
                        (Value::Number(num_left), Value::Number(num_right)) => {
                            return Value::Number(num_left + num_right);
                        },
                        (Value::String(str_left), Value::String(str_right)) => {
                            let mut result: String = str_left.clone();
                            result.push_str(&str_right);
                            return Value::String(result);
                        },
                        _ => {
                            panic!("both expressions side are not of the same type!");
                        }
                    }
                },
                TokenKind::Slash => {
                    match (val_left, val_right) {
                        (Value::Number(num_left), Value::Number(num_right)) => {
                            return Value::Number(num_left / num_right);
                        },
                        _ => {
                            panic!("both expressions side are not of the same type!");
                        }
                    }
                },
                TokenKind::Star => {
                    match (val_left, val_right) {
                        (Value::Number(num_left), Value::Number(num_right)) => {
                            return Value::Number(num_left * num_right);
                        },
                        _ => {
                            panic!("both expressions side are not of the same type!");
                        }
                    }
                },
                TokenKind::Greater => {
                    match (val_left, val_right) {
                        (Value::Number(num_left), Value::Number(num_right)) => {
                            return Value::Bool(num_left > num_right);
                        },
                        _ => {
                            panic!("both expressions side are not of the same type!");
                        }
                    }
                },
                TokenKind::GreaterEqual => {
                    match (val_left, val_right) {
                        (Value::Number(num_left), Value::Number(num_right)) => {
                            return Value::Bool(num_left >= num_right);
                        },
                        _ => {
                            panic!("both expressions side are not of the same type!");
                        }
                    }
                },
                TokenKind::Less => {
                    match (val_left, val_right) {
                        (Value::Number(num_left), Value::Number(num_right)) => {
                            return Value::Bool(num_left < num_right);
                        },
                        _ => {
                            panic!("both expressions side are not of the same type!");
                        }
                    }
                },
                TokenKind::LessEqual => {
                    match (val_left, val_right) {
                        (Value::Number(num_left), Value::Number(num_right)) => {
                            return Value::Bool(num_left <= num_right);
                        },
                        _ => {
                            panic!("both expressions side are not of the same type!");
                        }
                    }
                },
                TokenKind::EqualEqual => {
                    return Value::Bool(is_equal(val_left, val_right));
                },
                TokenKind::BangEqual => {
                    return Value::Bool(!is_equal(val_left, val_right));
                },
                _ => {
                    panic!("invalid token type for binary expression between numbers!");
                }
            }
        }  
    }
}

#[inline]
fn is_equal(val_left: Value, val_right: Value) -> bool {
    match (val_left, val_right) {
        (Value::Bool(left),     Value::Bool(right))       => left == right,
        (Value::Number(left),    Value::Number(right))      => left == right,
        (Value::String(left), Value::String(right))   => left == right,
        (Value::Nil,                  Value::Nil)                     => true,
        _                                                             => false
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
