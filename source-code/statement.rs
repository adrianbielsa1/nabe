use crate::Token;

// TODO: `Variable` and `Argument` variant's kind should be `Option<Token>`
// because VB6 treats untyped variables as if they were declared with the
// `Variant` data type. So should be `scope`s too.
#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    // TODO: Needed?
    Program,

    Type {
        name: Token,
        attributes: Vec<Statement>,
    },

    TypeAttribute {
        name: Token,
        kind: Token,
    },

    Variable {
        scope: Token,
        name: Token,
        kind: Token,
    },

    Constant {
        scope: Token,
        name: Token,
        kind: Option<Token>,
        value: Token,
    },

    Subroutine {
        scope: Token,
        name: Token,
        arguments: Vec<Statement>,
        body: Vec<Statement>,
    },

    Function {
        scope: Token,
        name: Token,
        arguments: Vec<Statement>,
        kind: Option<Token>,
        body: Vec<Statement>,
    },

    Argument {
        name: Token,
        kind: Token,
    },
}
