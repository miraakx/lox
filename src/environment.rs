use std::collections::HashMap;

use crate::{interpreter::Value, error::LoxError, tokens::Position};

pub struct Environment {
    env: HashMap<String, Value>
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            env: HashMap::new()
        }
    }

    pub fn define(&mut self, variable: String, var_value: Value) {
        self.env.insert(variable, var_value);
    }

    pub fn get(&self, variable: &str, position: Position) -> Result<Value, LoxError> {
        if let Some(value) = self.env.get(variable) {
            match value {
                Value::String(rc_str) => Ok(Value::String(rc_str.clone())),
                Value::Number(num) => Ok(Value::Number(*num)),
                Value::Bool(boolean) => Ok(Value::Bool(*boolean)),
                Value::Nil => Ok(Value::Nil),
            }
        } else {
            Err(LoxError::new(crate::error::LoxErrorKind::UdefinedVariable(variable.to_owned()), position))
        }
    }
}
