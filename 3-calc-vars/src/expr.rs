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

impl BinOp {
    pub fn to_fn(self) -> fn(f64, f64) -> f64 {
        match self {
            Add => |lhs, rhs| lhs + rhs,
            Sub => |lhs, rhs| lhs - rhs,
            Mul => |lhs, rhs| lhs * rhs,
            Div => |lhs, rhs| lhs / rhs
        }
    }
}

impl Expr {
    pub fn eval(self, vars: &HashMap<String, f64>) -> f64 {
        match self {
            Lit(n) => n,
            Var(v) => vars[&v],
            Bin(op, l, r) => op.to_fn()(l.eval(vars), r.eval(vars)),
        }
    }
}
