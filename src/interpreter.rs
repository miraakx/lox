use std::{fmt::Debug, rc::Rc, cell::RefCell};

use rustc_hash::FxHashMap;
use string_interner::StringInterner;

use crate::{parser_stmt::{Stmt, FunctionDeclaration, ClassDeclaration}, tokens::{Position, BinaryOperatorKind, UnaryOperatorKind, LogicalOperatorKind, LiteralKind}, environment::{Environment, Scope}, error::{LoxError, InterpreterErrorKind}, parser_expr::{Expr, ExprKind}, native::clock, value::{Value, is_truthy, is_equal}, alias::{IdentifierSymbol, ExprId, SideTable}};

pub struct Interpreter<'a>
{
    string_interner:   &'a mut StringInterner,
    side_table:        SideTable,
    global_scope:      Scope,
    this_symbol:       IdentifierSymbol,
    init_symbol:       IdentifierSymbol
}

impl <'a> Interpreter<'a>
{
    pub fn new(string_interner: &'a mut StringInterner, side_table: SideTable) -> Self
    {
        let this_symbol   = string_interner.get_or_intern_static("this");
        let init_symbol   = string_interner.get_or_intern_static("init");

        let interpreter = Interpreter {
            string_interner,
            side_table,
            global_scope: Scope::new(),
            this_symbol,
            init_symbol
        };
        return interpreter;
    }

    pub fn execute(&mut self, stmts: &[Stmt]) -> Result<(), ()>
    {
        let mut environment = Environment::new();
        let clock_symbol  = self.string_interner.get_or_intern_static("clock");
        self.define_variable(&mut environment, clock_symbol, Value::Callable(Callable::Clock));

        for stmt in stmts
        {
            match self.execute_stmt(stmt, &mut environment)
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

    fn execute_stmt(&mut self, stmt: &Stmt, environment: &mut Environment) -> Result<State, LoxError>
    {
        match stmt
        {
            Stmt::Print(expr) =>
            {
                match self.evaluate(expr, environment)? {
                    Value::String(string)   => println!("{}", string),
                    Value::Number(number)          =>  println!("{}", number),
                    Value::Bool(boolean)          =>  println!("{}", boolean),
                    Value::Nil                          => println!("{}", "nil"),
                    Value::Callable(callable) => {
                        match callable {
                            Callable::Function(fun_decl, _, _) => println!("Function: '{}()'", self.string_interner.resolve(fun_decl.identifier.name).unwrap()),
                            Callable::Class(class_decl, _)        => println!("Class: '{}'", self.string_interner.resolve(class_decl.identifier.name).unwrap()),
                            Callable::Clock                                             => println!("Native function: clock()"),
                        }
                    },
                    Value::ClassInstance(class_decl, _) => println!("Instance of class: '{}'", self.string_interner.resolve(class_decl.identifier.name).unwrap()),
                }
                return Ok(State::Normal);
            },
            Stmt::ExprStmt(expr) =>
            {
                self.evaluate(expr, environment)?;
                return Ok(State::Normal);
            }
            Stmt::Var(identifier, opt_expr) =>
            {
                match opt_expr
                {
                    Some(expr) =>
                    {
                        let value = self.evaluate(expr, environment)?;
                        self.define_variable(environment, identifier.name, value);
                    },
                    None =>
                    {
                        self.define_variable(environment, identifier.name, Value::Nil);
                    },
                }
                return Ok(State::Normal);
            }
            Stmt::Block(statements) =>
            {
                environment.new_local_scope();
                for stmt in statements
                {
                    let state = self.execute_stmt(stmt, environment)?;
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
                environment.remove_loval_scope();
                return Ok(State::Normal);
            },
            Stmt::If(condition, then_stmt) =>
            {
                let condition_value = self.evaluate(condition, environment)?;
                if is_truthy(&condition_value) {
                    return self.execute_stmt(then_stmt, environment);
                } else {
                    return Ok(State::Normal);
                }
            },
            Stmt::IfElse(condition, then_stmt, else_stmt) =>
            {
                let condition_value = self.evaluate(condition, environment)?;
                if is_truthy(&condition_value) {
                    return self.execute_stmt(then_stmt, environment);
                } else {
                    return self.execute_stmt(else_stmt, environment);
                }
            },
            Stmt::While(condition, body) =>
            {
                while is_truthy(&self.evaluate(condition, environment)?)
                {
                    let state = self.execute_stmt(body.as_ref(), environment)?;
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
                environment.new_local_scope();

                if let Some(initializer) = opt_initializer.as_ref() {
                    self.execute_stmt(initializer, environment)?;
                }

                while is_truthy(&self.evaluate_or(opt_condition, Value::Bool(true), environment)?)
                {
                    let state = self.execute_stmt(body, environment)?;
                    match state
                    {
                        State::Normal | State::Continue =>
                        {
                            self.evaluate_or(opt_increment, Value::Bool(true), environment)?;
                            continue;
                        },
                        State::Break =>
                        {
                            break;
                        },
                        State::Return(_) => return Ok(state),
                    }
                }
                environment.remove_loval_scope();
                return Ok(State::Normal);
            },
            Stmt::FunctionDeclaration(declaration) =>
            {
                //let cloned_environment = Environment::from(&self.environment_stack);
                let function = Callable::Function(Rc::clone(&declaration), environment.clone(), false);
                self.define_variable(environment,
                        declaration.identifier.name,
                        Value::Callable(function)
                    );
                return Ok(State::Normal);
            },
            Stmt::ClassDeclaration(class_declaration) =>
            {
                //class is callable to construct a new instance. The new instance must have its own class template.
                //let cloned_environment = Environment::from(&self.environment_stack);
                let callable = Callable::Class(Rc::clone(class_declaration), environment.clone());
                self.define_variable(environment,
                    class_declaration.identifier.name,
                    Value::Callable(callable)
                );
                return Ok(State::Normal);
            },
            Stmt::Return(opt_expr, _) =>
            {
                let value = if let Some(expr) = opt_expr {
                    self.evaluate(expr, environment)?
                } else {
                    Value::Nil
                };
                return Ok(State::Return(value));
            },
        }
    }

    fn evaluate_or(&mut self, opt_expr: &Option<Expr>, or_value: Value, environment: &mut Environment) ->  Result<Value, LoxError>
    {
        match opt_expr {
            Some(expr) => {
                return self.evaluate(expr, environment);
            },
            None => {
                return Ok(or_value);
            },
        };
    }

    fn evaluate(&mut self, expr: &Expr, environment:&mut Environment) -> Result<Value, LoxError>
    {
        match &expr.kind {
            ExprKind::Literal( literal) =>
            {
                match &literal.kind {
                    LiteralKind::String(val) =>
                    {
                        return Ok(val.clone());
                    },
                    LiteralKind::Number(val) =>
                    {
                        return Ok(val.clone());
                    },
                    LiteralKind::True(val) =>
                    {
                        return Ok(val.clone());
                    },
                    LiteralKind::False(val) =>
                    {
                        return Ok(val.clone());
                    },
                    LiteralKind::Nil =>
                    {
                        return Ok(Value::Nil);
                    }
                }
            },
            ExprKind::Unary(operator, right) =>
            {
                let val_right: Value = self.evaluate(right.as_ref(), environment)?;
                match operator.kind
                {
                    UnaryOperatorKind::Minus =>
                    {
                        match val_right
                        {
                            Value::Number(num) =>
                            {
                                return Ok(Value::Number(-num));
                            },
                            _ => {
                                return Err(LoxError::interpreter_error(InterpreterErrorKind::InvalidUnaryType, operator.position));
                            }
                        }
                    },
                    UnaryOperatorKind::Bang =>
                    {
                        Ok(Value::Bool(!is_truthy(&val_right)))
                    }
                }
            },
            ExprKind::Grouping(expr) =>
            {
                self.evaluate(expr.as_ref(), environment)
            },
            ExprKind::Binary(left, operator, right) =>
            {
                let val_left  = self.evaluate(left.as_ref(), environment)?;
                let val_right = self.evaluate(right.as_ref(), environment)?;
                match operator.kind {
                    BinaryOperatorKind::Minus =>
                    {
                        match (val_left, val_right)
                        {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                return Ok(Value::Number(num_left - num_right));
                            },
                            _ => {
                                return Err(LoxError::interpreter_error(InterpreterErrorKind::IncompatibleBinaryOpTypes, operator.position));
                            }
                        }
                    },
                    BinaryOperatorKind::Plus =>
                    {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                return Ok(Value::Number(num_left + num_right));
                            },
                            (Value::String(str_left), Value::String(str_right)) => {
                                return Ok(Value::String(Rc::new(format!("{}{}", str_left, str_right))));
                            },
                            _ => {
                                return Err(LoxError::interpreter_error(InterpreterErrorKind::IncompatibleBinaryOpTypes, operator.position));
                            }
                        }
                    },
                    BinaryOperatorKind::Slash =>
                    {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                return Ok(Value::Number(num_left / num_right));
                            },
                            _ => {
                                return Err(LoxError::interpreter_error(InterpreterErrorKind::IncompatibleBinaryOpTypes, operator.position));
                            }
                        }
                    },
                    BinaryOperatorKind::Star =>
                    {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                return Ok(Value::Number(num_left * num_right));
                            },
                            _ => {
                                return Err(LoxError::interpreter_error(InterpreterErrorKind::IncompatibleBinaryOpTypes, operator.position));
                            }
                        }
                    },
                    BinaryOperatorKind::Greater =>
                    {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                return Ok(Value::Bool(num_left > num_right));
                            },
                            _ => {
                                return Err(LoxError::interpreter_error(InterpreterErrorKind::IncompatibleBinaryOpTypes, operator.position));
                            }
                        }
                    },
                    BinaryOperatorKind::GreaterEqual =>
                    {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                return Ok(Value::Bool(num_left >= num_right));
                            },
                            _ => {
                                return Err(LoxError::interpreter_error(InterpreterErrorKind::IncompatibleBinaryOpTypes, operator.position));
                            }
                        }
                    },
                    BinaryOperatorKind::Less => {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                return Ok(Value::Bool(num_left < num_right));
                            },
                            _ => {
                                return Err(LoxError::interpreter_error(InterpreterErrorKind::IncompatibleBinaryOpTypes, operator.position));
                            }
                        }
                    },
                    BinaryOperatorKind::LessEqual =>
                    {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                return Ok(Value::Bool(num_left <= num_right));
                            },
                            _ => {
                                return Err(LoxError::interpreter_error(InterpreterErrorKind::IncompatibleBinaryOpTypes, operator.position));
                            }
                        }
                    },
                    BinaryOperatorKind::EqualEqual =>
                    {
                        return Ok(Value::Bool(is_equal(val_left, val_right)));
                    },
                    BinaryOperatorKind::BangEqual =>
                    {
                        return Ok(Value::Bool(!is_equal(val_left, val_right)));
                    }
                }
            }
            ExprKind::Variable(identifier) =>
            {
                match self.lookup_variable(environment, identifier.name, expr.id) {
                    Some(variable) => {
                        return Ok(variable);
                    },
                    None => {
                        return Err(LoxError::interpreter_error(InterpreterErrorKind::UdefinedVariableUsage(self.string_interner.resolve(identifier.name).unwrap().to_owned()), identifier.position));
                    },
                }
            },
            ExprKind::Assign(identifier, expr) =>
            {
                let value = self.evaluate(expr.as_ref(), environment)?;
                match self.assign_variable(environment, identifier.name, value, expr.id)
                {
                    Ok(value) => { return Ok(value); },
                    Err(_) => {
                        return Err(LoxError::interpreter_error(InterpreterErrorKind::UdefinedVariableAssignment(self.string_interner.resolve(identifier.name).unwrap().to_owned()), identifier.position));
                    },
                }
            },
            ExprKind::Logical(left, operator, right) =>
            {
                let val_left = self.evaluate(left.as_ref(), environment)?;
                match operator.kind
                {
                    LogicalOperatorKind::Or => {
                        if is_truthy(&val_left) {
                            return Ok(val_left);
                        } else {
                            return self.evaluate(right.as_ref(), environment);
                        }
                    },
                    LogicalOperatorKind::And => {
                        if !is_truthy(&val_left) {
                            return Ok(val_left);
                        } else {
                            return self.evaluate(right.as_ref(), environment);
                        }
                    }
                }
            },
            ExprKind::Call(callee_expr, opt_args_expr, position) => {
                let mut args: Vec<Value>;
                if let Some(args_expr) = opt_args_expr
                {
                    args = Vec::with_capacity(args_expr.len());
                    for arg_expr in args_expr
                    {
                        args.push(self.evaluate(arg_expr, environment)?);
                    }
                }
                else
                {
                    args = vec!();
                }
                match self.evaluate(callee_expr, environment)?
                {
                    Value::Callable(mut function) =>
                    {
                        if function.arity(self.init_symbol) != args.len() {
                            return Err(LoxError::interpreter_error(InterpreterErrorKind::WrongArity(function.arity(self.init_symbol), args.len()), *position));
                        }
                        return function.call(self, &args, position);
                    },
                    _ => {
                        return Err(LoxError::interpreter_error(InterpreterErrorKind::NotCallable, *position));
                    }
                }
            },
            ExprKind::Get(get_expr, property) =>
            {
                let instance = self.evaluate(get_expr, environment)?;
                match &instance
                {
                    Value::ClassInstance(class, attributes) =>
                    {
                        {
                            if let Some(value) = attributes.borrow().get(&property.name) {
                                return Ok(value.clone());
                            }
                        }
                        {
                            if let Some(method) = class.methods.get(&property.name) {
                                //>define new closure for the current method
                                let mut environment_clone = environment.clone();
                                let scope: Rc<RefCell<Scope>> = environment_clone.new_local_scope();
                                scope.borrow_mut().define_variable(self.this_symbol, instance.clone());
                                let callable = Callable::Function(Rc::clone(method), environment_clone, method.identifier.name == self.init_symbol);
                                return Ok(Value::Callable(callable));
                            }
                        }
                        return Err(LoxError::interpreter_error(InterpreterErrorKind::UdefinedProperty(self.string_interner.resolve(property.name).unwrap().to_owned()), property.position));
                    },
                    _ =>
                    {
                        return Err(LoxError::interpreter_error(InterpreterErrorKind::InvalidPropertyAccess, property.position));
                    }
                }
            },
            ExprKind::Set(object, identifier, value) =>
            {
                match self.evaluate(object, environment)?
                {
                    Value::ClassInstance(_, attributes) =>
                    {
                        let value = self.evaluate(value, environment)?;
                        attributes.borrow_mut().insert(identifier.name, value.clone());
                        return Ok(value);
                    },
                    _ => {
                        return Err(LoxError::interpreter_error(InterpreterErrorKind::InvalidPropertyAccess, identifier.position));
                    }
                }
            },
            ExprKind::This(position) => {
                //println!("looking up variable: {}", "this");
                match self.lookup_variable(environment, self.this_symbol, expr.id)
                {
                    Some(variable) => {
                        return Ok(variable);
                    },
                    None => {
                        return Err(LoxError::interpreter_error(InterpreterErrorKind::UdefinedVariableUsage(self.string_interner.resolve(self.this_symbol).unwrap().to_owned()), *position));
                    },
                }
            },
        }
    }

    pub fn lookup_variable(&self, environment: &mut Environment, name: IdentifierSymbol, expr_id: ExprId) -> Option<Value>
    {
        {
            if let Some(index) = self.side_table.get(&expr_id)
            {
                return environment.get_variable_from_local_at(*index, name);
            }
        }
        return self.global_scope.get_variable(name);
    }

    pub fn assign_variable(&mut self, environment: &mut Environment, variable: IdentifierSymbol, var_value: Value, expr_id: i64) -> Result<Value, ()>
    {
        {
            if let Some(index) = self.side_table.get(&expr_id)
            {
                return environment.assign_variable_to_local_at(*index, variable, var_value);
            }
        }
        return self.global_scope.assign_variable(variable, var_value);
    }

    pub fn define_variable(&mut self, environment: &mut Environment, variable: IdentifierSymbol, var_value: Value)
    {
        {
            if let Some(scope) = environment.last_scope()
            {
                scope.borrow_mut().define_variable(variable, var_value);
                return;
            }
        }
        self.global_scope.define_variable(variable, var_value);
        return;
    }

}

