use rust::zero;

pub enum Option<T> {
    Some(T),
    None
}

impl<T> Option<T> {
    #[inline(always)]
    pub fn is_some(&self) -> bool {
        match *self {
            Some(_) => true,
            None => false
        }
    }

    #[inline(always)]
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    #[inline]
    #[fixed_stack_segment]
    pub fn get(self) -> T {
        match self {
            Some(x) => return x,
            None => unsafe { zero::abort() }
        }
    }
}
