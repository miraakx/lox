use std::{fmt::{Display, Debug}, rc::Rc, cell::RefCell};

use crate::{parser_stmt::{Stmt, FunctionDeclaration}, tokens::{TokenKind, LiteralValue, Position}, environment::Environment, error::{LoxError, LoxErrorKind}, parser_expr::{Expr, ExprKind}, native::clock};

#[derive(Clone, Debug)]
pub enum Value {
    String(Rc<String>), Number(f64), Bool(bool), Nil, Callable(Rc<Function>)
}

#[derive(Clone, Debug)]
pub enum Function {
    Lox(Rc<FunctionDeclaration>, Rc<RefCell<Environment>>),
    Clock
}

impl Function
{
    fn arity(&self) -> u32 {
        match self {
            Function::Lox(declaration, _) => {
                declaration.parameters.len() as u32
            },
            Function::Clock => 0,
        }
    }

    fn call(&self,  interpreter: &Interpreter, args: &[Value], position: Position) -> Result<Value, LoxError> {
        match self
        {
            Function::Lox(declaration, environment) =>
            {
                let mut env = Environment::from(&environment.borrow());
                for (index, param) in declaration.parameters.iter().enumerate()
                {
                    env.define_variable(param.get_identifier(), args.get(index).unwrap().clone());
                }
                let mut local_interpreter = Interpreter::from(Rc::new(RefCell::new(env)));
                let state = local_interpreter.execute_stmt(&declaration.body)?;
                match state {
                    State::Return(value) => Ok(value),
                    _                           => Ok(Value::Nil)
                }
            },
            Function::Clock =>
            {
                return clock(interpreter, args, position);
            },
        }
    }
}


impl Display for Value
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Value::String(str) => { write!(f, "{}", str ) },
            Value::Number(num) => { write!(f, "{}", num ) },
            Value::Bool(bool) => { write!(f, "{}", bool) },
            Value::Nil => { write!(f, "nil") },
            Value::Callable(_) => { write!(f, "callable()") },
        }
    }
}

pub enum State {
    Normal,
    Break,
    Continue,
    Return(Value)
}

pub struct Interpreter
{
    env: Rc<RefCell<Environment>>
}

impl Interpreter
{
    #[inline]
    pub fn new() -> Self
    {
        let mut env = Environment::new();
        env.define_variable("clock".to_owned(), Value::Callable(Rc::new(Function::Clock)));
        Interpreter {
            env: Rc::new(RefCell::new(env))
        }
    }

    #[inline]
    pub fn from(environment: Rc<RefCell<Environment>>) -> Self
    {
        Interpreter {
            env: environment
        }
    }

    #[inline]
    pub fn resolve(&mut self, expr_id: i64, depth: usize) {
        self.env.borrow_mut().insert_into_side_table(expr_id, depth);
    }

    #[inline]
    pub fn execute(&mut self, stmts: &[Stmt]) -> Result<(), ()>
    {
        for stmt in stmts
        {
            let result = self.execute_stmt(stmt);
            match result
            {
                Ok(_) => {}
                Err(err) => {
                    println!("{}", err);
                    return Err(());
                },
            }
        }
        Ok(())
    }

