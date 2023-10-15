use std::{fmt::Debug, rc::Rc, cell::RefCell, collections::HashMap};

use string_interner::{StringInterner, Symbol};

use crate::{parser_stmt::{Stmt, FunctionDeclaration, ClassDeclaration}, tokens::{TokenKind, LiteralValue, Position}, environment::{Environment, Scope}, error::{LoxError, InterpreterErrorKind}, parser_expr::{Expr, ExprKind}, native::clock, value::{Value, is_truthy, is_equal}, alias::{Identifier, ExprId}};

pub struct Interpreter
{
    environment_stack: Rc<RefCell<Environment>>,
    string_interner: Rc<RefCell<StringInterner>>,
    side_table: Rc<RefCell<HashMap<i64, usize>>>,
    global_scope: Rc<RefCell<Scope>>,
    this_symbol: Identifier
}

impl Interpreter
{
    #[inline]
    pub fn new(string_interner: Rc<RefCell<StringInterner>>) -> Self
    {
        let env = Environment::new();
        let this_symbol = string_interner.borrow_mut().get_or_intern_static("this");
        let clock_symbol = string_interner.borrow_mut().get_or_intern_static("clock");
        let mut interpreter = Interpreter {
            environment_stack: Rc::new(RefCell::new(env)),
            string_interner,
            side_table:  Rc::new(RefCell::new(HashMap::new())),
            global_scope: Rc::new(RefCell::new(Scope::new())),
            this_symbol
        };
        //>define native functions
        interpreter.define_variable(clock_symbol, Value::Callable(Callable::Clock));
        //<define native functions
        return interpreter;
    }

    #[inline]
    pub fn from(environment_stack: Rc<RefCell<Environment>>, intrepreter: &Interpreter) -> Self
    {
        Interpreter {
            environment_stack: environment_stack,
            string_interner: Rc::clone(&intrepreter.string_interner),
            side_table: Rc::clone(&intrepreter.side_table),
            global_scope: Rc::clone(&intrepreter.global_scope),
            this_symbol: intrepreter.this_symbol
        }
    }

    #[inline]
    pub fn insert_into_side_table(&mut self, expr_id: i64, depth: usize) {
        self.side_table.borrow_mut().insert(expr_id, depth);
    }

