#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct AnyU8 {
    pub type_id: std::any::TypeId,
    pub raw_val: u8,
}

impl AnyU8 {
    pub fn from<T>(e: T) -> Self
    where
        T: Clone + Copy + 'static,
    {
        assert!(std::mem::size_of::<T>() == 1, "Size of E must be 1");
        unsafe {
            AnyU8 {
                type_id: std::any::TypeId::of::<T>(),
                raw_val: *(&e as *const T as *const u8),
            }
        }
    }

    pub fn is<T>(&self) -> bool
    where
        T: 'static,
    {
        self.type_id == std::any::TypeId::of::<T>()
    }

    pub fn to<T>(&self) -> Option<T>
    where
        T: Clone + Copy + 'static,
    {
        if self.is::<T>() {
            Some(unsafe { *(&self.raw_val as *const u8 as *const T) })
        } else {
            None
        }
    }
}

impl std::fmt::Debug for AnyU8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnyU8")
            .field("type_id", &self.type_id)
            .field("raw_val", &self.raw_val)
            .finish()
    }
}
