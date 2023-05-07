#![allow(dead_code)]

use std::{cell::RefCell, collections::HashMap, io::Error, rc::Rc};

use crate::engine::{
    ast::{
        ArrayExpression, BlockStatement, CallExpression, ConstStatement, Expression, ForInit,
        ForStatement, IfStatement, LetStatement, MemberExpression, ObjectExpression, Program,
        Statement, SwitchStatement, UpdateExpression,
    },
    core::host::{
        api::Context,
        handles::{Variable, VariableKind},
        objects::{
            JSBoolean, JSFunction, JSNull, JSNumber, JSObject, JSString, JSUndefined, RuntimeObject,
        },
    },
    parsing::Parser,
};

pub mod api;
pub mod handles;
pub mod objects;

pub struct HostInterpreter<'a> {
    ctx: &'a mut Context,
    exec_ctx_this: Rc<RefCell<JSObject>>,
}
impl<'a> HostInterpreter<'a> {
    pub fn new(ctx: &'a mut Context) -> Self {
        // TODO: bind global object
        let global_obj = Rc::new(RefCell::new(JSObject {
            properties: HashMap::new(),
        }));

        HostInterpreter {
            ctx,
            exec_ctx_this: global_obj,
        }
    }
}

impl<'a> HostInterpreter<'a> {
    pub fn run(&mut self, source: String, parser: impl Parser) {
        let ast = parser.parse(source);
        match self.eval(&ast) {
            Ok(o) => println!("{}", o),
            Err(e) => println!("{}", e),
        }
    }

    fn eval(&mut self, program: &Program) -> Result<RuntimeObject, Error> {
        let mut result = RuntimeObject::Undefined(JSUndefined);
        for statement in &program.statements {
            result = self.eval_statement(statement, ScopeType::Block)?;

            if let RuntimeObject::Return(o) = result {
                return Ok(*o);
            };
        }
        Ok(result)
    }

    fn eval_statement(
        &mut self,
        statement: &Statement,
        scope_type: ScopeType,
    ) -> Result<RuntimeObject, Error> {
        match statement {
            Statement::Expression(expr) => self.eval_expression(expr),
            Statement::Let(stmt) => self.eval_let_statement(stmt),
            Statement::Const(stmt) => self.eval_const_statement(stmt),
            Statement::Block(stmt) => self.eval_block_statement(stmt, scope_type),
            Statement::If(stmt) => self.eval_if_statement(stmt, scope_type),
            Statement::Switch(stmt) => self.eval_switch_statement(stmt, scope_type),
            Statement::For(stmt) => self.eval_for_statement(stmt, scope_type),
            Statement::Return(expr) => self.eval_return_statement(expr, scope_type),
            Statement::Break => Ok(RuntimeObject::Break),
            Statement::Continue => Ok(RuntimeObject::Continue),
        }
    }

    fn eval_expression(&mut self, expr: &Expression) -> Result<RuntimeObject, Error> {
        match expr {
            // literals
            Expression::Boolean(b) => Ok(RuntimeObject::Boolean(JSBoolean { value: *b })),
            Expression::Number(i) => Ok(RuntimeObject::Number(JSNumber { value: *i })),
            Expression::String(s) => Ok(RuntimeObject::String(JSString { value: s.clone() })),
            Expression::Function(f) => Ok(RuntimeObject::Function(JSFunction::new(
                f.clone().parameters,
                f.clone().body,
            ))),
            Expression::Null => Ok(RuntimeObject::Null(JSNull)),
            Expression::Undefined => Ok(RuntimeObject::Undefined(JSUndefined)),
            Expression::This => Ok(RuntimeObject::Object(self.exec_ctx_this.clone())),

            // objects
            Expression::Object(o) => self.eval_object_expression(o),
            Expression::Member(m) => self.eval_member_expression(m),

            Expression::Array(a) => self.eval_array_expression(a),

            Expression::Identifier(name) => self.eval_identifier(name),

            // operators
            Expression::Unary(expr) => self.eval_unary_expression(expr),
            Expression::Binary(expr) => {
                if expr.operator == "=" {
                    self.eval_assign_expression(&expr.left, &expr.right)
                } else {
                    let left = self.eval_expression(&expr.left)?;
                    let right = self.eval_expression(&expr.right)?;
                    self.eval_binary_expression(expr.operator.clone(), left, right)
                }
            }

            // others
            Expression::Call(expr) => self.eval_call_expression(expr),
            Expression::Update(expr) => self.eval_update_expression(expr),
        }
    }

