use std::time::{SystemTime, UNIX_EPOCH};

use super::types::Value;

pub fn clock() -> Result<Value, ()>
{
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or_else(
        |_| Err(()),
        |value| Ok(Value::Number(value.as_secs_f64()))
    )
}

pub fn assert_eq(actual: Value, expected: Value) -> Result<(), ()> {
    if actual == expected {
        Ok(())
    } else {
        Err(())
    }
}