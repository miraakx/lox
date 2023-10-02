use std::{rc::Rc, collections::HashMap, cell::RefCell};

use string_interner::StringInterner;

use crate::{interpreter::Callable, parser_stmt::ClassDeclaration, alias::{Identifier, InternedString}};

#[derive(Clone, Debug)]
pub enum Value {
    InternedString(InternedString),
    String(Rc<String>),
    Number(f64),
    Bool(bool),
    Nil,
    Callable(Callable),
    ClassInstance(Rc<ClassDeclaration>, Rc<RefCell<HashMap<Identifier, Value>>>)
}

#[inline]
pub fn is_equal(val_left: Value, val_right: Value, string_interner: &RefCell<StringInterner>) -> bool
{
    match (val_left, val_right)
    {
        (Value::Bool(left),                Value::Bool(right))                 => left == right,
        (Value::Number(left),               Value::Number(right))                => left == right,
        (Value::String(left),        Value::String(right))         => Rc::ptr_eq(&left, &right) || *left == *right,
        (Value::Nil,                             Value::Nil)                               => true,
        (Value::InternedString(left), Value::InternedString(right))  => left == right,
        (Value::String(left),        Value::InternedString(right))  => left.as_str()  == string_interner.borrow().resolve(right).unwrap(),
        (Value::InternedString(left), Value::String(right), )       => right.as_str() == string_interner.borrow().resolve(left).unwrap(),
        _                                                                                  => false
    }
}

#[inline]
pub fn is_truthy(value: &Value) -> bool
{
    match value
    {
        Value::String(_)            => true,
        Value::Number(_)            => true,
        Value::Bool(boolean) => *boolean,
        Value::Nil                  => false,
        Value::Callable(_)          => true,
        Value::ClassInstance(_, _)  => true,
        Value::InternedString(_)    => true,
    }
}
