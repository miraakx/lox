use std::{rc::Rc, fmt::Display};

use crate::{interpreter::Callable, parser_stmt::ClassDeclaration};

#[derive(Clone, Debug)]
pub enum Value {
    String(Rc<String>), Number(f64), Bool(bool), Nil, Callable(Rc<Callable>), ClassInstance(Rc<ClassDeclaration>)
}

impl Display for Value
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Value::String(str) => { write!(f, "{}", str ) },
            Value::Number(num) => { write!(f, "{}", num ) },
            Value::Bool(bool) => { write!(f, "{}", bool) },
            Value::Nil => { write!(f, "nil") },
            Value::Callable(_) => { write!(f, "callable()") },
            Value::ClassInstance(declaration) => { write!(f, "{}", declaration.get_name()) },
        }
    }
}

#[inline]
pub fn is_equal(val_left: Value, val_right: Value) -> bool
{
    match (val_left, val_right)
    {
        (Value::Bool(left),         Value::Bool(right))         => left == right,
        (Value::Number(left),        Value::Number(right))        => left == right,
        (Value::String(left), Value::String(right)) => left == right,
        (Value::Nil,                      Value::Nil)                       => true,
        _                                                                   => false
    }
}

#[inline]
pub fn is_truthy(value: &Value) -> bool
{
    match value
    {
        Value::String(_)           => true,
        Value::Number(_)           => true,
        Value::Bool(boolean) => *boolean,
        Value::Nil                 => false,
        Value::Callable(_)         => true,
        Value::ClassInstance(_)    => true,
    }
}