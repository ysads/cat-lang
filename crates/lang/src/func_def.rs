use crate::{statement::Statement, utils, Env};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FuncDef {
    pub(crate) name: String,
    pub(crate) params: Vec<String>,
    pub(crate) body: Box<Statement>,
}

impl FuncDef {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("fn", s)?;
        let (s, _) = utils::extract_whitespaces_1(s)?;

        let (s, name) = utils::extract_id(s)?;
        let (s, _) = utils::extract_whitespaces(s);

        let (s, params) = utils::sequence(
            |s| utils::extract_id(s).map(|(s, id)| (s, id.to_string())),
            s,
        )?;

        let s = utils::tag("=>", s)?;
        let (s, _) = utils::extract_whitespaces(s);

        let (s, body) = Statement::new(s)?;

        Ok((
            s,
            Self {
                name: name.to_string(),
                params,
                body: Box::new(body),
            },
        ))
    }

    pub(crate) fn eval(&self, env: &mut Env) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        expr::{BindingUsage, Block, Expr, Number, Op},
        Val,
    };

    #[test]
    fn parse_func_def_with_no_params_and_empty_body() {
        assert_eq!(
            FuncDef::new("fn nothing => {}"),
            Ok((
                "",
                FuncDef {
                    name: "nothing".to_string(),
                    params: Vec::new(),
                    body: Box::new(Statement::Expr(Expr::Block(Block {
                        statements: Vec::new()
                    }))),
                },
            )),
        );
    }

    #[test]
    fn parse_func_def_with_one_param_and_empty_body() {
        assert_eq!(
            FuncDef::new("fn greet name => {}"),
            Ok((
                "",
                FuncDef {
                    name: "greet".to_string(),
                    params: vec!["name".to_string()],
                    body: Box::new(Statement::Expr(Expr::Block(Block {
                        statements: Vec::new()
                    }))),
                },
            )),
        );
    }

    #[test]
    fn parse_func_def_with_multiple_params() {
        assert_eq!(
            FuncDef::new("fn add x y => x + y"),
            Ok((
                "",
                FuncDef {
                    name: "add".to_string(),
                    params: vec!["x".to_string(), "y".to_string()],
                    body: Box::new(Statement::Expr(Expr::Operation {
                        lhs: Box::new(Expr::BindingUsage(BindingUsage {
                            name: "x".to_string()
                        })),
                        rhs: Box::new(Expr::BindingUsage(BindingUsage {
                            name: "y".to_string()
                        })),
                        op: Op::Add
                    }))
                }
            ))
        )
    }

    #[test]
    fn eval_func_def() {
        assert_eq!(
            Statement::FuncDef(FuncDef {
                name: "eq_to_one".to_string(),
                params: Vec::new(),
                body: Box::new(Statement::Expr(Expr::Number(Number(1))))
            })
            .eval(&mut Env::default()),
            Ok(Val::Unit)
        )
    }
}
