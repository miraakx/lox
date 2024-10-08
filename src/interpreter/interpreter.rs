use std::{fmt::Debug, rc::Rc, cell::RefCell, io::Write};

use rustc_hash::FxHashMap;
use string_interner::StringInterner;

use crate::{alias::{ExprId, IdentifierSymbol, SideTable}, error::{ExecutionResult, InterpreterErrorKind, LoxError}, parser::{position::Position, types::{BinaryOperatorKind, Expr, ExprKind, Literal, LogicalOperatorKind, Stmt, UnaryOperatorKind}}};

use super::{environment::Environment, native_functions::{assert_eq, clock}, types::{LoxClass, LoxFunction, LoxInstance, Value}};

pub struct Interpreter<'a, T:Write>
{
    string_interner:   &'a mut StringInterner,
    side_table:        SideTable,
    global_scope:      Rc<RefCell<Environment>>,
    this_symbol:       IdentifierSymbol,
    init_symbol:       IdentifierSymbol,
    super_symbol:      IdentifierSymbol,
    writer:            Rc<RefCell<T>>
}

impl <'a, T:Write> Interpreter<'a, T>
{
    pub fn new_with_writer(string_interner: &'a mut StringInterner, side_table: SideTable, writer: Rc<RefCell<T>>) -> Self
    {
        let this_symbol  = string_interner.get("this").unwrap();
        let init_symbol  = string_interner.get("init").unwrap();
        let super_symbol = string_interner.get("super").unwrap();
        Interpreter {
            string_interner,
            side_table,
            global_scope: Environment::default(),
            this_symbol,
            init_symbol,
            super_symbol,
            writer
        }
    }

    fn define_native_functions(&mut self) {
        let clock_symbol     = self.string_interner.get_or_intern_static("clock");
        let assert_eq_symbol = self.string_interner.get_or_intern_static("assertEq");
        let str_symbol       = self.string_interner.get_or_intern_static("str");
        self.global_scope.borrow_mut().define_variable(clock_symbol, Value::Callable(Callable::Clock));
        self.global_scope.borrow_mut().define_variable(assert_eq_symbol, Value::Callable(Callable::AssertEq));
        self.global_scope.borrow_mut().define_variable(str_symbol, Value::Callable(Callable::Str));
    }

    /// Interpreter's entry point for running a program.
    ///
    /// Defines native functions and delegates the execution of all the statements to `execute_stmts`.
    pub fn execute(&mut self, stmts: &[Stmt]) -> Result<(), ExecutionResult>
    {
        self.define_native_functions();

        let environment = Rc::clone(&self.global_scope);

        match self.execute_stmts(stmts, &environment)
        {
            Ok(_) => {}
            Err(_) => {
                return Err(ExecutionResult::RuntimeError);
            },
        }
        Ok(())
    }

    /// Loops though all the statements and executes them one by one.
    fn execute_stmts(&mut self, stmts: &[Stmt], environment: &Rc<RefCell<Environment>>) -> Result<State, ()>
    {
        for stmt in stmts
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
        Ok(State::Normal)
    }

    #[inline]
    fn write_line(&mut self, message: &str) {
        let _ = writeln!(self.writer.borrow_mut(), "{}", message);
    }

    /// Executes a single statement.
    ///
    fn execute_stmt(&mut self, stmt: &Stmt, environment: &Rc<RefCell<Environment>>) -> Result<State, ()>
    {
        match stmt
        {
            Stmt::Print(expr) =>
            {
                let val = self.evaluate(expr, environment)?;
                self.write_line(&val.to_string(self.string_interner));
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
                        environment.borrow_mut().define_variable(identifier.name, value);
                    },
                    None =>
                    {
                        environment.borrow_mut().define_variable(identifier.name, Value::Nil);
                    },
                }
                Ok(State::Normal)
            }
            Stmt::Block(statements) =>
            {

                let new_env = Environment::new(environment);
                for stmt in statements
                {
                    let state = self.execute_stmt(stmt, &new_env)?;
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
                Ok(State::Normal)
            },
            Stmt::If(if_stmt) =>
            {
                let condition_value = self.evaluate(&if_stmt.condition, environment)?;
                if condition_value.is_truthy() {
                    self.execute_stmt(&if_stmt.then_stmt, environment)
                } else {
                    Ok(State::Normal)
                }
            },
            Stmt::IfElse(if_else_stmt) =>
            {
                let condition_value = self.evaluate(&if_else_stmt.condition, environment)?;
                if condition_value.is_truthy() {
                    self.execute_stmt(&if_else_stmt.then_stmt, environment)
                } else {
                    self.execute_stmt(&if_else_stmt.else_stmt, environment)
                }
            },
            Stmt::While(while_stmt) =>
            {
                while self.evaluate(&while_stmt.condition, environment)?.is_truthy()
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
            //Interpret a function declariation (fun my_function(...) {...}) by converting its compile time represtation 'FunctionDeclaration' to its runtime representation 'Callable::Function'
            Stmt::FunctionDeclaration(function_declaration) =>
            {
                let lox_function = LoxFunction { declaration: Rc::clone(function_declaration), closure: Rc::clone(environment) };
                let function = Callable::Function(Rc::new(RefCell::new(lox_function)));
                environment.borrow_mut().define_variable(function_declaration.identifier.name, Value::Callable(function));
                Ok(State::Normal)
            },
            Stmt::ClassDeclaration(class_stmt) =>
            {
                let opt_superclass: Option<Rc<LoxClass>> =
                    match &class_stmt.superclass_expr
                    {
                        Some(superclass_var) =>
                        {
                            //Evaluate super class expression
                            let superclass_value = self.evaluate(superclass_var, environment)?;

                            //Check if super class expression refers to a class
                            match &superclass_value
                            {
                                Value::Callable(Callable::Class(rc_lox_class)) =>
                                {
                                    Some(Rc::clone(rc_lox_class))
                                },
                                _ => {
                                    let _ = writeln!(self.writer.borrow_mut(), "{}", LoxError::interpreter_error(InterpreterErrorKind::SuperclassMustBeAClass, class_stmt.identifier.position));
                                    return Err(());
                                }
                            }
                        },
                        None => {
                            None
                        },
                    };

                let class_env: Rc<RefCell<Environment>> =
                    match &opt_superclass
                    {
                        Some(superclass) =>
                        {
                            let env = Environment::new(environment);
                            env.borrow_mut().define_variable(self.super_symbol, Value::Callable(Callable::Class(Rc::clone(superclass))));
                            env
                        }
                        None =>
                        {
                            Rc::clone(environment)
                        },
                    };

                let mut methods_map: FxHashMap<IdentifierSymbol, LoxFunction> = FxHashMap::default();
                for (id, fun_stmt) in class_stmt.methods.iter() {
                    let fun = LoxFunction {declaration: Rc::clone(fun_stmt), closure: Rc::clone(&class_env) };
                    methods_map.insert(*id, fun);
                }
                let lox_class = LoxClass::new(class_stmt.identifier.clone(), methods_map, opt_superclass);
                let callable = Callable::Class(Rc::new(lox_class));

                environment.borrow_mut().define_variable(class_stmt.identifier.name, Value::Callable(callable));
                Ok(State::Normal)
            },
            Stmt::Return(opt_expr, _) =>
            {
                let value =
                    match opt_expr {
                        Some(expr) => { self.evaluate(expr, environment)? },
                        None => Value::Nil,
                    };
                Ok(State::Return(value))
            },
        }
    }

    /// Evaluates an expression.
    ///
    /// Recursivly evaluates an expression.
    ///
    fn evaluate(&mut self, expr: &Expr, environment: &Rc<RefCell<Environment>>) -> Result<Value, ()>
    {
        match &expr.kind {
            ExprKind::Literal(literal) =>
            {
                match literal {
                    Literal::String(rc_string, _) => {
                        Ok(Value::String(Rc::clone(rc_string)))
                    },
                    Literal::Number(number, _) => {
                        Ok(Value::Number(*number))
                    },
                    Literal::True(_) => {
                        Ok(Value::Bool(true))
                    },
                    Literal::False(_) => {
                        Ok(Value::Bool(false))
                    },
                    Literal::Nil(_) => {
                        Ok(Value::Nil)
                    },
                }
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
                                let _ = writeln!(self.writer.borrow_mut(), "{}", LoxError::interpreter_error(InterpreterErrorKind::CheckNumberOperand, unary_expr.operator.position));
                                Err(())
                            }
                        }
                    },
                    UnaryOperatorKind::Bang =>
                    {
                        Ok(Value::Bool(!val_right.is_truthy()))
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
                                let _ = writeln!(self.writer.borrow_mut(), "{}", LoxError::interpreter_error(InterpreterErrorKind::CheckNumberOperands, binary_expr.operator.position));
                                Err(())
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
                                let _ = writeln!(self.writer.borrow_mut(), "{}", LoxError::interpreter_error(InterpreterErrorKind::InvalidPlusOperands, binary_expr.operator.position));
                                Err(())
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
                                let _ = writeln!(self.writer.borrow_mut(), "{}", LoxError::interpreter_error(InterpreterErrorKind::CheckNumberOperands, binary_expr.operator.position));
                                Err(())
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
                                let _ = writeln!(self.writer.borrow_mut(), "{}", LoxError::interpreter_error(InterpreterErrorKind::CheckNumberOperands, binary_expr.operator.position));
                                Err(())
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
                                let _ = writeln!(self.writer.borrow_mut(), "{}", LoxError::interpreter_error(InterpreterErrorKind::CheckNumberOperands, binary_expr.operator.position));
                                Err(())
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
                                let _ = writeln!(self.writer.borrow_mut(), "{}", LoxError::interpreter_error(InterpreterErrorKind::CheckNumberOperands, binary_expr.operator.position));
                                Err(())
                            }
                        }
                    },
                    BinaryOperatorKind::Less => {
                        match (val_left, val_right) {
                            (Value::Number(num_left), Value::Number(num_right)) => {
                                Ok(Value::Bool(num_left < num_right))
                            },
                            _ => {
                                let _ = writeln!(self.writer.borrow_mut(), "{}", LoxError::interpreter_error(InterpreterErrorKind::CheckNumberOperands, binary_expr.operator.position));
                                Err(())
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
                                let _ = writeln!(self.writer.borrow_mut(), "{}", LoxError::interpreter_error(InterpreterErrorKind::CheckNumberOperands, binary_expr.operator.position));
                                Err(())
                            }
                        }
                    },
                    BinaryOperatorKind::EqualEqual =>
                    {
                        Ok(Value::Bool(val_left == val_right))
                    },
                    BinaryOperatorKind::BangEqual =>
                    {
                        Ok(Value::Bool(val_left != val_right))
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
                        let _ = writeln!(self.writer.borrow_mut(), "{}", LoxError::interpreter_error(InterpreterErrorKind::UndefinedVariable(self.string_interner.resolve(identifier.name).unwrap().to_owned()), identifier.position));
                        Err(())
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
                        let _ = writeln!(self.writer.borrow_mut(), "{}", LoxError::interpreter_error(InterpreterErrorKind::UndefinedVariable(self.string_interner.resolve(assign_expr.identifier.name).unwrap().to_owned()), assign_expr.identifier.position));
                        Err(())
                    },
                }
            },
            ExprKind::Logical(logica_expr) =>
            {
                let val_left = self.evaluate(&logica_expr.left, environment)?;
                match logica_expr.operator.kind
                {
                    LogicalOperatorKind::Or => {
                        if val_left.is_truthy() {
                            Ok(val_left)
                        } else {
                            self.evaluate(&logica_expr.right, environment)
                        }
                    },
                    LogicalOperatorKind::And => {
                        if !val_left.is_truthy() {
                            Ok(val_left)
                        } else {
                            self.evaluate(&logica_expr.right, environment)
                        }
                    }
                }
            },
            ExprKind::Call(call_expr) =>
            {
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
                            let _ = writeln!(self.writer.borrow_mut(), "{}", LoxError::interpreter_error(InterpreterErrorKind::WrongArity(function.arity(self.init_symbol), call_expr.arguments.len()), call_expr.position));
                            Err(())
                        }
                    },
                    _ => {
                        let _ = writeln!(self.writer.borrow_mut(), "{}", LoxError::interpreter_error(InterpreterErrorKind::NotCallable, call_expr.position));
                        Err(())
                    }
                }
            },
            ExprKind::Get(get_expr) =>
            {
                //Valuta l'Expr su cui agisce il punto (Get)
                let instance = self.evaluate(&get_expr.expr, environment)?;

                //L'uso del punto (Get) ha senso solo se agisce sull'istanza di una classe ( Value::ClassInstance(LoxClass) )
                match &instance
                {
                    Value::ClassInstance(class_instance) =>
                    {
                        //Verifica se sia stato richiamato un attributo
                        if let Some(value) = class_instance.attributes.borrow().get(&get_expr.identifier.name) {
                            return Ok(value.clone());
                        }

                        //Verifica se sia stato richiamato un metodo
                        if let Some(method) = class_instance.declaration.find_method(&get_expr.identifier.name)
                        {
                            let callable = method.bind(Value::ClassInstance(Rc::clone(class_instance)), self.this_symbol);
                            return Ok(Value::Callable(callable));
                        }

                        let _ = writeln!(self.writer.borrow_mut(), "{}", LoxError::interpreter_error(InterpreterErrorKind::UdefinedProperty(self.string_interner.resolve(get_expr.identifier.name).unwrap().to_owned()), get_expr.identifier.position));
                        return Err(());
                    },
                    _ =>
                    {
                        let _ = writeln!(self.writer.borrow_mut(), "{}", LoxError::interpreter_error(InterpreterErrorKind::OnlyInstancesHaveProperties, get_expr.identifier.position));
                        return Err(());
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
                        let _ = writeln!(self.writer.borrow_mut(), "{}", LoxError::interpreter_error(InterpreterErrorKind::OnlyInstancesHaveFields, set_expr.identifier.position));
                        Err(())
                    }
                }
            },
            ExprKind::This(position) =>
            {
                match self.lookup_variable(environment, self.this_symbol, expr.id)
                {
                    Some(variable) => {
                        Ok(variable)
                    },
                    None => {
                        let _ = writeln!(self.writer.borrow_mut(), "{}", LoxError::interpreter_error(InterpreterErrorKind::UndefinedVariable(self.string_interner.resolve(self.this_symbol).unwrap().to_owned()), *position));
                        Err(())
                    },
                }
            },
            ExprKind::Super(identifier) =>
            {
                let index: usize = *self.side_table.get(&expr.id).unwrap();
                let superclass  : Value = environment.borrow().get_at(index,     &self.super_symbol).unwrap();
                let object      : Value = environment.borrow().get_at(index - 1, &self.this_symbol).unwrap();

                match superclass
                {
                    Value::Callable(Callable::Class(lox_superclass)) =>
                    {
                        let opt_method = lox_superclass.find_method(&identifier.name);

                        //Verifica se sia stato richiamato un metodo
                        match opt_method
                        {
                            Some(method) =>
                            {
                                let callable = method.bind(object, self.this_symbol);
                                Ok(Value::Callable(callable))
                            },
                            None =>
                            {
                                let _ = writeln!(self.writer.borrow_mut(), "{}", LoxError::interpreter_error(InterpreterErrorKind::UdefinedProperty(self.string_interner.resolve(identifier.name).unwrap().to_owned()), identifier.position));
                                Err(())
                            },
                        }
                    },
                    _ => {
                        panic!()
                    }
                }
            },
        }
    }

    #[inline]
    fn lookup_variable(&self, environment: &Rc<RefCell<Environment>>, name: IdentifierSymbol, expr_id: ExprId) -> Option<Value>
    {
        match self.side_table.get(&expr_id) {
            Some(distance) => {
                environment.borrow().get_at(*distance, &name)
            },
            None => {
                self.global_scope.borrow().get(&name)
            },
        }
    }

    #[inline]
    fn assign_variable(&mut self, environment: &Rc<RefCell<Environment>>, name: IdentifierSymbol, var_value: &Value, expr_id: i64) -> Result<(), ()>
    {
        match self.side_table.get(&expr_id) {
            Some(distance) => {
                environment.borrow_mut().assign_at(*distance, name, var_value)
            },
            None => {
                self.global_scope.borrow_mut().assign(name, var_value)
            },
        }
    }
}

