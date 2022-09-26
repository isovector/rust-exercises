mod expr;

use std::collections::HashMap;

use crate::expr::*;
use Expr::*;
use BinOp::*;


fn main() {
    let ast =
        Bin(Div,
            Box::new(
                Bin(Mul,
                    Box::new(
                        Bin(Add,
                            Box::new(Var(String::from("x"))),
                            Box::new(Lit(2.)))),
                    Box::new(Var(String::from("y"))))),
            Box::new(Lit(4.)));
    let vals = HashMap::from([(String::from("x"), 1_f64), (String::from("y"), 3_f64)]);
    let res = ast.eval(&vals);
    print!("{res}")
}
