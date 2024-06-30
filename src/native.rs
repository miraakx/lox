use std::time::{SystemTime, UNIX_EPOCH};

use string_interner::StringInterner;

use crate::{error::InterpreterErrorKind, value::Value, interpreter::Callable};

pub fn clock() -> Result<Value, InterpreterErrorKind>
{
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or_else(
     |_                 | Err(InterpreterErrorKind::NativeClockSysTimeError),
           |value   | Ok(Value::Number(value.as_secs_f64()))
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
        Value::String(string)   => format!("{}", string),
        Value::Number(number)          => format!("{}", number),
        Value::Bool(boolean)          => format!("{}", boolean),
        Value::Nil                          => format!("nil"),
        Value::Callable(callable) => {
            match callable {
                Callable::Function(fun_decl, _)     => format!("<fn: '{}'>",        string_interner.resolve(fun_decl.identifier.name).unwrap()),
                Callable::InitFunction(fun_decl, _) => format!("<fn: '{}'>",        string_interner.resolve(fun_decl.identifier.name).unwrap()),
                Callable::Class(class_decl, _)         => format!("{}",     string_interner.resolve(class_decl.identifier.name).unwrap()),
                Callable::Clock                                              => format!("<fn (native): 'clock'>"),
                Callable::AssertEq                                           => format!("<fn (native): 'assertEq'>"),
                Callable::Str                                                => format!("<fn (native): 'str'>"),
            }
        },
        Value::ClassInstance(class_instance) => {
            format!("<inst: '{}'", string_interner.resolve(class_instance.declaration.identifier.name).unwrap())
        }
    }
}