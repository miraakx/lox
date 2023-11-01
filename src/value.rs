use std::{rc::Rc, cell::RefCell};

use rustc_hash::FxHashMap;

use crate::{interpreter::Callable, parser_stmt::ClassDeclaration, alias::IdentifierSymbol, tokens::{Token, TokenKind}};

#[derive(Clone, Debug)]
pub enum Value {
    String(Rc<String>),
    Number(f64),
    Bool(bool),
    Nil,
    Callable(Callable),
    ClassInstance(ClassInstance)
}

impl PartialEq for Value
{
    #[inline]
    fn eq(&self, other: &Self) -> bool
    {
        match (self, other)
        {
            (Value::Bool(left),         Value::Bool(right))         => left == right,
            (Value::Number(left),        Value::Number(right))        => left == right,
            (Value::String(left), Value::String(right)) => { Rc::ptr_eq(&left, &right) || *left == *right  }
            (Value::Nil,                       Value::Nil)                        => true,
            (Value::Callable(_), Value::Callable(_))                              => todo!(),
            (Value::ClassInstance(_), Value::ClassInstance(_))                    => todo!(),
            _                                                                     => false
        }
    }
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

    #[inline]
    pub fn is_truthy(&self) -> bool
    {
        match self
        {
            Value::String(_)            => true,
            Value::Number(_)            => true,
            Value::Bool(boolean) => *boolean,
            Value::Nil                  => false,
            Value::Callable(_)          => true,
            Value::ClassInstance(_)     => true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ClassInstance {
    pub declaration: Rc<ClassDeclaration>,
    pub attributes: Rc<RefCell<FxHashMap<IdentifierSymbol, Value>>>
}
