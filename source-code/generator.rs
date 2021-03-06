use crate::statement::*;
use crate::viewer::Viewer;

struct Generator {
    statements: Viewer<Statement>,
}

impl Generator {
    pub fn new(statements: Vec<Statement>) -> Self {
        return Self {
            statements: Viewer::new(statements),
        };
    }

    pub fn generate(&mut self) -> String {
        let mut generated_code = String::new();

        while let Some(statement) = self.statements.next() {
            let statement_code = match statement {
                Statement::Constant(data) => self.generate_constant(&data),
                Statement::Subroutine(data) => self.generate_subroutine(&data),
                Statement::Function(data) => self.generate_function(&data),
                Statement::Type(data) => self.generate_type(&data),
                Statement::Enum(data) => self.generate_enum(&data),
                Statement::Variable(data) => self.generate_variable(&data),
                Statement::Option(data) => self.generate_option(&data),
                Statement::Attribute(data) => self.generate_attribute(&data),

                // TODO: Handle all cases.
                _ => String::from("__POLYFILL__\n"),
            };

            generated_code.push_str(&statement_code);
        }

        return generated_code;
    }

    fn generate_subroutine(&mut self, data: &SubroutineStatement) -> String {
        let generated_signature = self.generate_subroutine_signature(&data);
        let generated_body = self.generate_subroutine_body(&data);
        let generated_end = "end sub\n";

        return generated_signature + &generated_body + generated_end;
    }

    fn generate_subroutine_signature(&mut self, data: &SubroutineStatement) -> String {
        let mut generated_signature = String::new();

        generated_signature.push_str(&String::from_utf8_lossy(&data.scope.get_lexeme()));
        generated_signature.push_str(" sub ");
        generated_signature.push_str(&String::from_utf8_lossy(&data.name.get_lexeme()));

        generated_signature.push_str("(");

        for statement in &data.arguments {
            let argument_code = match statement {
                Statement::Argument(argument) => self.generate_argument(argument),

                // TODO: Is it correct to `panic`?
                //
                // TODO: Add a message?
                _ => unreachable!(),
            };

            generated_signature.push_str(&argument_code);
        }

        // Remove the last comma (and space) because it isn't followed by anything
        // (and is also a Visual Basic 6 syntax error).
        //
        // TODO: This feels hardcoded.
        if data.arguments.len() > 0 {
            let _ = generated_signature.pop(); // Space.
            let _ = generated_signature.pop(); // Comma.
        }

        generated_signature.push_str(")");
        generated_signature.push('\n');

        return generated_signature;
    }

    fn generate_subroutine_body(&mut self, data: &SubroutineStatement) -> String {
        let mut generated_body = String::new();

        for statement in &data.body {
            let generated_statement = match statement {
                Statement::Assignment(data) => self.generate_assignment(data),
                Statement::Constant(data) => self.generate_constant(data),
                Statement::Variable(data) => self.generate_variable(data),
                Statement::Exit(data) => self.generate_exit(data),
                Statement::Attribute(data) => self.generate_attribute(&data),

                // TODO: Handle all cases.
                _ => String::from("__POLYFILL__\n"),
            };

            generated_body.push_str(&generated_statement);
        }

        return generated_body;
    }

    fn generate_function(&mut self, data: &FunctionStatement) -> String {
        let generated_signature = self.generate_function_signature(&data);
        let generated_body = self.generate_function_body(&data);
        let generated_end = "end function\n";

        return generated_signature + &generated_body + generated_end;
    }

    fn generate_function_signature(&mut self, data: &FunctionStatement) -> String {
        let mut generated_signature = String::new();

        generated_signature.push_str(&String::from_utf8_lossy(&data.scope.get_lexeme()));
        generated_signature.push_str(" function ");
        generated_signature.push_str(&String::from_utf8_lossy(&data.name.get_lexeme()));

        generated_signature.push_str("(");

        for statement in &data.arguments {
            let argument_code = match statement {
                Statement::Argument(argument) => self.generate_argument(argument),

                // TODO: Is it correct to `panic`?
                //
                // TODO: Add a message?
                _ => unreachable!(),
            };

            generated_signature.push_str(&argument_code);
        }

        // Remove the last comma (and space) because it isn't followed by anything
        // (and is also a Visual Basic 6 syntax error).
        //
        // TODO: This feels hardcoded.
        if data.arguments.len() > 0 {
            let _ = generated_signature.pop(); // Space.
            let _ = generated_signature.pop(); // Comma.
        }

        generated_signature.push_str(")");

        // TODO: This seems too imperative.
        match &data.kind {
            Some(kind) => {
                generated_signature.push_str(" as ");
                generated_signature.push_str(&String::from_utf8_lossy(&kind.get_lexeme()));
            },

            None => (),
        }

        generated_signature.push('\n');

        return generated_signature;
    }

    fn generate_function_body(&mut self, data: &FunctionStatement) -> String {
        let mut generated_body = String::new();

        for statement in &data.body {
            let generated_statement = match statement {
                Statement::Assignment(data) => self.generate_assignment(data),
                Statement::Constant(data) => self.generate_constant(data),
                Statement::Variable(data) => self.generate_variable(data),
                Statement::Exit(data) => self.generate_exit(data),
                Statement::Attribute(data) => self.generate_attribute(&data),

                // TODO: Handle all cases.
                _ => String::from("__POLYFILL__\n"),
            };

            generated_body.push_str(&generated_statement);
        }

        return generated_body;
    }

