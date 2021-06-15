use crate::token::Token;

// TODO: `Variable` and `Argument` variant's kind should be `Option<Token>`
// because VB6 treats untyped variables as if they were declared with the
// `Variant` data type. So should be `scope`s too.
#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    // TODO: Needed?
    Program,

    // NOTE: The statement's information is often encapsulated
    // into structs because destructuring the variant while
    // pattern matching becomes really cumbersome when there are
    // multiple attributes to destructure.
    Type(TypeStatement),
    TypeAttribute(TypeAttributeStatement),
    Enum(EnumStatement),
    EnumAttribute(EnumAttributeStatement),
    Variable(VariableStatement),
    Constant(ConstantStatement),
    Subroutine(SubroutineStatement),
    Function(FunctionStatement),
    Exit(ExitStatement),
    Argument(ArgumentStatement),
    Assignment(AssignmentStatement),
    Return(ReturnStatement),
    Option(OptionStatement),
    Attribute(AttributeStatement),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeStatement {
    pub name: Token,
    pub attributes: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeAttributeStatement {
    pub name: Token,
    pub kind: Token, // TODO: Should this be Option<Token>?
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnumStatement {
    pub scope: Option<Token>,
    pub name: Token,
    pub attributes: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnumAttributeStatement {
    pub name: Token,
    pub value: Option<Token>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VariableStatement {
    pub scope: Token,
    pub name: Token,
    pub kind: Token,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantStatement {
    pub scope: Token,
    pub name: Token,
    pub kind: Option<Token>,
    pub length: Option<Token>,
    pub value: Token,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SubroutineStatement {
    pub scope: Token,
    pub name: Token,
    pub arguments: Vec<Statement>,
    pub body: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionStatement {
    pub scope: Token,
    pub name: Token,
    pub arguments: Vec<Statement>,
    pub kind: Option<Token>,
    pub body: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExitStatement {
    pub block: Token,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArgumentStatement {
    pub modifier: Option<Token>,
    pub name: Token,
    pub kind: Token,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssignmentStatement {
    pub left: Token,
    pub right: Box<Token>, // TODO: This should be `Box<Statement>`.
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReturnStatement {
    pub value: Option<Token>, // TODO: This should be `Option<Box<Statement>>`.
}

#[derive(Clone, Debug, PartialEq)]
pub struct OptionStatement {
    pub configuration: Token,
    pub value: Option<Token>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AttributeStatement {
    // TODO: This should be a `Box<Statement>` since `name` could be a field
    // (i.e. `something.like.this`).
    pub name: Token,
    pub value: Token,
}
