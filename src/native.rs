use std::time::{SystemTime, UNIX_EPOCH};

use crate::{error::InterpreterErrorKind, value::Value};

pub fn clock() -> Result<Value, InterpreterErrorKind>
{
    let result = SystemTime::now().duration_since(UNIX_EPOCH);
    match result
    {
        Ok(value) => {
            Ok(Value::Number(value.as_secs_f64()))
        },
        Err(_) => {
            Err(InterpreterErrorKind::NativeClockSysTimeError)
        },
    }
}