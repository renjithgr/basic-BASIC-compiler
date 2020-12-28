use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Print(Expression)
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    StringLiteral(String)
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
            Statement::Print(ident) => write!(f, "PRINT {}\n", ident)
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::StringLiteral(value) => write!(f, "{}", value)
        }
    }
}