    #[inline]
    pub fn execute_stmt(&mut self, stmt: &Stmt) -> Result<State, LoxError>
    {
        match stmt
        {
            Stmt::Print(expr) =>
            {
                let value = self.evaluate(expr)?;
                println!("{}", value);
                return Ok(State::Normal);
            },
            Stmt::ExprStmt(expr) =>
            {
                let _ = self.evaluate(expr)?;
                return Ok(State::Normal);
            }
            Stmt::Var(variable, _, opt_expr) =>
            {
                match opt_expr
                {
                    Some(expr) =>
                    {
                        let value = self.evaluate(expr)?;
                        self.env.borrow_mut().define_variable(variable.to_owned(), value);
                    },
                    None =>
                    {
                        self.env.borrow_mut().define_variable(variable.to_owned(), Value::Nil);
                    },
                }
                return Ok(State::Normal);
            }
            Stmt::Block(statements) =>
            {
                self.env.borrow_mut().new_local_scope();
                for stmt in statements
                {
                    let state = self.execute_stmt(stmt)?;
                    match state {
                        State::Normal => {
                            continue;
                        },
                        State::Break => {
                            return Ok(State::Break);
                        },
                        State::Continue => {
                            return Ok(State::Continue);
                        },
                        State::Return(_) => return Ok(state),
                    };
                }
                self.env.borrow_mut().remove_loval_scope();
                return Ok(State::Normal);
            },
            Stmt::If(condition, then_stmt) =>
            {
                let condition_value = self.evaluate(condition)?;
                if is_truthy(&condition_value) {
                    return self.execute_stmt(then_stmt);
                } else {
                    return Ok(State::Normal);
                }
            },
            Stmt::IfElse(condition, then_stmt, else_stmt) =>
            {
                let condition_value = self.evaluate(condition)?;
                if is_truthy(&condition_value) {
                    return self.execute_stmt(then_stmt);
                } else {
                    return self.execute_stmt(else_stmt);
                }
            },
            Stmt::While(condition, body) =>
            {
                while is_truthy(&self.evaluate(condition)?)
                {
                    let state = self.execute_stmt(body.as_ref())?;
                    match state
                    {
                        State::Normal  | State::Continue =>
                        {
                            continue;
                        },
                        State::Break =>
                        {
                            break;
                        },
                        State::Return(_) => return Ok(state),
                    }
                }
                return Ok(State::Normal);
            },
            Stmt::Break => {
                return Ok(State::Break);
            },
            Stmt::Continue => {
                return Ok(State::Continue);
            },
            Stmt::For(opt_initializer, opt_condition, opt_increment, body) =>
            {
                self.env.borrow_mut().new_local_scope();

                if let Some(initializer) = opt_initializer.as_ref() {
                    self.execute_stmt(initializer)?;
                }

                while is_truthy(&self.evaluate_or(opt_condition, Value::Bool(true))?)
                {
                    let state = self.execute_stmt(body)?;
                    match state
                    {
                        State::Normal | State::Continue =>
                        {
                            self.evaluate_or(opt_increment, Value::Bool(true))?;
                            continue;
                        },
                        State::Break =>
                        {
                            break;
                        },
                        State::Return(_) => return Ok(state),
                    }
                }

                self.env.borrow_mut().remove_loval_scope();

                return Ok(State::Normal);
            },
            Stmt::Function(declaration) => {
                let function = Function::Lox(declaration.clone(), self.env.clone());
                self.env
                    .as_ref()
                    .borrow_mut()
                    .define_variable(
                        declaration.name.get_identifier(),
                        Value::Callable(Rc::new(function))
                    );
                return Ok(State::Normal);
            },
            Stmt::Return(_, opt_expr) => {
                let value = if let Some(expr) = opt_expr {
                    self.evaluate(expr)?
                } else {
                    Value::Nil
                };
                return Ok(State::Return(value));
            },
        }
    }

