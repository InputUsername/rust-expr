/// Records the current token type while tokenizing (for multi-character tokens).
#[derive(PartialEq)]
enum TokenType {
    None,
    Number,
    Variable,
}

fn is_symbol(c: char) -> bool {
    c == '+' || c == '-' || c == '*' || c == '/' || c == '%' || c == '(' || c == ')'
}

/// Tokenize input text into a vector of string slices referring to the input text
/// wrapped into an Option which is None if tokenization fails (illegal characters).
pub fn tokenize<'a>(text: &'a str) -> Option<Vec<&'a str>> {
    let mut tokens = vec![];
    let mut current_token = TokenType::None;
    let mut start: usize = 0;
    let mut end: usize = 0;

    for (i, c) in text.char_indices() {
        if (current_token == TokenType::Number && !c.is_digit(10))
            || (current_token == TokenType::Variable && !c.is_alphabetic())
        {
            println!("ending num/var parsing");

            tokens.push(&text[start..end]);
            current_token = TokenType::None;
        }

        if c == ' ' {
            continue;
        } else if is_symbol(c) {
            tokens.push(&text[i..i+1]);
        } else if c.is_digit(10) {
            match current_token {
                TokenType::Number => {
                    println!("inside number {}", c);
                    end += 1;
                }
                _ => {
                    println!("found number {}", c);
                    current_token = TokenType::Number;
                    start = i;
                    end = i+1;
                }
            }
        } else if c.is_alphabetic() {
            match current_token {
                TokenType::Variable => {
                    println!("inside variable {}", c);
                    end += 1;
                }
                _ => {
                    println!("found variable {}", c);
                    current_token = TokenType::Variable;
                    start = i;
                    end = i+1;
                }
            }
        } else {
            return None;
        }
    }

    if current_token != TokenType::None {
        tokens.push(&text[start..end]);
    }

    Some(tokens)
}