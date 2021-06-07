// TODO: Review.
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Keyword(Vec<u8>),
    Identifier(Vec<u8>),

    Number(Vec<u8>),
    String(Vec<u8>),

    Public,
    Private,
    Static,
    Dim,

    As,
    If,
    Sub,
    Function,
    Type,
    Const,
    End,

    Do,
    Loop,

    While,
    Wend,

    For,
    Next,

    And,
    Or,
    Xor,

    LeftParentheses,
    RightParentheses,

    LeftBracket,
    RightBracket,

    Plus,
    Minus,
    Times,
    Divide,

    Less,
    LessOrEqual,

    Greater,
    GreaterOrEqual,

    Assignment,
    Dot,
}


// TODO: Maybe it would be better to store the lexeme no matter what, and just return
// a reference to it (for the sake of efficiency).
impl Token {
    pub fn get_lexeme(&self) -> Vec<u8> {
        return match &self {
            Token::Keyword(lexeme) | Token::Identifier(lexeme) => lexeme.clone(),
            Token::Number(lexeme) | Token::String(lexeme) => lexeme.clone(),

            Token::If => b"if".to_vec(),
            Token::End => b"end".to_vec(),

            Token::LeftParentheses => b"(".to_vec(),
            Token::RightParentheses => b")".to_vec(),

            Token::Plus => b"+".to_vec(),
            Token::Minus => b"-".to_vec(),

            Token::Less => b"<".to_vec(),
            Token::LessOrEqual => b"<=".to_vec(),

            Token::Greater => b">".to_vec(),
            Token::GreaterOrEqual => b">=".to_vec(),

            Token::Dot => b".".to_vec(),

            // TODO: Temporary polyfill.
            _ => b"__POLYFILL__".to_vec(),
        };
    }
}
