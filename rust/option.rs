use rust::zero;

pub enum Option<T> {
    Some(T),
    None
}

impl<T> Option<T> {
    #[inline(always)]
    pub fn is_some(&const self) -> bool {
        match *self {
            Some(_) => true,
            None => false
        }
    }

    #[inline(always)]
    pub fn is_none(&const self) -> bool {
        !self.is_some()
    }

    #[inline]
    pub fn get(self) -> T {
        match self {
            Some(x) => return x,
            None => unsafe { zero::abort() }
        }
    }
}
