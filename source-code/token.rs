// TODO: Review.
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // TODO: Remove or use another enumeration, such as:
    //
    //  Keyword(KeywordToken)
    //  KeywordToken { Public, Private, Static, Dim, ... }
    Keyword(Vec<u8>),
    Identifier(Vec<u8>),

    Number(Vec<u8>),
    String(Vec<u8>),

    Public,
    Private,
    Static,
    Dim,

    ByVal,
    ByRef,

    As,
    If,
    Sub,
    Function,
    Type,
    Enum,
    Const,
    End,

    Exit,
    Return,

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

    // Metadata.
    Attribute,
}


// TODO: Maybe it would be better to store the lexeme no matter what, and just return
// a reference to it (for the sake of efficiency).
impl Token {
    pub fn get_lexeme(&self) -> Vec<u8> {
        return match &self {
            Token::Keyword(lexeme) | Token::Identifier(lexeme) => lexeme.clone(),
            Token::Number(lexeme) | Token::String(lexeme) => lexeme.clone(),

            Token::Public => b"public".to_vec(),
            Token::Private => b"private".to_vec(),
            Token::Static => b"static".to_vec(),
            Token::Dim => b"dim".to_vec(),

            Token::ByVal => b"byval".to_vec(),
            Token::ByRef => b"byref".to_vec(),

            Token::As => b"as".to_vec(),
            Token::If => b"if".to_vec(),
            Token::Sub => b"sub".to_vec(),
            Token::Function => b"function".to_vec(),
            Token::Type => b"type".to_vec(),
            Token::End => b"end".to_vec(),

            Token::Exit => b"exit".to_vec(),
            Token::Return => b"return".to_vec(),

            Token::LeftParentheses => b"(".to_vec(),
            Token::RightParentheses => b")".to_vec(),

            Token::Plus => b"+".to_vec(),
            Token::Minus => b"-".to_vec(),

            Token::Less => b"<".to_vec(),
            Token::LessOrEqual => b"<=".to_vec(),

            Token::Greater => b">".to_vec(),
            Token::GreaterOrEqual => b">=".to_vec(),

            Token::Dot => b".".to_vec(),

            Token::Attribute => b"attribute".to_vec(),

            // TODO: Temporary polyfill.
            _ => b"__POLYFILL__".to_vec(),
        };
    }
}
