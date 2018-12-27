mod parse;
mod eval;
mod tokenize;

use std::collections::HashMap;

use parse::Expr;
use eval::evaluate;
use tokenize::tokenize;

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
