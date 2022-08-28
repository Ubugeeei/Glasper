// TODO: remove this
#![allow(dead_code)]

use super::token::Token;

/*
 *
 * interfaces
 *
 */

pub trait Node {
    fn token_literal(&self) -> String;
}

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

pub enum Expression {
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

/*
 *
 * conclete types
 *
 */

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

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}

pub struct Identifier {
    pub token: Token,
    pub value: String,
}
