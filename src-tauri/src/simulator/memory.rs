use std::{
    mem::MaybeUninit,
    ops::{Index, IndexMut},
};

use crate::utility::ptr::Ptr;

pub const MAX_MEMORY_SIZE: usize = 1 << 32;
pub const PAGE_SIZE: usize = 4096;
pub const PAGE_TABLE_ENTRY_SIZE: usize = 4;
pub const PAGE_TABLE_ENTRY_COUNT: usize = PAGE_SIZE / PAGE_TABLE_ENTRY_SIZE;
pub const PAGE_TABLE_COUNT: usize = MAX_MEMORY_SIZE / PAGE_SIZE;
pub const FIRST_PAGE_SHIFT: usize = 22;
pub const SECOND_PAGE_SHIFT: usize = 12;

pub type FirstPageTable = [Option<Box<SecondPageTable>>; PAGE_TABLE_ENTRY_COUNT];
pub type SecondPageTable = [Option<Box<Page>>; PAGE_TABLE_ENTRY_COUNT];
pub type Page = [u8; PAGE_SIZE];

#[derive(Clone)]
pub struct Memory {
    page_table: FirstPageTable,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            page_table: unsafe {
                let mut page_table: [MaybeUninit<Option<Box<SecondPageTable>>>;
                    PAGE_TABLE_ENTRY_COUNT] = MaybeUninit::uninit().assume_init();
                for ele in &mut page_table {
                    ele.write(None);
                }
                std::mem::transmute::<_, FirstPageTable>(page_table)
            },
        }
    }

    pub fn reset(&mut self) {
        for first_page_table in &mut self.page_table {
            first_page_table.take();
        }
    }

    pub fn get_range(&self, start: u32, len: u32) -> Vec<u8> {
        if len == 0 {
            return Vec::new();
        }
        let start = start as usize;
        let end = start + len as usize;
        let self_ptr = Ptr::new(self);
        let mut_self = self_ptr.as_mut();
        let mut result = Vec::with_capacity(len as usize);
        let start_page = Self::align(start);
        let end_page = Self::align(end);
        if start_page == end_page {
            let start = Self::to_index(start);
            let end = Self::to_index(end);
            mut_self.build_page_table(start);
            result.extend_from_slice(&self.get_page(start)[start.2..end.2]);
        } else {
            {
                let idx = Self::to_index(start);
                mut_self.build_page_table(idx);
                result.extend_from_slice(&self.get_page(idx)[idx.2..]);
            }
            let mut i = start_page + PAGE_SIZE;
            while i < end_page {
                let index = Self::to_index(i);
                mut_self.build_page_table(index);
                result.extend_from_slice(self.get_page(index));
                i += PAGE_SIZE;
            }
            {
                let idx = Self::to_index(end);
                mut_self.build_page_table(idx);
                result.extend_from_slice(&self.get_page(idx)[..idx.2]);
            }
        }
        result
    }

    pub fn set_range(&mut self, start: u32, data: &[u8]) {
        if data.is_empty() {
            return;
        }
        let mut data_idx = 0;
        let start = start as usize;
        let end = start + data.len();
        let start_page = Self::align(start);
        let end_page = Self::align(end);
        unsafe {
            if start_page == end_page {
                let idx = Self::to_index(start);
                (&mut *(self as *const Memory as *mut Memory)).build_page_table(idx);
                self.get_page_mut(idx)[idx.2..idx.2 + data.len()].clone_from_slice(data);
            } else {
                {
                    let idx = Self::to_index(start);
                    (&mut *(self as *const Memory as *mut Memory)).build_page_table(idx);
                    let len = PAGE_SIZE - idx.2;
                    self.get_page_mut(idx)[idx.2..].clone_from_slice(&data[..len]);
                    data_idx += len;
                }
                let start_page = Self::align(start) + PAGE_SIZE;
                let end_page = Self::align(end);
                let mut i = start_page;
                while i < end_page {
                    let index = Self::to_index(i);
                    (&mut *(self as *const Memory as *mut Memory)).build_page_table(index);
                    self.get_page_mut(index)
                        .clone_from_slice(&data[data_idx..data_idx + PAGE_SIZE]);
                    data_idx += PAGE_SIZE;
                    i += PAGE_SIZE;
                }
                {
                    let idx = Self::to_index(end);
                    (&mut *(self as *const Memory as *mut Memory)).build_page_table(idx);
                    self.get_page_mut(idx)[..idx.2].clone_from_slice(&data[data_idx..]);
                }
            }
        }
    }

    fn build_page_table(&mut self, index: (usize, usize, usize)) {
        let first_page_table = &mut self.page_table;
        if first_page_table[index.0].is_none() {
            first_page_table[index.0] = Some(Box::new(unsafe {
                let mut page_table: [MaybeUninit<Option<Box<Page>>>; PAGE_TABLE_ENTRY_COUNT] =
                    MaybeUninit::uninit().assume_init();
                for ele in &mut page_table {
                    ele.write(None);
                }
                std::mem::transmute::<_, SecondPageTable>(page_table)
            }));
        }
        let second_page_table = first_page_table[index.0].as_mut().unwrap();
        if second_page_table[index.1].is_none() {
            second_page_table[index.1] = Some(Box::new([0; PAGE_SIZE]));
        }
    }

    fn to_index(index: usize) -> (usize, usize, usize) {
        (
            index >> FIRST_PAGE_SHIFT,
            (index >> SECOND_PAGE_SHIFT) & (PAGE_TABLE_ENTRY_COUNT - 1),
            index & (PAGE_SIZE - 1),
        )
    }

    fn get(&self, index: (usize, usize, usize)) -> &u8 {
        &self.page_table[index.0].as_ref().unwrap()[index.1]
            .as_ref()
            .unwrap()[index.2]
    }

    fn get_mut(&mut self, index: (usize, usize, usize)) -> &mut u8 {
        &mut self.page_table[index.0].as_mut().unwrap()[index.1]
            .as_mut()
            .unwrap()[index.2]
    }

    fn get_page(&self, index: (usize, usize, usize)) -> &Page {
        &self.page_table[index.0].as_ref().unwrap()[index.1]
            .as_ref()
            .unwrap()
    }

    fn get_page_mut(&mut self, index: (usize, usize, usize)) -> &mut Page {
        let page = &mut self.page_table[index.0].as_mut().unwrap()[index.1];
        return page.as_mut().unwrap();
    }

    fn align(index: usize) -> usize {
        index & !(PAGE_SIZE - 1)
    }
}

impl Index<u32> for Memory {
    type Output = u8;

    fn index(&self, index: u32) -> &Self::Output {
        let index = Self::to_index(index as usize);
        Ptr::new(self).as_mut().build_page_table(index);
        self.get(index)
    }
}

impl IndexMut<u32> for Memory {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        let index = Self::to_index(index as usize);
        self.build_page_table(index);
        self.get_mut(index)
    }
}
