use std::collections::HashMap;

use crate::{interpreter::Value, error::LoxError, tokens::Position};

pub struct Environment {
    env: Vec<HashMap<String, Value>>
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            env: vec!(HashMap::new())
        }
    }

    pub fn define(&mut self, variable: String, var_value: Value) {
        let last = self.env.len() - 1;
        self.env[last].insert(variable, var_value);
    }

    pub fn get(&self, variable: &str, position: Position) -> Result<Value, LoxError> {
        for block_env in self.env.iter().rev() {
            if let Some(value) = block_env.get(variable) {
                return match value {
                    Value::String(rc_str) => Ok(Value::String(rc_str.clone())),
                    Value::Number(num) => Ok(Value::Number(*num)),
                    Value::Bool(boolean) => Ok(Value::Bool(*boolean)),
                    Value::Nil => Ok(Value::Nil),
                };
            }
        }
        return Err(LoxError::new(crate::error::LoxErrorKind::UdefinedVariable(variable.to_owned()), position));

    }

    pub fn assign(&mut self, variable: String, var_value: Value, position: Position) -> Result<Value, LoxError> {
        for block_env in self.env.iter_mut().rev() {
            if block_env.contains_key(&variable) {
                block_env.insert(variable, var_value.clone());
                return Ok(var_value);
            }
        }
        return Err(LoxError::new(crate::error::LoxErrorKind::UdefinedVariable(variable.to_owned()), position));
    }

    pub fn new_scope(&mut self) {
        self.env.push(HashMap::new());
    }

    pub fn remove_scope(&mut self) {
        self.env.pop();
    }
}