    #[inline]
    pub fn resolve(&mut self, expr_id: i64, depth: usize) {
        self.insert_into_side_table(expr_id, depth);
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
                match value {
                    Value::String(string) => println!("{}", string),
                    Value::Number(number) =>  println!("{}", number),
                    Value::Bool(boolean) =>  println!("{}", boolean),
                    Value::Nil => println!("{}", "nil"),
                    Value::Callable(callable) => {
                        match callable {
                            Callable::Function(fun_decl, _) =>  println!("Function: '{}()'", self.string_interner.borrow().resolve(fun_decl.name.get_identifier()).unwrap()),
                            Callable::Class(class_decl, _) => println!("Class: '{}'", self.string_interner.borrow().resolve(class_decl.name.get_identifier()).unwrap()),
                            Callable::Clock =>  println!("Native function: clock()"),
                        }
                    },
                    Value::ClassInstance(class_decl, _) => println!("Instance of class: '{}'", self.string_interner.borrow().resolve(class_decl.name.get_identifier()).unwrap()),
                }
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
                        self.define_variable(variable.to_owned(), value);
                    },
                    None =>
                    {
                        self.define_variable(variable.to_owned(), Value::Nil);
                    },
                }
                return Ok(State::Normal);
            }
            Stmt::Block(statements) =>
            {
                self.environment_stack.borrow_mut().new_local_scope();
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
                self.environment_stack.borrow_mut().remove_loval_scope();
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
                self.environment_stack.borrow_mut().new_local_scope();

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

                self.environment_stack.borrow_mut().remove_loval_scope();

                return Ok(State::Normal);
            },
            Stmt::FunctionDeclaration(declaration) => {
                let cloned_environment = Environment::from(&self.environment_stack.borrow());
                let function = Callable::Function(Rc::clone(&declaration), Rc::new(RefCell::new(cloned_environment)));
                self.define_variable(
                        declaration.name.get_identifier(),
                        Value::Callable(function)
                    );
                return Ok(State::Normal);
            },
            Stmt::ClassDeclaration(class_declaration) => {
                //class is callable to construct a new instance. The new instance must have its own class template.
                let callable = Callable::Class(Rc::clone(class_declaration), Rc::clone(&self.environment_stack));
                self.define_variable(
                    class_declaration.name.get_identifier(),
                    Value::Callable(callable)
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
                            return Ok(Value::String(Rc::clone(&val)));
                        },
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
                                return Ok(Value::String(Rc::new(format!("{}{}", str_left, str_right))));
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
            ExprKind::Variable(name, position) =>
            {
                println!("looking up variable: {} ({})", self.string_interner.borrow().resolve(*name).unwrap(), name.to_usize());
                match self.lookup_variable(*name, expr.id) {
                    Some(variable) => {
                        return Ok(variable);
                    },
                    None => {
                        return Err(LoxError::interpreter_error(InterpreterErrorKind::UdefinedVariableUsage(*name, Rc::clone(&self.string_interner)), *position));
                    },
                }
            },
            ExprKind::Assign(name, expr, position) =>
            {
                let value: Value = self.evaluate(expr.as_ref())?;
                let result: Result<Value, ()> = self.assign_variable(*name, value, expr.id);
                match result {
                    Ok(value) => { return Ok(value); },
                    Err(_) => {
                        return Err(LoxError::interpreter_error(InterpreterErrorKind::UdefinedVariableAssignment(*name, Rc::clone(&self.string_interner)), *position));
                    },
                }
            },
            ExprKind::Logical(left, token, right) =>
            {
                let val_left:  Value = self.evaluate(left.as_ref())?;
                match token.kind
                {
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
                match callee
                {
                    Value::Callable(function) =>
                    {
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
            ExprKind::Get(get_expr, property) =>
            {
                let value: Value = self.evaluate(get_expr)?;
                match &value
                {
                    Value::ClassInstance(class, attributes) =>
                    {
                        {
                            let attributes = attributes.borrow();
                            let opt_value = attributes.get(&property.get_identifier());
                            if let Some(value) = opt_value {
                                return Ok(value.clone());
                            }
                        }
                        {
                            let opt_method = class.methods.get(&property.get_identifier());
                            if let Some(method) = opt_method {
                                //>define new closure for the current method
                                let mut cloned_envoronment = Environment::from(&self.environment_stack.borrow());
                                {
                                    //create new scope for the THIS keyword and bind it to the current class instance
                                    let scope: Rc<RefCell<Scope>> = cloned_envoronment.new_local_scope();
                                    let class_instance = value.clone();
                                    scope.borrow_mut().define_variable(self.this_symbol, class_instance);
                                }
                                return Ok(Value::Callable(Callable::Function(Rc::clone(&method), Rc::new(RefCell::new(cloned_envoronment)))));
                            }
                        }
                        return Err(LoxError::interpreter_error(InterpreterErrorKind::UdefinedProperty(property.get_identifier(), Rc::clone(&self.string_interner)), property.position));
                    },
                    _ =>
                    {
                        return Err(LoxError::interpreter_error(InterpreterErrorKind::InvalidPropertyAccess, property.position));
                    }
                }
            },
            ExprKind::Set(object, name, value) =>
            {
                let object = self.evaluate(object)?;
                match object
                {
                    Value::ClassInstance(_, attributes) =>
                    {
                        let value = self.evaluate(value)?;
                        attributes.borrow_mut().insert(name.get_identifier(), value.clone());
                        return Ok(value);
                    },
                    _ => {
                        return Err(LoxError::interpreter_error(InterpreterErrorKind::InvalidPropertyAccess, name.position));
                    }
                }
            },
            ExprKind::This(this_token) => {
                println!("looking up variable: {}", "this");
                match self.lookup_variable(self.this_symbol, expr.id) {
                    Some(variable) => {
                        return Ok(variable);
                    },
                    None => {
                        return Err(LoxError::interpreter_error(InterpreterErrorKind::UdefinedVariableUsage(self.this_symbol, Rc::clone(&self.string_interner)), this_token.position));
                    },
                }
            },
        }
    }

    #[inline]
    pub fn lookup_variable(&self, name: Identifier, expr_id: ExprId) -> Option<Value>
    {
        {
            let borrowed_table = self.side_table.borrow();
            let opt_index = borrowed_table.get(&expr_id);
            if let Some(index) = opt_index {
                println!("searching variable '{}' ad index '{}' of {}", self.string_interner.borrow().resolve(name).unwrap(), *index, borrowed_table.len() - 1);
                return self.environment_stack.borrow().get_variable_from_local_at(*index, name);
            }
        }
        return self.global_scope.borrow().get_variable(name);
    }

    #[inline]
    pub fn assign_variable(&mut self, variable: Identifier, var_value: Value, expr_id: i64) -> Result<Value, ()>
    {
        {
            let borrowed_table = self.side_table.borrow_mut();
            let opt_index = borrowed_table.get(&expr_id);
            if let Some(index) = opt_index {
                return self.environment_stack.borrow_mut().assign_variable_to_local_at(*index, variable, var_value);
            }
        }
        return self.global_scope.borrow_mut().assign_variable(variable, var_value);

    }

    #[inline]
    pub fn define_variable(&mut self, variable: Identifier, var_value: Value)
    {

        {
            let borrowed_env = self.environment_stack.borrow_mut();
            let last_scope = borrowed_env.last_scope();
            if let Some(scope) = last_scope {
                println!("defining variable '{}' in local scope", self.string_interner.borrow().resolve(variable).unwrap());
                scope.borrow_mut().define_variable(variable, var_value);
                return;
            }
        }
        println!("defining variable '{}' in global scope", self.string_interner.borrow().resolve(variable).unwrap());
        self.global_scope.borrow_mut().define_variable(variable, var_value);
        return;
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
    Function(Rc<FunctionDeclaration>, Rc<RefCell<Environment>>/* Deep clone of the original environment */),
    Class(Rc<ClassDeclaration>, Rc<RefCell<Environment>>),
    Clock
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
                match clock() {
                    Ok(value) => Ok(value),
                    Err(error) => Err(LoxError::interpreter_error(error, position))
                }
            },
            Callable::Function(declaration, environment) =>
            {
                let mut local_interpreter = Interpreter::from(Rc::clone(environment), interpreter);
                local_interpreter.environment_stack.borrow_mut().new_local_scope();
                for (index, param) in declaration.parameters.iter().enumerate()
                {
                    local_interpreter.define_variable(param.get_identifier(), args.get(index).unwrap().clone());
                }

                let state = local_interpreter.execute_stmt(&declaration.body)?;
                local_interpreter.environment_stack.borrow_mut().remove_loval_scope();
                match state {
                    State::Return(value) => Ok(value),
                    _                           => Ok(Value::Nil)
                }

            },
            /* Call on class name construnct a new instance of the given class (there is no 'new' keyword in Lox) */
            Callable::Class(declaration, _) => {
                Ok(Value::ClassInstance(
                    Rc::clone(&declaration), Rc::new(RefCell::new(HashMap::new()))
                ))
            },
        }
    }
}