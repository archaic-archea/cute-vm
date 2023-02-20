/* 
| Instr  | Stack desc                | Desc                            |
| ------ | ------------------------- | ------------------------------- |
| `nop`  | ( -- )                    | Does nothing                    |
| `lit`  | ( -- x )                  | push the next byte to the stack |
| `dup`  | ( x -- x x )              | duplicate the top of stack      |
| `over` | ( x y z -- z x y )        | standard stack over             |
| `str`  | ( addr -- value )         | load data into memory           |
| `load` | ( value addr -- )         | write data into memory          |
| `push` | ( value -- )              | write to other stack            |
| `jsr`  | ( addr -- ) [ -- retaddr] | jump to the address             |
*/

use num_derive::{FromPrimitive, ToPrimitive};

#[derive(FromPrimitive)]
pub enum Instr {
    Nop = 0,
    Lit,
    Dup,
    Over,
    Str,
    Load,
    Push,
    Jsr
}

#[derive(FromPrimitive, ToPrimitive, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Status {
    Keep = 0x1, //Copies values off stack instead of popping them
    Return = 0x2,
    Short = 0x4, //Allows 32 bit operations
    Reserved1 = 0x8,
    Reserved2 = 0x16,
    Reserved3 = 0x32,
    Reserved4 = 0x64,
    Reserved5 = 0x128,
    None = 0x00
}

impl Instr {
    pub fn read_instr() -> Instr {
        let instr_ptr = unsafe {super::MEM.read_u16(0x400)};
        let instr = unsafe {super::MEM.read_u16(instr_ptr as usize)};

        num::FromPrimitive::from_u16(instr).expect("Invalid opcode")
    }

    pub fn execute(&self, flags: &Vec<Status>) {
        let instr_ptr = unsafe {&mut super::MEM.read_u16(0x400)};

        match self {
            Instr::Nop => (),
            Instr::Lit => {
                let data = unsafe {MEM[(*instr_ptr + 2) as usize]};

                let mut tmpflags = Vec::new();
                if flags.contains(&Status::Return) {tmpflags.push(Status::Return)}

                crate::push(data as u32, &tmpflags);

                if flags.contains(&Status::Short) {
                    *instr_ptr += 1;
                }
            },
            Instr::Dup => {
                let mut tmpflags = flags.clone();
                tmpflags.push(Status::Keep);

                let data = pop(&tmpflags);

                if tmpflags.contains(&Status::Return) {
                    for i in (0..tmpflags.len()).rev() {
                        if tmpflags[i as usize] == Status::Return {
                            tmpflags.remove(i as usize);
                        }
                    }
                } else {
                    tmpflags.push(Status::Return);
                }

                push(data, &tmpflags);
            },
            Instr::Over => {
                let mut tmpflags = flags.clone();

                if tmpflags.contains(&Status::Keep) {
                    for i in (0..tmpflags.len()).rev() {
                        if tmpflags[i as usize] == Status::Return {
                            tmpflags.remove(i as usize);
                        }
                    }
                }

                let mut pop_buf = Vec::new();

                pop_buf.push(pop(flags));
                pop_buf.push(pop(flags));
                pop_buf.push(pop(flags));

                let len = pop_buf.len();

                let top = *pop_buf.last().unwrap();
                let bottom = pop_buf[0];

                pop_buf[0] = top;
                pop_buf[len - 1] = bottom;

                for entry in pop_buf.iter() {
                    push(*entry, flags)
                }
            },
            Instr::Str => (),
            Instr::Load => (),
            Instr::Push => (),
            Instr::Jsr => ()
        }

        *instr_ptr += 2;
    }
}

impl Status {
    pub const fn null() -> Status {
        unsafe {
            StatusUnion {s: 0}.f
        }
    }
}

union StatusUnion {
    f: Status,
    s: u8
}

use core::ops::{Add, Sub};

use crate::{MEM, pop, push};

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