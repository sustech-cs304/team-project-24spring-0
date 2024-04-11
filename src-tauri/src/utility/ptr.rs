use std::hash::Hash;

#[derive(Debug)]
pub struct Ptr<T>(*mut T);

impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        Ptr(self.0)
    }
}

impl<T> Copy for Ptr<T> {}

impl<T> PartialEq for Ptr<T> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}

impl<T> Eq for Ptr<T> {}

impl<T> PartialEq<&T> for Ptr<T> {
    fn eq(&self, other: &&T) -> bool {
        std::ptr::eq(self.0, *other)
    }
}

impl<T> PartialOrd for Ptr<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Ptr<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.0 as *const T as usize).cmp(&(other.0 as *const T as usize))
    }
}

impl<T> Hash for Ptr<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.0, state)
    }
}

unsafe impl<T> Send for Ptr<T> {}
unsafe impl<T> Sync for Ptr<T> {}

impl<T> Ptr<T> {
    pub fn new(t: &T) -> Self {
        Ptr(t as *const T as *mut T)
    }

    pub fn as_ref(&self) -> &T {
        unsafe { &*self.0 }
    }

    pub fn as_mut(&self) -> &mut T {
        unsafe { &mut *self.0 }
    }
}
