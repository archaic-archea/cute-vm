pub struct Stack {
    location: u16,
    offset: u16
}

impl Stack {
    pub const fn new(location: u16) -> Stack {
        Stack { location, offset: 0}
    }

    pub unsafe fn set_offset(&mut self, offset: u16) {
        self.offset = offset;
    }

    pub fn push(&mut self, data: u32, flags: &Vec<Status>) {
        if self.offset >= 0x200 {
            panic!("Stack overflow");
        }
        
        let base = self.location as usize;

        let index = self.offset as usize;

        let bytes: [u8; 4] = data.to_le_bytes();

        self[index] = bytes[0];
        self[index + 1] = bytes[1];

        self.offset += 2;

        if flags.contains(&Status::Short) {
            let index = self.offset as usize;

            self[index] = bytes[2];
            self[index + 1] = bytes[3];
    
            self.offset += 2;
        }
    }

    pub fn pop(&mut self, flags: &Vec<Status>) -> u32 {
        if self.offset < 2 {
            self.offset = 2
        }
        self.offset -= 2;

        if flags.contains(&Status::Short) {
            if self.offset < 2 {
                self.offset = 2
            }
            self.offset -= 2;
        }

        let index = self.offset as usize;

        let bytes: [u8; 4];


        if flags.contains(&Status::Short) {
            let mbytes = [self[index + 2], self[index + 3]];
            let lbytes = [self[index], self[index + 1]];

            bytes = [mbytes[1], mbytes[0], lbytes[1], lbytes[0]];
            self[index] = 0;
            self[index + 1] = 0;
            self[index + 2] = 0;
            self[index + 3] = 0;
        } else {
            let lbytes = [self[index], self[index + 1]];

            bytes = [0, 0, lbytes[1], lbytes[0]];
            self[index] = 0;
            self[index + 1] = 0;
        }

        let ret = u32::from_be_bytes(bytes);

        if flags.contains(&Status::Keep) {
            self.push(ret, &flags);
        }

        ret
    }

    pub fn copy(&self, index: usize, flags: &Vec<Status>) -> u32 {
        let bytes: [u8; 4];


        if flags.contains(&Status::Short) {
            let mbytes = [self[index + 2], self[index + 3]];
            let lbytes = [self[index], self[index + 1]];

            bytes = [mbytes[1], mbytes[0], lbytes[1], lbytes[0]];
        } else {
            let lbytes = [self[index], self[index + 1]];

            bytes = [0, 0, lbytes[1], lbytes[0]];
        }

        let ret = u32::from_be_bytes(bytes);

        ret
    }

    pub fn top(&self) -> usize {
        self.offset as usize
    }
}

use core::ops::{ Index, IndexMut };

use crate::instructions::Status;

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
        if rhs >= 128 {
            panic!("index out of bounds: the len is 256 but the index is {}", rhs);
        }

        unsafe{
            return &mut super::MEM[(self.location as usize) - rhs];
        }
    }
}

use std::fmt;

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stack:\n")?;

        let range = 0..self.offset;

        for i in range.step_by(2) {
            write!(f, "{}", self.copy(i as usize, &vec![Status::None]))?;
            if i != self.offset - 2 {
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stack:\n")?;

        let range = 0..self.offset;

        for i in range.step_by(4) {
            write!(f, "{}", self.copy(i as usize, &vec![Status::Short]))?;
            if i != self.offset - 4 {
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}