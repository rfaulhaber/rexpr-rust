use std::error::Error;
use std::fmt;

trait Evaluable {}

enum Expr {
    Binary(Box<BinaryExpr>),
    Unary(Box<UnaryExpr>),
    Float(f64),
    Integer(i64),
}

struct BinaryExpr {
    l: Expr,
    r: Expr,
    op: Operation,
}

struct UnaryExpr {
    l: Expr,
    op: Operation,
}

#[derive(Debug)]
struct ParseError {
    details: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.details
    }
}

fn expr_from_string(s: String) -> Result<Expr, ParseError> {
    let tokens: Vec<&str> = s.split_whitespace().collect();
    let mut temp: Vec<String> = Vec::new();

    for token in tokens {
        if let Some(op) = str_to_op(String::from(token)) {}
    }

    return Result::Ok(Expr::Integer(1));
}

enum Operation {
    ADD,
    SUB,
    MULT,
    DIV,
    IDIV,
    EXP,
    FACT,
}

fn str_to_op(s: String) -> Option<Operation> {
    return match s.as_ref() {
        "+" => Some(Operation::ADD),
        "-" => Some(Operation::SUB),
        "*" => Some(Operation::MULT),
        "/" => Some(Operation::DIV),
        "//" => Some(Operation::IDIV),
        "^" => Some(Operation::EXP),
        "!" => Some(Operation::FACT),
        _ => None,
    };
}

fn op_to_str(o: Operation) -> Option<String> {
    return match o {
        Operation::ADD => Some(String::from("+")),
        Operation::SUB => Some(String::from("-")),
        Operation::MULT => Some(String::from("*")),
        Operation::DIV => Some(String::from("/")),
        Operation::IDIV => Some(String::from("//")),
        Operation::EXP => Some(String::from("^")),
        Operation::FACT => Some(String::from("!")),
    };
}
