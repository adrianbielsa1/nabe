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
use crate::Viewer;

struct Block {
    name: Token,
}

struct Transformer {
    statements: Viewer<Statement>,
    blocks: Vec<Block>,
}

impl Transformer {
    pub fn new(statements: Vec<Statement>) -> Self {
        return Self {
            statements: Viewer::new(statements),
            blocks: vec!(),
        };
    }

    pub fn transform(&mut self) -> Vec<Statement> {
        let mut transformed_statements = vec!();

        while let Some(statement) = self.statements.next() {
            let transformed_statement = match statement {
                Statement::Function(data) => self.transform_function(data),

                // No transformation aplicable.
                _ => statement,
            };

            transformed_statements.push(transformed_statement);
        }

        return transformed_statements;
    }

    fn transform_function(&mut self, data: FunctionStatement) -> Statement {
        self.blocks.push(Block { name: data.name.clone() });

        let transformed_data = FunctionStatement {
            scope: data.scope,
            name: data.name,
            arguments: data.arguments,
            kind: data.kind,
            body: self.transform_function_body(data.body),
        };

        self.blocks.pop();

        return Statement::Function(transformed_data);
    }

    fn transform_function_body(&mut self, body: Vec<Statement>) -> Vec<Statement> {
        let mut transformed_statements = vec!();
        let mut viewer = Viewer::new(body);

        while let Some(statement) = viewer.next() {
            let transformed_statement = match statement {
                Statement::Return(data) => self.transform_function_return(data),

                // No transformation aplicable.
                _ => statement,
            };

            transformed_statements.push(transformed_statement);
        }

        return transformed_statements;
    }

    fn transform_function_return(&mut self, data: ReturnStatement) -> Statement {
        return Statement::Assignment(AssignmentStatement {
            left: self.blocks.last().unwrap().name.clone(),

            // NOTE: I unwrap thee because the parser should have already
            // checked if a function's return is complete (empty returns
            // are allowed only in subroutines).
            //
            // Failing to do so means the parser is broken, and panicking
            // is a good way (TODO: or not?) to signal it.
            right: Box::new(data.value.unwrap()),
        });
    }
}

pub fn transform(statements: Vec<Statement>) -> Vec<Statement> {
    let mut transformer = Transformer::new(statements);

    return transformer.transform();
}
