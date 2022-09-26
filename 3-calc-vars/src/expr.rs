use Expr::*;
use BinOp::*;

pub enum Expr {
    Lit(f64),
    Bin(BinOp, Box<Expr>, Box<Expr>),
}

pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl Expr {
    pub fn eval(self) -> f64 {
        match self {
            Lit(n) => n,
            Bin(op, l, r) => match op {
                Add => l.eval() + r.eval(),
                Sub => l.eval() - r.eval(),
                Mul => l.eval() * r.eval(),
                Div => l.eval() / r.eval(),
            }
        }
    }
}