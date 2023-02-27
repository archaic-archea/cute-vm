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

    pub fn vm_read_u16(&self, index: usize) -> u16 {
        println!("Reading 0x{:x}", index);

        if index & 0b1 != 0 {
            INT_CONTROLLER.lock().unwrap().gen_int(0, true);

            println!("VM u16 address not aligned 0x{:x}", index);

            return 0;
        }

        self.read_u16(index)
    }

    pub fn read_u16(&self, index: usize) -> u16 {
        let bytes = [
            self[index],
            self[index + 1],
        ];

        u16::from_le_bytes(bytes)
    }

    pub fn vm_read_u32(&self, index: usize) -> u32 {
        println!("Reading 0x{:x}", index);

        if index & 0b11 != 0 {
            INT_CONTROLLER.lock().unwrap().gen_int(0, true);

            println!("VM u32 address not aligned 0x{:x}", index);

            return 0;
        }

        self.read_u32(index)
    }

    pub fn read_u32(&self, index: usize) -> u32 {

        let bytes = [
            self[index],
            self[index + 1],
            self[index + 2],
            self[index + 3],
        ];

        u32::from_le_bytes(bytes)
    }

    pub fn read_u64(&self, index: usize) -> u64 {
        assert!(index & 0b111 == 0, "Address for long not aligned");

        let bytes = [
            self[index],
            self[index + 1],
            self[index + 2],
            self[index + 3],
            self[index + 4],
            self[index + 5],
            self[index + 6],
            self[index + 7],
        ];

        u64::from_le_bytes(bytes)
    }

    pub fn vm_write_u16(&mut self, index: usize, num: u16) {
        println!("Writing 0x{:x} to 0x{:x}", num, index);

        if index & 0b1 != 0 {
            println!("u16 address not aligned");

            INT_CONTROLLER.lock().unwrap().gen_int(1, true);
        }

        self.write_u16(index, num);
    }

    pub fn write_u16(&mut self, index: usize, num: u16) {
        let bytes = num.to_le_bytes();

        self[index] = bytes[0];
        self[index + 1] = bytes[1];
    }

    pub fn vm_write_u32(&mut self, index: usize, num: u32) {
        println!("Writing 0x{:x} to 0x{:x}", num, index);

        if index & 0b11 != 0 {
            println!("u32 address not aligned");

            INT_CONTROLLER.lock().unwrap().gen_int(1, true);
        }

        self.write_u32(index, num);
    }

    pub fn write_u32(&mut self, index: usize, num: u32) {
        let bytes = num.to_le_bytes();

        self[index] = bytes[0];
        self[index + 1] = bytes[1];
        self[index + 2] = bytes[2];
        self[index + 3] = bytes[3];
    }

    pub fn write_u64(&mut self, index: usize, num: u64) {
        assert!(index & 0b111 == 0, "Address for long not aligned");

        let bytes = num.to_le_bytes();

        self[index] = bytes[0];
        self[index + 1] = bytes[1];
        self[index + 2] = bytes[2];
        self[index + 3] = bytes[3];
        self[index + 4] = bytes[4];
        self[index + 5] = bytes[5];
        self[index + 6] = bytes[6];
        self[index + 7] = bytes[7];
    }
}

use core::ops::{ Index, IndexMut };

use crate::INT_CONTROLLER;

impl Index<usize> for Memory {
    type Output = u8;

    fn index(&self, rhs: usize) -> &u8 {
        if rhs >= self.size {
            INT_CONTROLLER.lock().unwrap().gen_int(2, true);

            panic!("index out of bounds: the len is {} but the index is {}", self.size, rhs);
        }

        unsafe { return &*self.base.add(rhs); }
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, rhs: usize) -> &mut u8 {
        if rhs > self.size {
            INT_CONTROLLER.lock().unwrap().gen_int(2, true);

            panic!("index out of bounds: the len is {} but the index is {}", self.size, rhs);
        }
        
        unsafe { return &mut *self.base.add(rhs); }
    }
}

unsafe impl Send for Memory {} 
unsafe impl Sync for Memory {}