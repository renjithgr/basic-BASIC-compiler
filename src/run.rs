use crate::ast::Expression;
use crate::ast::Program;
use crate::ast::Statement;

pub fn run(program: Program) {
    for statement in program.statements {
        match statement {
            Statement::Print(expression) => match expression {
                Expression::StringLiteral(s) => println!("{}", s),
                Expression::FloatLiteral(f) => println!("{:.2}", f),
                Expression::IntegerLiteral(i) => println!("{}", i)
            },
        }
    }
}
