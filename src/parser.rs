use crate::lexer::Token;
use crate::lexer::Lexer;
use crate::common::NthPeekable;
use crate::lexer::TokenKind;

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Unary(Token, Box<Expr>),
    Primary(Token)    
}


pub fn print(expr: &Expr) {
    match expr {
        Expr::Binary(expr_left, _, expr_right) => { print(expr_left); print!(" op "); print(expr_right); }, 
        Expr::Grouping(expr) => {print!("(");  print(expr); print!(")");}, 
        Expr::Unary(_,expr) => {print!(" op "); print(expr);},
        Expr::Primary(_) => {print!(" token ");}
    }
}

type Peekable<'a> = NthPeekable<Lexer<'a>, Token>;

fn expression<'a>(iter: &'a mut Peekable) -> Expr {
    equality(iter)
}

fn equality<'a>(iter: &'a mut Peekable) -> Expr {
    let mut expr = comparison(iter);
    loop {
        if let Some(peek) = iter.peek() {
            match peek.kind {
                TokenKind::BangEqual|TokenKind::EqualEqual => {
                    let token = iter.next().unwrap();
                    let right = comparison(iter);
                    expr = Expr::Binary(Box::new(expr), token, Box::new(right));
                },
                _ => {
                    return expr;
                }
            }
        }
    }
} 

fn comparison<'a>(iter: &'a mut Peekable) -> Expr {
    let mut expr = term(iter);

    loop {
        if let Some(peek) = iter.peek() {
            match peek.kind {
                TokenKind::Greater|TokenKind::GreaterEqual|TokenKind::Less|TokenKind::LessEqual => {
                    let token = iter.next().unwrap();
                    let right = term(iter);
                    expr = Expr::Binary(Box::new(expr), token, Box::new(right));
                },
                _ => {
                    return expr;
                }
            }
        } else {
            return expr;
        }
    }
}

fn term<'a>(iter: &'a mut Peekable) -> Expr {
    let mut expr = factor(iter);
    loop {
        if let Some(peek) = iter.peek() {
            match peek.kind {
                TokenKind::Minus|TokenKind::Plus => {
                    let token = iter.next().unwrap();
                    let right = factor(iter);
                    expr = Expr::Binary(Box::new(expr), token, Box::new(right));
                },
                _ => {
                    return expr;
                }
            }
        } else {
            return expr;
        }
    }
}



fn factor<'a>(iter: &'a mut Peekable) -> Expr {
    let mut expr = unary(iter);
    loop {
        if let Some(peek) = iter.peek() {
            match peek.kind {
                TokenKind::Slash|TokenKind::Star => {
                    let token = iter.next().unwrap();
                    let right = unary(iter);
                    expr = Expr::Binary(Box::new(expr), token, Box::new(right));
                },
                _ => {
                    return expr;
                }
            }
        } else {
            return expr;
        }
    }
}

fn unary<'a>(iter: &'a mut Peekable) -> Expr {
    if let Some(peek) = iter.peek() {
        match peek.kind {
            TokenKind::Bang|TokenKind::Minus => {
                let token = iter.next().unwrap();
                let right = unary(iter);
                return Expr::Unary(token, Box::new(right));
            },
            _ => {
                return primary(iter);
            }
        }
    } else {
        return primary(iter);
    }
}

fn primary<'a>(iter: &'a mut Peekable) -> Expr {
    if let Some(peek) = iter.peek() {
        match peek.kind {
            TokenKind::False|TokenKind::True|TokenKind::Nil|TokenKind::Number(_)|TokenKind::String(_) => {
                Expr::Primary(iter.next().unwrap())
            },
            _ => {panic!("");}


        }
    } else {
        panic!("error");
    }
}


pub fn parse<'a>(code: &'a str) -> Expr { 
    let lexer = Lexer::new(code);
    let mut iter = NthPeekable::new(lexer, 1);
    return expression(&mut iter);
}


