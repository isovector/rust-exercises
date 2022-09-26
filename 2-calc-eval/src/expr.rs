pub enum Expr {
    Lit(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

pub fn eval(e: Expr) -> f64 {
    match e {
        Expr::Lit(f) => f,
        Expr::Add(lhs, rhs) => eval(*lhs) + eval(*rhs),
        Expr::Sub(lhs, rhs) => eval(*lhs) - eval(*rhs),
        Expr::Mul(lhs, rhs) => eval(*lhs) * eval(*rhs),
        Expr::Div(lhs, rhs) => eval(*lhs) / eval(*rhs),
    }
}
