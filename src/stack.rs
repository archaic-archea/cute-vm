pub struct Stack {
    location: u16,
    offset: u16
}

impl Stack {
    pub fn new(location: u16) -> Stack {
        Stack { location, offset: 0}
    }

    pub fn push(&mut self, data: u16) {
        if self.offset >= 0x200 {
            panic!("Stack overflow");
        }

        let index = self.offset as usize;

        self[index] = data.to_be_bytes()[0];
        self[index + 1] = data.to_be_bytes()[1];

        self.offset += 2;
    }

    pub fn pop(&mut self) -> u16 {
        if self.offset < 2 {
            self.offset = 2
        }
        self.offset -= 2;

        let index = self.offset as usize;

        let ret = u16::from_be_bytes([self[index], self[index + 1]]);

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

        unsafe {
            return &super::MEM[(self.location as usize) - rhs];
        }
    }
}

impl IndexMut<usize> for Stack {
    fn index_mut(&mut self, rhs: usize) -> &mut u8 {
        if rhs >= 256 {
            panic!("index out of bounds: the len is 256 but the index is {}", rhs);
        }
        unsafe{
            return &mut super::MEM[(self.location as usize) - rhs];
        }
    }
}