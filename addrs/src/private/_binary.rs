mod add;
mod div;
mod mul;
mod sub;

use std::{
    collections::VecDeque,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    rc::Rc,
};

use crate::{scalar::Scalar, Expr};

use super::{_expr::_Expr, _node::_Node};

#[derive(Debug, Clone)]
enum _BOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
enum _In<T> {
    L(Rc<_Expr<T>>),
    R(Rc<_Expr<T>>),
    LR { l: Rc<_Expr<T>>, r: Rc<_Expr<T>> },
}

#[derive(Debug, Clone)]
pub(crate) struct _Binary<T> {
    i: _In<T>,
    o: T,
    gl: T,
    gr: T,
    #[allow(dead_code)]
    op: _BOp, // debug purpose
}

impl<T> _Binary<T> {
    fn create(l: Expr<T>, r: Expr<T>, o: T, gl: T, gr: T, op: _BOp) -> Expr<T> {
        if l._is_const() && r._is_const() {
            return Expr::constant(o);
        }
        let (gen, i) = match (l._is_const(), r._is_const()) {
            (true, false) => {
                let r = Rc::new(r._take());
                (r.generation() + 1, _In::R(r))
            }
            (false, true) => {
                let l = Rc::new(l._take());
                (l.generation() + 1, _In::L(l))
            }
            _ => {
                let l = Rc::new(l._take());
                let r = Rc::new(r._take());
                (l.generation().max(r.generation()) + 1, _In::LR { l, r })
            }
        };
        let b = _Binary { i, o, gl, gr, op };
        _Expr::Node(gen, _Node::Binary(b)).into()
    }

    #[inline]
    pub fn output(&self) -> &T {
        &self.o
    }
    #[inline]
    pub fn _ref_expr_for_drop(&mut self) -> (Option<&mut _Expr<T>>, Option<&mut _Expr<T>>) {
        match &mut self.i {
            _In::L(l) => (Rc::get_mut(l), None),
            _In::R(r) => (None, Rc::get_mut(r)),
            _In::LR { l, r } => (Rc::get_mut(l), Rc::get_mut(r)),
        }
    }
}

impl<T: Scalar> _Binary<T> {
    pub fn push_grads<'a>(&'a self, grads: &mut VecDeque<(&'a _Expr<T>, T)>, grad: T) {
        match &self.i {
            _In::L(l) => {
                if !l.is_const() {
                    grads.push_back((l, grad * &self.gl))
                }
            }
            _In::R(r) => {
                if !r.is_const() {
                    grads.push_back((r, grad * &self.gr))
                }
            }
            _In::LR { l, r } => match (l.is_const(), r.is_const()) {
                (true, false) => grads.push_back((r, grad * &self.gr)),
                (false, true) => grads.push_back((l, grad * &self.gl)),
                (false, false) => {
                    grads.push_back((l, grad.clone() * &self.gl));
                    grads.push_back((r, grad * &self.gr));
                }
                _ => {}
            },
        }
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
                $trait::$func(self, Expr::constant(rhs))
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
