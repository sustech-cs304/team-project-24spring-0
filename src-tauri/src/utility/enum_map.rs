pub const MAP_SIZE: usize = u8::MAX as usize;

macro_rules! build_map_helper {
    ($data: expr, $val_fn: expr) => {
        unsafe {
            let mut map: Vec<V> = Vec::with_capacity(MAP_SIZE);
            map.set_len(MAP_SIZE);
            for ele in $data {
                let (key, val) = $val_fn(ele);
                let idx = *(&key as *const E as *const u8) as usize;
                std::ptr::write(&mut map[idx] as *mut _, val);
            }
            map
        }
    };
}

pub fn build_map<E, V, D, F>(data: &[D], val_fn: F) -> Vec<V>
where
    E: Copy,
    F: Fn(&D) -> (E, V),
{
    assert!(
        std::mem::size_of::<E>() == 1,
        "Size of {} must be 1",
        std::any::type_name::<E>()
    );
    build_map_helper!(data, val_fn)
}

pub fn build_map_mut_data<E, V, D, F>(data: &mut [D], val_fn: F) -> Vec<V>
where
    E: Copy,
    F: Fn(&mut D) -> (E, V),
{
    assert!(
        std::mem::size_of::<E>() == 1,
        "Size of {} must be 1",
        std::any::type_name::<E>()
    );
    build_map_helper!(data, val_fn)
}

pub struct EnumMap<E, V>
where
    E: Copy,
{
    pub map: Vec<V>,
    _phantom: std::marker::PhantomData<E>,
}

impl<E, V> EnumMap<E, V>
where
    E: Copy,
{
    pub fn new<D, F>(data: &[D], val_fn: F) -> Self
    where
        F: Fn(&D) -> (E, V),
    {
        Self {
            map: build_map(data, val_fn),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn new_mut_data<D, F>(data: &mut [D], val_fn: F) -> Self
    where
        F: Fn(&mut D) -> (E, V),
    {
        Self {
            map: build_map_mut_data(data, val_fn),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn get(&self, key: E) -> &V {
        unsafe { &self.map[*(&key as *const E as *const u8) as usize] }
    }

    pub fn get_mut(&mut self, key: E) -> &mut V {
        unsafe { &mut self.map[*(&key as *const E as *const u8) as usize] }
    }
}
