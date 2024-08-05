use std::time::{SystemTime, UNIX_EPOCH};

use string_interner::StringInterner;

use crate::{error::InterpreterErrorKind, value::Value, interpreter::Callable};

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

pub fn to_string(value: Value, string_interner: &StringInterner) -> String {
    match value {
        Value::String(string)       => format!("{}", string),
        Value::Number(number)       => format!("{}", number),
        Value::Bool(boolean)        => format!("{}", boolean),
        Value::Nil                  => format!("nil"),
        Value::Callable(callable)   => {
            match callable {
                Callable::Function(fun_decl)    => format!("<fn {}>", string_interner.resolve(fun_decl.borrow().declaration.identifier.name).unwrap()),
                Callable::Class(class_decl, _)  => format!("{}", string_interner.resolve(class_decl.identifier.name).unwrap()),
                Callable::Clock                 => format!("<native fn>"),
                Callable::AssertEq              => format!("<native fn>"),
                Callable::Str                   => format!("<native fn>"),
            }
        },
        Value::ClassInstance(class_instance) => {
            format!("{} instance", string_interner.resolve(class_instance.declaration.identifier.name).unwrap())
        }
    }
}