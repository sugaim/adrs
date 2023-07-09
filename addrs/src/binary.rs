mod add;
mod div;
mod mul;
mod sub;

use std::{
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    rc::Rc,
};

use crate::{
    expr::{_Expr, _Leaf, _Node},
    scalar::Scalar,
    Expr,
};

#[derive(Debug, Clone)]
enum _BOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub(crate) struct _Binary<T> {
    l: Rc<_Expr<T>>,
    r: Rc<_Expr<T>>,
    o: T,
    is_cl: bool,
    is_cr: bool,
    op: _BOp,
}

impl<T> _Binary<T> {
    #[inline]
    pub fn output(&self) -> &T {
        &self.o
    }
    #[inline]
    pub fn is_const_each(&self) -> (bool, bool) {
        (self.is_cl, self.is_cr)
    }
    #[inline]
    pub fn _ref_expr_for_drop(&mut self) -> (Option<&mut _Expr<T>>, Option<&mut _Expr<T>>) {
        let _Binary { l, r, .. } = self;
        (Rc::get_mut(l), Rc::get_mut(r))
    }
}
impl<T: Scalar> _Binary<T> {
    fn _adj_der_l(&self, grad: T) -> T {
        match &self.op {
            _BOp::Add => grad,
            _BOp::Sub => grad,
            _BOp::Mul => grad * self.r.output(),
            _BOp::Div => grad / self.r.output(),
        }
    }
    fn _adj_der_r(&self, grad: T) -> T {
        match &self.op {
            _BOp::Add => grad,
            _BOp::Sub => -grad,
            _BOp::Mul => grad * self.l.output(),
            _BOp::Div => -grad * &self.o / self.r.output(),
        }
    }
    pub fn backward_l(&self, grad: T) -> (&_Expr<T>, T) {
        (&self.l, self._adj_der_l(grad))
    }
    pub fn backward_r(&self, grad: T) -> (&_Expr<T>, T) {
        (&self.r, self._adj_der_r(grad))
    }
}

impl<T> From<_Binary<T>> for Expr<T> {
    #[inline]
    fn from(b: _Binary<T>) -> Self {
        let _Binary { l, r, .. } = &b;
        let g = l.generation().max(r.generation()) + 1;
        _Expr::Node(g, _Node::Binary(b)).into()
    }
}

macro_rules! define_binary_operations {
    ($trait:ident, $func:ident) => {
        impl<T> $trait<Expr<T>> for &Expr<T>
        where
            T: Scalar,
        {
            type Output = Expr<T>;
            #[inline]
            fn $func(self, rhs: Expr<T>) -> Self::Output {
                $trait::$func(self.clone(), rhs)
            }
        }
        impl<T> $trait<&Self> for Expr<T>
        where
            T: Scalar,
        {
            type Output = Self;
            #[inline]
            fn $func(self, rhs: &Self) -> Self::Output {
                $trait::$func(self, rhs.clone())
            }
        }
        impl<T> $trait<&Self> for &Expr<T>
        where
            T: Scalar,
        {
            type Output = Expr<T>;
            #[inline]
            fn $func(self, rhs: &Self) -> Self::Output {
                $trait::$func(self.clone(), rhs.clone())
            }
        }
        impl<T> $trait<T> for Expr<T>
        where
            T: Scalar,
        {
            type Output = Self;
            #[inline]
            fn $func(self, rhs: T) -> Self::Output {
                $trait::$func(self, Expr::from(_Expr::Leaf(_Leaf::Const(rhs))))
            }
        }
        impl<T> $trait<&T> for Expr<T>
        where
            T: Scalar,
        {
            type Output = Self;
            #[inline]
            fn $func(self, rhs: &T) -> Self::Output {
                $trait::$func(self, rhs.clone())
            }
        }
        impl<T> $trait<T> for &Expr<T>
        where
            T: Scalar,
        {
            type Output = Expr<T>;
            #[inline]
            fn $func(self, rhs: T) -> Self::Output {
                $trait::$func(self.clone(), rhs)
            }
        }
        impl<T> $trait<&T> for &Expr<T>
        where
            T: Scalar,
        {
            type Output = Expr<T>;
            #[inline]
            fn $func(self, rhs: &T) -> Self::Output {
                $trait::$func(self.clone(), rhs.clone())
            }
        }
    };
}

define_binary_operations!(Add, add);
define_binary_operations!(Sub, sub);
define_binary_operations!(Mul, mul);
define_binary_operations!(Div, div);

macro_rules! define_binary_assign_operations {
    ($trait:ident, $func:ident, $op:ident) => {
        impl<T> $trait<Self> for Expr<T>
        where
            T: Scalar,
        {
            #[inline]
            fn $func(&mut self, rhs: Expr<T>) {
                let this = self.clone();
                *self = this.$op(rhs);
            }
        }
        impl<T> $trait<&Self> for Expr<T>
        where
            T: Scalar,
        {
            #[inline]
            fn $func(&mut self, rhs: &Expr<T>) {
                self.$func(rhs.clone());
            }
        }
    };
}
define_binary_assign_operations!(AddAssign, add_assign, add);
define_binary_assign_operations!(SubAssign, sub_assign, sub);
define_binary_assign_operations!(MulAssign, mul_assign, mul);
define_binary_assign_operations!(DivAssign, div_assign, div);
