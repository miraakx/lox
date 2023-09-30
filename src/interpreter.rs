use std::{fmt::Debug, rc::Rc, cell::RefCell};

use crate::{parser_stmt::{Stmt, FunctionDeclaration, ClassDeclaration}, tokens::{TokenKind, LiteralValue, Position}, environment::Environment, error::{LoxError, InterpreterErrorKind}, parser_expr::{Expr, ExprKind}, native::clock, value::{Value, is_truthy, is_equal}};

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
        env.define_variable("clock".to_owned(), Value::Callable(Rc::new(Callable::Clock)));
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
    fn execute_stmt(&mut self, stmt: &Stmt) -> Result<State, LoxError>
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
            Stmt::FunctionDeclaration(declaration) => {
                let instance = Callable::Function(declaration.clone(), self.env.clone());
                self.env
                    .as_ref()
                    .borrow_mut()
                    .define_variable(
                        declaration.name.get_identifier(),
                        Value::Callable(Rc::new(instance))
                    );
                return Ok(State::Normal);
            },
            Stmt::ClassDeclaration(declaration) => {
                let instance = Callable::Class(declaration.clone(), self.env.clone());
                self.env
                    .as_ref()
                    .borrow_mut()
                    .define_variable(
                        declaration.name.get_identifier(),
                        Value::Callable(Rc::new(instance))
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
                                return Err(LoxError::interpreter_error(InterpreterErrorKind::InvalidUnaryType, token.position));
                            }
                        }
                    },
                    TokenKind::Bang =>
                    {
                        Ok(Value::Bool(!is_truthy(&val_right)))
                    },
                    _ => {
                        return Err(LoxError::interpreter_error(InterpreterErrorKind::InvalidUnaryType, token.position));
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
                                return Err(LoxError::interpreter_error(InterpreterErrorKind::IncompatibleBinaryOpTypes, token.position));
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
                                return Err(LoxError::interpreter_error(InterpreterErrorKind::IncompatibleBinaryOpTypes, token.position));
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
                                return Err(LoxError::interpreter_error(InterpreterErrorKind::IncompatibleBinaryOpTypes, token.position));
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
                                return Err(LoxError::interpreter_error(InterpreterErrorKind::IncompatibleBinaryOpTypes, token.position));
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
                                return Err(LoxError::interpreter_error(InterpreterErrorKind::IncompatibleBinaryOpTypes, token.position));
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
                                return Err(LoxError::interpreter_error(InterpreterErrorKind::IncompatibleBinaryOpTypes, token.position));
                            }
                        }
                    },
                    TokenKind::Less => {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                return Ok(Value::Bool(num_left < num_right));
                            },
                            _ => {
                                return Err(LoxError::interpreter_error(InterpreterErrorKind::IncompatibleBinaryOpTypes, token.position));
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
                                return Err(LoxError::interpreter_error(InterpreterErrorKind::IncompatibleBinaryOpTypes, token.position));
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
                        return Err(LoxError::interpreter_error(InterpreterErrorKind::InvalidBinaryType, token.position));
                    }
                }
            }
            ExprKind::Variable(name, _) =>
            {
                Ok(self.env.borrow().lookup_variable(name, expr.id))
            },
            ExprKind::Assign(name, expr, _) =>
            {
                let value: Value = self.evaluate(expr.as_ref())?;
                return Ok(self.env
                            .borrow_mut()
                            .assign_variable(name.to_owned(), value, expr.id));
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
                        return Err(LoxError::interpreter_error(InterpreterErrorKind::InvalidBinaryType, token.position));
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
                            return Err(LoxError::interpreter_error(InterpreterErrorKind::WrongArity(function.arity(), args.len() as u32), token.position));
                        }
                        return function.call(self, &args, token.position);
                    },
                    _ => {
                        return Err(LoxError::interpreter_error(InterpreterErrorKind::NotCallable, token.position));
                    }
                }
            },
        }
    }

}

pub enum State {
    Normal,
    Break,
    Continue,
    Return(Value)
}


#[derive(Clone, Debug)]
pub enum Callable {
    Function(Rc<FunctionDeclaration>, Rc<RefCell<Environment>>),
    Clock,
    Class(Rc<ClassDeclaration>, Rc<RefCell<Environment>>)
}

impl Callable
{
    fn arity(&self) -> u32 {
        match self {
            Callable::Function(declaration, _) => {
                declaration.parameters.len() as u32
            },
            Callable::Clock => 0,
            Callable::Class(_, _) => 0,
        }
    }

    fn call(&self,  interpreter: &Interpreter, args: &[Value], position: Position) -> Result<Value, LoxError> {
        match self
        {
            Callable::Clock =>
            {
                return clock(interpreter, args, position);
            },
            Callable::Function(declaration, environment) =>
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
            /* Call on class name construnct a new instance of the given class (there is no 'new' keyword in Lox) */
            Callable::Class(declaration, _) => {
                Ok(Value::ClassInstance(declaration.clone()))
            },
        }
    }
}
