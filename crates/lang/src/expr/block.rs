use crate::env::Env;
use crate::statement::Statement;
use crate::utils;
use crate::val::Val;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Block {
    pub(crate) statements: Vec<Statement>,
}

impl Block {
    pub(super) fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("{", s)?;
        let (s, _) = utils::extract_whitespaces(s);

        let (s, statements) = utils::sequence(Statement::new, s)?;

        let (s, _) = utils::extract_whitespaces(s);
        let s = utils::tag("}", s)?;

        Ok((
            s,
            Block {
                statements: statements,
            },
        ))
    }

    pub(super) fn eval(&self, env: &Env) -> Result<Val, String> {
        if self.statements.is_empty() {
            return Ok(Val::Unit);
        }

        let mut env = env.create_child();

        let statements_but_last = &self.statements[..self.statements.len() - 1];
        for stmt in statements_but_last {
            stmt.eval(&mut env)?;
        }

        self.statements.last().unwrap().eval(&mut env)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::super::{BindingUsage, Expr, Number};
    use super::*;
    use crate::binding_def::BindingDef;
    use crate::expr::Op;
    use crate::statement::Statement;

    #[test]
    fn parse_empty_blocks() {
        assert_eq!(
            Block::new("{}"),
            Ok((
                "",
                Block {
                    statements: Vec::new()
                }
            ))
        );
        assert_eq!(
            Block::new("{ }"),
            Ok((
                "",
                Block {
                    statements: Vec::new()
                }
            ))
        );
    }

    #[test]
    fn parse_block_with_one_statement() {
        assert_eq!(
            Block::new("{ 5  }"),
            Ok((
                "",
                Block {
                    statements: vec![Statement::Expr(Expr::Number(Number(5)))]
                }
            ))
        )
    }

    #[test]
    fn parse_block_with_multiple_statements() {
        assert_eq!(
            Block::new(
                "{
            let a = 10
            let b = a
            b
        }"
            ),
            Ok((
                "",
                Block {
                    statements: vec![
                        Statement::BindingDef(BindingDef {
                            name: "a".to_string(),
                            val: Expr::Number(Number(10))
                        }),
                        Statement::BindingDef(BindingDef {
                            name: "b".to_string(),
                            val: Expr::BindingUsage(BindingUsage {
                                name: "a".to_string()
                            })
                        }),
                        Statement::Expr(Expr::BindingUsage(BindingUsage {
                            name: "b".to_string()
                        }))
                    ]
                }
            ))
        )
    }

    #[test]
    fn eval_empty_block() {
        assert_eq!(
            Block { statements: vec![] }.eval(&Env::default()),
            Ok(Val::Unit)
        )
    }

    #[test]
    fn eval_block_with_one_expr() {
        assert_eq!(
            Block {
                statements: vec![Statement::Expr(Expr::Number(Number(10)))]
            }
            .eval(&Env::default()),
            Ok(Val::Number(10))
        )
    }

    #[test]
    fn eval_block_with_binding_def_and_usage() {
        assert_eq!(
            Block {
                statements: vec![
                    Statement::BindingDef(BindingDef {
                        name: "foo".to_string(),
                        val: Expr::Number(Number(1))
                    }),
                    Statement::Expr(Expr::BindingUsage(BindingUsage {
                        name: "foo".to_string(),
                    }))
                ]
            }
            .eval(&Env::default()),
            Ok(Val::Number(1))
        )
    }

    #[test]
    fn eval_block_with_multiple_binding_defs() {
        assert_eq!(
            Block {
                statements: vec![
                    Statement::BindingDef(BindingDef {
                        name: "foo".to_string(),
                        val: Expr::Number(Number(1))
                    }),
                    Statement::BindingDef(BindingDef {
                        name: "bar".to_string(),
                        val: Expr::Number(Number(2))
                    })
                ]
            }
            .eval(&Env::default()),
            Ok(Val::Unit)
        )
    }

    #[test]
    fn eval_block_with_multiple_exprs() {
        assert_eq!(
            Block {
                statements: vec![
                    Statement::Expr(Expr::Number(Number(100))),
                    Statement::Expr(Expr::Operation {
                        lhs: Box::new(Expr::Number(Number(3))),
                        rhs: Box::new(Expr::Number(Number(7))),
                        op: Op::Mul
                    })
                ]
            }
            .eval(&Env::default()),
            Ok(Val::Number(21))
        )
    }

    #[test]
    fn eval_block_using_bindings_from_parent_env() {
        let mut env = Env::default();
        env.add_binding("foo".to_string(), Val::Number(5));

        assert_eq!(
            Block {
                statements: vec![
                    Statement::BindingDef(BindingDef {
                        name: "bar".to_string(),
                        val: Expr::BindingUsage(BindingUsage {
                            name: "foo".to_string()
                        })
                    }),
                    Statement::Expr(Expr::BindingUsage(BindingUsage {
                        name: "bar".to_string()
                    }))
                ]
            }
            .eval(&env),
            Ok(Val::Number(5))
        )
    }
}
