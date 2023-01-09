use crate::env::Env;
use crate::func_call::FuncCall;
use crate::utils;
use crate::val::Val;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct BindingUsage {
    pub(crate) name: String,
}

impl BindingUsage {
    pub(super) fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, name) = utils::extract_id(s)?;

        Ok((
            s,
            Self {
                name: name.to_string(),
            },
        ))
    }

    pub(super) fn eval(&self, env: &Env) -> Result<Val, String> {
        env.get_binding(&self.name).or_else(|err_msg| {
            if env.get_func(&self.name).is_ok() {
                FuncCall {
                    callee: self.name.clone(),
                    params: vec![],
                }
                .eval(env)
            } else {
                Err(err_msg)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        expr::{Expr, Number},
        statement::Statement,
    };

    use super::*;

    #[test]
    fn parse_binding_usage() {
        assert_eq!(
            BindingUsage::new("foo"),
            Ok((
                "",
                BindingUsage {
                    name: "foo".to_string()
                }
            ))
        )
    }

    #[test]
    fn eval_existing_binding_usage() {
        let mut env = Env::default();
        env.add_binding("foo".to_string(), Val::Number(10));

        assert_eq!(
            BindingUsage {
                name: "foo".to_string()
            }
            .eval(&env),
            Ok(Val::Number(10))
        )
    }

    #[test]
    fn eval_func_call_with_no_params() {
        let mut env = Env::default();
        env.add_func(
            "fn_no_param".to_string(),
            vec![],
            Statement::Expr(Expr::Number(Number(2))),
        );

        assert_eq!(
            BindingUsage {
                name: "fn_no_param".to_string()
            }
            .eval(&env),
            Ok(Val::Number(2))
        )
    }

    #[test]
    fn fail_to_eval_non_existing_binding_usage() {
        let empty_env = Env::default();

        assert_eq!(
            BindingUsage {
                name: "unknown".to_string()
            }
            .eval(&empty_env),
            Err("Binding with name `unknown` not found".to_string())
        )
    }
}
