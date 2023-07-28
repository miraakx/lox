use crate::error::{LoxError, LoxErrorKind};
use crate::common::Peekable;
use crate::tokens::{Token,TokenKind};

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Unary(Token, Box<Expr>),
    Literal(Token)    
}

pub enum Stmt {
    Print(Expr), ExprStmt(Expr)
}

pub fn print(expr: &Expr) {
    match &expr {
        Expr::Binary(expr_left, operator, expr_right) => { print!("( "); print(expr_left); print!(" {:?} ", operator.kind); print(expr_right); print!(") "); }, 
        Expr::Grouping(expr)                                              => { print!("( "); print(expr); print!(" ) "); }, 
        Expr::Unary(_,expr)                                               => { print!("( "); print!("op"); print(expr); print!(" ) "); },
        Expr::Literal(literal)                                                => { print!(" {:?} ", literal.kind); }
    }
}

type TokenSource<'a> = Peekable<&'a mut dyn Iterator<Item=Token>, Token>;

pub struct Parser<'a> {
    token_source: Peekable<&'a mut dyn Iterator<Item=Token>, Token>
}

impl <'a> Parser<'a> {
    pub fn new(token_iter: &'a mut dyn Iterator<Item=Token>) -> Self {
        Parser { token_source: Peekable::new(token_iter) }
    }
}

impl <'a> Iterator for Parser<'a> {
    type Item = Stmt;

    fn next(&mut self) -> Option<Self::Item> {
        let opt_stmt = statement(&mut self.token_source);
        if opt_stmt.is_none() {
            return None;
        }

        let r_stmt = opt_stmt.unwrap();
        match r_stmt {
            Ok(stmt) => Some(stmt),
            Err(_) => todo!(),
        }
    }
}

fn statement(token_source: &mut TokenSource) -> Option<Result<Stmt, LoxError>>{
    let opt_token = token_source.next();
    if opt_token.is_none() {
        return None;
    }

    let token = opt_token.unwrap();
    match token.kind {
        TokenKind::Print => {
            return print_statement(token_source);
        },
        _ => {
            return expression_statement(token_source);
        }
    }
}

fn print_statement(token_source: &mut TokenSource) -> Option<Result<Stmt, LoxError>> {
    let opt_expr = expression(token_source);
    if opt_expr.is_none() {
        return None;
    }

    let r_expr = opt_expr.unwrap();
    if r_expr.is_err() {
        return Some(Err(r_expr.err().unwrap()));
    }

    let stmt = Stmt::Print(r_expr.unwrap());

    return consume_semicolon(stmt, token_source);
}

fn expression_statement(token_source: &mut TokenSource) -> Option<Result<Stmt, LoxError>> {
    let opt_expr = expression(token_source);
    if opt_expr.is_none() {
        return None;
    }

    let r_expr = opt_expr.unwrap();
    if r_expr.is_err() {
        return Some(Err(r_expr.err().unwrap()));
    }

    let stmt = Stmt::ExprStmt(r_expr.unwrap());

    return consume_semicolon(stmt, token_source);
    
}

#[inline]
fn consume_semicolon(stmt: Stmt, token_source: &mut TokenSource) ->Option<Result<Stmt, LoxError>>{
    let opt_token = token_source.next();
    if opt_token.is_none() {
        panic!("Caso non gestito");
    }

    let token = opt_token.unwrap();
    match token.kind {
        TokenKind::Semicolon => {
            return Some(Ok(stmt));
        },
        _ => {
            return Some(Err(LoxError::new(LoxErrorKind::MissingSemicolon, token.position)));
        }
    }
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
                let operator                             = token_source.next().unwrap();
                
                let opt_right   = comparison(token_source);
                if opt_right.is_none() {
                    return Some(Err(LoxError::new(LoxErrorKind::UnexpectedEndOfFile, operator.position)))
                }

                let r_right = opt_right.unwrap();
                if r_right.is_err() {
                    return Some(r_right);
                }

                expr = Expr::Binary(Box::new(expr), operator, Box::new(r_right.unwrap()));
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
            TokenKind::Greater | 
            TokenKind::GreaterEqual | 
            TokenKind::Less | 
            TokenKind::LessEqual => 
            {
                let operator = token_source.next().unwrap();
                
                let opt_right = term(token_source);
                if opt_right.is_none() {
                    return Some(Err(LoxError::new(LoxErrorKind::UnexpectedEndOfFile, operator.position)))
                }

                let r_right = opt_right.unwrap();
                if r_right.is_err() {
                    return Some(r_right);
                }

                expr = Expr::Binary(Box::new(expr), operator, Box::new(r_right.unwrap()));
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
                let operator = token_source.next().unwrap();

                let opt_right = factor(token_source);
                if opt_right.is_none() {
                    return Some(Err(LoxError::new(LoxErrorKind::UnexpectedEndOfFile, operator.position)))
                }

                let r_right = opt_right.unwrap();
                if r_right.is_err() {
                    return Some(r_right);
                }

                expr = Expr::Binary(Box::new(expr), operator, Box::new(r_right.unwrap()));
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
                let operator = token_source.next().unwrap();

                let opt_right = unary(token_source);
                if opt_right.is_none() {
                    return Some(Err(LoxError::new(LoxErrorKind::UnexpectedEndOfFile, operator.position)))
                }

                let r_right = opt_right.unwrap();
                if r_right.is_err() {
                    return Some(r_right);
                }
                
                expr = Expr::Binary(Box::new(expr), operator, Box::new(r_right.unwrap()));
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
            let operator = token_source.next().unwrap();

            let opt_right = unary(token_source);
            if opt_right.is_none() {
                return Some(Err(LoxError::new(LoxErrorKind::UnexpectedEndOfFile, operator.position)))
            }

            let right = opt_right.unwrap();
            if right.is_err() {
                return Some(right);
            }

            return Some(Ok(Expr::Unary(operator, Box::new(right.unwrap()))));
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
    let position = token.position;

    match &token.kind {
        TokenKind::False|TokenKind::True|TokenKind::Nil|TokenKind::Number|TokenKind::String|TokenKind::Identifier => {
            Some(Ok(Expr::Literal(token_source.next().unwrap())))
        },
        TokenKind::LeftParen => {
            //consume left parenthesis
            token_source.next();
            
            let opt_expr = expression(token_source);
            if opt_expr.is_none() {
                return Some(Err(LoxError::new(LoxErrorKind::UnexpectedEndOfFile, position)));
            }

            let r_expr = opt_expr.unwrap();
            if r_expr.is_err() {
                return Some(r_expr);
            }

            //consume right parenthesis and check if it's present
            let opt_right_paren = token_source.next();
            if opt_right_paren.is_none() {
                return Some(Err(LoxError::new(LoxErrorKind::UnexpectedEndOfFile, position)));
            }

            let right_paren = opt_right_paren.unwrap();
            if right_paren.kind != TokenKind::RightParen {
                return Some(Err(LoxError::new(LoxErrorKind::MissingClosingParenthesis, position)));
            }

            return Some(Ok(Expr::Grouping(Box::new(r_expr.unwrap()))));
        },
        found => {
            return Some(Err(LoxError::new(LoxErrorKind::LiteralExpected(*found), position)));
        }
    }
}

pub fn parse(token_iter: &mut dyn Iterator<Item=Token>) -> Option<Result<Expr,LoxError>> {
    let mut token_source: TokenSource = Peekable::new(token_iter);
    expression(&mut token_source)
}