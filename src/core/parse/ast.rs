// TODO: remove this
#![allow(dead_code)]

use std::fmt::Debug;

use crate::core::tokenize::token::Token;

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
    // TODO: impl
    // if
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
    pub token: Token,
    pub name: String,
    pub value: Expression,
}
impl LetStatement {
    pub fn new(token: Token, name: String, value: Expression) -> LetStatement {
        LetStatement { token, name, value }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Integer(i64),
    Identifier(String),
    Prefix(PrefixExpression),
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

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(x)
}
