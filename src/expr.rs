use combine::*;

enum Expr {
    Binary(Box<BinaryExpr>),
    Unary(Box<UnaryExpr>),
    Number(f64),
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
