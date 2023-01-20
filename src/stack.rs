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

        self[index] = data.to_be();

        self.offset += 2;
    }

    pub fn pop(&mut self) -> u16 {
        if self.offset < 2 {
            self.offset = 2
        }
        self.offset -= 2;

        let index = self.offset as usize;

        let ret = u16::from_be(self[index]);

        self[index] = 0;

        ret
    }
}

use core::ops::{ Index, IndexMut };

impl Index<usize> for Stack {
    type Output = u16;

    fn index(&self, rhs: usize) -> &u16 {
        if rhs >= 256 {
            panic!("index out of bounds: the len is 256 but the index is {}", rhs);
        }

        unsafe {
            let reference = &*(&mut super::MEM[(self.location as usize) - rhs] as *mut u8 as *mut u16);
            return reference;
        }
    }
}

impl IndexMut<usize> for Stack {
    fn index_mut(&mut self, rhs: usize) -> &mut u16 {
        if rhs >= 256 {
            panic!("index out of bounds: the len is 256 but the index is {}", rhs);
        }
        unsafe{
            let reference = &mut *(&mut super::MEM[(self.location as usize) - rhs] as *mut u8 as *mut u16);
            return reference;
        }
    }
}