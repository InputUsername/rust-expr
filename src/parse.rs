use std::fmt;

use tokenize::Token;

pub enum Expr {
    Number(i32),
    Variable(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Number(num) => write!(f, "{}", num),
            Expr::Variable(name) => write!(f, "{}", name),
            Expr::Add(lhs, rhs) => write!(f, "({} + {})", lhs, rhs),
            Expr::Sub(lhs, rhs) => write!(f, "({} - {})", lhs, rhs),
            Expr::Mul(lhs, rhs) => write!(f, "({} * {})", lhs, rhs),
            Expr::Div(lhs, rhs) => write!(f, "({} / {})", lhs, rhs),
            Expr::Mod(lhs, rhs) => write!(f, "({} % {})", lhs, rhs)
        }
    }
}

type ParseResult<'a> = Result<(Box<Expr>, &'a [Token]), String>;

pub fn parse(tokens: &[Token]) -> Result<Box<Expr>, String> {
    parse_expr(tokens).and_then(|(expr, rest)| {
        if rest.is_empty() {
            Ok(expr)
        } else {
            Err(format!("Unconsumed input '{:?}'", rest))
        }
    })
}

fn parse_expr<'a>(tokens: &'a [Token]) -> ParseResult<'a> {
    let (lhs, rest) = parse_term(tokens)?;
    parse_expr_rhs(lhs, rest)
}

fn parse_expr_rhs<'a>(lhs: Box<Expr>, tokens: &'a [Token]) -> ParseResult<'a> {
    match tokens.first() {
        Some(Token::Add) => {
            let (rhs, rest) = parse_term(&tokens[1..])?;
            let expr = Box::new(Expr::Add(lhs, rhs));
            parse_expr_rhs(expr, rest)
        },
        Some(Token::Sub) => {
            let (rhs, rest) = parse_term(&tokens[1..])?;
            let expr = Box::new(Expr::Sub(lhs, rhs));
            parse_expr_rhs(expr, rest)
        },
        _ => Ok((lhs, tokens)),
    }
}

fn parse_term<'a>(tokens: &'a [Token]) -> ParseResult<'a> {
    let (lhs, rest) = parse_factor(tokens)?;
    parse_term_rhs(lhs, rest)
}

fn parse_term_rhs<'a>(lhs: Box<Expr>, tokens: &'a [Token]) -> ParseResult<'a> {
    match tokens.first() {
        Some(Token::Mul) => {
            let (rhs, rest) = parse_factor(&tokens[1..])?;
            let expr = Box::new(Expr::Mul(lhs, rhs));
            parse_term_rhs(expr, rest)
        },
        Some(Token::Div) => {
            let (rhs, rest) = parse_factor(&tokens[1..])?;
            let expr = Box::new(Expr::Div(lhs, rhs));
            parse_term_rhs(expr, rest)
        },
        Some(Token::Mod) => {
            let (rhs, rest) = parse_factor(&tokens[1..])?;
            let expr = Box::new(Expr::Mod(lhs, rhs));
            parse_term_rhs(expr, rest)
        }
        _ => Ok((lhs, tokens)),
    }
}

fn parse_factor<'a>(tokens: &'a [Token]) -> ParseResult<'a> {
    if tokens.is_empty() {
        return Err(String::from("Expected a factor"));
    }

    match tokens.first().unwrap() {
        Token::Number(n) => {
            let expr = Box::new(Expr::Number(*n));
            Ok((expr, &tokens[1..]))
        },
        Token::Variable(x) => {
            let expr = Box::new(Expr::Variable(x.clone()));
            Ok((expr, &tokens[1..]))
        },
        Token::OpenPar => {
            let (result, rest) = parse_expr(&tokens[1..])?;
            match rest.first() {
                Some(Token::ClosePar) => Ok((result, &rest[1..])),
                _ => Err(String::from("Expected closing parenthesis")),
            }
        }
        token => Err(format!("Unexpected token '{:?}'", token)),
    }
}