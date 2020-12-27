#[derive(Clone, Debug, PartialEq)]
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


pub fn keyword_to_token(keyword: &str) -> Option<Token> {
    match keyword {
        "LABEL" => Some(Token::LABEL),
        "GOTO" => Some(Token::GOTO),
        "PRINT" => Some(Token::PRINT),
        "INPUT" => Some(Token::INPUT),
        "LET" => Some(Token::LET),
        "IF" => Some(Token::IF),
        "THEN" => Some(Token::THEN),
        "ENDIF" => Some(Token::ENDIF),
        "WHILE" => Some(Token::WHILE),
        "REPEAT" => Some(Token::REPEAT),
        "ENDWHILE" => Some(Token::ENDWHILE),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_keyword_label() {
        let token = keyword_to_token("LABEL");
        assert_eq!(token, Some(Token::LABEL));
    }

    #[test]
    fn detect_keyword_goto() {
        let token = keyword_to_token("GOTO");
        assert_eq!(token, Some(Token::GOTO));
    }

    #[test]
    fn detect_keyword_print() {
        let token = keyword_to_token("PRINT");
        assert_eq!(token, Some(Token::PRINT));
    }

    #[test]
    fn detect_keyword_input() {
        let token = keyword_to_token("INPUT");
        assert_eq!(token, Some(Token::INPUT));
    }

    #[test]
    fn detect_keyword_let() {
        let token = keyword_to_token("LET");
        assert_eq!(token, Some(Token::LET));
    }

    #[test]
    fn detect_keyword_if() {
        let token = keyword_to_token("IF");
        assert_eq!(token, Some(Token::IF));
    }

    #[test]
    fn detect_keyword_then() {
        let token = keyword_to_token("THEN");
        assert_eq!(token, Some(Token::THEN));
    }

    #[test]
    fn detect_keyword_endif() {
        let token = keyword_to_token("ENDIF");
        assert_eq!(token, Some(Token::ENDIF));
    }

    #[test]
    fn detect_keyword_while() {
        let token = keyword_to_token("WHILE");
        assert_eq!(token, Some(Token::WHILE));
    }

    #[test]
    fn detect_keyword_repeat() {
        let token = keyword_to_token("REPEAT");
        assert_eq!(token, Some(Token::REPEAT));
    }

    #[test]
    fn detect_keyword_end_while() {
        let token = keyword_to_token("ENDWHILE");
        assert_eq!(token, Some(Token::ENDWHILE));
    }
}