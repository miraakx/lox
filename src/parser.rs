use crate::LoxError;
use crate::LoxErrorKind;
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
    match &expr {
        Expr::Binary(expr_left, operator, expr_right) => { print!("( "); print(expr_left); print!(" {:?} ", operator.kind); print(expr_right); print!(") ");}, 
        Expr::Grouping(expr) => {print!("( ");  print(expr); print!(" ) ");}, 
        Expr::Unary(_,expr) => {print!("( "); print!("op"); print(expr); print!(" ) ");},
        Expr::Primary(literal) => {print!(" {:?} ", literal.kind);}
    }
}

type Peekable<'a> = NthPeekable<Lexer<'a>, Token>;

fn expression<'a>(iter: &'a mut Peekable) -> Option<Result<Expr,LoxError>> {
    equality(iter)
}

fn equality<'a>(iter: &'a mut Peekable) -> Option<Result<Expr,LoxError>> {
    let opt_expr = comparison(iter);
    if opt_expr.is_none() {
        return None;
    }
    let r_expr = opt_expr.unwrap();
    if r_expr.is_err() {
        return Some(r_expr);
    }
    let mut expr = r_expr.unwrap();
    loop {
        if iter.is_last() {
            return Some(Ok(expr));
        }
        let peek_token = iter.peek().unwrap();
        match &peek_token.kind {
            TokenKind::BangEqual|TokenKind::EqualEqual => {
                let token = iter.next().unwrap();
                let opt_right = comparison(iter);
                if opt_right.is_none() {
                    todo!("unexpected end of file!");
                }
                let r_right = opt_right.unwrap();
                if r_right.is_err() {
                    return Some(r_right);
                }
                expr = Expr::Binary(Box::new(expr), token, Box::new(r_right.unwrap()));
            },
            _ => {
                return Some(Ok(expr));
            }
        }
    }
} 

fn comparison<'a>(iter: &'a mut Peekable) -> Option<Result<Expr,LoxError>> {
    let opt_expr = term(iter);
    if opt_expr.is_none() {
        return None;
    }
    let r_expr = opt_expr.unwrap();
    if r_expr.is_err() {
        return Some(r_expr);
    }
    let mut expr = r_expr.unwrap();
    loop {
        if iter.is_last() {
            return Some(Ok(expr));
        }
        let peek_token = iter.peek().unwrap();
        match &peek_token.kind {
            TokenKind::Greater|TokenKind::GreaterEqual|TokenKind::Less|TokenKind::LessEqual => {
                let token = iter.next().unwrap();
                let opt_right = term(iter);
                if opt_right.is_none() {
                    todo!("unexpected end of file!");
                }
                let r_right = opt_right.unwrap();
                if r_right.is_err() {
                    return Some(r_right);
                }
                expr = Expr::Binary(Box::new(expr), token, Box::new(r_right.unwrap()));
            },
            _ => {
                return Some(Ok(expr));
            }
        }
    }
}

fn term<'a>(iter: &'a mut Peekable) -> Option<Result<Expr,LoxError>> {
    let opt_expr = factor(iter);
    if opt_expr.is_none() {
        return None;
    }
    let r_expr = opt_expr.unwrap();
    if r_expr.is_err() {
        return Some(r_expr);
    }
    let mut expr = r_expr.unwrap();
    loop {
        if iter.is_last() {
            return Some(Ok(expr));
        }
        let peek_token = iter.peek().unwrap();
        match &peek_token.kind {
            TokenKind::Minus|TokenKind::Plus => {
                let token = iter.next().unwrap();
                let opt_right = factor(iter);
                if opt_right.is_none() {
                    todo!("unexpected end of file!");
                }
                let r_right = opt_right.unwrap();
                if r_right.is_err() {
                    return Some(r_right);
                }
                expr = Expr::Binary(Box::new(expr), token, Box::new(r_right.unwrap()));
            },
            _ => {
                return Some(Ok(expr));
            }
        }
    }
}



fn factor<'a>(iter: &'a mut Peekable) -> Option<Result<Expr,LoxError>> {
    let opt_expr = unary(iter);
    if opt_expr.is_none() {
        return None;
    }
    let r_expr = opt_expr.unwrap();
    if r_expr.is_err() {
        return Some(r_expr);
    }
    let mut expr = r_expr.unwrap();
    loop {
        if iter.is_last() {
            return Some(Ok(expr));
        }
        let peek_token = iter.peek().unwrap();
        match &peek_token.kind {
            TokenKind::Slash | TokenKind::Star => {
                let token = iter.next().unwrap();
                let opt_right = unary(iter);
                if opt_right.is_none() {
                    todo!("unexpected end of file!");
                }
                let r_right = opt_right.unwrap();
                if r_right.is_err() {
                    return Some(r_right);
                }
                expr = Expr::Binary(Box::new(expr), token, Box::new(r_right.unwrap()));
            },
            _ => {
                return Some(Ok(expr));
            }
        }
       
    }
}

fn unary<'a>(iter: &'a mut Peekable) -> Option<Result<Expr, LoxError>> {
    if iter.is_last() {
        return None;
    }
    let peek_token = iter.peek().unwrap();
    match &peek_token.kind {
        TokenKind::Bang | TokenKind::Minus => {
            let token = iter.next().unwrap();
            let opt_right = unary(iter);
            if opt_right.is_none() {
                todo!("unexpected end of file!");
            }
            let right = opt_right.unwrap();
            if right.is_err() {
                return Some(right);
            }
            return Some(Ok(Expr::Unary(token, Box::new(right.unwrap()))));
        },
        _ => {
            return primary(iter);
        }
    }
}

fn primary<'a>(iter: &'a mut Peekable) -> Option<Result<Expr, LoxError>> {
    if iter.is_last() {
       return None;
    }
    let token = iter.peek().unwrap();
    match &token.kind {
        TokenKind::False|TokenKind::True|TokenKind::Nil|TokenKind::Number(_)|TokenKind::String(_) => {
            Some(Ok(Expr::Primary(iter.next().unwrap())))
        },
        TokenKind::LeftParen => {
            //consume left parenthesis
            iter.next();
            let opt_expr = expression(iter);
            if opt_expr.is_none() {
                panic!("unexpected end of file 1!");
            }
            let r_expr = opt_expr.unwrap();
            if r_expr.is_err() {
                return Some(r_expr);
            }
            //consume right parenthesis and check if it's present
            let opt_right_paren = iter.next();
            if opt_right_paren.is_none() {
                panic!("Unexpected end of file 2!");   
            }
            let right_paren = opt_right_paren.unwrap();
            if right_paren.kind != TokenKind::RightParen {
                panic!("Expected \")\", found {:?}", right_paren.kind);   
            }
            return Some(Ok(Expr::Grouping(Box::new(r_expr.unwrap()))));
        },
        found => {
            panic!("Expected literal, found {:?}", found);   
        }
    }
}


pub fn parse<'a>(code: &'a str) -> Option<Result<Expr,LoxError>> { 
    let lexer = Lexer::new(code);
    let mut iter = NthPeekable::new(lexer, 1);
    let opt_expr = expression(&mut iter);
    return opt_expr;
}


