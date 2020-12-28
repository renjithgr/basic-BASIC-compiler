use crate::ast::Program;
use crate::ast::Statement;
use crate::ast::Expression;

pub fn run(program: Program) {
    for statement in program.statements {
        match statement {
            Statement::Print(expression) => {
                match expression {
                    Expression::StringLiteral(s) => println!("{}", s)
                }
            }
        }
    }
}