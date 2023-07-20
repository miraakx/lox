use crate::{parser::Expr, tokens::{Literal, TokenKind}};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Identifier(String), String(String),  Number(f64), Bool(bool), Nil
}

fn evaluate(expr: Expr) -> Value {
    match expr {
        Expr::Primary(expr) => {
            todo!()
        },
        Expr::Unary(token, right) => {

            let val_right: Value = evaluate(*right);

            match token.kind {
                TokenKind::Minus => {
                    match val_right {
                        Value::Number(_) => {
                            return val_right;
                        },
                        _ => {
                            panic!("not a number");
                        }
                    }
                }, 
                TokenKind::Bang => {
                    todo!()
                },
                _ => {
                    panic!("expected minus or bang token");
                }
            }
           

        },Expr::Grouping(expr) => {
            evaluate(*expr)
        },
        _ => { todo!() }
    }
}

pub fn interpret(expr: Expr) {
    evaluate(expr);
}