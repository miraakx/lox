use crate::error::LoxError;
use crate::common::Peekable;
use crate::tokens::{Token,TokenKind};

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

type TokenSource<'a> = Peekable<&'a mut dyn Iterator<Item=Token>, Token>;

pub fn parse(token_iter: &mut dyn Iterator<Item=Token>) -> Option<Expr> {
    let mut token_source: TokenSource = Peekable::new(token_iter);
    expression(&mut token_source);
    todo!();
}

fn expression(token_source: &mut TokenSource) -> Option<Result<Expr,LoxError>> {
    equality(token_source)
}

fn equality(token_source: &mut TokenSource) -> Option<Result<Expr,LoxError>> {

    let opt_expr = comparison(token_source);
    if opt_expr.is_none() {
        return None;
    }

    let r_expr = opt_expr.unwrap();
    if r_expr.is_err() {
        return Some(r_expr);
    }

    let mut expr = r_expr.unwrap();

    loop {
        if token_source.is_last() {
            return Some(Ok(expr));
        }

        let peek_token = token_source.peek().unwrap();
        
        match &peek_token.kind {
            TokenKind::BangEqual|TokenKind::EqualEqual => {
                let token = token_source.next().unwrap();
                let opt_right = comparison(token_source);
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

fn comparison(token_source: &mut TokenSource) -> Option<Result<Expr,LoxError>> {

    let opt_expr = term(token_source);
    if opt_expr.is_none() {
        return None;
    }

    let r_expr = opt_expr.unwrap();
    if r_expr.is_err() {
        return Some(r_expr);
    }

    let mut expr = r_expr.unwrap();

    loop {
        if token_source.is_last() {
            return Some(Ok(expr));
        }

        let peek_token = token_source.peek().unwrap();

        match &peek_token.kind {
            TokenKind::Greater|TokenKind::GreaterEqual|TokenKind::Less|TokenKind::LessEqual => {
                let token = token_source.next().unwrap();
                let opt_right = term(token_source);
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

fn term(token_source: &mut TokenSource) -> Option<Result<Expr,LoxError>> {

    let opt_expr = factor(token_source);
    if opt_expr.is_none() {
        return None;
    }

    let r_expr = opt_expr.unwrap();
    if r_expr.is_err() {
        return Some(r_expr);
    }

    let mut expr = r_expr.unwrap();

    loop {
        if token_source.is_last() {
            return Some(Ok(expr));
        }

        let peek_token = token_source.peek().unwrap();

        match &peek_token.kind {
            TokenKind::Minus|TokenKind::Plus => {
                let token = token_source.next().unwrap();
                let opt_right = factor(token_source);
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



fn factor(token_source: &mut TokenSource) -> Option<Result<Expr,LoxError>> {
    
    let opt_expr = unary(token_source);
    if opt_expr.is_none() {
        return None;
    }

    let r_expr = opt_expr.unwrap();
    if r_expr.is_err() {
        return Some(r_expr);
    }

    let mut expr = r_expr.unwrap();

    loop {

        if token_source.is_last() {
            return Some(Ok(expr));
        }

        let peek_token = token_source.peek().unwrap();

        match &peek_token.kind {
            TokenKind::Slash | TokenKind::Star => {
                let token = token_source.next().unwrap();
                let opt_right = unary(token_source);
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

fn unary(token_source: &mut TokenSource) -> Option<Result<Expr, LoxError>> {

    if token_source.is_last() {
        return None;
    }

    let peek_token = token_source.peek().unwrap();

    match &peek_token.kind {
        TokenKind::Bang | TokenKind::Minus => {
            let token = token_source.next().unwrap();
            let opt_right = unary(token_source);
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
            return primary(token_source);
        }
    }
}

fn primary(token_source: &mut TokenSource) -> Option<Result<Expr, LoxError>> {

    if token_source.is_last() {
        return None;
    }

    let token = token_source.peek().unwrap();

    match &token.kind {
        TokenKind::False|TokenKind::True|TokenKind::Nil|TokenKind::Number|TokenKind::String|TokenKind::Identifier => {
            Some(Ok(Expr::Primary(token_source.next().unwrap())))
        },
        TokenKind::LeftParen => {
            //consume left parenthesis
            token_source.next();
            
            let opt_expr = expression(token_source);
            if opt_expr.is_none() {
                panic!("unexpected end of file 1!");
            }

            let r_expr = opt_expr.unwrap();
            if r_expr.is_err() {
                return Some(r_expr);
            }

            //consume right parenthesis and check if it's present
            let opt_right_paren = token_source.next();
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
