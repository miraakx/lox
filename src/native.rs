use std::time::{SystemTime, UNIX_EPOCH};

use crate::{interpreter::{Value, Interpreter}, error::{LoxError, LoxErrorKind}, tokens::Position};

pub fn clock(_: &Interpreter, _: &[Value], position: Position) -> Result<Value, LoxError>
{
    let result = SystemTime::now().duration_since(UNIX_EPOCH);
    match result
    {
        Ok(value) => {
            Ok(Value::Number(value.as_secs_f64()))
        },
        Err(_) => {
            Err(LoxError::new(LoxErrorKind::NativeClockSysTimeError, position))
        },
    }
}