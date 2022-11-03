mod utils;

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, digits) = utils::extract_digits(s);

        (s, Self(digits.parse().unwrap()))
    }
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div
}

impl Op {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, op) = utils::extract_op(s);

        let op = match op {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => panic!("Op:: bad operator")
        };

        (s, op)
    }
}

struct Point { x: i32, y: i32 }

#[derive(Debug, PartialEq)]
pub struct Expr {
    lhs: Number,
    rhs: Number,
    op: Op
}

impl Expr {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, lhs) = Number::new(s);
        let (s, op) = Op::new(s);
        let (s, rhs) = Number::new(s);

        (s, Self { lhs, rhs, op })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutability() {

    }

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), ("", Number(123)));
        assert_eq!(Number::new("123+2"), ("+2", Number(123)));
    }

    #[test]
    fn parse_add_op() {
        assert_eq!(Op::new("+"), ("", Op::Add));
        assert_eq!(Op::new("+2"), ("2", Op::Add));
    }

    #[test]
    fn parse_sub_op() {
        assert_eq!(Op::new("-"), ("", Op::Sub));
        assert_eq!(Op::new("-2"), ("2", Op::Sub));
    }

    #[test]
    fn parse_mul_op() {
        assert_eq!(Op::new("*"), ("", Op::Mul));
        assert_eq!(Op::new("*2"), ("2", Op::Mul));
    }

    #[test]
    fn parse_div_op() {
        assert_eq!(Op::new("/"), ("", Op::Div));
        assert_eq!(Op::new("/2"), ("2", Op::Div));
    }

    #[test]
    fn parse_one_plus_one() {
        assert_eq!(
            Expr::new("1+2"),
            (
                "",
                Expr {
                    lhs: Number(1),
                    rhs: Number(2),
                    op: Op::Add
                }
            )
        )
    }
}