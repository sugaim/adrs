use std::{ops::Sub, rc::Rc};

use crate::{scalar::Scalar, Expr};

use super::{_BOp, _Binary};

impl<T> Sub for Expr<T>
where
    T: Scalar,
{
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let o = self.output().clone() - rhs.output();
        let l = Rc::new(self._take());
        let r = Rc::new(rhs._take());
        let op = _BOp::Sub;
        let is_cl = l.is_const();
        let is_cr = r.is_const();
        _Binary {
            l,
            r,
            o,
            op,
            is_cl,
            is_cr,
        }
        .into()
    }
}
