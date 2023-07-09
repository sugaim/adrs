use std::collections::{BTreeMap, VecDeque};

use derivative::Derivative;
use num_traits::{One, Zero};

use crate::{private::_Expr, scalar::Scalar, var::Id, Var};

#[derive(Clone, Derivative)]
#[derivative(Debug = "transparent")]
pub struct Expr<T>(_Expr<T>);

impl<T> From<Var<T>> for Expr<T> {
    #[inline]
    fn from(val: Var<T>) -> Self {
        Self(val.into())
    }
}
impl<T: Scalar> From<f64> for Expr<T> {
    #[inline]
    fn from(val: f64) -> Self {
        Expr::constant(val.into())
    }
}
impl<T> From<_Expr<T>> for Expr<T> {
    #[inline]
    fn from(e: _Expr<T>) -> Self {
        Self(e)
    }
}

impl<T> Expr<T> {
    #[inline]
    pub fn constant(val: T) -> Self {
        _Expr::constant(val).into()
    }
    #[inline]
    pub fn output(&self) -> &T {
        self.0.output()
    }
    #[inline]
    pub fn as_var(&self) -> Option<&Var<T>> {
        self.0.as_var()
    }
    #[inline]
    pub(crate) fn _take(mut self) -> _Expr<T> {
        std::mem::replace(&mut self.0, _Expr::_OnlyForDrop)
    }
    #[inline]
    pub(crate) fn _is_const(&self) -> bool {
        self.0.is_const()
    }
}

impl<T: Scalar> Expr<T> {
    #[inline]
    pub fn compress(&mut self) {
        let compressed = _Expr::Compressed {
            g: self.0.generation(),
            o: self.output().clone(),
            gs: self.grads(),
        };
        *self = compressed.into();
    }
    #[inline]
    pub fn grads(&self) -> BTreeMap<Id, T> {
        self.grads_with_seed(T::one())
    }
    #[inline]
    pub fn grads_with_seed(&self, seed: T) -> BTreeMap<Id, T> {
        self.0.grads(seed)
    }
}
impl<T> AsRef<T> for Expr<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self.output()
    }
}

impl<T: PartialEq> PartialEq for Expr<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.output().eq(other.output())
    }
}
impl<T: Eq> Eq for Expr<T> {}

impl<T: PartialOrd> PartialOrd for Expr<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.output().partial_cmp(other.output())
    }
}
impl<T: Ord> Ord for Expr<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.output().cmp(other.output())
    }
}

impl<T: Scalar + PartialOrd> Zero for Expr<T> {
    #[inline]
    fn zero() -> Self {
        Self::constant(T::zero())
    }
    #[inline]
    fn is_zero(&self) -> bool {
        self.output().is_zero()
    }
}
impl<T: Scalar + PartialOrd> One for Expr<T> {
    #[inline]
    fn one() -> Self {
        Self::constant(T::one())
    }
    #[inline]
    fn is_one(&self) -> bool {
        self.output().is_one()
    }
}

impl<T> Drop for Expr<T> {
    fn drop(&mut self) {
        // Expression tree is realized with recursion.
        // hence, naive drop leads to stack overflow.
        // To avoid this, we implement explicitly and use for-loop instead of recursion.
        if matches!(self.0, _Expr::Leaf(..))
            || matches!(self.0, _Expr::_OnlyForDrop)
            || matches!(self.0, _Expr::Compressed { .. })
        {
            return;
        }

        let mut exprs = VecDeque::new();
        exprs.push_back(std::mem::replace(&mut self.0, _Expr::_OnlyForDrop));
        while let Some(n) = &mut exprs.pop_back() {
            let _Expr::Node(_, n) = n else {
                continue;
            };
            n._take_expr_to_back_for_drop(&mut exprs);
        }
    }
}
