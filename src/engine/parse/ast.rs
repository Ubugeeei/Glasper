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
    // TODO: impl
    // switch
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
    pub condition: Expression,
    pub consequence: Box<Statement>,
    pub alternative: Box<Option<Statement>>,
}
impl IfStatement {
    pub fn new(
        condition: Expression,
        consequence: Box<Statement>,
        alternative: Box<Option<Statement>>,
    ) -> IfStatement {
        IfStatement {
            condition,
            consequence,
            alternative,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Number(f64),
    Boolean(bool),
    String(String),
    Object(ObjectExpression),
    Null,
    Undefined,
    NaN,
    Identifier(String),
    Prefix(PrefixExpression),
    Suffix(SuffixExpression),
    Infix(InfixExpression),
    Function(FunctionExpression),
    Call(CallExpression),
    Member(Box<MemberExpression>),
}
impl Expression {}

#[derive(Debug, PartialEq, Clone)]
pub struct PrefixExpression {
    pub operator: String,
    pub right: Box<Expression>,
}
impl PrefixExpression {
    pub fn new(operator: String, right: Box<Expression>) -> PrefixExpression {
        PrefixExpression { operator, right }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SuffixExpression {
    pub operator: String,
    pub target_var_name: String,
}
impl SuffixExpression {
    pub fn new(operator: String, target_var_name: String) -> SuffixExpression {
        SuffixExpression {
            operator,
            target_var_name,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InfixExpression {
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}
impl InfixExpression {
    pub fn new(left: Box<Expression>, operator: String, right: Box<Expression>) -> InfixExpression {
        InfixExpression {
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
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
}
impl CallExpression {
    pub fn new(function: Box<Expression>, arguments: Vec<Expression>) -> CallExpression {
        CallExpression {
            function,
            arguments,
        }
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
    Prefix,            // -X or !X
    Index,
    Call, // myFunction(x)
}
