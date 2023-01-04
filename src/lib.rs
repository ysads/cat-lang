mod binding_def;
mod env;
mod expr;
mod statement;
mod utils;
mod val;

use env::Env;
use val::Val;

struct Parse(statement::Statement);

impl Parse {
    fn eval(&self, env: &mut Env) -> Result<Val, String> {
        self.0.eval(env)
    }
}

fn parse(s: &str) -> Result<Parse, String> {
    let (s, stmt) = statement::Statement::new(s)?;

    if s.is_empty() {
        Ok(Parse(stmt))
    } else {
        Err("Input was not fully consumed".to_string())
    }
}
