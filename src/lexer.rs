#[derive(Debug, PartialEq)]
pub enum Token {
    // Operators
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
    // Keywords
    LABEL,
    GOTO,
    PRINT,
    INPUT,
    LET,
    IF,
    THEN,
    ENDIF,
    WHILE,
    REPEAT,
    ENDWHILE,
    // Other stuff
    String(String),
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
            '"' => {
                let mut value = String::new();

                while let Some(c) = char_iter.peek() {
                    match c {
                        '"' => break,
                        _ => value.push(char_iter.next().unwrap())
                    }
                }

                tokens.push(Token::String(value));
                char_iter.next().unwrap();
            },
            '\n' => tokens.push(Token::NEWLINE),
            _ => panic!(format!("Unknown character: {}", c)),
        }
    }

    tokens
}

pub fn keyword_to_token(keyword: &str) -> Option<Token> {
    match keyword {
        _ => None
    }
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

    #[test]
    fn detect_minus() {
        let tokens = get_tokens("-");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::MINUS);
    }

    #[test]
    fn detect_slash() {
        let tokens = get_tokens("/");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::SLASH);
    }

    #[test]
    fn detect_asterisk() {
        let tokens = get_tokens("*");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::ASTERISK);
    }

    #[test]
    fn detect_greater_than() {
        let tokens = get_tokens(">");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::GT);
    }

    #[test]
    fn detect_greater_than_or_equal_to() {
        let tokens = get_tokens(">=");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::GTEQ);
    }

    #[test]
    fn detect_less_than() {
        let tokens = get_tokens("<");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::LT);
    }

    #[test]
    fn detect_less_than_or_equal_to() {
        let tokens = get_tokens("<=");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::LTEQ);
    }

    #[test]
    fn detect_equal_to() {
        let tokens = get_tokens("=");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::EQ);
    }

    #[test]
    fn detect_equal_to_equal_to() {
        let tokens = get_tokens("==");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::EQEQ);
    }

    #[test]
    fn detect_not_equal_to() {
        let tokens = get_tokens("!=");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::NOTEQ);
    }

    #[test]
    fn detect_string() {
        let tokens = get_tokens("\"hello\"");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("hello".to_string()));
    }

    #[test]
    fn detect_empty_string() {
        let tokens = get_tokens("\"\"");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("".to_string()));
    }
}