    #[inline]
    fn evaluate_or(&mut self, opt_expr: &Option<Expr>, or_value: Value) ->  Result<Value, LoxError> {
        match opt_expr {
            Some(expr) => {
                return self.evaluate(expr);
            },
            None => {
                return Ok(or_value);
            },
        };
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, LoxError>
    {
        match &expr.kind {
            ExprKind::Literal( token) =>
            {
                if let Some(value) = &token.value {
                    match value {
                        LiteralValue::String(val) =>
                        {
                            return Ok(Value::String(val.clone()));
                        }
                        LiteralValue::Number(val) =>
                        {
                            return Ok(Value::Number(*val));
                        },
                        LiteralValue::Bool(val) =>
                        {
                            return Ok(Value::Bool(*val));
                        },
                        LiteralValue::Nil =>
                        {
                            return Ok(Value::Nil);
                        },
                        LiteralValue::Identifier(_) =>
                        {
                            panic!("unexpected state");
                        },
                    }
                } else {
                    panic!("unsupported token!");
                }
            },
            ExprKind::Unary(token, right) =>
            {
                let val_right: Value = self.evaluate(right.as_ref())?;
                match token.kind
                {
                    TokenKind::Minus =>
                    {
                        match val_right
                        {
                            Value::Number(num) =>
                            {
                                return Ok(Value::Number(-num));
                            },
                            _ => {
                                panic!("unsupported unary expression!");
                            }
                        }
                    },
                    TokenKind::Bang =>
                    {
                        Ok(Value::Bool(!is_truthy(&val_right)))
                    },
                    _ => {
                        panic!("invalid token type for unary expression!");
                    }
                }
            },
            ExprKind::Grouping(expr) =>
            {
                self.evaluate(expr.as_ref())
            },
            ExprKind::Binary(left, token, right) =>
            {
                let val_left:  Value = self.evaluate(left.as_ref())?;
                let val_right: Value = self.evaluate(right.as_ref())?;
                match token.kind {
                    TokenKind::Minus =>
                    {
                        match (val_left, val_right)
                        {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                return Ok(Value::Number(num_left - num_right));
                            },
                            _ => {
                                panic!("both expressions side are not of the same type!");
                            }
                        }
                    },
                    TokenKind::Plus =>
                    {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                return Ok(Value::Number(num_left + num_right));
                            },
                            (Value::String(str_left), Value::String(str_right)) => {
                                let mut result = (*str_left).clone();
                                result.push_str(&str_right);
                                return Ok(Value::String(Rc::new(result)));
                            },
                            _ => {
                                panic!("both expressions side are not of the same type!");
                            }
                        }
                    },
                    TokenKind::Slash =>
                    {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                return Ok(Value::Number(num_left / num_right));
                            },
                            _ => {
                                panic!("both expressions side are not of the same type!");
                            }
                        }
                    },
                    TokenKind::Star =>
                    {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                return Ok(Value::Number(num_left * num_right));
                            },
                            _ => {
                                panic!("both expressions side are not of the same type!");
                            }
                        }
                    },
                    TokenKind::Greater =>
                    {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                return Ok(Value::Bool(num_left > num_right));
                            },
                            _ => {
                                panic!("both expressions side are not of the same type!");
                            }
                        }
                    },
                    TokenKind::GreaterEqual =>
                    {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                return Ok(Value::Bool(num_left >= num_right));
                            },
                            _ => {
                                panic!("both expressions side are not of the same type!");
                            }
                        }
                    },
                    TokenKind::Less => {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                return Ok(Value::Bool(num_left < num_right));
                            },
                            _ => {
                                panic!("both expressions side are not of the same type!");
                            }
                        }
                    },
                    TokenKind::LessEqual =>
                    {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                return Ok(Value::Bool(num_left <= num_right));
                            },
                            _ => {
                                panic!("both expressions side are not of the same type!");
                            }
                        }
                    },
                    TokenKind::EqualEqual =>
                    {
                        return Ok(Value::Bool(is_equal(val_left, val_right)));
                    },
                    TokenKind::BangEqual =>
                    {
                        return Ok(Value::Bool(!is_equal(val_left, val_right)));
                    },
                    _ => {
                        panic!("invalid token type for binary expression between numbers!");
                    }
                }
            }
            ExprKind::Variable(name, position) =>
            {
                return self.env.borrow().lookup_variable(name, expr.id).ok_or(LoxError::new(LoxErrorKind::InternalErrorVariableNotFoundWhereExpected(name.to_owned()), *position));
            },
            ExprKind::Assign(name, expr, pos) =>
            {
                let value: Value = self.evaluate(expr.as_ref())?;
                return self.env
                        .borrow_mut()
                        .assign_variable(name.to_owned(), value, expr.id)
                        .map_err(|_| LoxError::new(LoxErrorKind::InternalErrorVariableNotFoundWhereExpected(name.to_owned()), *pos));
            },
            ExprKind::Logical(left, token, right) => {
                let val_left:  Value = self.evaluate(left.as_ref())?;
                match token.kind {
                    TokenKind::Or => {
                        if is_truthy(&val_left) {
                            return Ok(val_left);
                        } else {
                            return self.evaluate(right.as_ref());
                        }
                    },
                    TokenKind::And => {
                        if !is_truthy(&val_left) {
                            return Ok(val_left);
                        } else {
                            return self.evaluate(right.as_ref());
                        }
                    },
                    _ => {
                        panic!("invalid operator type for logical expression!");
                    }
                }
            },
            ExprKind::Call(callee_expr, opt_args_expr, token) => {
                let callee = self.evaluate(callee_expr)?;
                let mut args: Vec<Value>;
                if let Some(args_expr) = opt_args_expr {
                    args = Vec::with_capacity(args_expr.len());
                    for arg_expr in args_expr {
                        args.push(self.evaluate(arg_expr)?);
                    }
                } else {
                    args = vec!();
                }
                match callee {
                    Value::Callable(function) => {
                        if function.arity() != args.len() as u32 {
                            return Err(LoxError { kind: LoxErrorKind::WrongArity(function.arity(), args.len() as u32), position: token.position })
                        }
                        return function.call(self, &args, token.position);
                    },
                    _ => {
                        return Err(LoxError { kind: LoxErrorKind::NotCallable, position: token.position })
                    }
                }
            },
        }
    }

}



#[inline]
fn is_equal(val_left: Value, val_right: Value) -> bool
{
    match (val_left, val_right)
    {
        (Value::Bool(left),         Value::Bool(right))         => left == right,
        (Value::Number(left),        Value::Number(right))        => left == right,
        (Value::String(left), Value::String(right)) => left == right,
        (Value::Nil,                      Value::Nil)                       => true,
        _                                                                   => false
    }
}

#[inline]
fn is_truthy(value: &Value) -> bool
{
    match value
    {
        Value::String(_)           => true,
        Value::Number(_)           => true,
        Value::Bool(boolean) => *boolean,
        Value::Nil                 => false,
        Value::Callable(_)         => true,
    }
}
