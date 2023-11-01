use std::{fmt::Debug, rc::Rc, cell::RefCell};

use rustc_hash::FxHashMap;
use string_interner::StringInterner;

use crate::{parser_stmt::{Stmt, FunctionDeclaration, ClassDeclaration}, tokens::{Position, BinaryOperatorKind, UnaryOperatorKind, LogicalOperatorKind}, environment::{Environment, Scope}, error::{LoxError, InterpreterErrorKind}, parser_expr::{Expr, ExprKind}, native::clock, value::{Value, is_truthy, is_equal, ClassInstance}, alias::{IdentifierSymbol, ExprId, SideTable}};

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
        Interpreter {
            string_interner,
            side_table,
            global_scope: Scope::new(),
            this_symbol,
            init_symbol
        }
    }

    fn define_native_functions(&mut self) {
        let clock_symbol  = self.string_interner.get_or_intern_static("clock");
        self.global_scope.define_variable(clock_symbol, Value::Callable(Box::new(Callable::Clock)));
    }

    pub fn execute(&mut self, stmts: &[Stmt]) -> Result<(), ()>
    {
        let mut environment = Environment::new();

        self.define_native_functions();

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
                    Value::Number(number)          => println!("{}", number),
                    Value::Bool(boolean)          => println!("{}", boolean),
                    Value::Nil                          => println!("nil"),
                    Value::Callable(callable) => {
                        match *callable {
                            Callable::Function(fun_decl, _)     => println!("<fn: '{}'>",        self.string_interner.resolve(fun_decl.identifier.name).unwrap()),
                            Callable::InitFunction(fun_decl, _) => println!("<fn (init): '{}'>", self.string_interner.resolve(fun_decl.identifier.name).unwrap()),
                            Callable::Class(class_decl, _)         => println!("<class: '{}'>",     self.string_interner.resolve(class_decl.identifier.name).unwrap()),
                            Callable::Clock                                              => println!("<fn (native): 'clock'>"),
                        }
                    },
                    Value::ClassInstance(class_instance) => println!("Instance of class: '{}'", self.string_interner.resolve(class_instance.declaration.identifier.name).unwrap()),
                }
                Ok(State::Normal)
            },
            Stmt::Expr(expr) =>
            {
                self.evaluate(expr, environment)?;
                Ok(State::Normal)
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
                Ok(State::Normal)
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
                environment.remove_local_scope();
                Ok(State::Normal)
            },
            Stmt::If(if_stmt) =>
            {
                let condition_value = self.evaluate(&if_stmt.condition, environment)?;
                if is_truthy(&condition_value) {
                    self.execute_stmt(&if_stmt.then_stmt, environment)
                } else {
                    Ok(State::Normal)
                }
            },
            Stmt::IfElse(if_else_stmt) =>
            {
                let condition_value = self.evaluate(&if_else_stmt.condition, environment)?;
                if is_truthy(&condition_value) {
                    self.execute_stmt(&if_else_stmt.then_stmt, environment)
                } else {
                    self.execute_stmt(&if_else_stmt.else_stmt, environment)
                }
            },
            Stmt::While(while_stmt) =>
            {
                while is_truthy(&self.evaluate(&while_stmt.condition, environment)?)
                {
                    let state = self.execute_stmt(&while_stmt.body, environment)?;
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
                Ok(State::Normal)
            },
            Stmt::Break => {
                Ok(State::Break)
            },
            Stmt::Continue => {
                Ok(State::Continue)
            },
            Stmt::For(for_stmt) =>
            {
                environment.new_local_scope();

                if let Some(initializer) = &for_stmt.opt_initializer {
                    self.execute_stmt(initializer, environment)?;
                }

                while is_truthy(&self.evaluate_or(&for_stmt.opt_condition, Value::Bool(true), environment)?)
                {
                    let state = self.execute_stmt(&for_stmt.body, environment)?;
                    match state
                    {
                        State::Normal | State::Continue =>
                        {
                            self.evaluate_or(&for_stmt.opt_increment, Value::Bool(true), environment)?;
                            continue;
                        },
                        State::Break =>
                        {
                            break;
                        },
                        State::Return(_) => return Ok(state),
                    }
                }
                environment.remove_local_scope();
                Ok(State::Normal)
            },
            Stmt::FunctionDeclaration(declaration) =>
            {
                let function = Callable::Function(Rc::clone(declaration), environment.clone());
                self.define_variable(environment,
                        declaration.identifier.name,
                        Value::Callable(Box::new(function))
                    );
                Ok(State::Normal)
            },
            Stmt::ClassDeclaration(class_declaration) =>
            {
                //class is callable to construct a new instance. The new instance must have its own class template.
                let callable = Callable::Class(Rc::clone(class_declaration), environment.clone());
                self.define_variable(environment,
                    class_declaration.identifier.name,
                    Value::Callable(Box::new(callable))
                );
                Ok(State::Normal)
            },
            Stmt::Return(opt_expr, _) =>
            {
                let value = if let Some(expr) = opt_expr {
                    self.evaluate(expr, environment)?
                } else {
                    Value::Nil
                };
                Ok(State::Return(value))
            },
        }
    }

    fn evaluate_or(&mut self, opt_expr: &Option<Expr>, or_value: Value, environment: &mut Environment) ->  Result<Value, LoxError>
    {
        opt_expr.as_ref().map_or_else(|| Ok(or_value), |expr| self.evaluate(expr, environment))
    }

    fn evaluate(&mut self, expr: &Expr, environment:&mut Environment) -> Result<Value, LoxError>
    {
        match &expr.kind {
            ExprKind::Literal(value) =>
            {
                Ok(value.clone())
            },
            ExprKind::Unary(unary_expr) =>
            {
                let val_right: Value = self.evaluate(&unary_expr.expr, environment)?;
                match unary_expr.operator.kind
                {
                    UnaryOperatorKind::Minus =>
                    {
                        match val_right
                        {
                            Value::Number(num) =>
                            {
                                Ok(Value::Number(-num))
                            },
                            _ => {
                                Err(LoxError::interpreter_error(InterpreterErrorKind::InvalidUnaryType, unary_expr.operator.position))
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
                self.evaluate(expr, environment)
            },
            ExprKind::Binary(binary_expr) =>
            {
                let val_left  = self.evaluate(&binary_expr.left, environment)?;
                let val_right = self.evaluate(&binary_expr.right, environment)?;
                match binary_expr.operator.kind {
                    BinaryOperatorKind::Minus =>
                    {
                        match (val_left, val_right)
                        {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                Ok(Value::Number(num_left - num_right))
                            },
                            _ => {
                                Err(LoxError::interpreter_error(InterpreterErrorKind::IncompatibleBinaryOpTypes, binary_expr.operator.position))
                            }
                        }
                    },
                    BinaryOperatorKind::Plus =>
                    {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                Ok(Value::Number(num_left + num_right))
                            },
                            (Value::String(str_left), Value::String(str_right)) => {
                                Ok(Value::String(Rc::new(format!("{}{}", str_left, str_right))))
                            },
                            _ => {
                                Err(LoxError::interpreter_error(InterpreterErrorKind::IncompatibleBinaryOpTypes, binary_expr.operator.position))
                            }
                        }
                    },
                    BinaryOperatorKind::Slash =>
                    {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                Ok(Value::Number(num_left / num_right))
                            },
                            _ => {
                                Err(LoxError::interpreter_error(InterpreterErrorKind::IncompatibleBinaryOpTypes, binary_expr.operator.position))
                            }
                        }
                    },
                    BinaryOperatorKind::Star =>
                    {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                Ok(Value::Number(num_left * num_right))
                            },
                            _ => {
                                Err(LoxError::interpreter_error(InterpreterErrorKind::IncompatibleBinaryOpTypes, binary_expr.operator.position))
                            }
                        }
                    },
                    BinaryOperatorKind::Greater =>
                    {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                Ok(Value::Bool(num_left > num_right))
                            },
                            _ => {
                                Err(LoxError::interpreter_error(InterpreterErrorKind::IncompatibleBinaryOpTypes, binary_expr.operator.position))
                            }
                        }
                    },
                    BinaryOperatorKind::GreaterEqual =>
                    {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                Ok(Value::Bool(num_left >= num_right))
                            },
                            _ => {
                                Err(LoxError::interpreter_error(InterpreterErrorKind::IncompatibleBinaryOpTypes, binary_expr.operator.position))
                            }
                        }
                    },
                    BinaryOperatorKind::Less => {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                Ok(Value::Bool(num_left < num_right))
                            },
                            _ => {
                                Err(LoxError::interpreter_error(InterpreterErrorKind::IncompatibleBinaryOpTypes, binary_expr.operator.position))
                            }
                        }
                    },
                    BinaryOperatorKind::LessEqual =>
                    {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                Ok(Value::Bool(num_left <= num_right))
                            },
                            _ => {
                                Err(LoxError::interpreter_error(InterpreterErrorKind::IncompatibleBinaryOpTypes, binary_expr.operator.position))
                            }
                        }
                    },
                    BinaryOperatorKind::EqualEqual =>
                    {
                        Ok(Value::Bool(is_equal(val_left, val_right)))
                    },
                    BinaryOperatorKind::BangEqual =>
                    {
                        Ok(Value::Bool(!is_equal(val_left, val_right)))
                    }
                }
            }
            ExprKind::Variable(identifier) =>
            {
                match self.lookup_variable(environment, identifier.name, expr.id) {
                    Some(variable) => {
                        Ok(variable)
                    },
                    None => {
                        Err(LoxError::interpreter_error(InterpreterErrorKind::UdefinedVariableUsage(self.string_interner.resolve(identifier.name).unwrap().to_owned()), identifier.position))
                    },
                }
            },
            ExprKind::Assign(assign_expr) =>
            {
                let value = self.evaluate(&assign_expr.expr, environment)?;
                match self.assign_variable(environment, assign_expr.identifier.name, &value, expr.id)
                {
                    Ok(_) => {
                        Ok(value)
                    },
                    Err(_) => {
                        Err(LoxError::interpreter_error(InterpreterErrorKind::UdefinedVariableAssignment(self.string_interner.resolve(assign_expr.identifier.name).unwrap().to_owned()), assign_expr.identifier.position))
                    },
                }
            },
            ExprKind::Logical(logica_expr) =>
            {
                let val_left = self.evaluate(&logica_expr.left, environment)?;
                match logica_expr.operator.kind
                {
                    LogicalOperatorKind::Or => {
                        if is_truthy(&val_left) {
                            Ok(val_left)
                        } else {
                            self.evaluate(&logica_expr.right, environment)
                        }
                    },
                    LogicalOperatorKind::And => {
                        if !is_truthy(&val_left) {
                            Ok(val_left)
                        } else {
                            self.evaluate(&logica_expr.right, environment)
                        }
                    }
                }
            },
            ExprKind::Call(call_expr) => {
                match self.evaluate(&call_expr.callee, environment)?
                {
                    Value::Callable(mut function) =>
                    {
                        if function.arity(self.init_symbol) == call_expr.arguments.len()
                        {
                            function.call(self, environment, &call_expr.arguments, &call_expr.position)
                        }
                        else
                        {
                            Err(LoxError::interpreter_error(InterpreterErrorKind::WrongArity(function.arity(self.init_symbol), call_expr.arguments.len()), call_expr.position))
                        }
                    },
                    _ => {
                        Err(LoxError::interpreter_error(InterpreterErrorKind::NotCallable, call_expr.position))
                    }
                }
            },
            ExprKind::Get(get_expr) =>
            {
                let instance = self.evaluate(&get_expr.expr, environment)?;
                match &instance
                {
                    Value::ClassInstance(class_instance) =>
                    {
                        {
                            if let Some(value) = class_instance.attributes.borrow().get(&get_expr.identifier.name) {
                                return Ok(value.clone());
                            }
                        }
                        {
                            if let Some(method) = class_instance.declaration.methods.get(&get_expr.identifier.name) {
                                //>define new closure for the current method
                                let mut environment_clone = environment.clone();
                                let scope: Rc<RefCell<Scope>> = environment_clone.new_local_scope();
                                scope.borrow_mut().define_variable(self.this_symbol, instance.clone());
                                let callable =
                                if method.identifier.name == self.init_symbol {
                                    Callable::InitFunction(Rc::clone(method), environment_clone)
                                } else {
                                    Callable::Function(Rc::clone(method), environment_clone)
                                };
                                return Ok(Value::Callable(Box::new(callable)));
                            }
                        }
                        return Err(LoxError::interpreter_error(InterpreterErrorKind::UdefinedProperty(self.string_interner.resolve(get_expr.identifier.name).unwrap().to_owned()), get_expr.identifier.position));
                    },
                    _ =>
                    {
                        Err(LoxError::interpreter_error(InterpreterErrorKind::InvalidPropertyAccess, get_expr.identifier.position))
                    }
                }
            },
            ExprKind::Set(set_expr) =>
            {
                match self.evaluate(&set_expr.target, environment)?
                {
                    Value::ClassInstance(class_instance) =>
                    {
                        let value = self.evaluate(&set_expr.value, environment)?;
                        class_instance.attributes.borrow_mut().insert(set_expr.identifier.name, value.clone());
                        Ok(value)
                    },
                    _ => {
                        Err(LoxError::interpreter_error(InterpreterErrorKind::InvalidPropertyAccess, set_expr.identifier.position))
                    }
                }
            },
            ExprKind::This(position) => {
                match self.lookup_variable(environment, self.this_symbol, expr.id)
                {
                    Some(variable) => {
                        Ok(variable)
                    },
                    None => {
                        Err(LoxError::interpreter_error(InterpreterErrorKind::UdefinedVariableUsage(self.string_interner.resolve(self.this_symbol).unwrap().to_owned()), *position))
                    },
                }
            },

        }
    }

    #[inline]
    pub fn lookup_variable(&self, environment: &Environment, name: IdentifierSymbol, expr_id: ExprId) -> Option<Value>
    {
        self.side_table.get(&expr_id).map_or_else(|| self.global_scope.get_variable(name), |index| environment.get_variable_from_local_at(*index, name))
    }

    #[inline]
    pub fn assign_variable(&mut self, environment: &mut Environment, variable: IdentifierSymbol, var_value: &Value, expr_id: i64) -> Result<(), ()>
    {
        match self.side_table.get(&expr_id) {
            Some(index) => {
                environment.assign_variable_to_local_at(*index, variable, var_value)
            },
            None => {
                self.global_scope.assign_variable(variable, var_value)
            },
        }
    }

    #[inline]
    pub fn define_variable(&mut self, environment: &mut Environment, variable: IdentifierSymbol, var_value: Value)
    {
        match environment.last_scope() {
            Some(scope) => {
                scope.borrow_mut().define_variable(variable, var_value);
            },
            None => {
                self.global_scope.define_variable(variable, var_value);
            },
        }
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
    Function(Rc<FunctionDeclaration>, Environment),
    InitFunction(Rc<FunctionDeclaration>, Environment),
    Class(Rc<ClassDeclaration>, Environment),
    Clock
}

