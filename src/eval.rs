use std::collections::HashMap;

use parse::Expr;

/// Combine two Options into a single Option by applying a function to their contents.
fn combine<T, F>(lhs: Option<T>, rhs: Option<T>, f: F) -> Option<T>
    where F: Fn(T, T) -> T
{
    lhs.and_then(|a| rhs.map(|b| f(a, b)))
}

/// Recursively evaluate an Expr.
pub fn evaluate(expr: &Expr, variables: &HashMap<String, i32>) -> Option<i32> {
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
