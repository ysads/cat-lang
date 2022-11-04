use crate::utils;
use crate::val::Val;

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
    Div,
}

impl Op {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, op) = utils::extract_op(s);

        let op = match op {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => panic!("Op:: bad operator"),
        };

        (s, op)
    }
}

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub lhs: Number,
    pub rhs: Number,
    pub op: Op,
}

impl Expr {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, lhs) = Number::new(s);
        let (s, _) = utils::extract_whitespaces(s);
        let (s, op) = Op::new(s);
        let (s, _) = utils::extract_whitespaces(s);
        let (s, rhs) = Number::new(s);

        (s, Self { lhs, rhs, op })
    }

    pub(crate) fn eval(&self) -> Val {
        let Number(lhs) = self.lhs;
        let Number(rhs) = self.rhs;

        let result = match self.op {
            Op::Add => lhs + rhs,
            Op::Sub => lhs - rhs,
            Op::Div => lhs / rhs,
            Op::Mul => lhs * rhs,
        };

        Val::Number(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn parse_expr_with_whitespace() {
        assert_eq!(
            Expr::new("2 *    2"),
            (
                "",
                Expr {
                    lhs: Number(2),
                    rhs: Number(2),
                    op: Op::Mul,
                },
            ),
        );
    }

    #[test]
    fn eval_add() {
        let (_, expr) = Expr::new("2 + 2");

        assert_eq!(expr.eval(), Val::Number(4))
    }

    #[test]
    fn eval_sub() {
        let (_, expr) = Expr::new("12 - 2");

        assert_eq!(expr.eval(), Val::Number(10))
    }

    #[test]
    fn eval_div() {
        let (_, expr) = Expr::new("20 / 2");

        assert_eq!(expr.eval(), Val::Number(10))
    }

    #[test]
    fn eval_mult() {
        let (_, expr) = Expr::new("10 * 2");

        assert_eq!(expr.eval(), Val::Number(20))
    }
}
