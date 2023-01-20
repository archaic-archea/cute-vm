/* 
| Instr  | Stack desc                | Desc                            |
| ------ | ------------------------- | ------------------------------- |
| `lit`  | ( -- x )                  | push the next byte to the stack |
| `dup`  | ( x -- x x )              | duplicate the top of stack      |
| `over` | ( x y z -- z x y )        | standard stack over             |
| `str`  | ( addr -- value )         | load data into memory           |
| `load` | ( value addr -- )         | write data into memory          |
| `push` | ( value -- )              | write to other stack            |
| `jsr`  | ( addr -- ) [ -- retaddr] | jump to the address             |
|        |                           |                                 |
*/

use num_derive::{FromPrimitive, ToPrimitive};

#[derive(FromPrimitive)]
pub enum Instr {
    Lit = 0,
    Dup,
    Over,
    Str,
    Load,
    Push,
    Jsr
}

#[derive(FromPrimitive, ToPrimitive, Clone, Copy, Debug)]
pub enum Status {
    Keep = 0x1,
    Return = 0x2,
    Short = 0x4,
    Reserved1 = 0x8,
    Reserved2 = 0x16,
    Reserved3 = 0x32,
    Reserved4 = 0x64,
    Reserved5 = 0x128,
}

impl Instr {
    pub fn read_instr() -> Instr {
        let instr_ptr = unsafe {super::MEM.read_u16(0x400)};
        let instr = unsafe {super::MEM.read_u16(instr_ptr as usize)};

        num::FromPrimitive::from_u16(instr).expect("Invalid opcode")
    }
}

union StatusUnion {
    f: Status,
    s: u8
}

impl Status {
    pub fn to_vec(&self) -> Vec<Status> {
        let mut vec = Vec::new();

        let union = StatusUnion {f: *self};
        for i in 0..8 {
            unsafe {
                let result = (union.s >> i) & 1;

                if result == 1 {
                    let parsed_union = StatusUnion {s: result << i};

                    vec.push(parsed_union.f);
                }
            }
        }

        vec
    }
}

use core::ops::{Add, Sub};

impl Add for Status {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        unsafe {
            let first = StatusUnion {f: self};
            let second = StatusUnion {f: other};

            let su = StatusUnion {s: first.s + second.s};
        
            return su.f
        }
    }
}

impl Sub for Status {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        unsafe {
            let first = StatusUnion {f: self};
            let second = StatusUnion {f: other};

            let su = StatusUnion {s: first.s - second.s};
            
            return su.f
        }
    }
}