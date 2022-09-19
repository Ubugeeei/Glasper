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

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    Const(ConstStatement),
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

#[derive(Debug, PartialEq)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}
impl BlockStatement {
    pub fn new(statements: Vec<Statement>) -> BlockStatement {
        BlockStatement { statements }
    }
}

#[derive(Debug, PartialEq)]
pub struct LetStatement {
    pub name: String,
    pub value: Expression,
}
impl LetStatement {
    pub fn new(name: String, value: Expression) -> LetStatement {
        LetStatement { name, value }
    }
}

#[derive(Debug, PartialEq)]
pub struct ConstStatement {
    pub name: String,
    pub value: Expression,
}
impl ConstStatement {
    pub fn new(name: String, value: Expression) -> ConstStatement {
        ConstStatement { name, value }
    }
}

#[derive(Debug, PartialEq)]
pub struct IfStatement {
    pub condition: Expression,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}
impl IfStatement {
    pub fn new(
        condition: Expression,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    ) -> IfStatement {
        IfStatement {
            condition,
            consequence,
            alternative,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(f64),
    Boolean(bool),
    Null,
    Undefined,
    Identifier(String),
    Prefix(PrefixExpression),
    Suffix(SuffixExpression),
    Infix(InfixExpression),
    Function(FunctionExpression),
    Call(CallExpression),
}
impl Expression {}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct FunctionExpression {
    pub parameters: Vec<FunctionParameter>,
    pub body: BlockStatement,
}
impl FunctionExpression {
    pub fn new(parameters: Vec<FunctionParameter>, body: BlockStatement) -> FunctionExpression {
        FunctionExpression { parameters, body }
    }
}

#[derive(Debug, PartialEq)]
pub struct FunctionParameter {
    pub name: String,
    pub default: Option<Expression>,
}
impl FunctionParameter {
    pub fn new(name: String, default: Option<Expression>) -> FunctionParameter {
        FunctionParameter { name, default }
    }
}

#[derive(Debug, PartialEq)]
pub struct CallExpression {
    function: Box<Expression>,
    arguments: Vec<Expression>,
}
impl CallExpression {
    pub fn new(function: Box<Expression>, arguments: Vec<Expression>) -> CallExpression {
        CallExpression {
            function,
            arguments,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equals,            // == or !==
    NullishCoalescing, // ??
    LessGreater,       // > or <
    Sum,               // + or -
    Product,           // * or /
    Prefix,            // -X or !X
    Call,              // myFunction(x)
}
