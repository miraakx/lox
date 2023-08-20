use std::time::{SystemTime, UNIX_EPOCH};

use crate::{interpreter::{Value, Interpreter}, error::LoxError};

pub fn clock(_: &Interpreter, _: &[Value]) -> Result<Value, LoxError> {
    let since_the_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards!");
    return Ok(Value::Number(since_the_epoch.as_secs_f64()));
}