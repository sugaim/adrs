mod expr;
mod private;
mod scalar;
mod var;

pub use expr::Expr;
pub use var::{Var, VarFactory};

#[cfg(test)]
mod tests {
    use num_traits::Zero;

    use super::*;
    #[test]
    fn it_works() {
        let mut vf = VarFactory::new("x");
        let x = vf.gen(4.2).into_expr();
        let y = vf.gen(2.5).into_expr();
        let z = vf.gen(3.1).into_expr();

        let mut res: Expr<f64> = Zero::zero();
        for _ in 0..100000 {
            res += -x.clone() * y.clone() * x.clone() * 3.1 - z.clone() * 3.2;
        }
        // println!("{:?}", res);
        // println!("{:?}", res.inputs());
        println!("{:?}", res.output());
        println!("{:?}", res.grads());
    }
}
