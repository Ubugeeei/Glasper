#![allow(dead_code)]

use std::fmt::Debug;

#[derive(Default, Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}
impl Program {
    pub fn new() -> Program {
        Program {
            statements: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Let(LetStatement),
    Const(ConstStatement),
    Return(Expression),
    Expression(Expression),
    If(IfStatement),
    Block(BlockStatement),
    Switch(SwitchStatement),
    // TODO: impl
    // for
    // while
    // break
    // continue
    // block
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}
impl BlockStatement {
    pub fn new(statements: Vec<Statement>) -> BlockStatement {
        BlockStatement { statements }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LetStatement {
    pub name: String,
    pub value: Expression,
}
impl LetStatement {
    pub fn new(name: String, value: Expression) -> LetStatement {
        LetStatement { name, value }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConstStatement {
    pub name: String,
    pub value: Expression,
}
impl ConstStatement {
    pub fn new(name: String, value: Expression) -> ConstStatement {
        ConstStatement { name, value }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfStatement {
    pub test: Expression,
    pub consequence: Box<Statement>,
    pub alternate: Box<Option<Statement>>,
}
impl IfStatement {
    pub fn new(
        test: Expression,
        consequence: Box<Statement>,
        alternate: Box<Option<Statement>>,
    ) -> IfStatement {
        IfStatement {
            test,
            consequence,
            alternate,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SwitchStatement {
    pub discriminant: Expression,
    pub cases: Vec<SwitchCase>,
}
impl SwitchStatement {
    pub fn new(discriminant: Expression, cases: Vec<SwitchCase>) -> SwitchStatement {
        SwitchStatement {
            discriminant,
            cases,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SwitchCase {
    pub test: Option<Expression>,
    pub consequent: Vec<Statement>,
}
impl SwitchCase {
    pub fn new(test: Option<Expression>, consequent: Vec<Statement>) -> SwitchCase {
        SwitchCase { test, consequent }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Number(f64),
    Boolean(bool),
    String(String),
    Object(ObjectExpression),
    Array(ArrayExpression),
    Null,
    Undefined,
    NaN,
    This,
    Identifier(String),
    Unary(UnaryExpression),
    Update(UpdateExpression),
    Binary(BinaryExpression),
    Function(FunctionExpression),
    Call(CallExpression),
    Member(Box<MemberExpression>),
}
impl Expression {}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpression {
    pub operator: String,
    pub right: Box<Expression>,
}
impl UnaryExpression {
    pub fn new(operator: String, right: Box<Expression>) -> UnaryExpression {
        UnaryExpression { operator, right }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UpdateExpression {
    pub operator: String,
    pub target_var_name: String,
}
impl UpdateExpression {
    pub fn new(operator: String, target_var_name: String) -> UpdateExpression {
        UpdateExpression {
            operator,
            target_var_name,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}
impl BinaryExpression {
    pub fn new(
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    ) -> BinaryExpression {
        BinaryExpression {
            left,
            operator,
            right,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionExpression {
    pub parameters: Vec<FunctionParameter>,
    pub body: BlockStatement,
}
impl FunctionExpression {
    pub fn new(parameters: Vec<FunctionParameter>, body: BlockStatement) -> FunctionExpression {
        FunctionExpression { parameters, body }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionParameter {
    pub name: String,
    pub default: Option<Expression>,
}
impl FunctionParameter {
    pub fn new(name: String, default: Option<Expression>) -> FunctionParameter {
        FunctionParameter { name, default }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpression {
    pub callee: Box<Expression>,
    pub arguments: Vec<Expression>,
}
impl CallExpression {
    pub fn new(callee: Box<Expression>, arguments: Vec<Expression>) -> CallExpression {
        CallExpression { callee, arguments }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectExpression {
    pub properties: Vec<ObjectProperty>,
}
impl ObjectExpression {
    pub fn new(properties: Vec<ObjectProperty>) -> ObjectExpression {
        ObjectExpression { properties }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectProperty {
    pub key: String,
    pub value: Expression,
}
impl ObjectProperty {
    pub fn new(key: String, value: Expression) -> ObjectProperty {
        ObjectProperty { key, value }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MemberExpression {
    pub object: Box<Expression>,
    pub property: Box<Expression>,
}
impl MemberExpression {
    pub fn new(object: Box<Expression>, property: Box<Expression>) -> MemberExpression {
        MemberExpression { object, property }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayExpression {
    pub elements: Vec<Expression>,
}
impl ArrayExpression {
    pub fn new(elements: Vec<Expression>) -> ArrayExpression {
        ArrayExpression { elements }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Assign,
    Equals,            // == or !==
    NullishCoalescing, // ??
    Bool,              // && or ||
    LessGreater,       // > or <
    Shift,             // << or >> or >>>
    Sum,               // + or -
    Product,           // * or /
    Exp,               // **
    Unary,             // -X or !X
    Index,
    Call, // myFunction(x)
}
