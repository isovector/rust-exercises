use std::collections::HashMap;

use Expr::*;
use BinOp::*;

pub enum Expr {
    Lit(f64),
    Var(String),
    Bin(BinOp, Box<Expr>, Box<Expr>),
}

pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl Expr {
    pub fn eval(self, vars: &HashMap<String, f64>) -> f64 {
        match self {
            Lit(n) => n,
            Var(v) => vars[&v],
            Bin(op, l, r) => match op {
                Add => l.eval(vars) + r.eval(vars),
                Sub => l.eval(vars) - r.eval(vars),
                Mul => l.eval(vars) * r.eval(vars),
                Div => l.eval(vars) / r.eval(vars),
            }
        }
    }
}