impl Callable
{
    #[inline]
    fn arity(&self, init_symbol: IdentifierSymbol) -> usize
    {
        match self {
            Self::Function(declaration, _) =>
            {
                declaration.parameters.len()
            },
            Self::InitFunction(declaration, _) => {
                declaration.parameters.len()
            },
            Self::Class(rc_declaration, _) =>
            {
                //>If class has an initializer determine the number of parameters of the initializer to be passed to the class contructor
                if let Some(init) = rc_declaration.methods.get(&init_symbol) {
                    init.parameters.len()
                } else {
                    0
                }
            },
            Self::Clock => { 0 }
        }
    }

    #[inline]
    fn call(&mut self, interpreter: &mut Interpreter, interpreter_environment: &mut Environment, args_expr: &[Expr], position: &Position) -> Result<Value, LoxError>
    {
        match self
        {
            Self::Function(declaration, function_environment) =>
            {

                match function_call(interpreter, interpreter_environment, function_environment, declaration, args_expr)? {
                    State::Return(value) => {
                        Ok(value)
                    },
                    _ => {
                        Ok(Value::Nil)
                    }
                }
            },
            Self::InitFunction(declaration, function_environment) =>
            {

                let _ = function_call(interpreter, interpreter_environment, function_environment, declaration, args_expr)?;

                return Ok(function_environment.last_scope().unwrap().borrow().get_variable(interpreter.this_symbol).unwrap());
            },

            /* Call on class name construct a new instance of the given class (there is no 'new' keyword in Lox) */
            Self::Class(declaration, environment) =>
            {
                let instance = Value::ClassInstance(
                    Box::new(ClassInstance { declaration: Rc::clone(declaration), attributes: Rc::new(RefCell::new(FxHashMap::default())) })
                );
                //>call init method (if it exists)
                if let Some(init) = declaration.methods.get(&interpreter.init_symbol)
                {
                    let mut cloned_environment = environment.clone();
                    let scope = cloned_environment.new_local_scope();
                    scope.borrow_mut().define_variable(interpreter.this_symbol, instance.clone());
                    let mut callable = Self::InitFunction(Rc::clone(init), cloned_environment);
                    callable.call(interpreter, interpreter_environment, args_expr, &declaration.identifier.position)?;
                }
                Ok(instance)
            },
            Self::Clock =>
            {
                match clock() {
                    Ok(value) => Ok(value),
                    Err(error) => Err(LoxError::interpreter_error(error, *position))
                }
            },
        }
    }

}

#[inline]
fn function_call(
    interpreter:             &mut Interpreter<'_>,
    interpreter_environment: &mut Environment,
    function_environment:    &mut Environment,
    declaration:             &mut Rc<FunctionDeclaration>,
    args_expr:               &[Expr]
) -> Result<State, LoxError>
{
    let rc_scope = function_environment.new_local_scope();

    for (name, arg_expr) in declaration.parameters.iter().zip(args_expr.iter())
    {
        //do not inline value
        let value = interpreter.evaluate(arg_expr, interpreter_environment)?;
        rc_scope.borrow_mut().define_variable(*name, value);
    }

    let state = interpreter.execute_stmt(&declaration.body, function_environment)?;

    function_environment.remove_local_scope();

    Ok(state)
}