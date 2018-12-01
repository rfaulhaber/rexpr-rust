use std::error;
use std::fmt;

enum NodeVal {
    Number(f64),
    Node(Box<Node>),
    None,
}

struct Node {
    left: NodeVal,
    right: NodeVal,
    op: Operator,
}

#[derive(Debug, Clone)]
struct EvalError {
    details: String,
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for EvalError{
    fn description(&self) -> &str {
        "invalid first item to double"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl NodeVal {
    fn evaluate(&self) -> Result<f64, EvalError> {
        return match self {
            NodeVal::Number(f) => Result::Ok(*f),
            NodeVal::Node(n) => n.evaluate(),
            NodeVal::None => Result::Err(EvalError {details: String::from("cannot be evaluated")} )
        }
    }
}

impl Node {
    fn evaluate(&self) -> Result<f64, EvalError> {
        if let NodeVal::None = self.right {
            return self.left.evaluate();
        } else if let NodeVal::None = self.left {
            return self.right.evaluate();
        } else {
            let left = match self.left.evaluate() {
                Result::Ok(f) => f,
                Result::Err(err) => return Result::Err(err),
            };

            let right = match self.right.evaluate() {
                Result::Ok(f) => f,
                Result::Err(err) => return Result::Err(err),
            };

            return match self.op {
                Operator::ADD => Result::Ok(left + right),
                Operator::SUB => Result::Ok(left - right),
                Operator::MULT => Result::Ok(left * right),
                Operator::DIV => Result::Ok(left / right),
                Operator::IDIV => Result::Ok((left / right).floor()),
                Operator::EXP => Result::Ok(left.powf(right)),
                Operator::FACT => Result::Ok(fact_from_float(left))
            }
        }
    }

    fn reparent(&self, right: &Node, op: Operator) -> &Node {
        return Node{
            left: NodeVal::Node(Box::new(*self)),
            right: NodeVal::Node(Box::new(*right)),
            op: op
        }.to_owned();
    }

    // fn parse(s: String) -> Node {
    //     let mut left: String;
    //     let mut right: String;

    //     let stack: Vec<String> = Vec::new();
    //     let ret = &Node { };

    //     for token in s.split_whitespace() {
    //         if token.parse::f64().is_ok() {
    //             stack.push(token);
    //         } else if (str_to_op(token).is_some()) {

    //         }
    //     }

    //     return Node {left: NodeVal::None, right: NodeVal:: None, op: Operator::ADD};
    // }
}

fn fact_from_float(f: f64) -> f64 {
    if f <= 1.0 {
        f
    } else {
        f * fact_from_float(f - 1.0)
    }
}

enum Operator {
    ADD,
    SUB,
    MULT,
    DIV,
    IDIV,
    EXP,
    FACT,
}

fn str_to_op(s: String) -> Option<Operator> {
    return match s.as_ref() {
        "+" => Some(Operator::ADD),
        "-" => Some(Operator::SUB),
        "*" => Some(Operator::MULT),
        "/" => Some(Operator::DIV),
        "//" => Some(Operator::IDIV),
        "^" => Some(Operator::EXP),
        "!" => Some(Operator::FACT),
        _ => None,
    };
}

fn op_to_str(o: Operator) -> Option<String> {
    return match o {
        Operator::ADD => Some(String::from("+")),
        Operator::SUB => Some(String::from("-")),
        Operator::MULT => Some(String::from("*")),
        Operator::DIV => Some(String::from("/")),
        Operator::IDIV => Some(String::from("//")),
        Operator::EXP => Some(String::from("^")),
        Operator::FACT => Some(String::from("!")),
    };
}

mod tests {
    use super::*;

    #[test]
    fn node_one_level_evaluates() {
        let n = Node {
            left: NodeVal::Number(5.0),
            right: NodeVal::Number(1.0),
            op: Operator::ADD
        };

        let result = n.evaluate().unwrap();

        assert_eq!(result, 6.0);
    }
}
