use std::rc::Rc;

use rustc_hash::FxHashMap;
use string_interner::StringInterner;

use crate::{parser_stmt::{Stmt, FunctionDeclaration}, parser_expr::{Expr, ExprKind}, common::Stack, error::{LoxError, ErrorLogger, ResolverErrorKind, ExecutionResult}, alias::{IdentifierSymbol, SideTable}, tokens::{Position, THIS}};

pub struct Resolver<'a>
{
    stack:            Stack<FxHashMap<IdentifierSymbol, bool>>,
    string_interner:  &'a StringInterner,
    has_error:        bool,
    current_function: FunctionType,
    current_class:    ClassType,
    this_symbol:      IdentifierSymbol,
    init_symbol:      IdentifierSymbol,
    error_logger:     Box<dyn ErrorLogger>,
}

impl <'a> Resolver<'a>
{
    pub fn new(error_logger: impl ErrorLogger + 'static, string_interner: &'a mut StringInterner) -> Self
    {
        let this_symbol = string_interner.get_or_intern_static(THIS);
        let init_symbol = string_interner.get_or_intern_static("init");
        Resolver
        {
            stack: Stack::new(),
            error_logger: Box::new(error_logger),
            string_interner,
            has_error: false,
            current_function: FunctionType::None,
            current_class: ClassType::None,
            this_symbol,
            init_symbol
        }
    }

    fn error(&mut self, err_kind: ResolverErrorKind, position: &Position)
    {
        self.error_logger.log(LoxError::resolver_error(err_kind, *position));
        self.has_error = true;
    }

    pub fn resolve(&mut self, stmts: &[Stmt]) -> Result<SideTable, ExecutionResult>
    {
        let mut side_table: SideTable = FxHashMap::default();
        for stmt in stmts
        {
            self.resolve_stmt(stmt, self.current_function, self.current_class, &mut side_table);
        }
        if self.has_error {
            Err(ExecutionResult::ResolverError)
        } else {
            Ok(side_table)
        }
    }

    fn resolve_stmt(&mut self, stmt: &Stmt, function_type: FunctionType, class_type: ClassType, side_table: &mut SideTable)
    {
        let enclosing_function = self.current_function;
        let enclosing_class = self.current_class;
        //> set-current-function
        self.current_function = function_type;
        self.current_class = class_type;
        //< set-current-function
        match stmt
        {
            Stmt::Print(print_expr) =>
            {
                self.resolve_expr(print_expr, side_table);
            },
            Stmt::Expr(expr) =>
            {
                self.resolve_expr(expr, side_table);
            },
            Stmt::Var(identifier, opt_expr) =>
            {
                if let Err(err_kind) = self.declare(identifier.name) {
                    self.error(err_kind, &identifier.position);
                }
                if let Some(expr) = opt_expr {
                    self.resolve_expr(expr, side_table);
                }
                self.define(identifier.name)
            },
            Stmt::Block(stmt_list) =>
            {
                self.begin_scope();
                for stmt in stmt_list {
                    self.resolve_stmt(stmt, self.current_function, self.current_class, side_table);
                }
                self.end_scope();
            },
            Stmt::If(if_stmt) =>
            {
                self.resolve_expr(&if_stmt.condition, side_table);
                self.resolve_stmt(&if_stmt.then_stmt, self.current_function, self.current_class, side_table);
            },
            Stmt::IfElse(if_else_stmt) =>
            {
                self.resolve_expr(&if_else_stmt.condition, side_table);
                self.resolve_stmt(&if_else_stmt.then_stmt, self.current_function, self.current_class, side_table);
                self.resolve_stmt(&if_else_stmt.else_stmt, self.current_function, self.current_class, side_table);
            },
            Stmt::While(while_stmt) =>
            {
                self.resolve_expr(&while_stmt.condition, side_table);
                self.resolve_stmt(&while_stmt.body, self.current_function, self.current_class, side_table);
            },
            Stmt::For(for_stmt) =>
            {
                if let Some(initializer) = for_stmt.opt_initializer.as_ref()
                {
                    self.resolve_stmt(initializer, self.current_function, self.current_class, side_table);
                }
                if let Some(condition) = &for_stmt.opt_condition
                {
                    self.resolve_expr(condition, side_table);
                }
                if let Some(increment) = &for_stmt.opt_increment
                {
                    self.resolve_expr(increment, side_table);
                }
                self.resolve_stmt(&for_stmt.body, self.current_function, self.current_class, side_table);
            },
            Stmt::Break     => { /*do nothing*/ },
            Stmt::Continue  => { /*do nothing*/ },
            Stmt::FunctionDeclaration(func_decl) =>
            {
                self.resolve_function(func_decl, FunctionType::Function, self.current_class, side_table);
            },
            Stmt::Return(opt_expr, position) =>
            {
                match self.current_function {
                    FunctionType::None => {
                        self.error(ResolverErrorKind::ReturnFromTopLevelCode, position)
                    },
                    FunctionType::Initializer => {
                        if opt_expr.is_some() {
                            self.error(ResolverErrorKind::ReturnFromInitializer, position)
                        }
                    },
                    _ => {
                        if let Some(expr) = opt_expr {
                            self.resolve_expr(expr, side_table);
                        }
                    }
                }
            },
            Stmt::ClassDeclaration(class_declaration) =>
            {
                //resolve class name
                if let Err(err_kind) = self.declare(class_declaration.identifier.name) {
                    self.error(err_kind, &class_declaration.identifier.position);
                }

                self.define(class_declaration.identifier.name);

                //>start THIS scope wrapping around methods declarations
                self.begin_scope();

                self.define(self.this_symbol);

                //>resolve methods
                let methods = &class_declaration.methods;
                for (_, rc_method) in methods.iter() {

                    let function_type = if rc_method.identifier.name == self.init_symbol {
                        FunctionType::Initializer
                    } else {
                        FunctionType::Method
                    };
                    self.resolve_function(rc_method, function_type, ClassType::Class, side_table);
                }
                //<resolve methods

                self.end_scope();
                //<end THIS scope wrapping around methods declarations
            },
        }
        //> restore-current-function
        self.current_function = enclosing_function;
        self.current_class = enclosing_class;
        //< restore-current-function
    }

