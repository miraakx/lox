use core::fmt;
use std::rc::Rc;

use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;
use string_interner::Symbol;
use unique_id::sequence::SequenceGenerator;
use unique_id::Generator;

use crate::alias::IdentifierSymbol;

use super::{position::Position, tokens::{Token, TokenKind}};

#[derive(Clone, Debug, PartialEq)]
pub struct Identifier
{
    pub name: IdentifierSymbol,
    pub position: Position
}

impl fmt::Display for Identifier
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "Identifier (Symbol={})", self.name.to_usize())
    }
}

#[derive(Clone, Debug)]
pub struct Operator<Kind>
{
    pub kind: Kind,
    pub position: Position
}

#[derive(Clone, Debug)]
pub enum BinaryOperatorKind
{
    Minus,           Plus,
    Slash,           Star,
    BangEqual,       EqualEqual,
    Greater,         GreaterEqual,
    Less,            LessEqual,
}

impl Operator<BinaryOperatorKind>
{
    pub fn from_token(token: &Token) -> Self
    {
        let bonary_op_kind = match token.kind
        {
            TokenKind::Minus        => BinaryOperatorKind::Minus,
            TokenKind::Plus         => BinaryOperatorKind::Plus,
            TokenKind::Slash        => BinaryOperatorKind::Slash,
            TokenKind::Star         => BinaryOperatorKind::Star,
            TokenKind::BangEqual    => BinaryOperatorKind::BangEqual,
            TokenKind::EqualEqual   => BinaryOperatorKind::EqualEqual,
            TokenKind::Greater      => BinaryOperatorKind::Greater,
            TokenKind::GreaterEqual => BinaryOperatorKind::GreaterEqual,
            TokenKind::Less         => BinaryOperatorKind::Less,
            TokenKind::LessEqual    => BinaryOperatorKind::LessEqual,
            _ =>
            {
                panic!("Internal error, unexpecter operator type");
            }
        };
        Self { kind: bonary_op_kind, position: token.position }
    }
}

#[derive(Clone, Debug)]
pub enum UnaryOperatorKind
{
    Bang, Minus,
}

impl Operator<UnaryOperatorKind>
{
    pub fn from_token(token: &Token) -> Self
    {
        let bonary_op_kind = match token.kind {
            TokenKind::Bang  => UnaryOperatorKind::Bang,
            TokenKind::Minus => UnaryOperatorKind::Minus,
            _ => {
                panic!("Internal error, unexpecter operator type");
            }
        };
        Self { kind: bonary_op_kind, position: token.position }
    }
}

#[derive(Clone, Debug)]
pub enum LogicalOperatorKind
{
    And, Or,
}

impl Operator<LogicalOperatorKind>
{
    pub fn from_token(token: &Token) -> Self
    {
        let bonary_op_kind = match token.kind
        {
            TokenKind::And  => LogicalOperatorKind::And,
            TokenKind::Or => LogicalOperatorKind::Or,
            _ => {
                panic!("Internal error, unexpecter operator type");
            }
        };
        Self { kind: bonary_op_kind, position: token.position }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String(Rc<String>, Position),
    Number(f64, Position),
    True(Position),
    False(Position),
    Nil(Position),
}

static ID_GENERATOR: Lazy<SequenceGenerator> = Lazy::new(SequenceGenerator::default);

#[derive(Clone, Debug)]
pub struct Expr
{
    pub id: i64,
    pub kind: ExprKind
}

impl Expr
{
    pub fn new(kind: ExprKind) -> Self {
        Self { id: ID_GENERATOR.next_id(), kind }
    }
}

#[derive(Clone, Debug)]
pub enum ExprKind
{
    Binary  (Box<BinaryExpr>),
    Grouping(Box<Expr>),
    Unary   (Box<UnaryExpr>),
    Literal (Literal),
    Variable(Identifier),
    Assign  (Box<AssignExpr>),
    Logical (Box<LogicalExpr>),
    Call    (Box<CallExpr>),
    Get     (Box<GetExpr>),
    Set     (Box<SetExpr>),
    This    (Position),
    Super   (Identifier)
}

#[derive(Clone, Debug)]
pub struct GetExpr {
    pub expr: Expr,
    pub identifier: Identifier
}

#[derive(Clone, Debug)]
pub struct AssignExpr {
    pub identifier: Identifier,
    pub expr: Expr
}

#[derive(Clone, Debug)]
pub struct UnaryExpr {
    pub operator: Operator<UnaryOperatorKind>,
    pub expr: Expr
}

#[derive(Clone, Debug)]
pub struct BinaryExpr {
    pub left: Expr,
    pub operator: Operator<BinaryOperatorKind>,
    pub right: Expr
}

#[derive(Clone, Debug)]
pub struct LogicalExpr {
    pub left: Expr,
    pub operator: Operator<LogicalOperatorKind>,
    pub right: Expr
}

#[derive(Clone, Debug)]
pub struct SetExpr {
    pub target: Expr,
    pub identifier: Identifier,
    pub value: Expr
}

#[derive(Clone, Debug)]
pub struct CallExpr {
    pub callee: Expr,
    pub arguments: Vec<Expr>,
    pub position: Position
}

#[derive(Clone, Debug)]
pub enum Stmt
{
    Expr    (Expr),
    Var     (Identifier, Option<Expr>),
    Block   (Vec<Stmt>),
    If      (Box<IfStmt>),
    IfElse  (Box<IfElseStmt>),
    While   (Box<WhileStmt>),
    Return  (Option<Expr>, Position),
    Break,
    Continue,
    FunctionDeclaration (Rc<FunctionDeclaration>),
    ClassDeclaration    (Rc<ClassDeclaration>),
    Print   (Expr),
}

#[derive(Clone, Debug)]
pub struct IfElseStmt {
    pub condition: Expr,
    pub then_stmt: Stmt,
    pub else_stmt: Stmt
}

#[derive(Clone, Debug)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_stmt: Stmt
}

#[derive(Clone, Debug)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: Stmt
}

#[derive(Clone, Debug)]
pub struct FunctionDeclaration
{
    pub identifier: Identifier,
    pub parameters: Vec<IdentifierSymbol>,
    pub positions: Vec<Position>,
    //Attenzione! non puo' essere uno Stmt altrimenti i parametri della funzione verrebbero definiti in uno scope esterno rispetto al body e l'utente potrebbe ridefinirli nel body!
    pub body: Vec<Stmt>,
    pub is_initializer: bool
}

impl FunctionDeclaration
{
    pub fn new(identifier: Identifier, parameters: Vec<Identifier>, body: Vec<Stmt>, is_initializer: bool) -> Self
    {
        Self
        {
            identifier,
            parameters: parameters.iter().map(|p| p.name).collect(),
            positions: parameters.iter().map(|p| p.position).collect(),
            body,
            is_initializer
        }
    }
}

#[derive(Clone, Debug)]
pub struct ClassDeclaration
{
    pub identifier: Identifier,
    pub methods: FxHashMap<IdentifierSymbol, Rc<FunctionDeclaration>>,
    pub superclass_expr: Option<Expr>
}

impl ClassDeclaration
{
    pub fn new(identifier: Identifier, superclass_expr: Option<Expr>) -> Self
    {
        Self {
            identifier,
            methods: FxHashMap::default(),
            superclass_expr
        }
    }
}