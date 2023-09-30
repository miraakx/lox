use std::{collections::HashMap, cell::RefCell, rc::Rc};

use crate::value::Value;

#[derive(Clone, Debug)]
pub struct Environment
{
    global_scope: Rc<RefCell<Scope>>,
    locals_scope: Vec<Rc<RefCell<Scope>>>,
    side_table: HashMap<i64, usize>
}

impl Environment
{
    #[inline]
    pub fn new() -> Self
    {
        Environment
        {
            global_scope: Rc::new(RefCell::new(Scope::new())),
            locals_scope: Vec::new(),
            side_table: HashMap::new()
        }
    }

    #[inline]
    pub fn from(environment: &Environment) -> Self
    {
        environment.clone()
    }

    #[inline]
    pub fn define_variable(&mut self, variable: String, var_value: Value)
    {
        let inner = self.locals_scope.last();
        match inner {
            Some(scope) => {
                scope.borrow_mut().define_variable(variable, var_value);
            },
            None => {
                self.global_scope.borrow_mut().define_variable(variable, var_value);
            },
        }
    }

    #[inline]
    pub fn lookup_variable(&self, name: &String, expr_id: i64) -> Value
    {
        let opt_index = self.side_table.get(&expr_id);
        if let Some(index) = opt_index {
            return self.get_variable_from_local_at(*index, name);
        } else {
            return self.get_variable_from_global(name);
        }
    }

    pub fn assign_variable(&mut self, variable: String, var_value: Value, expr_id: i64) -> Value
    {
        let opt_index = self.side_table.get(&expr_id);
        if let Some(index) = opt_index {
            return self.assign_variable_to_local_at(*index, variable, var_value);
        } else {
            return self.assign_variable_to_global(variable, var_value);
        }
    }

    #[inline]
    fn get_variable_from_local_at(&self, index: usize, name: &str) -> Value
    {
        return self.locals_scope[index].borrow().get_variable(name).unwrap();
    }

    #[inline]
    fn get_variable_from_global(&self, name: &str) -> Value
    {
        return self.global_scope.borrow().get_variable(name).unwrap();
    }

    #[inline]
    fn assign_variable_to_local_at(&mut self, index: usize, variable: String, var_value: Value) -> Value
    {
        return self.locals_scope[index].borrow_mut().assign_variable(variable, var_value);
    }

    #[inline]
    fn assign_variable_to_global(&mut self, variable: String, var_value: Value) -> Value
    {
        return self.global_scope.borrow_mut().assign_variable(variable, var_value);
    }

    #[inline]
    pub fn new_local_scope(&mut self)
    {
        self.locals_scope.push(Rc::new(RefCell::new(Scope::new())));
    }

    #[inline]
    pub fn remove_loval_scope(&mut self)
    {
        self.locals_scope.pop();
    }

    #[inline]
    pub fn insert_into_side_table(&mut self, expr_id: i64, depth: usize) {
        self.side_table.insert(expr_id, depth);
    }

}

#[derive(Clone, Debug)]
struct Scope {
    map: HashMap<String, Value>
}

impl Scope
{
    #[inline]
    pub fn new() -> Self
    {
        Scope { map: HashMap::new() }
    }

    #[inline]
    pub fn define_variable(&mut self, variable: String, var_value: Value)
    {
        self.map.insert(variable, var_value);
    }

    #[inline]
    pub fn get_variable(&self, variable: &str) -> Option<Value>
    {
        if let Some(value) = self.map.get(variable)
        {
            return match value
            {
                Value::String(rc_str) => Some(Value::String(rc_str.clone())),
                Value::Number(num) => Some(Value::Number(*num)),
                Value::Bool(boolean) => Some(Value::Bool(*boolean)),
                Value::Nil => Some(Value::Nil),
                Value::Callable(rc_function) => { Some(Value::Callable(rc_function.clone())) },
                Value::ClassInstance(rc_class) => { Some(Value::ClassInstance(rc_class.clone())) },
            };
        }
        return None;
    }

    #[inline]
    pub fn assign_variable(&mut self, variable: String, var_value: Value) -> Value
    {
        if self.map.contains_key(&variable)
        {
            self.map.insert(variable, var_value.clone());
            return var_value;
        }
        panic!("Variable not found");
    }

    #[inline]
    pub fn contains_variable(&self, variable: &String) -> bool
    {
        self.map.contains_key(variable)
    }

}