pub enum State
{
    Normal,
    Break,
    Continue,
    Return(Value)
}

#[derive(Clone, Debug)]
pub enum Callable
{
    Function(Rc<FunctionDeclaration>, Environment, bool),
    Class(Rc<ClassDeclaration>, Environment),
    Clock
}

impl Callable
{
    fn arity(&self, init_symbol: IdentifierSymbol) -> usize
    {
        match self {
            Callable::Function(declaration, _, _) => {
                declaration.parameters.len()
            },
            Callable::Class(rc_declaration, _) => {
                //>If class has an initializer determine the number of parameters of the initializer to be passed to the class contructor
                if let Some(init) = rc_declaration.methods.get(&init_symbol)
                {
                    return init.parameters.len();
                }
                return 0;
            },
            Callable::Clock => 0,
        }
    }

    fn call(&mut self,  interpreter: &mut Interpreter, args: &[Value], position: &Position) -> Result<Value, LoxError>
    {
        match self
        {
            Callable::Function(declaration, environment, is_initializer) =>
            {
                environment.new_local_scope();
                for (index, param) in declaration.parameters.iter().enumerate()
                {
                    interpreter.define_variable(environment, param.identifier.name, args.get(index).unwrap().clone());
                }

                let state = interpreter.execute_stmt(&declaration.body, environment)?;
                environment.remove_loval_scope();
                if *is_initializer {
                    return Ok(environment.last_scope().unwrap().borrow().get_variable(interpreter.this_symbol).unwrap());
                }
                match state {
                    State::Return(value) => {
                        return Ok(value);
                    },
                    _ => {
                        return Ok(Value::Nil);
                    }
                };
            },
            /* Call on class name construct a new instance of the given class (there is no 'new' keyword in Lox) */
            Callable::Class(declaration, environment) =>
            {
                let instance = Value::ClassInstance(
                    Rc::clone(&declaration), Rc::new(RefCell::new(FxHashMap::default()))
                );
                //>call init method (if it exists)
                if let Some(init) = declaration.methods.get(&interpreter.init_symbol)
                {
                    let mut cloned_environment = environment.clone();
                    let scope: Rc<RefCell<Scope>> = cloned_environment.new_local_scope();
                    scope.borrow_mut().define_variable(interpreter.this_symbol, instance.clone());
                    let mut callable: Callable = Callable::Function(Rc::clone(init), cloned_environment, true);
                    callable.call(interpreter, args, &declaration.identifier.position)?;
                }
                Ok(instance)
            },
            Callable::Clock =>
            {
                match clock() {
                    Ok(value) => Ok(value),
                    Err(error) => Err(LoxError::interpreter_error(error, *position))
                }
            },
        }
    }

}