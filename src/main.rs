mod parse;

use std::collections::HashMap;

use parse::Expr;

// Combine two Options into a single Option by applying a function to their contents
fn combine<T, F>(lhs: Option<T>, rhs: Option<T>, f: F) -> Option<T>
    where F: Fn(T, T) -> T
{
    lhs.and_then(|a| rhs.map(|b| f(a, b)))
}

// Recursively evaluate an Expr
fn evaluate(expr: &Expr, variables: &HashMap<String, i32>) -> Option<i32> {
    match expr {
        Expr::Val(num) => Some(*num),
        Expr::Var(name) => {
            if let Some(val) = variables.get(name) {
                Some(*val)
            } else {
                None
            }
        },
        Expr::Add(lhs, rhs) => {
            let lval = evaluate(lhs.as_ref(), variables);
            let rval = evaluate(rhs.as_ref(), variables);
            combine(lval, rval, |a, b| a + b)
        },
        Expr::Sub(lhs, rhs) => {
            let lval = evaluate(lhs.as_ref(), variables);
            let rval = evaluate(rhs.as_ref(), variables);
            combine(lval, rval, |a, b| a - b)
        },
        Expr::Mul(lhs, rhs) => {
            let lval = evaluate(lhs.as_ref(), variables);
            let rval = evaluate(rhs.as_ref(), variables);
            combine(lval, rval, |a, b| a * b)
        },
        Expr::Div(lhs, rhs) => {
            let lval = evaluate(lhs.as_ref(), variables);
            let rval = evaluate(rhs.as_ref(), variables);
            combine(lval, rval, |a, b| a / b)
        },
        Expr::Mod(lhs, rhs) => {
            let lval = evaluate(lhs.as_ref(), variables);
            let rval = evaluate(rhs.as_ref(), variables);
            combine(lval, rval, |a, b| a % b)
        },
    }
}

// Records the current token type while tokenizing (for multi-character tokens)
#[derive(PartialEq)]
enum TokenType {
    None,
    Number,
    Variable,
}

fn is_symbol(c: char) -> bool {
    c == '+' || c == '-' || c == '*' || c == '/' || c == '%' || c == '(' || c == ')'
}

// Tokenize input text into a vector of string slices referring to the input text
// wrapped into an Option which is None if tokenization fails (illegal characters)
fn tokenize<'a>(text: &'a str) -> Option<Vec<&'a str>> {
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

fn main() {
    // 1 + (2 * a)
    let expr = Expr::Add(
        Box::new(Expr::Val(1)),
        Box::new(
            Expr::Mul(
                Box::new(Expr::Val(2)),
                Box::new(Expr::Var(String::from("a")))
            )
        )
    );

    println!("expr = {}", expr);

    let mut variables: HashMap<String, i32> = HashMap::new();
    
    variables.insert(String::from("a"), 3);

    let value = evaluate(&expr, &variables).unwrap();

    println!("expr value = {}", value);

    println!("---");

    // Tokenize

    let text = "a+3*b";
    let tokens = tokenize(text);

    println!("text = {}", text);
    match tokens {
        Some(t) => println!("tokens = {:?}", t),
        None => println!("tokenize error")
    }
}
