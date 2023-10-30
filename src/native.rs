use std::time::{SystemTime, UNIX_EPOCH};

use crate::{error::InterpreterErrorKind, value::Value};

pub fn clock() -> Result<Value, InterpreterErrorKind>
{
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or_else(
     |_                 | Err(InterpreterErrorKind::NativeClockSysTimeError),
           |value   | Ok(Value::Number(value.as_secs_f64()))
        )
}