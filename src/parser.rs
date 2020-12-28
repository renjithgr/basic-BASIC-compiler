use crate::ast::Expression;
use crate::ast::Program;
use crate::ast::Statement;
use crate::token::Token;
pub struct Parser {

}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    pub fn parse_tokens(&mut self, tokens: Vec<Token>) -> Program {
        let mut statements = vec![];
    
        let mut iter = tokens.iter().peekable();
    
        while let Some(c) = iter.next() {
            match c {
                Token::PRINT => {
                    match iter.peek() {
                        Some(token) => match self.parse_print_statement(token) {
                            Ok(stmt) => {
                                statements.push(stmt);
                                iter.next();
                            },
                            Err(err) => {
                                panic!("{}", err);
                            }
                        },
                        None => panic!("No token found!")
                    }
                },
                _ => panic!("Unknown command!"),
            }
        }
    
        Program { statements }
    }

    fn parse_print_statement(&mut self, token: &Token) -> Result<Statement, String> {
        match token {
            Token::String(n) => Ok(Statement::Print(Expression::StringLiteral(n.to_string()))),
            Token::Float(f) => match f.parse() {
                Ok(value) => Ok(Statement::Print(Expression::FloatLiteral(value))),
                Err(_) => Err("blah".to_string())
            },
            Token::Integer(i) => match i.parse() {
                Ok(value) => Ok(Statement::Print(Expression::IntegerLiteral(value))),
                Err(_) => Err("blah".to_string())
            },
            _ => Err("blah".to_string())
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::get_tokens;

    #[test]
    fn parse_print_statement() {
        let tokens = get_tokens("PRINT \"HELLO WORLD\"");
        let mut parser = Parser::new();

        let program = parser.parse_tokens(tokens);

        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn parse_print_floating_point_number() {
        let tokens = get_tokens("PRINT 2.0");
        let mut parser = Parser::new();

        let program = parser.parse_tokens(tokens);
        let expected = Statement::Print(Expression::FloatLiteral(2.0));

        assert_eq!(program.statements.len(), 1);
        assert_eq!(program.statements[0], expected);
    }

    #[test]
    fn parse_print_integer_number() {
        let tokens = get_tokens("PRINT 2");
        let mut parser = Parser::new();

        let program = parser.parse_tokens(tokens);
        let expected = Statement::Print(Expression::IntegerLiteral(2));

        assert_eq!(program.statements.len(), 1);
        assert_eq!(program.statements[0], expected);
    }
}
