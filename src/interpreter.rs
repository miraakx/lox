use std::{fmt::Debug, rc::Rc, cell::RefCell};

use rustc_hash::FxHashMap;
use string_interner::StringInterner;

use crate::{parser_stmt::{Stmt, FunctionDeclaration, ClassDeclaration}, tokens::{Position, BinaryOperatorKind, UnaryOperatorKind, LogicalOperatorKind, LiteralKind}, environment::{Environment, Scope}, error::{LoxError, InterpreterErrorKind}, parser_expr::{Expr, ExprKind}, native::clock, value::{Value, is_truthy, is_equal}, alias::{IdentifierSymbol, ExprId, SideTable}};

pub struct Interpreter<'a>
{
    environment_stack: Environment,
    string_interner:   &'a StringInterner,
    side_table:        Rc<SideTable>,
    global_scope:      Rc<RefCell<Scope>>,
    this_symbol:       IdentifierSymbol,
    init_symbol:       IdentifierSymbol
}

impl <'a> Interpreter<'a>
{
    pub fn new(string_interner: &'a mut StringInterner, side_table: SideTable) -> Self
    {
        let environment = Environment::new();
        let this_symbol   = string_interner.get_or_intern_static("this");
        let init_symbol   = string_interner.get_or_intern_static("init");
        let clock_symbol  = string_interner.get_or_intern_static("clock");
        let mut interpreter = Interpreter {
            environment_stack: environment,
            string_interner,
            side_table:   Rc::new(side_table),
            global_scope: Rc::new(RefCell::new(Scope::new())),
            this_symbol,
            init_symbol
        };
        //>define native functions
        interpreter.define_variable(clock_symbol, Value::Callable(Callable::Clock));
        //<define native functions
        return interpreter;
    }

    pub fn from(environment_stack: &Environment, intrepreter: &'a Interpreter) -> Self
    {
        Interpreter {
            environment_stack: environment_stack.clone(),
            string_interner:   intrepreter.string_interner,
            side_table:        Rc::clone(&intrepreter.side_table),
            global_scope:      Rc::clone(&intrepreter.global_scope),
            this_symbol:       intrepreter.this_symbol,
            init_symbol:       intrepreter.init_symbol
        }
    }

    pub fn execute(&mut self, stmts: &[Stmt]) -> Result<(), ()>
    {
        for stmt in stmts
        {
            match self.execute_stmt(stmt)
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

    fn execute_stmt(&mut self, stmt: &Stmt) -> Result<State, LoxError>
    {
        match stmt
        {
            Stmt::Print(expr) =>
            {
                match self.evaluate(expr)? {
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
                self.evaluate(expr)?;
                return Ok(State::Normal);
            }
            Stmt::Var(identifier, opt_expr) =>
            {
                match opt_expr
                {
                    Some(expr) =>
                    {
                        let value = self.evaluate(expr)?;
                        self.define_variable(identifier.name, value);
                    },
                    None =>
                    {
                        self.define_variable(identifier.name, Value::Nil);
                    },
                }
                return Ok(State::Normal);
            }
            Stmt::Block(statements) =>
            {
                self.environment_stack.new_local_scope();
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
                self.environment_stack.remove_loval_scope();
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
                self.environment_stack.new_local_scope();

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
                self.environment_stack.remove_loval_scope();
                return Ok(State::Normal);
            },
            Stmt::FunctionDeclaration(declaration) =>
            {
                let cloned_environment = Environment::from(&self.environment_stack);
                let function = Callable::Function(Rc::clone(&declaration), cloned_environment, false);
                self.define_variable(
                        declaration.identifier.name,
                        Value::Callable(function)
                    );
                return Ok(State::Normal);
            },
            Stmt::ClassDeclaration(class_declaration) =>
            {
                //class is callable to construct a new instance. The new instance must have its own class template.
                let cloned_environment = Environment::from(&self.environment_stack);
                let callable = Callable::Class(Rc::clone(class_declaration), cloned_environment);
                self.define_variable(
                    class_declaration.identifier.name,
                    Value::Callable(callable)
                );
                return Ok(State::Normal);
            },
            Stmt::Return(opt_expr, _) =>
            {
                let value = if let Some(expr) = opt_expr {
                    self.evaluate(expr)?
                } else {
                    Value::Nil
                };
                return Ok(State::Return(value));
            },
        }
    }

    fn evaluate_or(&mut self, opt_expr: &Option<Expr>, or_value: Value) ->  Result<Value, LoxError>
    {
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
                let val_right: Value = self.evaluate(right.as_ref())?;
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
                self.evaluate(expr.as_ref())
            },
            ExprKind::Binary(left, operator, right) =>
            {
                let val_left  = self.evaluate(left.as_ref())?;
                let val_right = self.evaluate(right.as_ref())?;
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
                match self.lookup_variable(identifier.name, expr.id) {
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
                let value = self.evaluate(expr.as_ref())?;
                match self.assign_variable(identifier.name, value, expr.id)
                {
                    Ok(value) => { return Ok(value); },
                    Err(_) => {
                        return Err(LoxError::interpreter_error(InterpreterErrorKind::UdefinedVariableAssignment(self.string_interner.resolve(identifier.name).unwrap().to_owned()), identifier.position));
                    },
                }
            },
            ExprKind::Logical(left, operator, right) =>
            {
                let val_left = self.evaluate(left.as_ref())?;
                match operator.kind
                {
                    LogicalOperatorKind::Or => {
                        if is_truthy(&val_left) {
                            return Ok(val_left);
                        } else {
                            return self.evaluate(right.as_ref());
                        }
                    },
                    LogicalOperatorKind::And => {
                        if !is_truthy(&val_left) {
                            return Ok(val_left);
                        } else {
                            return self.evaluate(right.as_ref());
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
                        args.push(self.evaluate(arg_expr)?);
                    }
                }
                else
                {
                    args = vec!();
                }
                match self.evaluate(callee_expr)?
                {
                    Value::Callable(function) =>
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
                let instance = self.evaluate(get_expr)?;
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
                                let mut environment_clone = self.environment_stack.clone();
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
                match self.evaluate(object)?
                {
                    Value::ClassInstance(_, attributes) =>
                    {
                        let value = self.evaluate(value)?;
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
                match self.lookup_variable(self.this_symbol, expr.id)
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

    pub fn lookup_variable(&self, name: IdentifierSymbol, expr_id: ExprId) -> Option<Value>
    {
        {
            if let Some(index) = self.side_table.get(&expr_id)
            {
                return self.environment_stack.get_variable_from_local_at(*index, name);
            }
        }
        return self.global_scope.borrow().get_variable(name);
    }

    pub fn assign_variable(&mut self, variable: IdentifierSymbol, var_value: Value, expr_id: i64) -> Result<Value, ()>
    {
        {
            if let Some(index) = self.side_table.get(&expr_id)
            {
                return self.environment_stack.assign_variable_to_local_at(*index, variable, var_value);
            }
        }
        return self.global_scope.borrow_mut().assign_variable(variable, var_value);
    }

    pub fn define_variable(&mut self, variable: IdentifierSymbol, var_value: Value)
    {
        {
            if let Some(scope) = self.environment_stack.last_scope()
            {
                scope.borrow_mut().define_variable(variable, var_value);
                return;
            }
        }
        self.global_scope.borrow_mut().define_variable(variable, var_value);
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

    fn call(&self,  interpreter: &Interpreter, args: &[Value], position: &Position) -> Result<Value, LoxError>
    {
        match self
        {
            Callable::Function(declaration, environment, is_initializer) =>
            {
                let mut local_interpreter = Interpreter::from(environment, interpreter);
                local_interpreter.environment_stack.new_local_scope();
                for (index, param) in declaration.parameters.iter().enumerate()
                {
                    local_interpreter.define_variable(param.identifier.name, args.get(index).unwrap().clone());
                }

                let state = local_interpreter.execute_stmt(&declaration.body)?;
                local_interpreter.environment_stack.remove_loval_scope();
                if *is_initializer {
                    return Ok(environment.last_scope().unwrap().borrow().get_variable(local_interpreter.this_symbol).unwrap());
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
            /* Call on class name construnct a new instance of the given class (there is no 'new' keyword in Lox) */
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
                    let callable: Callable = Callable::Function(Rc::clone(init), cloned_environment, true);
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