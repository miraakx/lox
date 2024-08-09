use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::InterpreterErrorKind;

use super::types::Value;

pub fn clock() -> Result<Value, InterpreterErrorKind>
{
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or_else(
        |_| Err(InterpreterErrorKind::NativeClockSysTimeError),
        |value| Ok(Value::Number(value.as_secs_f64()))
    )
}

pub fn assert_eq(actual: Value, expected: Value) -> Result<(), InterpreterErrorKind> {
    if actual == expected {
        Ok(())
    } else {
        Err(InterpreterErrorKind::AssertionFailure)
    }
}