mod expr;

use expr::eval;
use expr::Expr::{Add, Mul, Div, Lit};

fn main() {
    let res = eval(
        Div(
            Box::new(
                Mul(
                    Box::new(
                        Add(
                            Box::new(Lit(1.)),
                            Box::new(Lit(2.)))),
                    Box::new(Lit(3.)))),
            Box::new(Lit(4.))));
    print!("{res}")
}
