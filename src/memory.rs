pub struct Memory {
    base: *mut u8,
    size: usize
}

use alloc::alloc::Layout;

impl Memory {
    pub const fn null() -> Memory {
        Memory { base: core::ptr::null_mut(), size: 0 }
    }

    pub fn new(size: usize) -> Memory {
        unsafe {
            Memory { 
                base: alloc::alloc::alloc_zeroed(Layout::from_size_align_unchecked(size, 0)), 
                size 
            }
        }
    }
}

use core::ops::{ Index, IndexMut };

impl Index<usize> for Memory {
    type Output = u8;

    fn index(&self, rhs: usize) -> &u8 {
        if rhs >= self.size {
            panic!("index out of bounds: the len is {} but the index is {}", self.size, rhs);
        }

        unsafe { return &*self.base.add(rhs); }
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, rhs: usize) -> &mut u8 {
        if rhs > self.size {
            panic!("index out of bounds: the len is {} but the index is {}", self.size, rhs);
        }
        
        unsafe { return &mut *self.base.add(rhs); }
    }
}

unsafe impl Send for Memory {} 
unsafe impl Sync for Memory {}