    fn eval_unary_expression(
        &mut self,
        expr: &crate::engine::ast::UnaryExpression,
    ) -> Result<RuntimeObject, Error> {
        let right = self.eval_expression(&expr.right)?;
        match expr.operator.as_str() {
            "!" => self.eval_bang_operator_expression(right),
            "-" => self.eval_minus_unary_operator_expression(right),
            "~" => self.eval_bit_not_operator_expression(right),
            "typeof" => self.eval_typeof_operator_expression(right),
            o => Err(Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Unexpected unary operator '{}'. at eval_unary_expression",
                    o
                ),
            )),
        }
    }

    fn eval_bang_operator_expression(&self, right: RuntimeObject) -> Result<RuntimeObject, Error> {
        Ok(RuntimeObject::Boolean(JSBoolean {
            value: !self.is_truthy(right),
        }))
    }

    fn eval_minus_unary_operator_expression(
        &self,
        right: RuntimeObject,
    ) -> Result<RuntimeObject, Error> {
        if let RuntimeObject::Number(JSNumber { value }) = right {
            Ok(RuntimeObject::Number(JSNumber { value: -value }))
        } else {
            Err(Error::new(
                std::io::ErrorKind::Other,
                "Unexpected unary operator. at eval_minus_unary_operator_expression",
            ))
        }
    }

    fn eval_bit_not_operator_expression(
        &self,
        right: RuntimeObject,
    ) -> Result<RuntimeObject, Error> {
        if let RuntimeObject::Number(JSNumber { value }) = right {
            Ok(RuntimeObject::Number(JSNumber {
                value: (!(value as i64)) as f64,
            }))
        } else {
            Err(Error::new(
                std::io::ErrorKind::Other,
                "Unexpected unary operator. at eval_bit_not_operator_expression",
            ))
        }
    }

    fn eval_typeof_operator_expression(
        &self,
        right: RuntimeObject,
    ) -> Result<RuntimeObject, Error> {
        Ok(RuntimeObject::String(JSString {
            value: right.get_type(),
        }))
    }

    fn eval_binary_expression(
        &self,
        operator: String,
        left: RuntimeObject,
        right: RuntimeObject,
    ) -> Result<RuntimeObject, Error> {
        match operator.as_str() {
            "+" => match (left.clone(), right.clone()) {
                (RuntimeObject::String(JSString { value }), _) => {
                    let r = match JSString::into(right) {
                        RuntimeObject::String(JSString { value: r }) => r,
                        _ => "".to_string(),
                    };
                    Ok(RuntimeObject::String(JSString {
                        value: format!("{}{}", value, r),
                    }))
                }
                (_, RuntimeObject::String(JSString { value })) => {
                    let l = match JSString::into(left) {
                        RuntimeObject::String(JSString { value: l }) => l,
                        _ => "".to_string(),
                    };
                    Ok(RuntimeObject::String(JSString {
                        value: format!("{}{}", l, value),
                    }))
                }
                _ => {
                    let l = JSNumber::into(left);
                    let r = JSNumber::into(right);
                    match (l, r) {
                        (RuntimeObject::Number(l), RuntimeObject::Number(r)) => {
                            Ok(RuntimeObject::Number(JSNumber {
                                value: l.value + r.value,
                            }))
                        }
                        _ => Ok(RuntimeObject::Number(JSNumber { value: f64::NAN })),
                    }
                }
            },
            "-" => {
                let l = JSNumber::into(left);
                let r = JSNumber::into(right);
                match (l, r) {
                    (RuntimeObject::Number(l), RuntimeObject::Number(r)) => {
                        Ok(RuntimeObject::Number(JSNumber {
                            value: l.value - r.value,
                        }))
                    }
                    _ => Ok(RuntimeObject::Number(JSNumber { value: f64::NAN })),
                }
            }
            "*" => {
                let l = JSNumber::into(left);
                let r = JSNumber::into(right);
                match (l, r) {
                    (RuntimeObject::Number(l), RuntimeObject::Number(r)) => {
                        Ok(RuntimeObject::Number(JSNumber {
                            value: l.value * r.value,
                        }))
                    }
                    _ => Ok(RuntimeObject::Number(JSNumber { value: f64::NAN })),
                }
            }
            "/" => {
                let l = JSNumber::into(left);
                let r = JSNumber::into(right);
                match (l, r) {
                    (RuntimeObject::Number(l), RuntimeObject::Number(r)) => {
                        Ok(RuntimeObject::Number(JSNumber {
                            value: l.value / r.value,
                        }))
                    }
                    _ => Ok(RuntimeObject::Number(JSNumber { value: f64::NAN })),
                }
            }
            "%" => {
                let l = JSNumber::into(left);
                let r = JSNumber::into(right);
                match (l, r) {
                    (RuntimeObject::Number(l), RuntimeObject::Number(r)) => {
                        Ok(RuntimeObject::Number(JSNumber {
                            value: l.value % r.value,
                        }))
                    }
                    _ => Ok(RuntimeObject::Number(JSNumber { value: f64::NAN })),
                }
            }
            "**" => {
                let l = JSNumber::into(left);
                let r = JSNumber::into(right);
                match (l, r) {
                    (RuntimeObject::Number(l), RuntimeObject::Number(r)) => {
                        Ok(RuntimeObject::Number(JSNumber {
                            value: l.value.powf(r.value),
                        }))
                    }
                    _ => Ok(RuntimeObject::Number(JSNumber { value: f64::NAN })),
                }
            }
            "|" => {
                let l = JSNumber::into(left);
                let r = JSNumber::into(right);
                match (l, r) {
                    (RuntimeObject::Number(l), RuntimeObject::Number(r)) => {
                        Ok(RuntimeObject::Number(JSNumber {
                            value: ((l.value as i64) | (r.value as i64)) as f64,
                        }))
                    }
                    _ => Ok(RuntimeObject::Number(JSNumber { value: f64::NAN })),
                }
            }
            "&" => {
                let l = JSNumber::into(left);
                let r = JSNumber::into(right);
                match (l, r) {
                    (RuntimeObject::Number(l), RuntimeObject::Number(r)) => {
                        Ok(RuntimeObject::Number(JSNumber {
                            value: ((l.value as i64) & (r.value as i64)) as f64,
                        }))
                    }
                    _ => Ok(RuntimeObject::Number(JSNumber { value: f64::NAN })),
                }
            }
            "<<" => match (left.clone(), right) {
                (RuntimeObject::Number(l), RuntimeObject::Number(r)) => {
                    Ok(RuntimeObject::Number(JSNumber {
                        value: ((l.value as i64) << (r.value as i64)) as f64,
                    }))
                }
                (RuntimeObject::Number(_), _) => Ok(left),
                _ => Ok(RuntimeObject::Number(JSNumber { value: 0.0 })),
            },
            ">>" => match (left.clone(), right) {
                (RuntimeObject::Number(l), RuntimeObject::Number(r)) => {
                    Ok(RuntimeObject::Number(JSNumber {
                        value: ((l.value as i64) >> (r.value as i64)) as f64,
                    }))
                }
                (RuntimeObject::Number(_), _) => Ok(left),
                _ => Ok(RuntimeObject::Number(JSNumber { value: 0.0 })),
            },
            // TODO: implement
            // ">>>" => ,
            "^" => {
                let l = JSNumber::into(left);
                let r = JSNumber::into(right);
                match (l, r) {
                    (RuntimeObject::Number(l), RuntimeObject::Number(r)) => {
                        Ok(RuntimeObject::Number(JSNumber {
                            value: (l.value as i64 ^ r.value as i64) as f64,
                        }))
                    }
                    _ => Ok(RuntimeObject::Number(JSNumber { value: f64::NAN })),
                }
            }
            "<" => {
                // TODO:
                let l = JSNumber::into(left);
                let r = JSNumber::into(right);
                match (l, r) {
                    (RuntimeObject::Number(l), RuntimeObject::Number(r)) => {
                        Ok(RuntimeObject::Boolean(JSBoolean {
                            value: l.value < r.value,
                        }))
                    }
                    _ => Ok(RuntimeObject::Number(JSNumber { value: f64::NAN })),
                }
            }
            ">" => {
                // TODO:
                let l = JSNumber::into(left);
                let r = JSNumber::into(right);
                match (l, r) {
                    (RuntimeObject::Number(l), RuntimeObject::Number(r)) => {
                        Ok(RuntimeObject::Boolean(JSBoolean {
                            value: l.value > r.value,
                        }))
                    }
                    _ => Ok(RuntimeObject::Number(JSNumber { value: f64::NAN })),
                }
            }
            "<=" => {
                // TODO:
                let l = JSNumber::into(left);
                let r = JSNumber::into(right);
                match (l, r) {
                    (RuntimeObject::Number(l), RuntimeObject::Number(r)) => {
                        Ok(RuntimeObject::Boolean(JSBoolean {
                            value: l.value <= r.value,
                        }))
                    }
                    _ => Ok(RuntimeObject::Number(JSNumber { value: f64::NAN })),
                }
            }
            ">=" => {
                // TODO:
                let l = JSNumber::into(left);
                let r = JSNumber::into(right);
                match (l, r) {
                    (RuntimeObject::Number(l), RuntimeObject::Number(r)) => {
                        Ok(RuntimeObject::Boolean(JSBoolean {
                            value: l.value >= r.value,
                        }))
                    }
                    _ => Ok(RuntimeObject::Number(JSNumber { value: f64::NAN })),
                }
            }
            "==" => {
                // TODO:
                let l = JSNumber::into(left);
                let r = JSNumber::into(right);
                match (l, r) {
                    (RuntimeObject::Number(l), RuntimeObject::Number(r)) => {
                        Ok(RuntimeObject::Boolean(JSBoolean {
                            value: l.value == r.value,
                        }))
                    }
                    _ => Ok(RuntimeObject::Number(JSNumber { value: f64::NAN })),
                }
            }
            "!=" => {
                // TODO:
                let l = JSNumber::into(left);
                let r = JSNumber::into(right);
                match (l, r) {
                    (RuntimeObject::Number(l), RuntimeObject::Number(r)) => {
                        Ok(RuntimeObject::Boolean(JSBoolean {
                            value: l.value != r.value,
                        }))
                    }
                    _ => Ok(RuntimeObject::Number(JSNumber { value: f64::NAN })),
                }
            }
            "===" => Ok(RuntimeObject::Boolean(JSBoolean {
                value: left == right,
            })),
            "!==" => Ok(RuntimeObject::Boolean(JSBoolean {
                value: left != right,
            })),

            // short-circuit evaluation
            "||" => {
                let l = JSBoolean::into(left.clone());
                match l {
                    RuntimeObject::Boolean(l) => {
                        if l.value {
                            Ok(left)
                        } else {
                            Ok(right)
                        }
                    }
                    _ => unreachable!("unreachable"),
                }
            }
            "&&" => {
                let l = JSBoolean::into(left.clone());
                let r = JSBoolean::into(right.clone());
                match (l, r) {
                    (RuntimeObject::Boolean(l), RuntimeObject::Boolean(r)) => {
                        if l.value && r.value {
                            Ok(right)
                        } else {
                            Ok(left)
                        }
                    }
                    _ => unreachable!("unreachable"),
                }
            }
            "??" => match left {
                RuntimeObject::Null(_) | RuntimeObject::Undefined(_) => Ok(right),
                _ => Ok(left),
            },

            _ => Err(Error::new(
                std::io::ErrorKind::Other,
                "Unexpected binary operator. at eval_binary_expression",
            )),
        }
    }

    fn eval_let_statement(&mut self, stmt: &LetStatement) -> Result<RuntimeObject, Error> {
        match self.ctx.scope.get(stmt.name.as_str()) {
            // validation
            Some(var) => match var.kind {
                VariableKind::Const => Err(Error::new(
                    std::io::ErrorKind::Other,
                    format!("Cannot reassign to const '{}'", stmt.name),
                )),
                VariableKind::Let | VariableKind::Var => {
                    let value = self.eval_expression(&stmt.value)?;
                    let var = Variable::new(VariableKind::Let, value);
                    self.ctx.scope.set(&stmt.name, var);
                    Ok(RuntimeObject::Undefined(JSUndefined))
                }
            },
            // initial set
            None => {
                let value = self.eval_expression(&stmt.value)?;
                let var = Variable::new(VariableKind::Let, value);
                self.ctx.scope.set(&stmt.name, var);
                Ok(RuntimeObject::Undefined(JSUndefined))
            }
        }
    }

    fn eval_const_statement(&mut self, stmt: &ConstStatement) -> Result<RuntimeObject, Error> {
        match self.ctx.scope.get(stmt.name.as_str()) {
            // validation
            Some(var) => match var.kind {
                VariableKind::Const => Err(Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "Uncaught SyntaxError: Identifier '{}' has already been declared",
                        stmt.name
                    ),
                )),
                VariableKind::Let | VariableKind::Var => {
                    let value = self.eval_expression(&stmt.value)?;
                    let var = Variable::new(VariableKind::Const, value);
                    self.ctx.scope.set(&stmt.name, var);
                    Ok(RuntimeObject::Undefined(JSUndefined))
                }
            },
            // initial set
            None => {
                let value = self.eval_expression(&stmt.value)?;
                let var = Variable::new(VariableKind::Const, value);
                self.ctx.scope.set(&stmt.name, var);
                Ok(RuntimeObject::Undefined(JSUndefined))
            }
        }
    }

    fn eval_identifier(&mut self, name: &str) -> Result<RuntimeObject, Error> {
        match self.ctx.scope.get(name) {
            Some(var) => Ok(var.value.clone()),
            None => match self.ctx.global().get(name) {
                Some(var) => Ok(var.clone()),
                None => Err(Error::new(
                    std::io::ErrorKind::Other,
                    format!("Uncaught ReferenceError: {} is not defined", name),
                )),
            },
        }
    }

    fn eval_object_expression(&mut self, obj: &ObjectExpression) -> Result<RuntimeObject, Error> {
        let mut properties = HashMap::new();
        for prop in &obj.properties {
            let key = prop.key.clone();
            let value = self.eval_expression(&prop.value)?;
            properties.insert(key, value);
        }
        Ok(RuntimeObject::Object(Rc::new(RefCell::new(JSObject {
            properties,
        }))))
    }

    fn eval_member_expression(&mut self, m: &MemberExpression) -> Result<RuntimeObject, Error> {
        let obj = self.eval_expression(&m.object)?;
        let prop = self.eval_expression(&m.property)?;

        match prop {
            RuntimeObject::String(s) => match obj {
                RuntimeObject::Object(o) => {
                    self.exec_ctx_this = o.clone();
                    Self::eval_property(o, s.value.as_ref())
                }
                _ => Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Uncaught SyntaxError: Invalid or unexpected token",
                )),
            },

            RuntimeObject::Number(n) => match obj {
                RuntimeObject::Object(o) => match o.borrow().properties.get(&n.value.to_string()) {
                    Some(v) => {
                        self.exec_ctx_this = o.clone();
                        Ok(v.clone())
                    }
                    None => Ok(RuntimeObject::Undefined(JSUndefined)),
                },
                _ => Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Uncaught SyntaxError: Invalid or unexpected token",
                )),
            },

            _ => Err(Error::new(
                std::io::ErrorKind::Other,
                "Uncaught SyntaxError: Invalid or unexpected token",
            )),
        }
    }

    fn eval_property(target_obj: Rc<RefCell<JSObject>>, key: &str) -> Result<RuntimeObject, Error> {
        let binding = target_obj.borrow();
        let p = binding.properties.get(key);
        match p {
            Some(v) => Ok(v.clone()),
            None => match target_obj.borrow().properties.get("__proto__") {
                Some(v) => match v {
                    RuntimeObject::Object(o) => Self::eval_property(o.clone(), key),
                    _ => unreachable!("unreachable"),
                },
                None => Ok(RuntimeObject::Undefined(JSUndefined)),
            },
        }
    }

    fn eval_array_expression(&mut self, arr: &ArrayExpression) -> Result<RuntimeObject, Error> {
        let mut properties = HashMap::new();
        for (i, e) in arr.elements.iter().enumerate() {
            let element = self.eval_expression(e)?;
            properties.insert(i.to_string(), element);
        }

        // set length
        properties.insert(
            "length".to_string(),
            RuntimeObject::Number(JSNumber {
                value: arr.elements.len() as f64,
            }),
        );

        // set prototype
        let prototype = match self.ctx.global().get("Array").unwrap() {
            RuntimeObject::Object(o) => o.borrow().properties.get("prototype").unwrap().clone(),
            _ => unreachable!("unreachable"),
        };
        properties.insert("__proto__".to_string(), prototype);

        Ok(RuntimeObject::Object(Rc::new(RefCell::new(JSObject {
            properties,
        }))))
    }

    fn eval_assign_expression(
        &mut self,
        left: &Expression,
        right: &Expression,
    ) -> Result<RuntimeObject, Error> {
        match left {
            Expression::Identifier(name) => {
                match self.ctx.scope.get(name.as_str()) {
                    Some(var) => match var.kind {
                        // validation
                        VariableKind::Const => Err(Error::new(
                            std::io::ErrorKind::Other,
                            "Uncaught TypeError: Assignment to constant variable.",
                        )),
                        // assign
                        VariableKind::Let => {
                            let value = self.eval_expression(right)?;
                            let var = Variable::new(VariableKind::Let, value.clone());
                            self.ctx.scope.assign(name, var);
                            Ok(value)
                        }
                        VariableKind::Var => {
                            let value = self.eval_expression(right)?;
                            let var = Variable::new(VariableKind::Var, value.clone());
                            self.ctx.scope.assign(name, var);
                            Ok(value)
                        }
                    },
                    // no var
                    None => {
                        let value = self.eval_expression(right)?;
                        let var = Variable::new(VariableKind::Var, value.clone());
                        self.ctx.scope.assign(name, var);
                        Ok(value)
                    }
                }
            }

            // object
            Expression::Member(m) => {
                let obj = self.eval_expression(&m.object)?;
                let prop = self.eval_expression(&m.property)?;
                let new_value = self.eval_expression(right)?;

                match prop {
                    RuntimeObject::String(s) => match obj {
                        RuntimeObject::Object(o) => {
                            let o_name = if let Expression::Identifier(name) = &m.object.as_ref() {
                                name.clone()
                            } else {
                                return Err(Error::new(
                                    std::io::ErrorKind::Other,
                                    "Uncaught SyntaxError: Invalid or unexpected token",
                                ));
                            };

                            let v = self.ctx.scope.get(&o_name);
                            match v {
                                Some(Variable { value, .. }) => {
                                    if let RuntimeObject::Object(o) = value.clone() {
                                        o.borrow_mut()
                                            .properties
                                            .insert(s.value.clone(), new_value.clone());
                                    }
                                }
                                None => {
                                    let v = self.ctx.global().get(&o_name);
                                    if let Some(RuntimeObject::Object(o)) = v {
                                        o.borrow_mut()
                                            .properties
                                            .insert(s.value.clone(), new_value.clone());
                                    }
                                }
                            }
                            o.borrow_mut().properties.insert(s.value, new_value.clone());
                            Ok(new_value)
                        }
                        _ => Err(Error::new(
                            std::io::ErrorKind::Other,
                            "Uncaught SyntaxError: Invalid or unexpected token",
                        )),
                    },
                    RuntimeObject::Number(n) => match obj {
                        RuntimeObject::Object(o) => {
                            let o_name = if let Expression::Identifier(name) = &m.object.as_ref() {
                                name.clone()
                            } else {
                                return Err(Error::new(
                                    std::io::ErrorKind::Other,
                                    "Uncaught SyntaxError: Invalid or unexpected token",
                                ));
                            };

                            let v = self.ctx.scope.get(&o_name);
                            match v {
                                Some(Variable { value, .. }) => {
                                    if let RuntimeObject::Object(o) = value.clone() {
                                        o.borrow_mut()
                                            .properties
                                            .insert(n.value.clone().to_string(), new_value.clone());
                                    }
                                }
                                None => {
                                    let v = self.ctx.global().get(&o_name);
                                    if let Some(RuntimeObject::Object(o)) = v {
                                        o.borrow_mut()
                                            .properties
                                            .insert(n.value.clone().to_string(), new_value.clone());
                                    }
                                }
                            }
                            o.borrow_mut()
                                .properties
                                .insert(n.value.to_string(), new_value.clone());
                            Ok(new_value)
                        }
                        _ => Err(Error::new(
                            std::io::ErrorKind::Other,
                            "Uncaught SyntaxError: Invalid or unexpected token",
                        )),
                    },
                    _ => Err(Error::new(
                        std::io::ErrorKind::Other,
                        "Uncaught SyntaxError: Invalid or unexpected token",
                    )),
                }
            }
            _ => Err(Error::new(
                std::io::ErrorKind::Other,
                "Uncaught SyntaxError: Invalid left-hand side in assignment",
            )),
        }
    }

    fn eval_if_statement(
        &mut self,
        statement: &IfStatement,
        scope_type: ScopeType,
    ) -> Result<RuntimeObject, Error> {
        let test = self.eval_expression(&statement.test)?;
        if self.is_truthy(test) {
            self.eval_statement(&statement.consequence, scope_type)
        } else {
            let un_boxed = statement.alternate.as_ref();
            match un_boxed {
                Some(ref alt) => self.eval_statement(alt, scope_type),
                None => Ok(RuntimeObject::Undefined(JSUndefined)),
            }
        }
    }

    fn eval_switch_statement(
        &mut self,
        statement: &SwitchStatement,
        scope_type: ScopeType,
    ) -> Result<RuntimeObject, Error> {
        let discriminant = self.eval_expression(&statement.discriminant)?;

        for case in &statement.cases {
            if let Some(ref test) = case.test {
                let test = self.eval_expression(test)?;
                if discriminant == test {
                    for s in &case.consequent {
                        let ro = self.eval_statement(s, scope_type)?;

                        if let RuntimeObject::Return(_) = ro {
                            return Ok(ro);
                        }

                        if let RuntimeObject::Break = ro {
                            return Ok(RuntimeObject::Undefined(JSUndefined));
                        }
                    }
                }
            } else {
                for s in &case.consequent {
                    let ro = self.eval_statement(s, scope_type)?;

                    if let RuntimeObject::Return(_) = ro {
                        return Ok(ro);
                    }

                    if let RuntimeObject::Break = ro {
                        return Ok(RuntimeObject::Undefined(JSUndefined));
                    }
                }
            }
        }

        Ok(RuntimeObject::Undefined(JSUndefined))
    }

    fn eval_for_statement(
        &mut self,
        statement: &ForStatement,
        scope_type: ScopeType,
    ) -> Result<RuntimeObject, Error> {
        self.ctx.scope.scope_in();

        if let Some(ref init) = statement.init {
            match init {
                ForInit::Expression(e) => {
                    self.eval_expression(e)?;
                }
                ForInit::Statement(s) => {
                    self.eval_statement(s, scope_type)?;
                }
            };
        }

        loop {
            if let Some(ref test) = statement.test {
                let test = self.eval_expression(test)?;
                if !self.is_truthy(test) {
                    break;
                }
            }

            let ro = self.eval_statement(&statement.body, scope_type)?;

            if let RuntimeObject::Return(_) = ro {
                return Ok(ro);
            }

            if let RuntimeObject::Break = ro {
                break;
            }

            if let Some(ref update) = statement.update {
                self.eval_expression(update)?;
            }
        }

        Ok(RuntimeObject::Undefined(JSUndefined))
    }

    fn eval_block_statement(
        &mut self,
        block: &BlockStatement,
        scope_type: ScopeType,
    ) -> Result<RuntimeObject, Error> {
        self.ctx.scope.scope_in();
        let mut result = RuntimeObject::Undefined(JSUndefined);
        for stmt in &block.statements {
            result = self.eval_statement(stmt, scope_type)?;

            match result {
                RuntimeObject::Return(_) | RuntimeObject::Break | RuntimeObject::Continue => {
                    self.ctx.scope.scope_out();
                    return Ok(result);
                }
                _ => {
                    continue;
                }
            }
        }

        self.ctx.scope.scope_out();
        if scope_type == ScopeType::Function {
            Ok(result)
        } else {
            Ok(RuntimeObject::Undefined(JSUndefined))
        }
    }

    fn eval_call_expression(&mut self, expr: &CallExpression) -> Result<RuntimeObject, Error> {
        let function = self.eval_expression(&expr.callee)?;
        let mut args = Vec::new();
        for arg in &expr.arguments {
            args.push(self.eval_expression(arg)?);
        }

        match function {
            RuntimeObject::BuiltinFunction(func) => {
                let func = func.func;
                Ok(func(args))
            }
            RuntimeObject::Function(func) => {
                self.ctx.scope.scope_in();
                for (i, param) in func.parameters.iter().enumerate() {
                    let name = param.clone().name;
                    let var = match param.default.clone() {
                        Some(v) => {
                            let value = self.eval_expression(&v)?;
                            Variable::new(VariableKind::Var, value)
                        }
                        None => {
                            Variable::new(VariableKind::Var, RuntimeObject::Undefined(JSUndefined))
                        }
                    };

                    // bind args
                    if let Some(a) = args.get(i) {
                        self.ctx
                            .scope
                            .set(&name, Variable::new(VariableKind::Var, a.clone()));
                    } else {
                        self.ctx.scope.set(&name, var);
                    }
                }

                let result = self.eval_block_statement(&func.body, ScopeType::Function)?;
                self.ctx.scope.scope_in();

                match result {
                    RuntimeObject::Return(ret) => Ok(*ret),
                    _ => Ok(RuntimeObject::Undefined(JSUndefined)),
                }
            }
            _ => Err(Error::new(
                std::io::ErrorKind::Other,
                "Uncaught TypeError: not a function",
            )),
        }
    }

    fn eval_update_expression(&mut self, expr: &UpdateExpression) -> Result<RuntimeObject, Error> {
        let left = self.eval_expression(&Expression::Identifier(expr.target_var_name.clone()))?;

        let right = match &*expr.operator {
            "++" => RuntimeObject::Number(JSNumber::new(1.0)),
            "--" => RuntimeObject::Number(JSNumber::new(-1.0)),
            _ => {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Uncaught SyntaxError: Unexpected token",
                ))
            }
        };

        let result = self.eval_binary_expression("+".to_string(), left, right)?;

        let v = self.ctx.scope.get(&expr.target_var_name);
        match v {
            Some(v) => match v.kind {
                VariableKind::Const => {
                    return Err(Error::new(
                        std::io::ErrorKind::Other,
                        "Uncaught TypeError: Assignment to constant variable.",
                    ))
                }
                _ => {
                    self.ctx.scope.set(
                        &expr.target_var_name,
                        Variable::new(VariableKind::Var, result.clone()),
                    );
                }
            },
            None => {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Uncaught ReferenceError: Cannot access 'a' before initialization",
                ))
            }
        }

        Ok(result)
    }

    fn eval_return_statement(
        &mut self,
        stmt: &Expression,
        scope_type: ScopeType,
    ) -> Result<RuntimeObject, Error> {
        if scope_type != ScopeType::Function {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Uncaught SyntaxError: Illegal return statement",
            ));
        }

        let value = self.eval_expression(stmt)?;
        Ok(RuntimeObject::Return(Box::new(value)))
    }

    fn is_truthy(&self, obj: RuntimeObject) -> bool {
        match obj {
            RuntimeObject::Boolean(b) => b.value,
            RuntimeObject::Number(n) => n.value != 0.0,
            RuntimeObject::String(s) => !s.value.is_empty(),
            RuntimeObject::Null(_) => false,
            RuntimeObject::Undefined(_) => false,
            _ => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScopeType {
    Block,
    Function,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{core::host::handles::HandleScope, parsing::BuiltinParser};

    #[test]
    fn test_eval_let_statement() {
        let program = BuiltinParser.parse("let a = 1;".to_string());
        let handle_scope = HandleScope::new();
        let mut context = Context::new(handle_scope);
        let mut ev = HostInterpreter::new(&mut context);
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            format!("{}", ev.eval(&program).unwrap()),
            "\x1b[30mundefined\x1b[0m"
        );
    }

    #[test]
    fn test_eval_int() {
        let program = BuiltinParser.parse("1".to_string());
        let handle_scope = HandleScope::new();
        let mut context = Context::new(handle_scope);
        let mut ev = HostInterpreter::new(&mut context);
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            format!("{}", ev.eval(&program).unwrap()),
            "\x1b[33m1\x1b[0m"
        );
    }

    #[test]
    fn test_eval_bool() {
        {
            let program = BuiltinParser.parse("true".to_string());
            let handle_scope = HandleScope::new();
            let mut context = Context::new(handle_scope);
            let mut ev = HostInterpreter::new(&mut context);
            assert_eq!(program.statements.len(), 1);
            assert_eq!(
                format!("{}", ev.eval(&program).unwrap()),
                "\x1b[33mtrue\x1b[0m"
            );
        }
        {
            let program = BuiltinParser.parse("false".to_string());
            let handle_scope = HandleScope::new();
            let mut context = Context::new(handle_scope);
            let mut ev = HostInterpreter::new(&mut context);
            assert_eq!(program.statements.len(), 1);
            assert_eq!(
                format!("{}", ev.eval(&program).unwrap()),
                "\x1b[33mfalse\x1b[0m"
            );
        }
    }

    #[test]
    fn test_eval_unary_expression() {
        let case = vec![
            ("!true", "\x1b[33mfalse\x1b[0m"),
            ("!false", "\x1b[33mtrue\x1b[0m"),
            ("!5", "\x1b[33mfalse\x1b[0m"),
            ("!!true", "\x1b[33mtrue\x1b[0m"),
            ("!!false", "\x1b[33mfalse\x1b[0m"),
            ("!!5", "\x1b[33mtrue\x1b[0m"),
            ("-5", "\x1b[33m-5\x1b[0m"),
            ("-10", "\x1b[33m-10\x1b[0m"),
            ("~10", "\x1b[33m-11\x1b[0m"), // 0b0101
            ("typeof 10", "\x1b[32m'number'\x1b[0m"),
            ("typeof !10", "\x1b[32m'boolean'\x1b[0m"),
        ];

        for (input, expected) in case {
            let program = BuiltinParser.parse(input.to_string());
            let handle_scope = HandleScope::new();
            let mut context = Context::new(handle_scope);
            let mut ev = HostInterpreter::new(&mut context);
            assert_eq!(program.statements.len(), 1);
            assert_eq!(format!("{}", ev.eval(&program).unwrap()), expected);
        }
    }

    #[test]
    fn test_eval_binary_expression() {
        let case = vec![
            ("1 + 1", "\x1b[33m2\x1b[0m"),
            ("1 - 1", "\x1b[33m0\x1b[0m"),
            ("1 * 1", "\x1b[33m1\x1b[0m"),
            ("1 / 1", "\x1b[33m1\x1b[0m"),
            ("5 % 5", "\x1b[33m0\x1b[0m"),
            ("5 % 2", "\x1b[33m1\x1b[0m"),
            ("2 + 3 * 4", "\x1b[33m14\x1b[0m"),
            ("2 * 3 + 4", "\x1b[33m10\x1b[0m"),
            ("2 * (3 + 4)", "\x1b[33m14\x1b[0m"),
            ("1 < 1", "\x1b[33mfalse\x1b[0m"),
            ("1 > 1", "\x1b[33mfalse\x1b[0m"),
            ("1 == 1", "\x1b[33mtrue\x1b[0m"),
            ("1 != 1", "\x1b[33mfalse\x1b[0m"),
            ("1 < 1", "\x1b[33mfalse\x1b[0m"),
            ("1 > 1", "\x1b[33mfalse\x1b[0m"),
            ("1 <= 1", "\x1b[33mtrue\x1b[0m"),
            ("1 >= 1", "\x1b[33mtrue\x1b[0m"),
            ("1 == 2", "\x1b[33mfalse\x1b[0m"),
            ("1 === 2", "\x1b[33mfalse\x1b[0m"),
            ("1 !== 2", "\x1b[33mtrue\x1b[0m"),
            ("1 != 2", "\x1b[33mtrue\x1b[0m"),
            ("true == true", "\x1b[33mtrue\x1b[0m"),
            ("true != true", "\x1b[33mfalse\x1b[0m"),
            ("true == false", "\x1b[33mfalse\x1b[0m"),
            ("true != false", "\x1b[33mtrue\x1b[0m"),
            ("false == false", "\x1b[33mtrue\x1b[0m"),
            ("false != false", "\x1b[33mfalse\x1b[0m"),
            ("false == true", "\x1b[33mfalse\x1b[0m"),
            ("false != true", "\x1b[33mtrue\x1b[0m"),
            ("2 ** 10", "\x1b[33m1024\x1b[0m"),
            ("null ?? 1", "\x1b[33m1\x1b[0m"),
            ("undefined ?? 1", "\x1b[33m1\x1b[0m"),
            ("1 ?? 2", "\x1b[33m1\x1b[0m"),
            ("false ?? true", "\x1b[33mfalse\x1b[0m"),
            ("null || 1", "\x1b[33m1\x1b[0m"),
            ("undefined || 1", "\x1b[33m1\x1b[0m"),
            ("1 || 2", "\x1b[33m1\x1b[0m"),
            ("0 || 2", "\x1b[33m2\x1b[0m"),
            ("false || true", "\x1b[33mtrue\x1b[0m"),
            ("null && 1", "null"),
            ("undefined && 1", "\x1b[30mundefined\x1b[0m"),
            ("1 && 2", "\x1b[33m2\x1b[0m"),
            ("0 && 2", "\x1b[33m0\x1b[0m"),
            ("false || true", "\x1b[33mtrue\x1b[0m"),
            ("0b1100 | 0b0011", "\x1b[33m15\x1b[0m"),
            ("0b1100 & 0b0011", "\x1b[33m0\x1b[0m"),
            ("0b1100 ^ 0b0011", "\x1b[33m15\x1b[0m"),
            ("0b0101 << 0b0010", "\x1b[33m20\x1b[0m"),
        ];

        for (input, expected) in case {
            let program = BuiltinParser.parse(input.to_string());
            let handle_scope = HandleScope::new();
            let mut context = Context::new(handle_scope);
            let mut ev = HostInterpreter::new(&mut context);
            assert_eq!(program.statements.len(), 1);
            assert_eq!(format!("{}", ev.eval(&program).unwrap()), expected);
        }
    }

    #[test]
    fn test_eval_variables() {
        let case = vec![
            ("let a = 1; a;", "\x1b[33m1\x1b[0m"),
            ("let a = 1; let b = a; b;", "\x1b[33m1\x1b[0m"),
            (
                "let a = 1; let b = a; let c = a + b + 5; c;",
                "\x1b[33m7\x1b[0m",
            ),
            ("let a = 1; a = 3;", "\x1b[33m3\x1b[0m"),
        ];

        for (input, expected) in case {
            let program = BuiltinParser.parse(input.to_string());
            let handle_scope = HandleScope::new();
            let mut context = Context::new(handle_scope);
            let mut ev = HostInterpreter::new(&mut context);
            assert_eq!(format!("{}", ev.eval(&program).unwrap()), expected);
        }
    }

    #[test]
    fn test_assign_var_validation() {
        // reassign to let variable
        {
            let program = BuiltinParser.parse("let a = 1; a = 2;".to_string());
            let handle_scope = HandleScope::new();
            let mut context = Context::new(handle_scope);
            let mut ev = HostInterpreter::new(&mut context);
            assert_eq!(
                format!("{}", ev.eval(&program).unwrap()),
                "\x1b[33m2\x1b[0m"
            );
        }

        // redeclare (let)
        {
            let program = BuiltinParser.parse("let a = 1; let a = 2; a;".to_string());
            let handle_scope = HandleScope::new();
            let mut context = Context::new(handle_scope);
            let mut ev = HostInterpreter::new(&mut context);
            assert_eq!(
                format!("{}", ev.eval(&program).unwrap()),
                "\x1b[33m2\x1b[0m"
            );
        }

        // redeclare (let -> const)
        {
            let program = BuiltinParser.parse("let a = 1; const a = 2; a;".to_string());
            let handle_scope = HandleScope::new();
            let mut context = Context::new(handle_scope);
            let mut ev = HostInterpreter::new(&mut context);
            assert_eq!(
                format!("{}", ev.eval(&program).unwrap()),
                "\x1b[33m2\x1b[0m"
            );
        }

        // reassign to const variable
        {
            let program = BuiltinParser.parse("const a = 1; a = 2;".to_string());
            let handle_scope = HandleScope::new();
            let mut context = Context::new(handle_scope);
            let mut ev = HostInterpreter::new(&mut context);
            ev.eval(&program).unwrap_err();
        }

        // redeclare (const -> const)
        {
            let program = BuiltinParser.parse("const a = 1; const a = 2; a;".to_string());
            let handle_scope = HandleScope::new();
            let mut context = Context::new(handle_scope);
            let mut ev = HostInterpreter::new(&mut context);
            ev.eval(&program).unwrap_err();
        }

        // redeclare (const -> let)
        {
            let program = BuiltinParser.parse("const a = 1; let a = 2; a;".to_string());
            let handle_scope = HandleScope::new();
            let mut context = Context::new(handle_scope);
            let mut ev = HostInterpreter::new(&mut context);
            ev.eval(&program).unwrap_err();
        }
    }

    #[test]
    fn test_eval_if_statement() {
        {
            let case = vec![
                (
                    String::from(
                        r#"
                            let a = 5;
                            if (a % 2 == 0) {
                                a = 0;
                            } else {
                                a = 1;
                            }
                            a;
                        "#,
                    ),
                    "\x1b[33m1\x1b[0m",
                ),
                (
                    String::from(
                        r#"
                            let a = 5;
                            if (a % 2 == 0) {
                                a = 0;
                            }
                            a;
                        "#,
                    ),
                    "\x1b[33m5\x1b[0m",
                ),
                (
                    String::from(
                        r#"
                            let a = 6;
                            if (a % 2 == 0) {
                                a = 0;
                            } else {
                                a = 1;
                            }
                            a;
                        "#,
                    ),
                    "\x1b[33m0\x1b[0m",
                ),
                (
                    String::from(
                        r#"
                            let a = 3;
                            if (a % 3 == 0) {
                                a = 0;
                            } else if (a % 3 == 1) {
                                a = 1;
                            } else {
                                a = 2;
                            }
                            a;
                        "#,
                    ),
                    "\x1b[33m0\x1b[0m",
                ),
                (
                    String::from(
                        r#"
                            let a = 4;
                            if (a % 3 == 0) {
                                a = 0;
                            } else if (a % 3 == 1) {
                                a = 1;
                            } else {
                                a = 2;
                            }
                            a;
                        "#,
                    ),
                    "\x1b[33m1\x1b[0m",
                ),
                (
                    String::from(
                        r#"
                            let a = 5;
                            if (a % 3 == 0) {
                                a = 0;
                            } else if (a % 3 == 1) {
                                a = 1;
                            } else {
                                a = 2;
                            }
                            a;
                        "#,
                    ),
                    "\x1b[33m2\x1b[0m",
                ),
                // scope
                (
                    String::from(
                        r#"
                            let a = 5;
                            if (a % 2 == 0) {
                                a = 0;

                            } else {
                                a = 1;
                                let a = 99;
                                a = 100;
                            }
                            a;
                        "#,
                    ),
                    "\x1b[33m1\x1b[0m",
                ),
            ];

            for (input, expected) in case {
                let program = BuiltinParser.parse(input.to_string());
                let handle_scope = HandleScope::new();
                let mut context = Context::new(handle_scope);
                let mut ev = HostInterpreter::new(&mut context);
                assert_eq!(format!("{}", ev.eval(&program).unwrap()), expected);
            }
        }
    }

    #[test]
    fn test_eval_function() {
        {
            let case = vec![
                (
                    String::from(
                        r#"
                            let a = 5;
                            let assign = function() {
                                a = 100;
                            };
                            assign();
                            a;
                    "#,
                    ),
                    "\x1b[33m100\x1b[0m",
                ),
                (
                    String::from(
                        r#"
                            
                            let add = function(a, b, c) {
                                return a + b + c;
                            };
                            add(1, 2, 3);
                    "#,
                    ),
                    "\x1b[33m6\x1b[0m",
                ),
                // default args
                (
                    String::from(
                        r#"
                            
                            let ret = function(a = 2, b = 1 + 2 * 3) {
                                return a + b;
                            };
                            ret();
                    "#,
                    ),
                    "\x1b[33m9\x1b[0m",
                ),
                // scope
                (
                    String::from(
                        r#"
                            let a = 1;
                            let ret = function(a) {
                                return a;
                            };
                            ret(100);
                    "#,
                    ),
                    "\x1b[33m100\x1b[0m",
                ),
                (
                    String::from(
                        r#"
                            let a = 1;
                            let ret = function() {
                                return a;
                            };
                            ret();
                    "#,
                    ),
                    "\x1b[33m1\x1b[0m",
                ),
            ];

            for (input, expected) in case {
                let program = BuiltinParser.parse(input.to_string());
                let handle_scope = HandleScope::new();
                let mut context = Context::new(handle_scope);
                let mut ev = HostInterpreter::new(&mut context);
                assert_eq!(format!("{}", ev.eval(&program).unwrap()), expected);
            }
        }
    }

    #[test]
    fn tets_eval_object() {
        let case = vec![
            (
                String::from(
                    r#"
                        let a = {
                            b: 1,
                            c: 2,
                            d: 3,
                        };
                        a;
                    "#,
                ),
                "\x1b[34m[Object]\x1b[0m",
            ),
            (
                String::from(
                    r#"
                        let a = {
                            b: 1,
                            c: 2,
                            d: 3,
                        };
                        a.b;
                    "#,
                ),
                "\x1b[33m1\x1b[0m",
            ),
            (
                String::from(
                    r#"
                        let a = {
                            b: 1,
                            c: 2,
                            d: 3,
                        };
                        a.e;
                    "#,
                ),
                "\x1b[30mundefined\x1b[0m",
            ),
        ];

        for (input, expected) in case {
            let program = BuiltinParser.parse(input.to_string());
            let handle_scope = HandleScope::new();
            let mut context = Context::new(handle_scope);
            let mut ev = HostInterpreter::new(&mut context);
            assert_eq!(format!("{}", ev.eval(&program).unwrap()), expected);
        }
    }

    #[test]
    fn test_eval_array() {
        let case = vec![
            (
                String::from(
                    r#"
                        let a = [1, 2, 3];
                        a.length;
                    "#,
                ),
                "\x1b[33m3\x1b[0m",
            ),
            (
                String::from(
                    r#"
                        let a = [1, 2, 3];
                        a[0];
                    "#,
                ),
                "\x1b[33m1\x1b[0m",
            ),
            (
                String::from(
                    r#"
                        let a = [1, 2, 3];
                        a[3];
                    "#,
                ),
                "\x1b[30mundefined\x1b[0m",
            ),
        ];

        for (input, expected) in case {
            let program = BuiltinParser.parse(input.to_string());
            let handle_scope = HandleScope::new();
            let mut context = Context::new(handle_scope);
            let mut ev = HostInterpreter::new(&mut context);
            assert_eq!(format!("{}", ev.eval(&program).unwrap()), expected);
        }
    }

    #[test]
    fn eval_switch_statement() {
        let case = vec![(
            r#"
                const f = function(a) {
                    switch (a) {
                        case 1:
                            return 1;
                        case 2:
                            return 2;
                        default:
                            return 3;
                    }
                };

                f(2);
            "#
            .to_string(),
            "\x1b[33m2\x1b[0m",
        )];

        for (input, expected) in case {
            let program = BuiltinParser.parse(input.to_string());
            let handle_scope = HandleScope::new();
            let mut context = Context::new(handle_scope);
            let mut ev = HostInterpreter::new(&mut context);
            assert_eq!(format!("{}", ev.eval(&program).unwrap()), expected);
        }
    }

    #[test]
    fn eval_function_this() {
        let case = vec![(
            r#"
                const a = {
                    b: 1,
                    c: function() {
                        return this.b;
                    }
                };

                a.c();
            "#
            .to_string(),
            "\x1b[33m1\x1b[0m",
        )];

        for (input, expected) in case {
            let program = BuiltinParser.parse(input.to_string());
            let handle_scope = HandleScope::new();
            let mut context = Context::new(handle_scope);
            let mut ev = HostInterpreter::new(&mut context);
            assert_eq!(format!("{}", ev.eval(&program).unwrap()), expected);
        }
    }

    #[test]
    fn eval_for_statement() {
        let case = vec![(
            r#"
                let a = 0;
                for (let i = 0; i < 10; i = i + 1) {
                    a = a + 1;
                }
                a;
            "#
            .to_string(),
            "\x1b[33m10\x1b[0m",
        )];

        for (input, expected) in case {
            let program = BuiltinParser.parse(input.to_string());
            let handle_scope = HandleScope::new();
            let mut context = Context::new(handle_scope);
            let mut ev = HostInterpreter::new(&mut context);
            assert_eq!(format!("{}", ev.eval(&program).unwrap()), expected);
        }
    }

    #[test]
    fn eval_update_ops() {
        let case = vec![(
            r#"
                let i = 0;
                i++;
                i++;
                i--;
            "#
            .to_string(),
            "\x1b[33m1\x1b[0m",
        )];

        for (input, expected) in case {
            let program = BuiltinParser.parse(input.to_string());
            let handle_scope = HandleScope::new();
            let mut context = Context::new(handle_scope);
            let mut ev = HostInterpreter::new(&mut context);
            assert_eq!(format!("{}", ev.eval(&program).unwrap()), expected);
        }
    }
}
