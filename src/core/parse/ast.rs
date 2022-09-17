// TODO: remove this
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
    pub fn token_literals(&self) -> String {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            String::from("")
        }
    }
}

pub trait Node: Debug {
    fn token_literal(&self) -> String;
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Let(LetStatement),
    Return(Expression),
    Expression(Expression),
    If(IfStatement),
    // TODO: impl
    // switch
    // for
    // while
    // break
    // continue
    // block
}
impl Statement {
    pub fn statement_node(&self) -> String {
        todo!()
    }
}
impl Node for Statement {
    fn token_literal(&self) -> String {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LetStatement {
    pub name: String,
    pub value: Expression,
}
impl LetStatement {
    pub fn new(name: String, value: Expression) -> LetStatement {
        LetStatement { name, value }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Integer(i64),
    Identifier(String),
    Boolean(bool),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
}
impl Expression {
    pub fn expression_node(&self) -> String {
        todo!()
    }
}
impl Node for Expression {
    fn token_literal(&self) -> String {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PrefixExpression {
    pub operator: String,
    pub right: Box<Expression>,
}
impl PrefixExpression {
    pub fn new(operator: String, right: Box<Expression>) -> PrefixExpression {
        PrefixExpression { operator, right }
    }
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub struct IfStatement {
    pub condition: Expression,
    pub consequence: Vec<Statement>,
    pub alternative: Option<Vec<Statement>>,
}
impl IfStatement {
    pub fn new(
        condition: Expression,
        consequence: Vec<Statement>,
        alternative: Option<Vec<Statement>>,
    ) -> IfStatement {
        IfStatement {
            condition,
            consequence,
            alternative,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equals,      // == or !==
    LessGreater, // > or <
    Sum,         // + or -
    Product,     // * or /
    Prefix,      // -X or !X
    Call,        // myFunction(x)
}
