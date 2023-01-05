mod binding_usage;
mod block;

use crate::env::Env;
use crate::utils;
use crate::val::Val;

pub(crate) use binding_usage::BindingUsage;
pub(crate) use block::Block;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Number(pub(crate) i32);

impl Number {
    fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, digits) = utils::extract_digits(s)?;

        Ok((s, Self(digits.parse().unwrap())))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn new(s: &str) -> Result<(&str, Self), String> {
        utils::tag("+", s)
            .map(|s| (s, Self::Add))
            .or_else(|_| utils::tag("-", s).map(|s| (s, Self::Sub)))
            .or_else(|_| utils::tag("*", s).map(|s| (s, Self::Mul)))
            .or_else(|_| utils::tag("/", s).map(|s| (s, Self::Div)))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Expr {
    Number(Number),
    Operation {
        lhs: Box<Self>,
        rhs: Box<Self>,
        op: Op,
    },
    BindingUsage(BindingUsage),
    Block(Block),
}

impl Expr {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        Self::new_operation(s).or_else(|_| Self::new_non_operation(s))
    }

    fn new_non_operation(s: &str) -> Result<(&str, Self), String> {
        Self::new_number(s)
            .or_else(|_| Self::new_binding_usage(s))
            .or_else(|_| Self::new_block(s))
    }

    fn new_operation(s: &str) -> Result<(&str, Self), String> {
        let (s, lhs) = Self::new_non_operation(s)?;
        let (s, _) = utils::extract_whitespaces(s);

        let (s, op) = Op::new(s)?;
        let (s, _) = utils::extract_whitespaces(s);

        let (s, rhs) = Self::new_non_operation(s)?;

        Ok((
            s,
            Self::Operation {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op,
            },
        ))
    }

    fn new_number(s: &str) -> Result<(&str, Self), String> {
        Number::new(s).map(|(s, number)| (s, Self::Number(number)))
    }

    fn new_binding_usage(s: &str) -> Result<(&str, Self), String> {
        BindingUsage::new(s).map(|(s, binding_usage)| (s, Self::BindingUsage(binding_usage)))
    }

    fn new_block(s: &str) -> Result<(&str, Self), String> {
        Block::new(s).map(|(s, block)| (s, Self::Block(block)))
    }

    pub(crate) fn eval(&self, env: &Env) -> Result<Val, String> {
        match self {
            Self::Number(Number(n)) => Ok(Val::Number(*n)),
            Self::Operation { lhs, rhs, op } => {
                let lhs = lhs.eval(env)?;
                let rhs = rhs.eval(env)?;

                let (lhs, rhs) = match (lhs, rhs) {
                    (Val::Number(lhs), Val::Number(rhs)) => (lhs, rhs),
                    _ => return Err("Both lhs and rhs need to be numbers".to_string()),
                };

                let result = match op {
                    Op::Add => lhs + rhs,
                    Op::Sub => lhs - rhs,
                    Op::Div => lhs / rhs,
                    Op::Mul => lhs * rhs,
                };

                Ok(Val::Number(result))
            }
            Self::BindingUsage(binding_usage) => binding_usage.eval(env),
            Self::Block(block) => block.eval(env),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::Env;
    use crate::statement::Statement;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), Ok(("", Number(123))));
        assert_eq!(Number::new("123+2"), Ok(("+2", Number(123))));
    }

    #[test]
    fn parse_add_op() {
        assert_eq!(Op::new("+"), Ok(("", Op::Add)));
        assert_eq!(Op::new("+2"), Ok(("2", Op::Add)));
    }

    #[test]
    fn parse_sub_op() {
        assert_eq!(Op::new("-"), Ok(("", Op::Sub)));
        assert_eq!(Op::new("-2"), Ok(("2", Op::Sub)));
    }

    #[test]
    fn parse_mul_op() {
        assert_eq!(Op::new("*"), Ok(("", Op::Mul)));
        assert_eq!(Op::new("*2"), Ok(("2", Op::Mul)));
    }

    #[test]
    fn parse_div_op() {
        assert_eq!(Op::new("/"), Ok(("", Op::Div)));
        assert_eq!(Op::new("/2"), Ok(("2", Op::Div)));
    }

    #[test]
    fn parse_number_as_expr() {
        assert_eq!(Expr::new("456"), Ok(("", Expr::Number(Number(456)))))
    }

    #[test]
    fn parse_one_plus_one() {
        assert_eq!(
            Expr::new("1+2"),
            Ok((
                "",
                Expr::Operation {
                    lhs: Box::new(Expr::Number(Number(1))),
                    rhs: Box::new(Expr::Number(Number(2))),
                    op: Op::Add
                }
            ))
        )
    }

    #[test]
    fn parse_op_with_binding_usage() {
        assert_eq!(
            Expr::new("a *  b"),
            Ok((
                "",
                Expr::Operation {
                    lhs: Box::new(Expr::BindingUsage(BindingUsage {
                        name: "a".to_string()
                    })),
                    rhs: Box::new(Expr::BindingUsage(BindingUsage {
                        name: "b".to_string()
                    })),
                    op: Op::Mul,
                },
            )),
        );
    }

    #[test]
    fn parse_expr_with_whitespace() {
        assert_eq!(
            Expr::new("2 *    2"),
            Ok((
                "",
                Expr::Operation {
                    lhs: Box::new(Expr::Number(Number(2))),
                    rhs: Box::new(Expr::Number(Number(2))),
                    op: Op::Mul,
                },
            )),
        );
    }

    #[test]
    fn parse_binding_usage() {
        assert_eq!(
            Expr::new("bar"),
            Ok((
                "",
                Expr::BindingUsage(BindingUsage {
                    name: "bar".to_string()
                })
            ))
        )
    }

    #[test]
    fn parse_block() {
        assert_eq!(
            Expr::new("{ 200 }"),
            Ok((
                "",
                Expr::Block(Block {
                    statements: vec![Statement::Expr(Expr::Number(Number(200)))]
                })
            ))
        )
    }

    #[test]
    fn eval_static_expr() {
        assert_eq!(
            Expr::Number(Number(2)).eval(&Env::default()),
            Ok(Val::Number(2))
        )
    }

    #[test]
    fn eval_add() {
        assert_eq!(
            Expr::Operation {
                lhs: Box::new(Expr::Number(Number(2))),
                rhs: Box::new(Expr::Number(Number(3))),
                op: Op::Add
            }
            .eval(&Env::default()),
            Ok(Val::Number(5))
        )
    }

    #[test]
    fn eval_sub() {
        assert_eq!(
            Expr::Operation {
                lhs: Box::new(Expr::Number(Number(12))),
                rhs: Box::new(Expr::Number(Number(5))),
                op: Op::Sub
            }
            .eval(&Env::default()),
            Ok(Val::Number(7))
        )
    }

    #[test]
    fn eval_div() {
        assert_eq!(
            Expr::Operation {
                lhs: Box::new(Expr::Number(Number(100))),
                rhs: Box::new(Expr::Number(Number(5))),
                op: Op::Div
            }
            .eval(&Env::default()),
            Ok(Val::Number(20))
        )
    }

    #[test]
    fn eval_mult() {
        assert_eq!(
            Expr::Operation {
                lhs: Box::new(Expr::Number(Number(10))),
                rhs: Box::new(Expr::Number(Number(2))),
                op: Op::Mul
            }
            .eval(&Env::default()),
            Ok(Val::Number(20))
        );
    }

    #[test]
    fn eval_binding_usage() {
        let mut env = Env::default();
        env.add_binding("foo".to_string(), Val::Number(10));

        assert_eq!(
            Expr::BindingUsage(BindingUsage {
                name: "foo".to_string()
            })
            .eval(&env),
            Ok(Val::Number(10))
        )
    }

    #[test]
    fn eval_block() {
        assert_eq!(
            Expr::Block(Block {
                statements: vec![Statement::Expr(Expr::Number(Number(10)))]
            })
            .eval(&Env::default()),
            Ok(Val::Number(10))
        )
    }

    #[test]
    fn eval_non_number_operation() {
        assert_eq!(
            Expr::Operation {
                lhs: Box::new(Expr::Number(Number(10))),
                rhs: Box::new(Expr::Block(Block {
                    statements: Vec::new()
                })),
                op: Op::Add,
            }
            .eval(&Env::default()),
            Err("Both lhs and rhs need to be numbers".to_string()),
        );
    }
}
