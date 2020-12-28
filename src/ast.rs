use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Print(Expression)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    StringLiteral(String),
    FloatLiteral(f64),
    IntegerLiteral(i64),
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for stmt in &self.statements {
            write!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Print(ident) => writeln!(f, "PRINT {}", ident),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::StringLiteral(value) => write!(f, "{}", value),
            Expression::FloatLiteral(value) => write!(f, "{}", value),
            Expression::IntegerLiteral(value) => write!(f, "{}", value),
        }
    }
}
