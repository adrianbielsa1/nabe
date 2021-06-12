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
use crate::Viewer;

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
                Statement::Function(data) => self.generate_function(data),

                // TODO: Handle all cases.
                _ => String::from("__POLYFILL__\n"),
            };

            generated_code.push_str(&statement_code);
        }

        return generated_code;
    }

    fn generate_function(&mut self, data: FunctionStatement) -> String {
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

                // TODO: Handle all cases.
                _ => String::from("__POLYFILL__\n"),
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
                Statement::Variable(data) => self.generate_variable(data),

                // TODO: Handle all cases.
                _ => String::from("__POLYFILL__\n"),
            };

            generated_body.push_str(&generated_statement);
        }

        return generated_body;
    }

    fn generate_argument(&mut self, data: &ArgumentStatement) -> String {
        let mut generated_code = String::new();

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
}

pub fn generate(statements: Vec<Statement>) -> String {
    let mut generator = Generator::new(statements);

    return generator.generate();
}
