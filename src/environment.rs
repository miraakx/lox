use std::{collections::HashMap, cell::RefCell, rc::Rc};

use crate::{interpreter::Value, error::LoxError, tokens::Position};

#[derive(Clone, Debug)]
struct Scope {
    scope: HashMap<String, Value>
}

impl Scope
{
    #[inline]
    pub fn new() -> Self
    {
        Scope { scope: HashMap::new() }
    }

    #[inline]
    pub fn define(&mut self, variable: String, var_value: Value)
    {
        self.scope.insert(variable, var_value);
    }

    #[inline]
    pub fn get(&self, variable: &str) -> Option<Value>
    {
        if let Some(value) = self.scope.get(variable)
        {
            return match value
            {
                Value::String(rc_str) => Some(Value::String(rc_str.clone())),
                Value::Number(num) => Some(Value::Number(*num)),
                Value::Bool(boolean) => Some(Value::Bool(*boolean)),
                Value::Nil => Some(Value::Nil),
                Value::Callable(rc_function) => { Some(Value::Callable(rc_function.clone())) },
            };
        }
        return None;
    }

    #[inline]
    pub fn assign(&mut self, variable: String, var_value: Value, position: Position) -> Result<Value, LoxError>
    {
        if self.scope.contains_key(&variable)
        {
            self.scope.insert(variable, var_value.clone());
            return Ok(var_value);
        }
        return Err(LoxError::new(crate::error::LoxErrorKind::UdefinedVariable(variable.to_owned()), position));
    }

    #[inline]
    pub fn contains(&self, variable: &String) -> bool
    {
        self.scope.contains_key(variable)
    }
}

#[derive(Clone, Debug)]
pub struct Environment
{
    scopes: Vec<Rc<RefCell<Scope>>>
}

impl Environment
{
    pub fn new() -> Self
    {
        let mut vec = Vec::new();
        vec.push(Rc::new(RefCell::new(Scope::new())));
        Environment { scopes: vec }
    }

    pub fn from(environment: &Environment) -> Self
    {
        let mut vec = Vec::with_capacity(environment.scopes.len());
        for scope in &environment.scopes {
            vec.push(scope.clone());
        }
        Environment { scopes: vec }
    }

    pub fn define(&mut self, variable: String, var_value: Value)
    {
        let last = self.scopes.len() - 1;
        self.scopes[last].borrow_mut().define(variable, var_value);
    }

    pub fn get(&self, variable: &str, position: Position) -> Result<Value, LoxError>
    {
        for scope in self.scopes.iter().rev()
        {
            if let Some(value) = scope.borrow().get(variable)
            {
                return Ok(value);
            }
        }
        return Err(LoxError::new(crate::error::LoxErrorKind::UdefinedVariable(variable.to_owned()), position));
    }

    pub fn assign(&mut self, variable: String, var_value: Value, position: Position) -> Result<Value, LoxError>
    {
        for scope in self.scopes.iter_mut().rev()
        {
            if scope.borrow().contains(&variable)
            {
                scope.borrow_mut().define(variable, var_value.clone());
                return Ok(var_value);
            }
        }
        return Err(LoxError::new(crate::error::LoxErrorKind::UdefinedVariable(variable.to_owned()), position));
    }

    pub fn new_scope(&mut self)
    {
        self.scopes.push(Rc::new(RefCell::new(Scope::new())));
    }

    pub fn remove_scope(&mut self)
    {
        self.scopes.pop();
    }

}
