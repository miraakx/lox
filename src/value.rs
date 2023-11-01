use std::{rc::Rc, cell::RefCell};

use rustc_hash::FxHashMap;

use crate::{interpreter::Callable, parser_stmt::ClassDeclaration, alias::IdentifierSymbol, tokens::{Token, TokenKind}};

#[derive(Clone, Debug)]
pub enum Value {
    String(Rc<String>),
    Number(f64),
    Bool(bool),
    Nil,
    Callable(Box<Callable>),
    ClassInstance(Box<ClassInstance>)
}

impl Value
{
    pub fn from_token(token: Token) -> Self
    {
        match token.kind
        {
            TokenKind::Nil                  => Value::Nil,
            TokenKind::False(value)  => value,
            TokenKind::True(value)   => value,
            TokenKind::Number(value) => value,
            TokenKind::String(value) => value,
            _ => {
                panic!("Internal error: unexpecter operator type.");
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ClassInstance {
    pub declaration: Rc<ClassDeclaration>,
    pub attributes: Rc<RefCell<FxHashMap<IdentifierSymbol, Value>>>
}

#[inline]
pub fn is_equal(val_left: Value, val_right: Value) -> bool
{
    match (val_left, val_right)
    {
        (Value::Bool(left),         Value::Bool(right))         => left == right,
        (Value::Number(left),        Value::Number(right))        => left == right,
        (Value::String(left), Value::String(right)) => { Rc::ptr_eq(&left, &right) || *left == *right  }
        (Value::Nil,                      Value::Nil)                       => true,
        _                                                                   => false
    }
}

#[inline]
pub const fn is_truthy(value: &Value) -> bool
{
    match value
    {
        Value::String(_)            => true,
        Value::Number(_)            => true,
        Value::Bool(boolean) => *boolean,
        Value::Nil                  => false,
        Value::Callable(_)          => true,
        Value::ClassInstance(_)     => true,
    }
}