// use super::token::Token;

// TODO: remove this
#![allow(dead_code)]

pub trait Node {
    fn token_literal(&self) -> String;
}

pub enum Statement {}
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

pub enum Expression {}
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
