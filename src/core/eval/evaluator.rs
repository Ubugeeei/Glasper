#![allow(dead_code)]

use std::io::Error;

use crate::core::{
    object::def::{GUndefined, Object},
    parse::ast::Program,
};

pub fn eval(_program: &Program) -> Result<Object, Error> {
    Ok(Object::Undefined(GUndefined))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{parse::parser::Parser, tokenize::lexer::Lexer};

    #[test]
    fn test_eval() {
        let mut l = Lexer::new("let a = 1;".to_string());
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            format!("{}", eval(&program).unwrap()),
            "\x1b[30mundefined\x1b[0m"
        );
    }
}
