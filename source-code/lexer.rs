use crate::Token;

// TODO: Replace return type with a `Result`.
pub fn lex(characters: &Vec<u8>) -> Vec<Token> {
    let mut tokens = vec!();
    let mut position = 0;

    while position < characters.len() {
        // NOTE: Order here is useful to prioritize, we want the lexeme to be as big as possible.
        if lex_whitespace(&characters, &mut position, &mut tokens) { continue; }
        if lex_identifier(&characters, &mut position, &mut tokens) { continue; }
        if lex_number(&characters, &mut position, &mut tokens) { continue; }
        if lex_string(&characters, &mut position, &mut tokens) { continue; }
        if lex_symbol(&characters, &mut position, &mut tokens) { continue; }

        // TODO: Return an error containing the lexeme.
        break;
    }

    return tokens;
}

fn lex_whitespace(characters: &Vec<u8>, position: &mut usize, tokens: &mut Vec<Token>) -> bool {
    let mut character = characters[*position] as char;

    if !(character.is_whitespace()) { return false; }

    // Count the first character.
    let mut length = 1usize;

    while (*position + length) < characters.len() {
        // Peek the next character.
        character = characters[*position + length] as char;

        // Analyze the next character.
        if !(character.is_whitespace()) { break; }

        // Count the previous character.
        length += 1;
    }

    // NOTE: Whitespace tokens are not saved.
    *position += length;

    return true;
}

fn lex_identifier(characters: &Vec<u8>, position: &mut usize, tokens: &mut Vec<Token>) -> bool {
    let mut character = characters[*position] as char;

    // The first character must be either a letter or a underscore.
    if !(character.is_alphabetic() || character == '_') { return false; }

    // Count the first character.
    let mut length = 1usize;

    while (*position + length) < characters.len() {
        // Peek the next character.
        character = characters[*position + length] as char;

        // Analyze the next character.
        if !(character.is_alphanumeric() || character == '_') { break; }

        // Count the previous character.
        length += 1;
    }

    let lexeme = characters[*position..*position + length].to_vec();
    let token = match &std::str::from_utf8(&lexeme).unwrap().to_lowercase() as &str {
        "public" => Token::Public,
        "private" => Token::Private,
        "static" => Token::Static,
        "dim" => Token::Dim,

        "as" => Token::As,
        "if" => Token::If,
        "sub" => Token::Sub,
        "function" => Token::Function,
        "type" => Token::Type,
        "const" => Token::Const,
        "end" => Token::End,

        "do" => Token::Do,
        "loop" => Token::Loop,

        "while" => Token::While,
        "wend" => Token::Wend,

        "for" => Token::For,
        "next" => Token::Next,

        "and" => Token::And,
        "or" => Token::Or,
        "xor" => Token::Xor,

        _ => Token::Identifier(lexeme),
    };

    tokens.push(token);
    *position += length;

    return true;
}

fn lex_number(characters: &Vec<u8>, position: &mut usize, tokens: &mut Vec<Token>) -> bool {
    let mut character = characters[*position] as char;

    // The first character must be a number.
    if !(character.is_numeric()) { return false; }

    // Count the first character.
    let mut length = 1usize;

    while (*position + length) < characters.len() {
        // Peek the next character.
        character = characters[*position + length] as char;

        // Analyze the next character.
        if !(character.is_numeric()) { break; }

        // Count the previous character.
        length += 1;
    }

    // TODO: This is a workaround to also lex numbers that have a decimal part.
    // We should add bounds checks here, and analyze if the construction of
    // decimal numbers should happen at the lexing stage or at the parsing
    // stage of the compiler.
    if characters[*position + length] as char == '.' {
        // TODO: Add bounds checks here!
        if (characters[*position + length + 1] as char).is_numeric() {
            length += 2;

            while (*position + length) < characters.len() {
                // Peek the next character.
                character = characters[*position + length] as char;

                // Analyze the next character.
                if !(character.is_numeric()) { break; }

                // Count the previous character.
                length += 1;
            }
        }
    }

    let lexeme = characters[*position..*position + length].to_vec();
    let token = Token::Number(lexeme);

    tokens.push(token);
    *position += length;

    return true;
}

fn lex_string(characters: &Vec<u8>, position: &mut usize, tokens: &mut Vec<Token>) -> bool {
    let mut character = characters[*position] as char;

    // The first character must be a " (quote).
    if !(character == '"') { return false; }

    // Count the first character.
    let mut length = 1usize;

    while (*position + length) < characters.len() {
        // Peek the next character.
        character = characters[*position + length] as char;

        // Analyze the next character.
        if character == '"' {
            // Count the closing quote.
            length += 1;
            break;
        }

        // Count the previous character.
        length += 1;
    }

    // If the last peeked character isn't a " (quote), then we've reached the end of the
    // characters and the string wasn't closed properly.
    if !(character == '"') { return false; }

    let lexeme = characters[*position..*position + length].to_vec();
    let token = Token::Identifier(lexeme);

    tokens.push(token);
    *position += length;

    return true;
}

fn lex_symbol(characters: &Vec<u8>, position: &mut usize, tokens: &mut Vec<Token>) -> bool {
    let character = characters[*position] as char;
    let next_character = *characters.get(*position + 1).unwrap_or(&b'\0') as char;

    let token = match (character, next_character) {
        ('(', _) => Some(Token::LeftParentheses),
        (')', _) => Some(Token::RightParentheses),

        ('[', _) => Some(Token::LeftBracket),
        (']', _) => Some(Token::RightBracket),

        ('+', _) => Some(Token::Plus),
        ('-', _) => Some(Token::Minus),
        ('*', _) => Some(Token::Times),
        ('/', _) => Some(Token::Divide),

        ('<', '=') => Some(Token::LessOrEqual),
        ('<', _) => Some(Token::Less),

        ('>', '=') => Some(Token::GreaterOrEqual),
        ('>', _) => Some(Token::Greater),

        ('=', _) => Some(Token::Assignment),
        ('.', _) => Some(Token::Dot),

        (_, _) => None,
    };

    let token = match token {
        Some(value) => value,
        None => return false,
    };

    let length = match token {
        Token::LessOrEqual | Token::GreaterOrEqual => 2,
        _ => 1,
    };

    tokens.push(token);
    *position += length;

    return true;
}
