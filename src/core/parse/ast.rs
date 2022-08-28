// TODO: remove this
#![allow(dead_code)]

use crate::core::tokenize::token::Token;

#[derive(Default)]
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

pub trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Debug)]
pub enum Statement {
    LetStatement(LetStatement),
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

#[derive(Debug)]
pub enum Expression {
    Integer(i32),
    Identifier(Identifier),
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

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}
impl LetStatement {
    pub fn new(token: Token, name: Identifier, value: Expression) -> LetStatement {
        LetStatement { token, name, value }
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}
impl Identifier {
    pub fn new(token: Token, value: String) -> Identifier {
        Identifier { token, value }
    }
}
