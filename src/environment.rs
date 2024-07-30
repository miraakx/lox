use std::{cell::RefCell, rc::Rc};

use rustc_hash::FxHashMap;

use crate::{value::Value, alias::IdentifierSymbol};

#[derive(Clone, Debug)]
pub struct Environment
{
    locals_scope: Vec<Rc<RefCell<Scope>>>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment
{
    pub const fn new() -> Self
    {
        Self {
            locals_scope: Vec::new()
        }
    }

    #[inline]
    pub fn get_variable_from_local_at(&self, index: usize, name: IdentifierSymbol) -> Option<Value>
    {
        self.locals_scope[index].borrow().get_variable(name)
    }

    #[inline]
    pub fn assign_variable_to_local_at(&mut self, index: usize, variable: IdentifierSymbol, var_value: &Value) -> Result<(), ()>
    {
        self.locals_scope[index].borrow_mut().assign_variable(variable, var_value)
    }

    #[inline]
    pub fn new_local_scope(&mut self) -> Rc<RefCell<Scope>>
    {
        let rc_scope = Rc::new(RefCell::new(Scope::new()));
        self.locals_scope.push(Rc::clone(&rc_scope));
        rc_scope
    }

    #[inline]
    pub fn remove_local_scope(&mut self)
    {
        self.locals_scope.pop();
    }

    #[inline]
    pub fn last_scope(&self) -> Option<&Rc<RefCell<Scope>>>
    {
        self.locals_scope.last()
    }

}

#[derive(Clone, Debug)]
pub struct Scope {
    map: FxHashMap<IdentifierSymbol, Value>
}

impl Default for Scope
{
    fn default() -> Self {
        Self::new()
    }
}

impl Scope
{
    #[inline]
    pub fn new() -> Self
    {
        Self { map: FxHashMap::default() }
    }

    #[inline]
    pub fn define_variable(&mut self, variable: IdentifierSymbol, var_value: Value)
    {
        //println!("VARIABLE {:?} = {:?}", variable, var_value);
        self.map.insert(variable, var_value);
    }

    /*#[inline]
    pub fn define_variables(&mut self, variables: &[IdentifierSymbol], mut var_values: Vec<Value>)
    {
        for variable in variables.iter().rev() {
            self.map.insert(*variable, var_values.pop().unwrap());
        }
    }*/

    #[inline]
    pub fn get_variable(&self, variable: IdentifierSymbol) -> Option<Value>
    {
        for (name, value) in &self.map {
            if *name == variable {
                return Some(value.clone());
            }
        }
        None
    }

    #[inline]
    pub fn assign_variable(&mut self, variable: IdentifierSymbol, var_value: &Value) -> Result<(), ()>
    {
        if self.map.contains_key(&variable) {
            self.map.insert(variable, var_value.clone());
            return Ok(());
        }
        Err(())
    }

}

/*
#[derive(Clone, Debug)]
pub struct Scope {
    map: Vec<(IdentifierSymbol, Value)>
}

impl Scope
{
    pub fn new() -> Self
    {
        Scope {
            map: Vec::<(IdentifierSymbol, Value)>::new()
        }
    }

    pub fn define_variable(&mut self, variable: IdentifierSymbol, var_value: Value)
    {
        self.map.push((variable, var_value));
    }

    pub fn get_variable(&self, variable: IdentifierSymbol) -> Option<Value>
    {
        for (name, value) in &self.map {
            if *name == variable {
                return Some(value.clone());
            }
        }
        return None;
    }

    pub fn assign_variable(&mut self, variable: IdentifierSymbol, var_value: Value) -> Result<Value, ()>
    {

        for (index, value) in self.map.iter().enumerate() {
            if value.0 == variable {
                let clone = var_value.clone();
                self.map[index] = (variable, clone);
                return Ok(var_value);

            }
        }
        return Err(());
    }
}
 */