#[derive(Debug, PartialEq)]
pub enum Token {
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    GT,
    GTEQ,
    LT,
    LTEQ,
    EQ,
    EQEQ,
    NOTEQ,
    NEWLINE,
}

fn get_tokens(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let mut char_iter = input.chars().peekable();

    while let Some(c) = char_iter.next() {
        match c {
            c if c.is_whitespace() => continue,
            '+' => tokens.push(Token::PLUS),
            '-' => tokens.push(Token::MINUS),
            '*' => tokens.push(Token::ASTERISK),
            '/' => tokens.push(Token::SLASH),
            '>' => match char_iter.peek() {
                Some(&'=') => {
                    char_iter.next();
                    tokens.push(Token::GTEQ);
                }
                _ => tokens.push(Token::GT),
            },
            '<' => match char_iter.peek() {
                Some(&'=') => {
                    char_iter.next();
                    tokens.push(Token::LTEQ);
                }
                _ => tokens.push(Token::LT),
            },
            '=' => match char_iter.peek() {
                Some(&'=') => {
                    char_iter.next();
                    tokens.push(Token::EQEQ)
                }
                _ => tokens.push(Token::EQ),
            },
            '!' => match char_iter.peek() {
                Some(&'=') => {
                    char_iter.next();
                    tokens.push(Token::NOTEQ);
                }
                Some(c) => panic!(format!("Expected !=, got !{}", c)),
                None => panic!("Expected !=, got nothing".to_string()),
            },
            '\n' => tokens.push(Token::NEWLINE),
            _ => panic!(format!("Unknown character: {}", c)),
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_plus() {
        let tokens = get_tokens("+");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::PLUS);
    }
}
