use crate::token::Token;
use crate::token::keyword_to_token;

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
                        '"' => {
                            char_iter.next().unwrap();
                            break;
                        },
                        _ => value.push(char_iter.next().unwrap())
                    }
                }

                tokens.push(Token::String(value));
            },
            '\n' => tokens.push(Token::NEWLINE),

            c if c.is_alphabetic() => {
                let mut value = String::new();
                value.push(c);

                while let Some(c) = char_iter.peek() {
                    if c.is_alphabetic() {
                        value.push(char_iter.next().unwrap());
                    } else {
                        break;
                    }
                }

                let identifier = keyword_to_token(&value).unwrap_or_else(|| Token::Identifier(value));
                
                tokens.push(identifier);
            },

            _ => panic!("Unknown character {}", c),
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

    #[test]
    fn detect_keywords() {
        let tokens = get_tokens("LABEL GOTO PRINT");
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::LABEL);
        assert_eq!(tokens[1], Token::GOTO);
        assert_eq!(tokens[2], Token::PRINT);
    }

    #[test]
    fn detect_identifiers() {
        let tokens = get_tokens("a b c");
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Identifier("a".to_string()));
        assert_eq!(tokens[1], Token::Identifier("b".to_string()));
        assert_eq!(tokens[2], Token::Identifier("c".to_string()));
    }
}
