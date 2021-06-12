use crate::Token;
use crate::Statement;
use crate::TypeStatement;
use crate::TypeAttributeStatement;
use crate::VariableStatement;
use crate::ConstantStatement;
use crate::SubroutineStatement;
use crate::FunctionStatement;
use crate::ArgumentStatement;
use crate::AssignmentStatement;
use crate::ReturnStatement;

struct Parser<'a> {
    tokens: &'a Vec<Token>,
    tokens_position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser {
        return Parser {
            tokens: tokens,
            tokens_position: 0,
        };
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let parsers = [
            Parser::parse_type, Parser::parse_variable, Parser::parse_constant,
            Parser::parse_subroutine, Parser::parse_function,
        ];

        let mut statements = vec!();

        'parse_next_statement: while self.tokens_position < self.tokens.len() {
            // Try each one of the specialized parsers to see if we can
            // produce an statement.
            for parser in &parsers {
                let position_before_parsing = self.tokens_position;

                if let Some(statement) = parser(self) {
                    statements.push(statement);
                    continue 'parse_next_statement;
                } else {
                    self.tokens_position = position_before_parsing;
                }
            }

            // None of the previous parsers could convert the remaining tokens
            // into a statement.
            //
            // TODO: Return an error?
            break;
        }

        return statements;
    }

    fn consume(&mut self, expected_token: Token) -> Option<Token> {
        if self.tokens_position >= self.tokens.len() {
            return None;
        }

        // TODO: Use a library (`strum`?).
        let left_discriminant = std::mem::discriminant(&self.tokens[self.tokens_position]);
        let right_discriminant = std::mem::discriminant(&expected_token);

        if left_discriminant != right_discriminant {
            return None;
        }

        self.tokens_position += 1;

        // TODO: Cloning here!
        return Some(self.tokens[self.tokens_position - 1].clone());
    }

    fn parse_type(&mut self) -> Option<Statement> {
        // Assert there is a `Type` keyword and a identifier containing the
        // type's name.
        let _ = self.consume(Token::Type)?;

        // TODO: Remove `vec!`.
        let name = self.consume(Token::Identifier(vec!()))?;

        let mut attributes = vec!();

        while let Some(attribute) = self.parse_type_attribute() {
            attributes.push(attribute);
        }

        // Assert there are the `End Type` keywords.
        let _ = self.consume(Token::End)?;
        let _ = self.consume(Token::Type)?;

        return Some(Statement::Type(TypeStatement {
            name: name,
            attributes: attributes,
        }));
    }

    fn parse_type_attribute(&mut self) -> Option<Statement> {
        // TODO: Remove `vec!`.
        let name = self.consume(Token::Identifier(vec!()))?;
        let _ = self.consume(Token::As)?;
        let kind = self.consume(Token::Identifier(vec!()))?;

        return Some(Statement::TypeAttribute(TypeAttributeStatement {
            name: name,
            kind: kind,
        }));
    }

    fn parse_variable(&mut self) -> Option<Statement> {
        let possible_scopes = [Token::Public, Token::Private, Token::Static, Token::Dim];

        // NOTE: This workaround is needed because (as of the time of writing), Rust's
        // `into_iter` method for arrays returns a reference instead of a (moved) value,
        // so, to avoid using vectors, we can directly construct the `IntoIter`
        // iterator. I like to think that this solution is VERY slightly more efficient,
        // but I haven't tested it.
        let scope = std::array::IntoIter::new(possible_scopes).find_map(|t| self.consume(t))?;

        // TODO: Remove `vec!`.
        let name = self.consume(Token::Identifier(vec!()))?;
        let _ = self.consume(Token::As)?;
        let kind = self.consume(Token::Identifier(vec!()))?;

        return Some(Statement::Variable(VariableStatement {
            scope: scope.clone(),
            name: name,
            kind: kind,
        }));
    }

    fn parse_constant(&mut self) -> Option<Statement> {
        let possible_scopes = [Token::Public, Token::Private, Token::Static, Token::Dim];

        // NOTE: See `parse_variable`.
        let scope = std::array::IntoIter::new(possible_scopes).find_map(|t| self.consume(t))?;

        let _ = self.consume(Token::Const)?;

        // TODO: Remove `vec!`.
        let name = self.consume(Token::Identifier(vec!()))?;

        let kind = match self.consume(Token::As) {
            Some(_) => self.consume(Token::Identifier(vec!())),
            None => None,
        };

        let _ = self.consume(Token::Assignment)?;

        // TODO: Remove `vec!`.
        let possible_values = [
            Token::Identifier(vec!()), Token::Number(vec!()), Token::String(vec!())
        ];

        // NOTE: See `parse_variable`.
        let value = std::array::IntoIter::new(possible_values).find_map(|t| self.consume(t))?;

        return Some(Statement::Constant(ConstantStatement {
            scope: scope.clone(),
            name: name,
            kind: kind,
            value: value,
        }));
    }

    fn parse_subroutine(&mut self) -> Option<Statement> {
        let possible_scopes = [Token::Public, Token::Private, Token::Static];

        // NOTE: See `parse_variable`.
        let scope = std::array::IntoIter::new(possible_scopes).find_map(|t| self.consume(t))?;

        let _ = self.consume(Token::Sub)?;

        // TODO: Remove `vec!`.
        let name = self.consume(Token::Identifier(vec!()))?;
        let _ = self.consume(Token::LeftParentheses)?;

        let mut arguments = vec!();

        while let Some(argument) = self.parse_callable_argument() {
            arguments.push(argument);
        }

        let _ = self.consume(Token::RightParentheses)?;

        // TODO: Consume a new line?

        let body = self.parse_callable_body();

        let _ = self.consume(Token::End)?;
        let _ = self.consume(Token::Sub)?;

        return Some(Statement::Subroutine(SubroutineStatement {
            scope: scope,
            name: name,
            arguments: arguments,
            body: body,
        }));
    }

    fn parse_function(&mut self) -> Option<Statement> {
        let possible_scopes = [Token::Public, Token::Private, Token::Static];

        // NOTE: See `parse_variable`.
        let scope = std::array::IntoIter::new(possible_scopes).find_map(|t| self.consume(t))?;

        let _ = self.consume(Token::Function)?;

        // TODO: Remove `vec!`.
        let name = self.consume(Token::Identifier(vec!()))?;
        let _ = self.consume(Token::LeftParentheses)?;

        let mut arguments = vec!();

        while let Some(argument) = self.parse_callable_argument() {
            arguments.push(argument);
        }

        let _ = self.consume(Token::RightParentheses)?;

        let kind = match self.consume(Token::As) {
            Some(_) => self.consume(Token::Identifier(vec!())),
            None => None,
        };

        // TODO: Consume a new line?

        let body = self.parse_callable_body();

        let _ = self.consume(Token::End)?;
        let _ = self.consume(Token::Function)?;

        return Some(Statement::Function(FunctionStatement {
            scope: scope,
            name: name,
            arguments: arguments,
            kind: kind,
            body: body,
        }));
    }

    // Used for both functions and subroutines.
    fn parse_callable_argument(&mut self) -> Option<Statement> {
        // TODO: Use `possible_modifiers` to check if there is a `ByRef` or `ByVal`
        // keyword before the argument's name. Otherwise default to `ByRef`.

        // TODO: Remove `vec!`.
        let name = self.consume(Token::Identifier(vec!()))?;
        let _ = self.consume(Token::As)?;
        let kind = self.consume(Token::Identifier(vec!()))?;

        return Some(Statement::Argument(ArgumentStatement {
            name: name,
            kind: kind,
        }));
    }

    // Used for both functions and subroutines.
    fn parse_callable_body(&mut self) -> Vec<Statement> {
        let parsers = [
            Parser::parse_variable, Parser::parse_constant, Parser::parse_assignment,
            Parser::parse_return,
        ];

        let mut statements = vec!();

        'parse_next_statement: while self.tokens_position < self.tokens.len() {
            // Try each one of the specialized parsers to see if we can
            // produce an statement.
            for parser in &parsers {
                let position_before_parsing = self.tokens_position;

                if let Some(statement) = parser(self) {
                    statements.push(statement);
                    continue 'parse_next_statement;
                } else {
                    self.tokens_position = position_before_parsing;
                }
            }

            // None of the previous parsers could convert the remaining tokens
            // into a statement.
            //
            // TODO: Return an error?
            break;
        }

        return statements;
    }

    fn parse_assignment(&mut self) -> Option<Statement> {
        // TODO: Remove `vec!`.
        let left = self.consume(Token::Identifier(vec!()))?;
        let _ = self.consume(Token::Assignment)?;

        // TODO: Use `parse_expression`.
        // TODO: Remove `vec!`.
        let possible_values = [
            Token::Identifier(vec!()), Token::Number(vec!()), Token::String(vec!())
        ];

        // NOTE: See `parse_variable`.
        let right = std::array::IntoIter::new(possible_values).find_map(|t| self.consume(t))?;

        return Some(Statement::Assignment(AssignmentStatement {
            left: left,
            right: Box::new(right),
        }));
    }

    fn parse_return(&mut self) -> Option<Statement> {
        let _ = self.consume(Token::Return)?;

        // TODO: Remove `vec!`.
        let possible_values = [
            Token::Identifier(vec!()), Token::Number(vec!()), Token::String(vec!())
        ];

        // NOTE: See `parse_variable`.
        let value = std::array::IntoIter::new(possible_values).find_map(|t| self.consume(t));

        return Some(Statement::Return(ReturnStatement {
            value: value,
        }));
    }
}

pub fn parse(tokens: &Vec<Token>) -> Vec<Statement> {
    let mut parser = Parser::new(tokens);

    return parser.parse();
}
