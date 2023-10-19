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
    pub fn new() -> Self
    {
        Environment
        {
            locals_scope: Vec::new()
        }
    }

    pub fn from(environment: &Environment) -> Self
    {
        environment.clone()
    }

    pub fn get_variable_from_local_at(&self, index: usize, name: Identifier) -> Option<Value>
    {
        return self.locals_scope[index].borrow().get_variable(name);
    }

    pub fn assign_variable_to_local_at(&mut self, index: usize, variable: Identifier, var_value: Value) -> Result<Value, ()>
    {
        return self.locals_scope[index].borrow_mut().assign_variable(variable, var_value);
    }

    pub fn new_local_scope(&mut self) -> Rc<RefCell<Scope>>
    {
        let rc_scope: Rc<RefCell<Scope>> = Rc::new(RefCell::new(Scope::new()));
        self.locals_scope.push(Rc::clone(&rc_scope));
        return rc_scope;
    }

    pub fn remove_loval_scope(&mut self)
    {
        self.locals_scope.pop();
    }

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

    pub fn new() -> Self
    {
        Scope { map: FxHashMap::default() }
    }


    pub fn define_variable(&mut self, variable: Identifier, var_value: Value)
    {
        self.map.insert(variable, var_value);
    }


    pub fn get_variable(&self, variable: Identifier) -> Option<Value>
    {
        match self.map.get(&variable) {
            Some(value) => Some(value.clone()),
            None => { None },
        }
    }


    pub fn assign_variable(&mut self, variable: Identifier, var_value: Value) -> Result<Value, ()>
    {
        if self.map.contains_key(&variable)
        {
            self.map.insert(variable, var_value.clone());
            return Ok(var_value);
        }
        Err(())
    }


    pub fn contains_variable(&self, variable: Identifier) -> bool
    {
        self.map.contains_key(&variable)
    }

}
