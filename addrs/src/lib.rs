mod expr;
mod private;
mod scalar;
mod var;

pub use expr::Expr;
pub use var::{Var, VarGroup};

#[cfg(test)]
mod tests {
    use num_traits::Zero;

    use super::*;
    #[test]
    fn it_works() {
        let grp = VarGroup::new("x");
        let x = grp.val(4.2).into_expr();
        let y = grp.val(2.5).into_expr();
        let z = grp.val(3.1).into_expr();

        let mut res: Expr<f64> = Zero::zero();
        for _ in 0..100000 {
            res += -x.clone() * y.clone() * x.clone() * 3.1
                - z.clone()
                    * grp.val(1.0).into_expr()
                    * grp.val(1.0).into_expr()
                    * z.clone()
                    * grp.val(1.0).into_expr();
        }
        // println!("{:?}", res);
        // println!("{:?}", res.inputs());
        println!("{:?}", res.output());

        let stopwatch = std::time::Instant::now();
        res.compress();
        println!("{:?}", stopwatch.elapsed());
        let stopwatch = std::time::Instant::now();
        let grads = res.grads();
        println!("{:?}", stopwatch.elapsed());
        println!("{:?}", grads.len());
    }
}
