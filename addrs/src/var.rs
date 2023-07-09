use std::{borrow::Cow, rc::Rc, sync::Mutex};

use crate::Expr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id {
    pub group: usize,
    pub name: Cow<'static, str>,
    pub num: usize,
}

#[derive(Debug, Clone)]
pub struct Var<T> {
    val: T,
    id: Id,
}

impl<T> Var<T> {
    #[inline]
    pub fn id(&self) -> &Id {
        &self.id
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
pub struct VarGroup {
    id: usize,
    name: Cow<'static, str>,
    cnt: Rc<Mutex<usize>>,
}

impl VarGroup {
    pub fn new(name: impl Into<String>) -> Self {
        static ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let id = ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let name = name.into().into();
        let cnt = Rc::new(Mutex::new(0));
        Self { id, name, cnt }
    }

    #[inline]
    pub fn id(&self) -> usize {
        self.id
    }
    #[inline]
    pub fn name(&self) -> Cow<'static, str> {
        self.name.clone()
    }

    pub fn val<T>(&self, val: T) -> Var<T> {
        let mut cnt = self.cnt.lock().unwrap();
        let id = Id {
            group: self.id,
            name: self.name.clone(),
            num: *cnt,
        };
        *cnt += 1;
        Var { val, id }
    }
}
