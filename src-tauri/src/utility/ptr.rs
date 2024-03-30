use std::hash::Hash;

pub struct Ptr<'a, T>(&'a T);

impl<'a, T> Clone for Ptr<'_, T> {
    fn clone(&self) -> Self {
        Ptr(self.0)
    }
}

impl<'a, T> Copy for Ptr<'_, T> {}

impl<'a, T> PartialEq for Ptr<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}

impl<'a, T> Eq for Ptr<'_, T> {}

impl<'a, T> PartialEq<&T> for Ptr<'_, T> {
    fn eq(&self, other: &&T) -> bool {
        std::ptr::eq(self.0, *other)
    }
}

impl<'a, T> PartialOrd for Ptr<'_, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, T> Ord for Ptr<'_, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.0 as *const T as usize).cmp(&(other.0 as *const T as usize))
    }
}

impl<'a, T> Hash for Ptr<'_, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.0, state)
    }
}

impl<'a, T> Ptr<'a, T> {
    pub fn new(t: &'a T) -> Self {
        Ptr(t)
    }

    pub fn as_ref(&self) -> &T {
        self.0
    }

    pub fn as_mut(&self) -> &mut T {
        unsafe { (self.0 as *const T as *mut T).as_mut().unwrap() }
    }
}
