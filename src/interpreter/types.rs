use std::{rc::Rc, cell::RefCell};

use rustc_hash::FxHashMap;

use crate::{alias::IdentifierSymbol, parser::types::{FunctionDeclaration, Identifier}};

use super::{environment::Environment, interpreter::Callable};

#[derive(Clone, Debug)]
pub struct LoxFunction
{
    pub declaration: Rc<FunctionDeclaration>,
    pub closure: Rc<RefCell<Environment>>
}

#[derive(Clone, Debug)]
pub struct LoxClass
{
    pub identifier: Identifier,
    pub methods: FxHashMap<IdentifierSymbol, LoxFunction>,
    pub super_class: Option<Rc<LoxClass>>
}

impl LoxClass
{
    pub fn new(
        identifier:     Identifier,
        methods:        FxHashMap<IdentifierSymbol, LoxFunction>,
        super_class:    Option<Rc<LoxClass>>
    ) -> Self
    {
        Self {
            identifier,
            methods,
            super_class
        }
    }

    pub fn find_method(&self, name: &IdentifierSymbol)  -> Option<&LoxFunction>
    {
        let method: Option<&LoxFunction> = self.methods.get(name);
        if method.is_some() {
            return method;
        }

        if let Some(super_class) = &self.super_class {
            return super_class.find_method(name);
        }

        None
    }
}

#[derive(Clone, Debug)]
pub struct LoxInstance
{
    pub declaration: Rc<LoxClass>,
    pub attributes: Rc<RefCell<FxHashMap<IdentifierSymbol, Value>>>
}


#[derive(Clone, Debug)]
pub enum Value
{
    String(Rc<String>),
    Number(f64),
    Bool(bool),
    Nil,
    Callable(Callable),
    ClassInstance(Rc<LoxInstance>)
}

impl PartialEq for Value
{
    #[inline]
    fn eq(&self, other: &Self) -> bool
    {
        match (self, other)
        {
            (Value::Bool(left),             Value::Bool(right))             => left == right,
            (Value::Number(left),           Value::Number(right))           => left == right,
            (Value::String(left),           Value::String(right))           => { Rc::ptr_eq(left, right) || *left == *right }
            (Value::Nil,                    Value::Nil)                     => true,
            (Value::ClassInstance(left),    Value::ClassInstance(right))    => { Rc::ptr_eq(&left.declaration, &right.declaration) },
            (Value::Callable(left),         Value::Callable(right)) => {
                match (left, right) {
                    (Callable::Function(l), Callable::Function(r))  => { Rc::ptr_eq(l, r) },
                    (Callable::Class(l),    Callable::Class(r))     => { Rc::ptr_eq(l, r) },
                    (Callable::Clock,       Callable::Clock)        => { true },
                    (Callable::AssertEq,    Callable::AssertEq)     => { true },
                    (Callable::Str,         Callable::Str)          => { true },
                    _ => false
                }
            },
            _   => false
        }
    }
}

impl Value
{
    #[inline]
    pub fn is_truthy(&self) -> bool
    {
        match self
        {
            Value::String(_)        => true,
            Value::Number(_)        => true,
            Value::Bool(boolean)    => *boolean,
            Value::Nil              => false,
            Value::Callable(_)      => true,
            Value::ClassInstance(_) => true,
        }
    }
}
