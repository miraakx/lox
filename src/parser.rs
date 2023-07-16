use crate::error::{LoxError, ErrorRepo};
use crate::common::Peekable;
use crate::tokens::{Token,TokenKind, TokenSource};

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

/*
pub fn parse(code: &'a str) -> Option<Result<Expr,LoxError>> { 
    let error_repo = ErrorRepoVec::new();
    let lexer: Lexer<'a> = Lexer::new(code);
    let mut iter: Peekable<Lexer<'a>, Token> = Peekable::new(lexer);
    let opt_expr: Option<Result<Expr, LoxError>> = expression(&mut iter);
    return opt_expr;
}
*/

pub struct Parser<'a> {
    error_repo: Box<dyn ErrorRepo + 'a>,
    token_source: Peekable<Box<dyn TokenSource<Item=Token> + 'a>, Token>,
}

impl<'a> Parser<'a> {

    pub fn new(token_source: Box<dyn TokenSource<Item=Token> + 'a>, error_repo: Box<dyn ErrorRepo + 'a>) -> Parser<'a> {
        Parser {
            error_repo, 
            token_source: Peekable::new(token_source)
        }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.expression();
        todo!();
    }

    fn expression(&mut self) -> Option<Result<Expr,LoxError>> {
        self.equality()
    }
    
    fn equality(&mut self) -> Option<Result<Expr,LoxError>> {
        let opt_expr = self.comparison();
        if opt_expr.is_none() {
            return None;
        }
        let r_expr = opt_expr.unwrap();
        if r_expr.is_err() {
            return Some(r_expr);
        }
        let mut expr = r_expr.unwrap();
        loop {
            if self.token_source.is_last() {
                return Some(Ok(expr));
            }
            let peek_token = self.token_source.peek().unwrap();
            match &peek_token.kind {
                TokenKind::BangEqual|TokenKind::EqualEqual => {
                    let token = self.token_source.next().unwrap();
                    let opt_right = self.comparison();
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
    
    fn comparison(&mut self) -> Option<Result<Expr,LoxError>> {
        let opt_expr = self.term();
        if opt_expr.is_none() {
            return None;
        }
        let r_expr = opt_expr.unwrap();
        if r_expr.is_err() {
            return Some(r_expr);
        }
        let mut expr = r_expr.unwrap();
        loop {
            if self.token_source.is_last() {
                return Some(Ok(expr));
            }
            let peek_token = self.token_source.peek().unwrap();
            match &peek_token.kind {
                TokenKind::Greater|TokenKind::GreaterEqual|TokenKind::Less|TokenKind::LessEqual => {
                    let token = self.token_source.next().unwrap();
                    let opt_right = self.term();
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
    
    fn term(&mut self) -> Option<Result<Expr,LoxError>> {
        let opt_expr = self.factor();
        if opt_expr.is_none() {
            return None;
        }
        let r_expr = opt_expr.unwrap();
        if r_expr.is_err() {
            return Some(r_expr);
        }
        let mut expr = r_expr.unwrap();
        loop {
            if self.token_source.is_last() {
                return Some(Ok(expr));
            }
            let peek_token = self.token_source.peek().unwrap();
            match &peek_token.kind {
                TokenKind::Minus|TokenKind::Plus => {
                    let token = self.token_source.next().unwrap();
                    let opt_right = self.factor();
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
    
    
    
    fn factor(&mut self) -> Option<Result<Expr,LoxError>> {
        let opt_expr = self.unary();
        if opt_expr.is_none() {
            return None;
        }
        let r_expr = opt_expr.unwrap();
        if r_expr.is_err() {
            return Some(r_expr);
        }
        let mut expr = r_expr.unwrap();
        loop {
            if self.token_source.is_last() {
                return Some(Ok(expr));
            }
            let peek_token = self.token_source.peek().unwrap();
            match &peek_token.kind {
                TokenKind::Slash | TokenKind::Star => {
                    let token = self.token_source.next().unwrap();
                    let opt_right = self.unary();
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
    
    fn unary(&mut self) -> Option<Result<Expr, LoxError>> {
        if self.token_source.is_last() {
            return None;
        }
        let peek_token = self.token_source.peek().unwrap();
        match &peek_token.kind {
            TokenKind::Bang | TokenKind::Minus => {
                let token = self.token_source.next().unwrap();
                let opt_right = self.unary();
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
                return self.primary();
            }
        }
    }
    
    fn primary(&mut self) -> Option<Result<Expr, LoxError>> {
        if self.token_source.is_last() {
           return None;
        }
        let token = self.token_source.peek().unwrap();
        match &token.kind {
            TokenKind::False|TokenKind::True|TokenKind::Nil|TokenKind::Literal => {
                Some(Ok(Expr::Primary(self.token_source.next().unwrap())))
            },
            TokenKind::LeftParen => {
                //consume left parenthesis
                self.token_source.next();
                let opt_expr = self.expression();
                if opt_expr.is_none() {
                    panic!("unexpected end of file 1!");
                }
                let r_expr = opt_expr.unwrap();
                if r_expr.is_err() {
                    return Some(r_expr);
                }
                //consume right parenthesis and check if it's present
                let opt_right_paren = self.token_source.next();
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
}