    fn resolve_function(&mut self, func_decl: &Rc<FunctionDeclaration>, function_type: FunctionType, class_type: ClassType, side_table: &mut SideTable)
    {
        if let Err(err_kind) = self.declare(func_decl.identifier.name) {
            self.error(err_kind, &func_decl.identifier.position);
        }
        self.define(func_decl.identifier.name);
        self.begin_scope();
        for (index, param) in func_decl.parameters.iter().enumerate()
        {
            if let Err(err_kind) = self.declare(*param) {
                self.error(err_kind, &func_decl.positions[index]);
            }
            self.define(*param);
        }
        //>Inside a function stmt set current function to FunctionType::Function
        self.resolve_stmt(&func_decl.body, function_type, class_type, side_table);
        //<Inside a function stmt set current function to FunctionType::Function
        self.end_scope();
    }

    fn resolve_expr(&mut self, expr: &Expr, side_table: &mut SideTable)
    {
        match &expr.kind
        {
            ExprKind::Binary(binary) =>
            {
                self.resolve_expr(&binary.left,  side_table);
                self.resolve_expr(&binary.right, side_table);
            },
            ExprKind::Grouping(expr) =>
            {
                self.resolve_expr(expr, side_table);
            },
            ExprKind::Unary(unary_expr) =>
            {
                self.resolve_expr(&unary_expr.expr, side_table);
            },
            ExprKind::Literal(_) =>
            {
                /*do nothing*/
            },
            ExprKind::Variable(identifier) =>
            {
                if !self.stack.is_empty()
                {
                    let opt_bool =self.stack.peek().unwrap().get(&identifier.name);
                    if opt_bool.is_none() || !(*opt_bool.unwrap())
                    {
                        LoxError::resolver_error(crate::error::ResolverErrorKind::LocalVariableNotFound(self.string_interner.resolve(identifier.name).unwrap().to_owned()), identifier.position);
                    }
                }
                self.resolve_local(expr, identifier.name, side_table);
            },
            ExprKind::Assign(assign_expr) =>
            {
                self.resolve_expr(&assign_expr.expr, side_table);
                self.resolve_local(&assign_expr.expr, assign_expr.identifier.name, side_table);
            },
            ExprKind::Logical(logical_expr) =>
            {
                self.resolve_expr(&logical_expr.left, side_table);
                self.resolve_expr(&logical_expr.right, side_table);
            },
            ExprKind::Call(call_expr) =>
            {
                self.resolve_expr(&call_expr.callee, side_table);
                for arg in &call_expr.arguments
                {
                    self.resolve_expr(arg, side_table);
                }
            },
            ExprKind::Get(get_expr) =>
            {
                self.resolve_expr(&get_expr.expr, side_table);
            },
            ExprKind::Set(set_expr) =>
            {
                self.resolve_expr(&set_expr.target, side_table);
                self.resolve_expr(&set_expr.value,  side_table);
            },
            ExprKind::This(position) => {
                match self.current_class
                {
                    ClassType::None => {
                        self.error(ResolverErrorKind::InvalidThisUsage, position)
                    },
                    _ => {
                        self.resolve_local(expr, self.this_symbol, side_table);
                    }
                }
            },


        }
    }

    fn begin_scope(&mut self)
    {
        self.stack.push(FxHashMap::default());
    }

    fn end_scope(&mut self)
    {
        self.stack.pop();
    }

    fn declare(&mut self, identifier: IdentifierSymbol) -> Result<(), ResolverErrorKind>
    {
        if let Some(scope) = self.stack.peek_mut()
        {
            if scope.contains_key(&identifier)
            {
                return Err(ResolverErrorKind::VariableAlreadyExists(self.string_interner.resolve(identifier).unwrap().to_owned()));
            }
            scope.insert(identifier, false);
        }
        Ok(())
    }

    fn define(&mut self, identifier: IdentifierSymbol)
    {
        if let Some(last) = self.stack.peek_mut()
        {
            last.insert(identifier, true);
        }
    }

    fn resolve_local(&mut self, expr: &Expr, identifier: IdentifierSymbol, side_table: &mut SideTable)
    {
        for (index, scope) in self.stack.iter().enumerate().rev()
        {
            if scope.contains_key(&identifier)
            {
                side_table.insert(expr.id, index);
            }
        }
    }
}

#[derive(Clone, Debug, Copy)]
enum FunctionType
{
    None, Function, Method, Initializer
}

#[derive(Clone, Debug, Copy)]
enum ClassType
{
    None, Class
}