    fn generate_type(&mut self, data: &TypeStatement) -> String {
        let mut generated_code = String::new();

        generated_code.push_str("type ");
        generated_code.push_str(&String::from_utf8_lossy(&data.name.get_lexeme()));
        generated_code.push('\n');

        for statement in &data.attributes {
            match statement {
                // TODO: This seems too imperative.
                Statement::TypeAttribute(data) => {
                    generated_code.push_str(&String::from_utf8_lossy(&data.name.get_lexeme()));
                    generated_code.push_str(" as ");
                    generated_code.push_str(&String::from_utf8_lossy(&data.kind.get_lexeme()));
                    generated_code.push('\n');
                },

                // TODO: Is it correct to `panic`?
                //
                // TODO: Add a message?
                _ => unreachable!(),
            }
        }

        generated_code.push_str("end type\n");

        return generated_code;
    }

    fn generate_enum(&mut self, data: &EnumStatement) -> String {
        let mut generated_code = String::new();

        if let Some(scope) = &data.scope {
            generated_code.push_str(&String::from_utf8_lossy(&scope.get_lexeme()));
            generated_code.push(' ');
        }

        generated_code.push_str("enum ");
        generated_code.push_str(&String::from_utf8_lossy(&data.name.get_lexeme()));
        generated_code.push('\n');

        for statement in &data.attributes {
            match statement {
                // TODO: This seems too imperative.
                Statement::EnumAttribute(data) => {
                    generated_code.push_str(&String::from_utf8_lossy(&data.name.get_lexeme()));

                    if let Some(value) = &data.value {
                        generated_code.push_str(" = ");
                        generated_code.push_str(&String::from_utf8_lossy(&value.get_lexeme()));
                    }

                    generated_code.push('\n');
                },

                // TODO: Is it correct to `panic`?
                //
                // TODO: Add a message?
                _ => unreachable!(),
            }
        }

        generated_code.push_str("end enum\n");

        return generated_code;
    }

    fn generate_argument(&mut self, data: &ArgumentStatement) -> String {
        let mut generated_code = String::new();

        if let Some(modifier) = &data.modifier {
            generated_code.push_str(&String::from_utf8_lossy(&modifier.get_lexeme()));
            generated_code.push(' ');
        }

        generated_code.push_str(&String::from_utf8_lossy(&data.name.get_lexeme()));
        generated_code.push_str(" as ");
        generated_code.push_str(&String::from_utf8_lossy(&data.kind.get_lexeme()));
        generated_code.push_str(", ");

        return generated_code;
    }

    fn generate_assignment(&mut self, data: &AssignmentStatement) -> String {
        let mut generated_code = String::new();

        generated_code.push_str(&String::from_utf8_lossy(&data.left.get_lexeme()));
        generated_code.push_str(" = ");
        generated_code.push_str(&String::from_utf8_lossy(&data.right.get_lexeme()));
        generated_code.push('\n');

        return generated_code;
    }

    fn generate_constant(&mut self, data: &ConstantStatement) -> String {
        let mut generated_code = String::new();

        generated_code.push_str(&String::from_utf8_lossy(&data.scope.get_lexeme()));
        generated_code.push_str(" const ");
        generated_code.push_str(&String::from_utf8_lossy(&data.name.get_lexeme()));

        // TODO: This seems too imperative.
        match &data.kind {
            Some(kind) => {
                generated_code.push_str(" as ");
                generated_code.push_str(&String::from_utf8_lossy(&kind.get_lexeme()));
            },

            None => (),
        }

        // TODO: This seems too imperative.
        match &data.length {
            Some(length) => {
                generated_code.push_str(" * ");
                generated_code.push_str(&String::from_utf8_lossy(&length.get_lexeme()));
            },

            None => (),
        }

        generated_code.push_str(" = ");
        generated_code.push_str(&String::from_utf8_lossy(&data.value.get_lexeme()));

        generated_code.push('\n');

        return generated_code;
    }

    fn generate_variable(&mut self, data: &VariableStatement) -> String {
        let mut generated_code = String::new();

        generated_code.push_str(&String::from_utf8_lossy(&data.scope.get_lexeme()));
        generated_code.push_str(" ");
        generated_code.push_str(&String::from_utf8_lossy(&data.name.get_lexeme()));
        generated_code.push_str(" as ");
        generated_code.push_str(&String::from_utf8_lossy(&data.kind.get_lexeme()));
        generated_code.push('\n');

        return generated_code;
    }

    fn generate_exit(&mut self, data: &ExitStatement) -> String {
        let mut generated_code = String::new();

        generated_code.push_str("exit ");
        generated_code.push_str(&String::from_utf8_lossy(&data.block.get_lexeme()));
        generated_code.push('\n');

        return generated_code;
    }

    fn generate_option(&mut self, data: &OptionStatement) -> String {
        let mut generated_code = String::new();

        generated_code.push_str("option ");
        generated_code.push_str(&String::from_utf8_lossy(&data.configuration.get_lexeme()));

        if let Some(value) = &data.value {
            generated_code.push(' ');
            generated_code.push_str(&String::from_utf8_lossy(&value.get_lexeme()));
        }

        generated_code.push('\n');

        return generated_code;
    }

    fn generate_attribute(&mut self, data: &AttributeStatement) -> String {
        let mut generated_code = String::new();

        generated_code.push_str("Attribute ");
        generated_code.push_str(&String::from_utf8_lossy(&data.name.get_lexeme()));
        generated_code.push_str(" = ");
        generated_code.push_str(&String::from_utf8_lossy(&data.value.get_lexeme()));
        generated_code.push('\n');

        return generated_code;
    }
}

pub fn generate(statements: Vec<Statement>) -> String {
    let mut generator = Generator::new(statements);

    return generator.generate();
}
