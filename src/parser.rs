use crate::ast::Expression;
use crate::ast::Program;
use crate::ast::Statement;
use crate::token::Token;
pub struct Parser {

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
        if let Token::String(n) = token {
            return Ok(Statement::Print(Expression::StringLiteral(n.to_string())))
        } else {
            return Err("blah".to_string())
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
}
