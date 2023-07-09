use std::borrow::Cow;

use crate::Expr;

#[derive(Debug, Clone)]
pub struct Var<T> {
    val: T,
    name: Cow<'static, str>,
    id: usize,
}

impl<T> Var<T> {
    #[inline]
    pub fn ident(&self) -> (&str, usize) {
        (self.name.as_ref(), self.id)
    }
    #[inline]
    pub fn val(&self) -> &T {
        &self.val
    }
    #[inline]
    pub fn into_expr(self) -> Expr<T> {
        self.into()
    }
}

#[derive(Debug, Clone)]
pub struct VarFactory {
    name: Cow<'static, str>,
    cnt: usize,
}

impl VarFactory {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into().into(),
            cnt: 0,
        }
    }
    pub fn gen<T>(&mut self, val: T) -> Var<T> {
        let id = self.cnt;
        self.cnt += 1;
        Var {
            val,
            name: self.name.clone(),
            id,
        }
    }
}