enum State
{
    Normal,
    Break,
    Continue,
    Return(Value)
}

#[derive(Clone, Debug)]
pub enum Callable
{
    Function(Rc<RefCell<LoxFunction>>),
    Class(Rc<LoxClass>),
    Clock,
    AssertEq,
    Str
}

impl Callable
{
    #[inline]
    /// Returns the number of parameters of the callable
    fn arity(&self, init_symbol: IdentifierSymbol) -> usize
    {
        match self {
            Self::Function(function) =>
            {
                function.borrow().declaration.parameters.len()
            },
            Self::Class(class) =>
            {
                //If class has an initializer determine the number of parameters of the initializer to be passed to the class contructor
                match class.find_method(&init_symbol) {
                    Some(init) => { init.declaration.parameters.len() },
                    None => 0,
                }
            },
            Self::Clock => { 0 }
            Self::AssertEq => { 2 },
            Self::Str => { 1 },
        }
    }

    #[inline]
    /// Executes a callable instance and returns its Value or an error.
    fn call<T:Write>(
        &mut self,
        interpreter:                &mut Interpreter<T>,
        interpreter_environment:    &Rc<RefCell<Environment>>,
        args_expr:                  &[Expr],
        position:                   &Position
    ) -> Result<Value, ()>
    {
        match self
        {
            Self::Function(function) =>
            {
                let state: State = {
                    let enclosing: &Rc<RefCell<Environment>> = &function.borrow().closure;

                    let rc_scope = Environment::new(enclosing);

                    for (name, arg_expr) in function.borrow().declaration.parameters.iter().zip(args_expr.iter())
                    {
                        //do not inline value
                        let value = interpreter.evaluate(arg_expr, interpreter_environment)?;
                        rc_scope.borrow_mut().define_variable(*name, value);
                    }

                    interpreter.execute_stmts(
                        &function.borrow().declaration.body,
                        &rc_scope
                    )?
                };

                // non spostare da qui! (init ritorna 'this' anche se non presente un return al suo interno)
                if function.borrow().declaration.is_initializer
                {
                    return Ok(function.borrow().closure.borrow().get(&interpreter.this_symbol).unwrap());
                }
                match state
                {
                    State::Return(value) => {
                        Ok(value)
                    },
                    _ => {
                        Ok(Value::Nil)
                    }
                }
            },
            // Construct a new class instance. Calls on class identifier construct a new instance of the given class (there is no 'new' keyword in Lox)
            Self::Class(lox_class) =>
            {
                //Create the new instance Value
                let instance = Value::ClassInstance(
                    Rc::new(
                        LoxInstance {
                            declaration: Rc::clone(lox_class),
                            attributes: Rc::new(RefCell::new(FxHashMap::default()))
                        }
                    )
                );

                // Call the init method (if it exists)
                if let Some(init) = lox_class.find_method(&interpreter.init_symbol)
                {
                    let mut callable = init.bind(instance.clone(), interpreter.this_symbol);
                    let _ = callable.call(interpreter, interpreter_environment, args_expr, &lox_class.identifier.position)?;
                }
                Ok(instance)
            },
            Self::Clock =>
            {
                match clock()
                {
                    Ok(value) => Ok(value),
                    Err(_) => {
                        let _ = writeln!(interpreter.writer.borrow_mut(), "{}", LoxError::interpreter_error(InterpreterErrorKind::NativeClockSysTimeError, *position));
                        return Err(());
                    }
                }
            },
            Self::AssertEq =>
            {
                let actual   = interpreter.evaluate(&args_expr[0], interpreter_environment)?;
                let expected = interpreter.evaluate(&args_expr[1], interpreter_environment)?;
                match assert_eq(actual, expected)
                {
                    Ok(_) => {
                        Ok(Value::Nil)
                    },
                    Err(_) => {
                        let _ = writeln!(interpreter.writer.borrow_mut(), "{}", LoxError::interpreter_error(InterpreterErrorKind::AssertionFailure, *position));
                        return Err(());
                    }
                }
            },
            Self::Str =>
            {
                let value = interpreter.evaluate(&args_expr[0], interpreter_environment)?;
                Ok(Value::String(Rc::new(value.to_string(interpreter.string_interner))))
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::run;

    mod assignment {
        use super::test;
        #[test]
        fn prefix_operator() {
            test("./lox_test/assignment/prefix_operator.lox");
        }
        #[test]
        fn associativity() {
            test("./lox_test/assignment/associativity.lox");
        }
        #[test]
        fn undefined() {
            test("./lox_test/assignment/undefined.lox");
        }
        #[test]
        fn to_this() {
            test("./lox_test/assignment/to_this.lox");
        }
        #[test]
        fn global() {
            test("./lox_test/assignment/global.lox");
        }
        #[test]
        fn grouping() {
            test("./lox_test/assignment/grouping.lox");
        }
        #[test]
        fn syntax() {
            test("./lox_test/assignment/syntax.lox");
        }
        #[test]
        fn infix_operator() {
            test("./lox_test/assignment/infix_operator.lox");
        }
        #[test]
        fn local() {
            test("./lox_test/assignment/local.lox");
        }
    }
    mod operator {
        use super::test;
        #[test]
        fn not() {
            test("./lox_test/operator/not.lox");
        }
        #[test]
        fn less_or_equal_nonnum_num() {
            test("./lox_test/operator/less_or_equal_nonnum_num.lox");
        }
        #[test]
        fn add_nil_nil() {
            test("./lox_test/operator/add_nil_nil.lox");
        }
        #[test]
        fn less_or_equal_num_nonnum() {
            test("./lox_test/operator/less_or_equal_num_nonnum.lox");
        }
        #[test]
        fn less_nonnum_num() {
            test("./lox_test/operator/less_nonnum_num.lox");
        }
        #[test]
        fn multiply() {
            test("./lox_test/operator/multiply.lox");
        }
        #[test]
        fn equals_method() {
            test("./lox_test/operator/equals_method.lox");
        }
        #[test]
        fn multiply_nonnum_num() {
            test("./lox_test/operator/multiply_nonnum_num.lox");
        }
        #[test]
        fn subtract() {
            test("./lox_test/operator/subtract.lox");
        }
        #[test]
        fn not_class() {
            test("./lox_test/operator/not_class.lox");
        }
        #[test]
        fn subtract_num_nonnum() {
            test("./lox_test/operator/subtract_num_nonnum.lox");
        }
        #[test]
        fn less_num_nonnum() {
            test("./lox_test/operator/less_num_nonnum.lox");
        }
        #[test]
        fn greater_num_nonnum() {
            test("./lox_test/operator/greater_num_nonnum.lox");
        }
        #[test]
        fn negate_nonnum() {
            test("./lox_test/operator/negate_nonnum.lox");
        }
        #[test]
        fn not_equals() {
            test("./lox_test/operator/not_equals.lox");
        }
        #[test]
        fn add_bool_nil() {
            test("./lox_test/operator/add_bool_nil.lox");
        }
        #[test]
        fn divide_nonnum_num() {
            test("./lox_test/operator/divide_nonnum_num.lox");
        }
        #[test]
        fn subtract_nonnum_num() {
            test("./lox_test/operator/subtract_nonnum_num.lox");
        }
        #[test]
        fn negate() {
            test("./lox_test/operator/negate.lox");
        }
        #[test]
        fn greater_or_equal_num_nonnum() {
            test("./lox_test/operator/greater_or_equal_num_nonnum.lox");
        }
        #[test]
        fn add() {
            test("./lox_test/operator/add.lox");
        }
        #[test]
        fn greater_or_equal_nonnum_num() {
            test("./lox_test/operator/greater_or_equal_nonnum_num.lox");
        }
        #[test]
        fn divide() {
            test("./lox_test/operator/divide.lox");
        }
        #[test]
        fn add_bool_string() {
            test("./lox_test/operator/add_bool_string.lox");
        }
        #[test]
        fn add_string_nil() {
            test("./lox_test/operator/add_string_nil.lox");
        }
        #[test]
        fn equals() {
            test("./lox_test/operator/equals.lox");
        }
        #[test]
        fn greater_nonnum_num() {
            test("./lox_test/operator/greater_nonnum_num.lox");
        }
        #[test]
        fn add_num_nil() {
            test("./lox_test/operator/add_num_nil.lox");
        }
        #[test]
        fn comparison() {
            test("./lox_test/operator/comparison.lox");
        }
        #[test]
        fn equals_class() {
            test("./lox_test/operator/equals_class.lox");
        }
        #[test]
        fn divide_num_nonnum() {
            test("./lox_test/operator/divide_num_nonnum.lox");
        }
        #[test]
        fn multiply_num_nonnum() {
            test("./lox_test/operator/multiply_num_nonnum.lox");
        }
        #[test]
        fn add_bool_num() {
            test("./lox_test/operator/add_bool_num.lox");
        }
    }
    mod field {
        use super::test;
        #[test]
        fn many() {
            test("./lox_test/field/many.lox");
        }
        #[test]
        fn method() {
            test("./lox_test/field/method.lox");
        }
        #[test]
        fn set_on_nil() {
            test("./lox_test/field/set_on_nil.lox");
        }
        #[test]
        fn method_binds_this() {
            test("./lox_test/field/method_binds_this.lox");
        }
        #[test]
        fn call_nonfunction_field() {
            test("./lox_test/field/call_nonfunction_field.lox");
        }
        #[test]
        fn get_on_function() {
            test("./lox_test/field/get_on_function.lox");
        }
        #[test]
        fn set_on_num() {
            test("./lox_test/field/set_on_num.lox");
        }
        #[test]
        fn get_on_nil() {
            test("./lox_test/field/get_on_nil.lox");
        }
        #[test]
        fn set_on_function() {
            test("./lox_test/field/set_on_function.lox");
        }
        #[test]
        fn undefined() {
            test("./lox_test/field/undefined.lox");
        }
        #[test]
        fn call_function_field() {
            test("./lox_test/field/call_function_field.lox");
        }
        #[test]
        fn get_on_string() {
            test("./lox_test/field/get_on_string.lox");
        }
        #[test]
        fn get_on_bool() {
            test("./lox_test/field/get_on_bool.lox");
        }
        #[test]
        fn on_instance() {
            test("./lox_test/field/on_instance.lox");
        }
        #[test]
        fn set_on_class() {
            test("./lox_test/field/set_on_class.lox");
        }
        #[test]
        fn set_on_bool() {
            test("./lox_test/field/set_on_bool.lox");
        }
        #[test]
        fn get_and_set_method() {
            test("./lox_test/field/get_and_set_method.lox");
        }
        #[test]
        fn set_on_string() {
            test("./lox_test/field/set_on_string.lox");
        }
        #[test]
        fn get_on_class() {
            test("./lox_test/field/get_on_class.lox");
        }
        #[test]
        fn set_evaluation_order() {
            test("./lox_test/field/set_evaluation_order.lox");
        }
        #[test]
        fn get_on_num() {
            test("./lox_test/field/get_on_num.lox");
        }
    }
    mod method {
        use super::test;
        #[test]
        fn missing_arguments() {
            test("./lox_test/method/missing_arguments.lox");
        }
        #[test]
        fn print_bound_method() {
            test("./lox_test/method/print_bound_method.lox");
        }
        #[test]
        fn too_many_parameters() {
            test("./lox_test/method/too_many_parameters.lox");
        }
        #[test]
        fn too_many_arguments() {
            test("./lox_test/method/too_many_arguments.lox");
        }
        #[test]
        fn extra_arguments() {
            test("./lox_test/method/extra_arguments.lox");
        }
        #[test]
        fn arity() {
            test("./lox_test/method/arity.lox");
        }
        #[test]
        fn empty_block() {
            test("./lox_test/method/empty_block.lox");
        }
        #[test]
        fn not_found() {
            test("./lox_test/method/not_found.lox");
        }
        #[test]
        fn refer_to_name() {
            test("./lox_test/method/refer_to_name.lox");
        }
    }
    mod this {
        use super::test;
        #[test]
        fn this_at_top_level() {
            test("./lox_test/this/this_at_top_level.lox");
        }
        #[test]
        fn nested_closure() {
            test("./lox_test/this/nested_closure.lox");
        }
        #[test]
        fn nested_class() {
            test("./lox_test/this/nested_class.lox");
        }
        #[test]
        fn this_in_method() {
            test("./lox_test/this/this_in_method.lox");
        }
        #[test]
        fn closure() {
            test("./lox_test/this/closure.lox");
        }
        #[test]
        fn this_in_top_level_function() {
            test("./lox_test/this/this_in_top_level_function.lox");
        }
    }
    mod regression {
        use super::test;
        #[test]
        fn regression_1() {
            test("./lox_test/regression/regression_1.lox");
        }
        #[test]
        fn regression_2() {
            test("./lox_test/regression/regression_2.lox");
        }
    }
    mod constructor {
        use super::test;
        #[test]
        fn missing_arguments() {
            test("./lox_test/constructor/missing_arguments.lox");
        }
        #[test]
        fn early_return() {
            test("./lox_test/constructor/early_return.lox");
        }
        #[test]
        fn extra_arguments() {
            test("./lox_test/constructor/extra_arguments.lox");
        }
        #[test]
        fn init_not_method() {
            test("./lox_test/constructor/init_not_method.lox");
        }
        #[test]
        fn arguments() {
            test("./lox_test/constructor/arguments.lox");
        }
        #[test]
        fn return_value() {
            test("./lox_test/constructor/return_value.lox");
        }
        #[test]
        fn return_in_nested_function() {
            test("./lox_test/constructor/return_in_nested_function.lox");
        }
        #[test]
        fn default_arguments() {
            test("./lox_test/constructor/default_arguments.lox");
        }
        #[test]
        fn call_init_explicitly() {
            test("./lox_test/constructor/call_init_explicitly.lox");
        }
        #[test]
        fn call_init_early_return() {
            test("./lox_test/constructor/call_init_early_return.lox");
        }
        #[test]
        fn default() {
            test("./lox_test/constructor/default.lox");
        }
    }
    mod expressions {
        use super::test;
        #[test]
        #[ignore]
        fn evaluate() {
            test("./lox_test/expressions/evaluate.lox");
        }
        #[test]
        #[ignore]
        fn parse() {
            test("./lox_test/expressions/parse.lox");
        }
    }
    mod function {
        use super::test;
        #[test]
        fn missing_arguments() {
            test("./lox_test/function/missing_arguments.lox");
        }
        #[test]
        fn too_many_parameters() {
            test("./lox_test/function/too_many_parameters.lox");
        }
        #[test]
        fn extra_arguments() {
            test("./lox_test/function/extra_arguments.lox");
        }
        #[test]
        fn nested_call_with_arguments() {
            test("./lox_test/function/nested_call_with_arguments.lox");
        }
        #[test]
        fn mutual_recursion() {
            test("./lox_test/function/mutual_recursion.lox");
        }
        #[test]
        fn parameters() {
            test("./lox_test/function/parameters.lox");
        }
        #[test]
        fn print() {
            test("./lox_test/function/print.lox");
        }
        #[test]
        fn too_many_arguments() {
            test("./lox_test/function/too_many_arguments.lox");
        }
        #[test]
        fn local_recursion() {
            test("./lox_test/function/local_recursion.lox");
        }
        #[test]
        fn body_must_be_block() {
            test("./lox_test/function/body_must_be_block.lox");
        }
        #[test]
        fn empty_body() {
            test("./lox_test/function/empty_body.lox");
        }
        #[test]
        fn local_mutual_recursion() {
            test("./lox_test/function/local_mutual_recursion.lox");
        }
        #[test]
        fn recursion() {
            test("./lox_test/function/recursion.lox");
        }
        #[test]
        fn missing_comma_in_parameters() {
            test("./lox_test/function/missing_comma_in_parameters.lox");
        }
    }
    mod comments {
        use super::test;
        #[test]
        fn line_at_eof() {
            test("./lox_test/comments/line_at_eof.lox");
        }
        #[test]
        fn unicode() {
            test("./lox_test/comments/unicode.lox");
        }
        #[test]
        fn only_line_comment_and_line() {
            test("./lox_test/comments/only_line_comment_and_line.lox");
        }
        #[test]
        fn only_line_comment() {
            test("./lox_test/comments/only_line_comment.lox");
        }
    }
    mod string {
        use super::test;
        #[test]
        fn error_after_multiline() {
            test("./lox_test/string/error_after_multiline.lox");
        }
        #[test]
        fn multiline() {
            test("./lox_test/string/multiline.lox");
        }
        #[test]
        fn unterminated() {
            test("./lox_test/string/unterminated.lox");
        }
        #[test]
        fn literals() {
            test("./lox_test/string/literals.lox");
        }
    }
    mod for_keyword {
        use super::test;
        #[test]
        fn scope() {
            test("./lox_test/for/scope.lox");
        }
        #[test]
        fn statement_initializer() {
            test("./lox_test/for/statement_initializer.lox");
        }
        #[test]
        fn return_inside() {
            test("./lox_test/for/return_inside.lox");
        }
        #[test]
        fn statement_condition() {
            test("./lox_test/for/statement_condition.lox");
        }
        #[test]
        fn statement_increment() {
            test("./lox_test/for/statement_increment.lox");
        }
        #[test]
        fn closure_in_body() {
            test("./lox_test/for/closure_in_body.lox");
        }
        #[test]
        fn var_in_body() {
            test("./lox_test/for/var_in_body.lox");
        }
        #[test]
        fn class_in_body() {
            test("./lox_test/for/class_in_body.lox");
        }
        #[test]
        fn syntax() {
            test("./lox_test/for/syntax.lox");
        }
        #[test]
        fn return_closure() {
            test("./lox_test/for/return_closure.lox");
        }
        #[test]
        fn fun_in_body() {
            test("./lox_test/for/fun_in_body.lox");
        }
    }
    mod inheritance {
        use super::test;
        #[test]
        fn set_fields_from_base_class() {
            test("./lox_test/inheritance/set_fields_from_base_class.lox");
        }
        #[test]
        fn inherit_methods() {
            test("./lox_test/inheritance/inherit_methods.lox");
        }
        #[test]
        fn parenthesized_superclass() {
            test("./lox_test/inheritance/parenthesized_superclass.lox");
        }
        #[test]
        fn constructor() {
            test("./lox_test/inheritance/constructor.lox");
        }
        #[test]
        fn inherit_from_function() {
            test("./lox_test/inheritance/inherit_from_function.lox");
        }
        #[test]
        fn inherit_from_number() {
            test("./lox_test/inheritance/inherit_from_number.lox");
        }
        #[test]
        fn inherit_from_nil() {
            test("./lox_test/inheritance/inherit_from_nil.lox");
        }
    }

    mod limit {
        use super::test;
        #[ignore]
        #[test]
        fn no_reuse_constants() {
            test("./lox_test/limit/no_reuse_constants.lox");
        }

        #[ignore]
        #[test]
        fn stack_overflow() {
            test("./lox_test/limit/stack_overflow.lox");
        }

        #[ignore]
        #[test]
        fn too_many_constants() {
            test("./lox_test/limit/too_many_constants.lox");
        }

        #[ignore]
        #[test]
        fn too_many_upvalues() {
            test("./lox_test/limit/too_many_upvalues.lox");
        }

        #[ignore]
        #[test]
        fn loop_too_large() {
            test("./lox_test/limit/loop_too_large.lox");
        }

        #[ignore]
        #[test]
        fn too_many_locals() {
            test("./lox_test/limit/too_many_locals.lox");
        }
    }
    mod closure {
        use super::test;
        #[test]
        fn close_over_later_variable() {
            test("./lox_test/closure/close_over_later_variable.lox");
        }
        #[test]
        fn unused_later_closure() {
            test("./lox_test/closure/unused_later_closure.lox");
        }
        #[test]
        fn assign_to_closure() {
            test("./lox_test/closure/assign_to_closure.lox");
        }
        #[test]
        fn reference_closure_multiple_times() {
            test("./lox_test/closure/reference_closure_multiple_times.lox");
        }
        #[test]
        fn close_over_method_parameter() {
            test("./lox_test/closure/close_over_method_parameter.lox");
        }
        #[test]
        fn assign_to_shadowed_later() {
            test("./lox_test/closure/assign_to_shadowed_later.lox");
        }
        #[test]
        fn reuse_closure_slot() {
            test("./lox_test/closure/reuse_closure_slot.lox");
        }
        #[test]
        fn nested_closure() {
            test("./lox_test/closure/nested_closure.lox");
        }
        #[test]
        fn open_closure_in_function() {
            test("./lox_test/closure/open_closure_in_function.lox");
        }
        #[test]
        fn unused_closure() {
            test("./lox_test/closure/unused_closure.lox");
        }
        #[test]
        fn shadow_closure_with_local() {
            test("./lox_test/closure/shadow_closure_with_local.lox");
        }
        #[test]
        fn closed_closure_in_function() {
            test("./lox_test/closure/closed_closure_in_function.lox");
        }
        #[test]
        fn close_over_function_parameter() {
            test("./lox_test/closure/close_over_function_parameter.lox");
        }
    }
    mod nil {
        use super::test;
        #[test]
        fn literal() {
            test("./lox_test/nil/literal.lox");
        }
    }
    mod others {
        use super::test;
        #[test]
        fn precedence() {
            test("./lox_test/others/precedence.lox");
        }
        #[test]
        fn unexpected_character() {
            test("./lox_test/others/unexpected_character.lox");
        }
        #[test]
        fn empty_file() {
            test("./lox_test/others/empty_file.lox");
        }
    }
    mod while_keyword {
        use super::test;
        #[test]
        fn return_inside() {
            test("./lox_test/while/return_inside.lox");
        }
        #[test]
        fn closure_in_body() {
            test("./lox_test/while/closure_in_body.lox");
        }
        #[test]
        fn var_in_body() {
            test("./lox_test/while/var_in_body.lox");
        }
        #[test]
        fn class_in_body() {
            test("./lox_test/while/class_in_body.lox");
        }
        #[test]
        fn syntax() {
            test("./lox_test/while/syntax.lox");
        }
        #[test]
        fn return_closure() {
            test("./lox_test/while/return_closure.lox");
        }
        #[test]
        fn fun_in_body() {
            test("./lox_test/while/fun_in_body.lox");
        }
    }
    mod return_keyword {
        use super::test;
        #[test]
        fn after_while() {
            test("./lox_test/return/after_while.lox");
        }
        #[test]
        fn in_method() {
            test("./lox_test/return/in_method.lox");
        }
        #[test]
        fn return_nil_if_no_value() {
            test("./lox_test/return/return_nil_if_no_value.lox");
        }
        #[test]
        fn after_else() {
            test("./lox_test/return/after_else.lox");
        }
        #[test]
        fn at_top_level() {
            test("./lox_test/return/at_top_level.lox");
        }
        #[test]
        fn in_function() {
            test("./lox_test/return/in_function.lox");
        }
        #[test]
        fn after_if() {
            test("./lox_test/return/after_if.lox");
        }
    }
    mod super_keyword {
        use super::test;
        #[test]
        fn missing_arguments() {
            test("./lox_test/super/missing_arguments.lox");
        }
        #[test]
        fn call_same_method() {
            test("./lox_test/super/call_same_method.lox");
        }
        #[test]
        fn super_in_top_level_function() {
            test("./lox_test/super/super_in_top_level_function.lox");
        }
        #[test]
        fn this_in_superclass_method() {
            test("./lox_test/super/this_in_superclass_method.lox");
        }
        #[test]
        fn extra_arguments() {
            test("./lox_test/super/extra_arguments.lox");
        }
        #[test]
        fn super_in_inherited_method() {
            test("./lox_test/super/super_in_inherited_method.lox");
        }
        #[test]
        fn super_at_top_level() {
            test("./lox_test/super/super_at_top_level.lox");
        }
        #[test]
        fn no_superclass_call() {
            test("./lox_test/super/no_superclass_call.lox");
        }
        #[test]
        fn no_superclass_method() {
            test("./lox_test/super/no_superclass_method.lox");
        }
        #[test]
        fn parenthesized() {
            test("./lox_test/super/parenthesized.lox");
        }
        #[test]
        fn call_other_method() {
            test("./lox_test/super/call_other_method.lox");
        }
        #[test]
        fn indirectly_inherited() {
            test("./lox_test/super/indirectly_inherited.lox");
        }
        #[test]
        fn constructor() {
            test("./lox_test/super/constructor.lox");
        }
        #[test]
        fn super_without_name() {
            test("./lox_test/super/super_without_name.lox");
        }
        #[test]
        fn closure() {
            test("./lox_test/super/closure.lox");
        }
        #[test]
        fn super_in_closure_in_inherited_method() {
            test("./lox_test/super/super_in_closure_in_inherited_method.lox");
        }
        #[test]
        fn reassign_superclass() {
            test("./lox_test/super/reassign_superclass.lox");
        }
        #[test]
        fn no_superclass_bind() {
            test("./lox_test/super/no_superclass_bind.lox");
        }
        #[test]
        fn bound_method() {
            test("./lox_test/super/bound_method.lox");
        }
        #[test]
        fn super_without_dot() {
            test("./lox_test/super/super_without_dot.lox");
        }
    }
    mod logical_operator {
        use super::test;
        #[test]
        fn and_truth() {
            test("./lox_test/logical_operator/and_truth.lox");
        }
        #[test]
        fn or_truth() {
            test("./lox_test/logical_operator/or_truth.lox");
        }
        #[test]
        fn or() {
            test("./lox_test/logical_operator/or.lox");
        }
        #[test]
        fn and() {
            test("./lox_test/logical_operator/and.lox");
        }
    }
    mod variable {
        use super::test;
        #[test]
        fn scope_reuse_in_different_blocks() {
            test("./lox_test/variable/scope_reuse_in_different_blocks.lox");
        }
        #[test]
        fn shadow_local() {
            test("./lox_test/variable/shadow_local.lox");
        }
        #[test]
        fn redefine_global() {
            test("./lox_test/variable/redefine_global.lox");
        }
        #[test]
        fn use_local_in_initializer() {
            test("./lox_test/variable/use_local_in_initializer.lox");
        }
        #[test]
        fn undefined_local() {
            test("./lox_test/variable/undefined_local.lox");
        }
        #[test]
        fn duplicate_parameter() {
            test("./lox_test/variable/duplicate_parameter.lox");
        }
        #[test]
        fn redeclare_global() {
            test("./lox_test/variable/redeclare_global.lox");
        }
        #[test]
        fn duplicate_local() {
            test("./lox_test/variable/duplicate_local.lox");
        }
        #[test]
        fn use_this_as_var() {
            test("./lox_test/variable/use_this_as_var.lox");
        }
        #[test]
        fn shadow_global() {
            test("./lox_test/variable/shadow_global.lox");
        }
        #[test]
        fn early_bound() {
            test("./lox_test/variable/early_bound.lox");
        }
        #[test]
        fn in_middle_of_block() {
            test("./lox_test/variable/in_middle_of_block.lox");
        }
        #[test]
        fn local_from_method() {
            test("./lox_test/variable/local_from_method.lox");
        }
        #[test]
        fn collide_with_parameter() {
            test("./lox_test/variable/collide_with_parameter.lox");
        }
        #[test]
        fn uninitialized() {
            test("./lox_test/variable/uninitialized.lox");
        }
        #[test]
        fn use_global_in_initializer() {
            test("./lox_test/variable/use_global_in_initializer.lox");
        }
        #[test]
        fn unreached_undefined() {
            test("./lox_test/variable/unreached_undefined.lox");
        }
        #[test]
        fn use_nil_as_var() {
            test("./lox_test/variable/use_nil_as_var.lox");
        }
        #[test]
        fn in_nested_block() {
            test("./lox_test/variable/in_nested_block.lox");
        }
        #[test]
        fn shadow_and_local() {
            test("./lox_test/variable/shadow_and_local.lox");
        }
        #[test]
        fn use_false_as_var() {
            test("./lox_test/variable/use_false_as_var.lox");
        }
        #[test]
        fn undefined_global() {
            test("./lox_test/variable/undefined_global.lox");
        }
    }
    mod bool {
        use super::test;
        #[test]
        fn not() {
            test("./lox_test/bool/not.lox");
        }
        #[test]
        fn equality() {
            test("./lox_test/bool/equality.lox");
        }
    }
    mod if_keyword {
        use super::test;
        #[test]
        fn class_in_else() {
            test("./lox_test/if/class_in_else.lox");
        }
        #[test]
        fn var_in_then() {
            test("./lox_test/if/var_in_then.lox");
        }
        #[test]
        fn class_in_then() {
            test("./lox_test/if/class_in_then.lox");
        }
        #[test]
        fn else_keyword() {
            test("./lox_test/if/else.lox");
        }
        #[test]
        fn fun_in_else() {
            test("./lox_test/if/fun_in_else.lox");
        }
        #[test]
        fn dangling_else() {
            test("./lox_test/if/dangling_else.lox");
        }
        #[test]
        fn fun_in_then() {
            test("./lox_test/if/fun_in_then.lox");
        }
        #[test]
        fn truth() {
            test("./lox_test/if/truth.lox");
        }
        #[test]
        fn var_in_else() {
            test("./lox_test/if/var_in_else.lox");
        }
        #[test]
        fn if_keyword() {
            test("./lox_test/if/if.lox");
        }
    }
    mod print {
        use super::test;
        #[test]
        fn missing_argument() {
            test("./lox_test/print/missing_argument.lox");
        }
    }
    mod class {
        use super::test;
        #[test]
        fn empty() {
            test("./lox_test/class/empty.lox");
        }
        #[test]
        fn local_reference_self() {
            test("./lox_test/class/local_reference_self.lox");
        }
        #[test]
        fn reference_self() {
            test("./lox_test/class/reference_self.lox");
        }
        #[test]
        fn inherit_self() {
            test("./lox_test/class/inherit_self.lox");
        }
        #[test]
        fn inherited_method() {
            test("./lox_test/class/inherited_method.lox");
        }
        #[test]
        fn local_inherit_self() {
            test("./lox_test/class/local_inherit_self.lox");
        }
        #[test]
        fn local_inherit_other() {
            test("./lox_test/class/local_inherit_other.lox");
        }
    }
    mod block {
        use super::test;
        #[test]
        fn scope() {
            test("./lox_test/block/scope.lox");
        }
        #[test]
        fn empty() {
            test("./lox_test/block/empty.lox");
        }
    }
    mod call {
        use super::test;
        #[test]
        fn nil() {
            test("./lox_test/call/nil.lox");
        }
        #[test]
        fn string() {
            test("./lox_test/call/string.lox");
        }
        #[test]
        fn bool() {
            test("./lox_test/call/bool.lox");
        }
        #[test]
        fn object() {
            test("./lox_test/call/object.lox");
        }
        #[test]
        fn num() {
            test("./lox_test/call/num.lox");
        }
    }
    mod number {
        use super::test;
        #[test]
        fn leading_dot() {
            test("./lox_test/number/leading_dot.lox");
        }
        #[test]
        fn literals() {
            test("./lox_test/number/literals.lox");
        }
        #[test]
        fn decimal_point_at_eof() {
            test("./lox_test/number/decimal_point_at_eof.lox");
        }
        #[test]
        fn trailing_dot() {
            test("./lox_test/number/trailing_dot.lox");
        }
        #[test]
        fn nan_equality() {
            test("./lox_test/number/nan_equality.lox");
        }
    }
    mod continue_keyword {
        use super::test;
        #[test]
        fn at_top_level() {
            test("./lox_test_mine/continue/at_top_level.lox");
        }
        #[test]
        fn in_loop() {
            test("./lox_test_mine/continue/in_loop.lox");
        }
        #[test]
        fn outside_loop() {
            test("./lox_test_mine/continue/outside_loop.lox");
        }
    }

    mod break_keyword {
        use super::test;
        #[test]
        fn at_top_level() {
            test("./lox_test_mine/break/at_top_level.lox");
        }
        #[test]
        fn in_loop() {
            test("./lox_test_mine/break/in_loop.lox");
        }
        #[test]
        fn outside_loop() {
            test("./lox_test_mine/break/outside_loop.lox");
        }
    }

    enum Expect {
        Output(Vec<String>), RuntimeError(Vec<String>), ErrorAt, Nothing
    }

    fn expected_result(file_path: &str) -> Expect
    {
        let code = fs::read_to_string(file_path).unwrap();

        let expect = regex::Regex::new(r"// expect: ").expect("Errore nell'espressione regolare 'expect'");
        let error_at = regex::Regex::new(r" Error at ").expect("Errore nell'espressione regolare 'error_at'");
        let runtime_error = regex::Regex::new(r"// expect runtime error: ").expect("Errore nell'espressione regolare 'runtime_error'");

        let mut vec = Vec::<String>::new();
        for line in code.lines()
        {
            if expect.is_match(line)//let Some(captures) = expect.captures(&line)
            {
                let parts: Vec<&str> = line.split("// expect: ").collect();
                if let Some(result) = parts.last()
                {
                    vec.push((*result).to_owned());
                }
            }
            if error_at.is_match(line)
            {
                if !vec.is_empty() {
                    panic!("expected result got error_at");
                }
                return Expect::ErrorAt;
            }
            if runtime_error.is_match(line)
            {
                /*if !vec.is_empty() {
                    panic!("expected result got runtime_error");
                }*/
                return Expect::RuntimeError(vec);
            }
        }
        if vec.is_empty() {
            return Expect::Nothing;
        }

        return Expect::Output(vec);
    }

    fn test(file_path: &str)
    {
        let mut buf_output = Vec::<u8>::new();
        match expected_result(file_path)
        {
            Expect::Output(buf_expected) =>
            {
                run::run_file(file_path, &mut buf_output).expect(&format!("Expected test to be Ok (1) but got Err at file: '{}'", file_path));
                let lines = std::str::from_utf8(&buf_output).unwrap().lines();
                if buf_expected.is_empty() {
                    panic!("test buf_expected should not be empty");
                }
                if buf_output.is_empty() {
                    panic!("test buf_output should not be empty");
                }
                for (expected_value, actual_value) in buf_expected.iter().zip(lines)
                {
                    assert_eq!(expected_value, actual_value);
                }
            },
            Expect::RuntimeError(buf_expected) =>
            {
                run::run_file(file_path, &mut buf_output).expect_err(&format!("Expected test to be Err but got Ok at file: '{}'", file_path));
                let lines = std::str::from_utf8(&buf_output).unwrap().lines();
                for (expected_value, actual_value) in buf_expected.iter().zip(lines)
                {
                    assert_eq!(expected_value, actual_value);
                }
            },
            Expect::ErrorAt =>
            {
                run::run_file(file_path, &mut buf_output).expect_err(&format!("Expected test to be Err but got Ok at file: '{}'", file_path));
            },
            Expect::Nothing =>
            {
                run::run_file(file_path, &mut buf_output).expect(&format!("Expected test to be Ok (2) but got Err at file: '{}'", file_path));
            },
        }
    }
}
