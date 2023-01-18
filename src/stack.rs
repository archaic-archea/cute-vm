use super::memory::Memory;

pub struct Stack {
    memory: Memory,
    location: u16,
    offset: u8
}

impl Stack {
    pub const fn null() -> Stack {
        Stack {
            memory: Memory::null(),
            location: 0,
            offset: 0
        }
    }

    pub fn new(memory: Memory, location: u16) -> Stack {
        Stack { memory, location, offset: 0}
    }

    pub fn push(&mut self, data: u8) {
        let index = self.offset as usize;

        self[index] = data;

        self.offset += 1;
    }

    pub fn pop(&mut self) -> u8 {
        self.offset -= 1;

        let index = self.offset as usize;

        let ret = self[index];

        self[index] = 0;

        ret
    }
}

use core::ops::{ Index, IndexMut };

impl Index<usize> for Stack {
    type Output = u8;

    fn index(&self, rhs: usize) -> &u8 {
        if rhs >= 256 {
            panic!("index out of bounds: the len is 256 but the index is {}", rhs);
        }

        &self.memory[(self.location as usize) - rhs]
    }
}

impl IndexMut<usize> for Stack {
    fn index_mut(&mut self, rhs: usize) -> &mut u8 {
        if rhs >= 256 {
            panic!("index out of bounds: the len is 256 but the index is {}", rhs);
        }

        &mut self.memory[(self.location as usize) - rhs]
    }
}