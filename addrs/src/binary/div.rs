use std::{ops::Div, rc::Rc};

use crate::{scalar::Scalar, Expr};

use super::{_BOp, _Binary};

impl<T> Div for Expr<T>
where
    T: Scalar,
{
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        let o = self.output().clone() / rhs.output();
        let l = Rc::new(self._take());
        let r = Rc::new(rhs._take());
        let op = _BOp::Div;
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
