use std::net::ToSocketAddrs;

use crate::{expr::Expr, utils, Env, Val};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FuncCall {
    pub(crate) callee: String,
    pub(crate) params: Vec<Expr>,
}

impl FuncCall {
    pub(super) fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, callee) = utils::extract_id(s)?;
        let (s, _) = utils::take_while(|c| c == ' ', s);

        let (s, params) = utils::sequence_1(Expr::new, |s| utils::take_while(|c| c == ' ', s), s)?;

        Ok((
            s,
            Self {
                callee: callee.to_string(),
                params,
            },
        ))
    }

    pub(crate) fn eval(&self, env: &Env) -> Result<Val, String> {
        let mut child_env = env.create_child();

        let (param_names, body) = env.get_func(&self.callee)?;

        let num_expected_params = param_names.len();
        let num_given_args = self.params.len();

        if num_expected_params != num_given_args {
            return Err(format!(
                "Function `{}` expected {} args but received {}",
                self.callee, num_expected_params, num_given_args
            ));
        }

        for (param_name, param_expr) in param_names.into_iter().zip(&self.params) {
            let param_val = param_expr.eval(&child_env)?;
            child_env.add_binding(param_name, param_val);
        }
        body.eval(&mut child_env)
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::Number;

    use super::*;

    #[test]
    #[ignore]
    fn parse_func_call_with_no_params() {
        assert_eq!(
            FuncCall::new("say_hi"),
            Ok((
                "",
                FuncCall {
                    callee: "say_hi".to_string(),
                    params: Vec::new(),
                },
            )),
        );
    }

    #[test]
    fn parse_func_call_with_one_parameter() {
        assert_eq!(
            FuncCall::new("factorial 10"),
            Ok((
                "",
                FuncCall {
                    callee: "factorial".to_string(),
                    params: vec![Expr::Number(Number(10))],
                },
            )),
        );
    }

    #[test]
    fn parse_func_call_with_multiple_parameters() {
        assert_eq!(
            FuncCall::new("sum 10 20"),
            Ok((
                "",
                FuncCall {
                    callee: "sum".to_string(),
                    params: vec![Expr::Number(Number(10)), Expr::Number(Number(20))],
                },
            )),
        );
    }
}
