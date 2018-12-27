/// Represents tokens of an expression.
#[derive(Debug)]
pub enum Token {
    Number(i32),
    Variable(String),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    OpenPar,
    ClosePar,
}

/// Records the current token type while tokenizing (for multi-character tokens).
#[derive(PartialEq)]
enum TokenType {
    None,
    Number,
    Variable,
}

/// Try to parse `symbol` into a `Token`.
fn symbol_to_token(symbol: char) -> Option<Token> {
    match symbol {
        '+' => Some(Token::Add),
        '-' => Some(Token::Sub),
        '*' => Some(Token::Mul),
        '/' => Some(Token::Div),
        '%' => Some(Token::Mod),
        '(' => Some(Token::OpenPar),
        ')' => Some(Token::ClosePar),
        _ => None,
    }
}

/// Parse `string` as a `Token` of type `token_type`.
fn string_to_token(string: &str, token_type: TokenType) -> Option<Token> {
    match token_type {
        TokenType::Number => string.parse().map(|n| Token::Number(n)).ok(),
        TokenType::Variable => Some(Token::Variable(String::from(string))),
        _ => None,
    }
}

/// Tokenize input text into a vector of tokens.
pub fn tokenize(text: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut current_type = TokenType::None;
    let mut start: usize = 0;
    let mut end: usize = 0;

    for (i, c) in text.char_indices() {
        if (current_type == TokenType::Number && !c.is_digit(10))
            || (current_type == TokenType::Variable && !c.is_alphabetic())
        {
            // Unwrap is fine because current_type is not TokenType::None
            let new_token = string_to_token(&text[start..end], current_type).unwrap();
            tokens.push(new_token);
            current_type = TokenType::None;
        }

        if c == ' ' {
            continue;
        } else if c.is_digit(10) {
            match current_type {
                TokenType::Number => {
                    end += 1;
                }
                _ => {
                    current_type = TokenType::Number;
                    start = i;
                    end = i+1;
                }
            }
        } else if c.is_alphabetic() {
            match current_type {
                TokenType::Variable => {
                    end += 1;
                }
                _ => {
                    current_type = TokenType::Variable;
                    start = i;
                    end = i+1;
                }
            }
        } else if let Some(symbol) = symbol_to_token(c) {
            tokens.push(symbol);
        } else {
            return Err(format!("Unknown character '{}'", c));
        }
    }

    // Add potential final token
    if current_type != TokenType::None {
        // Unwrap is fine because current_type is not TokenType::None
        let new_token = string_to_token(&text[start..end], current_type).unwrap();
        tokens.push(new_token);
    }

    Ok(tokens)
}