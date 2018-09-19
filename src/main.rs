use std::fmt;
use std::collections::HashMap;

enum Expr {
    Val(i32),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Val(num) => write!(f, "{}", num),
            Expr::Var(name) => write!(f, "{}", name),
            Expr::Add(lhs, rhs) => write!(f, "({} + {})", lhs, rhs),
            Expr::Sub(lhs, rhs) => write!(f, "({} - {})", lhs, rhs),
            Expr::Mul(lhs, rhs) => write!(f, "({} * {})", lhs, rhs),
            Expr::Div(lhs, rhs) => write!(f, "({} / {})", lhs, rhs),
            Expr::Mod(lhs, rhs) => write!(f, "({} % {})", lhs, rhs)
        }
    }
}

fn combine<T, F>(lhs: Option<T>, rhs: Option<T>, f: F) -> Option<T>
    where F: Fn(T, T) -> T
{
    lhs.and_then(|a| rhs.map(|b| f(a, b)))
}

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

fn tokenize(text: &str) -> Vec<&str> {
    let tokens = vec![];
    for (i, c) in text.char_indices() {
        
    }
    tokens
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

    println!("{}", expr);

    let mut variables: HashMap<String, i32> = HashMap::new();

    variables.insert(String::from("a"), 3);

    let value = evaluate(&expr, &variables).unwrap();

    println!("{}", value);
}
