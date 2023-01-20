pub struct Stack {
    location: u16,
    offset: u16
}

impl Stack {
    pub fn new(location: u16) -> Stack {
        Stack { location, offset: 0}
    }

    pub fn push(&mut self, data: u32, flags: &Vec<Status>) {
        if self.offset >= 0x200 {
            panic!("Stack overflow");
        }

        let index = self.offset as usize;

        let bytes = data.to_be_bytes();

        self[index] = u16::from_be_bytes([bytes[0], bytes[1]]);

        self.offset += 2;

        if flags.contains(&Status::Short) {
            let index = self.offset as usize;

            self[index] = u16::from_be_bytes([bytes[0], bytes[1]]);
    
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
            let mbytes = self[index + 2].to_be_bytes();
            let lbytes = self[index].to_be_bytes();

            bytes = [mbytes[0], mbytes[1], lbytes[0], lbytes[1]];
            self[index] = 0;
            self[index + 2] = 0;
        } else {
            let lbytes = self[index].to_be_bytes();

            bytes = [0, 0, lbytes[0], lbytes[1]];
            self[index] = 0;
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
            let mbytes = self[index + 2].to_be_bytes();
            let lbytes = self[index].to_be_bytes();

            bytes = [mbytes[0], mbytes[1], lbytes[0], lbytes[1]];
        } else {
            let lbytes = self[index].to_be_bytes();

            bytes = [0, 0, lbytes[0], lbytes[1]];
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

use std::fmt;

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stack:\n")?;

        let range = 0..self.offset;

        for i in range.step_by(2) {
            write!(f, "{}\n", self.copy(i as usize, &vec![Status::None]))?;
        }

        Ok(())
    }
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stack:\n")?;

        let range = 0..self.offset;

        for i in range.step_by(4) {
            write!(f, "{}\n", self.copy(i as usize, &vec![Status::Short]))?;
        }

        Ok(())
    }
}