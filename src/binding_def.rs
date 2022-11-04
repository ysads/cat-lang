use crate::env::Env;
use crate::expr::Expr;
use crate::utils;

#[derive(Debug, PartialEq)]
pub struct BindingDef {
    name: String,
    val: Expr,
}

impl BindingDef {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("let", s)?;
        let (s, _) = utils::extract_whitespaces(s);

        let (s, name) = utils::extract_id(s)?;
        let (s, _) = utils::extract_whitespaces(s);

        let s = utils::tag("=", s)?;
        let (s, _) = utils::extract_whitespaces(s);

        let (s, val) = Expr::new(s)?;

        Ok((
            s,
            Self {
                name: name.to_string(),
                val,
            },
        ))
    }

    pub(crate) fn eval(&self, env: &mut Env) {
        env.add_binding(self.name.clone(), self.val.eval())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Number, Op};

    #[test]
    fn parse_binding_def_with_ascii_id() {
        assert_eq!(
            BindingDef::new("let a = 10 /2"),
            Ok((
                "",
                BindingDef {
                    name: "a".to_string(),
                    val: Expr::Operation {
                        lhs: Number(10),
                        rhs: Number(2),
                        op: Op::Div
                    }
                }
            ))
        )
    }

    #[test]
    fn parse_binding_def_with_unicode_id() {
        assert_eq!(
            BindingDef::new("let id_愛    =3   * 5"),
            Ok((
                "",
                BindingDef {
                    name: "id_愛".to_string(),
                    val: Expr::Operation {
                        lhs: Number(3),
                        rhs: Number(5),
                        op: Op::Mul
                    }
                }
            ))
        )
    }
}
