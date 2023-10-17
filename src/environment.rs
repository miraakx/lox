use std::{cell::RefCell, rc::Rc};

use rustc_hash::FxHashMap;

use crate::{value::Value, alias::Identifier};

#[derive(Clone, Debug)]
pub struct Environment
{
    locals_scope: Vec<Rc<RefCell<Scope>>>,
}

impl Environment
{
    #[inline(always)]
    pub fn new() -> Self
    {
        Environment
        {
            locals_scope: Vec::new()
        }
    }

    #[inline(always)]
    pub fn from(environment: &Environment) -> Self
    {
        environment.clone()
    }

    #[inline(always)]
    pub fn get_variable_from_local_at(&self, index: usize, name: Identifier) -> Option<Value>
    {
        return self.locals_scope[index].borrow().get_variable(name);
    }

    #[inline(always)]
    pub fn assign_variable_to_local_at(&mut self, index: usize, variable: Identifier, var_value: Value) -> Result<Value, ()>
    {
        return self.locals_scope[index].borrow_mut().assign_variable(variable, var_value);
    }

    #[inline(always)]
    pub fn new_local_scope(&mut self) -> Rc<RefCell<Scope>>
    {
        let rc_scope: Rc<RefCell<Scope>> = Rc::new(RefCell::new(Scope::new()));
        self.locals_scope.push(rc_scope.clone());
        return rc_scope;
    }

    #[inline(always)]
    pub fn remove_loval_scope(&mut self)
    {
        self.locals_scope.pop();
    }

    #[inline(always)]
    pub fn last_scope(&self) -> Option<&Rc<RefCell<Scope>>>
    {
        self.locals_scope.last()
    }

}

#[derive(Clone, Debug)]
pub struct Scope {
    map: FxHashMap<Identifier, Value>
}

impl Scope
{
    #[inline(always)]
    pub fn new() -> Self
    {
        Scope { map: FxHashMap::default() }
    }

    #[inline(always)]
    pub fn define_variable(&mut self, variable: Identifier, var_value: Value)
    {
        self.map.insert(variable, var_value);
    }

    #[inline(always)]
    pub fn get_variable(&self, variable: Identifier) -> Option<Value>
    {
        match self.map.get(&variable) {
            Some(value) => Some(value.clone()),
            None => { None },
        }
    }

    #[inline(always)]
    pub fn assign_variable(&mut self, variable: Identifier, var_value: Value) -> Result<Value, ()>
    {
        if self.map.contains_key(&variable)
        {
            self.map.insert(variable, var_value.clone());
            return Ok(var_value);
        }
        Err(())
    }

    #[inline(always)]
    pub fn contains_variable(&self, variable: Identifier) -> bool
    {
        self.map.contains_key(&variable)
    }

}
