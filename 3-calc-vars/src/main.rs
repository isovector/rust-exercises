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
                            Box::new(Var(String::from("hello"))),
                            Box::new(Lit(2.)))),
                    Box::new(Lit(3.)))),
            Box::new(Lit(4.)));
    let vals = HashMap::from([(String::from("hello"), 1_f64)]);
    let res = ast.eval(&vals);
    print!("{res}")
}
