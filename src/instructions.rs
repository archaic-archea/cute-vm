/* 
| Instr  | Stack desc                 | Desc                            |
| ------ | -------------------------- | ------------------------------- |
| `nop`  | ( -- )                     | Does nothing                    |
| `lit`  | ( -- x )                   | push the next byte to the stack |
| `dup`  | ( x -- x ) [ -- x ]        | duplicate the top of stack      |
| `over` | ( x y z -- z x y )         | standard stack over             |
| `str`  | ( addr -- value )          | write data into memory          |
| `load` | ( value addr -- )          | load data from memory           |
| `push` | ( value -- ) [ -- value ]  | write to other stack            |
| `drop`  | ( value -- )               | Delete a value permanently      |
| `jsr`  | ( addr -- ) [ -- retaddr ] | jump to the address             |
*/

use num_derive::FromPrimitive;
use crate::{MEM, pop, push};

#[derive(FromPrimitive, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instr {
    Nop,
    Lit,
    Dup,
    Over,
    Str,
    Load,
    Push,
    Drop,
    Jsr
}

bitflags::bitflags! {
    pub struct Status: u8 {
        const KEEP = 0b1;
        const RETURN = 0b10;
        const SHORT = 0b100;
        const _RESERVED1 = 0b1000;
        const _RESERVED2 = 0b10000;
        const _RESERVED3 = 0b100000;
        const _RESERVED4 = 0b1000000;
        const _RESERVED5 = 0b10000000;
        const NONE = 0;
    }
}

impl Instr {
    pub fn read_instr() -> Instr {
        let instr_ptr = unsafe {super::MEM.read_u16(0x400)};
        let instr = unsafe {super::MEM.read_u16(instr_ptr as usize)};

        num::FromPrimitive::from_u16(instr).expect("Invalid opcode")
    }

    pub fn from_byte(byte: u8) -> Self {
        match byte {
            0 => Self::Nop,
            1 => Self::Lit,
            2 => Self::Dup,
            3 => Self::Over,
            4 => Self::Str,
            5 => Self::Load,
            6 => Self::Push,
            7 => Self::Drop,
            8 => Self::Jsr,
            _ => panic!("Invalid instruction 0b{:b}", byte)
        }
    }

    pub fn execute(&self, flags: Status) {
        let instr_ptr = unsafe {&mut super::MEM.read_u32(0x200)};

        match self {
            Instr::Nop => (),
            Instr::Lit => {
                let data: u32;
                if flags.contains(Status::SHORT) {
                    data = unsafe {MEM.read_u32((*instr_ptr + 2) as usize)};
                } else {
                    data = unsafe {MEM.read_u16((*instr_ptr + 2) as usize)} as u32;
                }

                crate::push(data as u32, flags);

                if flags.contains(Status::SHORT) {
                    *instr_ptr += 4;
                } else {
                    *instr_ptr += 2
                }
            },
            Instr::Dup => {
                let mut tmpflags = flags;
                tmpflags |= Status::KEEP;

                let data = pop(tmpflags);

                if tmpflags.contains(Status::RETURN) {
                    tmpflags.set(Status::RETURN, false);
                } else {
                    tmpflags |= Status::RETURN;
                }

                push(data, tmpflags);
            },
            Instr::Over => {
                let mut tmpflags = flags;

                tmpflags.set(Status::KEEP, false);

                let mut pop_buf = [0; 3];

                pop_buf[0] = pop(tmpflags);
                pop_buf[1] = pop(tmpflags);
                pop_buf[2] = pop(tmpflags);

                let top = pop_buf[2];
                let bottom = pop_buf[0];

                pop_buf[0] = top;
                pop_buf[2] = bottom;

                push(pop_buf[2], tmpflags);
                push(pop_buf[1], tmpflags);
                push(pop_buf[0], tmpflags);
            },
            Instr::Str => {
                let store_addr = pop(flags | Status::SHORT) as usize;
                let data = pop(flags);

                unsafe {
                    match flags.contains(Status::SHORT) {
                        true => {
                            MEM.write_u32(store_addr, data);
                        },
                        false => {
                            MEM.write_u16(store_addr, data as u16)
                        }
                    }
                }
            },
            Instr::Load => {
                let store_addr = pop(flags | Status::SHORT) as usize;

                unsafe {
                    match flags.contains(Status::SHORT) {
                        true => {
                            push(MEM.read_u32(store_addr), flags);
                        },
                        false => {
                            push(MEM.read_u16(store_addr) as u32, flags);
                        }
                    }
                }
            },
            Instr::Push => {
                let value = pop(flags);

                push(value, flags);
            },
            Instr::Drop => {
                let mut tmp_flags = flags;
                tmp_flags.set(Status::KEEP, false);

                pop(flags);
            },
            Instr::Jsr => {
                let old_ptr = *instr_ptr + 2;

                if flags.contains(Status::RETURN) {
                    let flag = Status::SHORT;
                    push(old_ptr, flag);

                    *instr_ptr = pop(flag | Status::RETURN);
                } else {
                    let flag = Status::RETURN | Status::SHORT;
                    push(old_ptr, flag);

                    *instr_ptr = pop(flag - Status::RETURN);
                }
            }
        }

        if *self != Instr::Jsr {
            *instr_ptr += 2;
        }
    }
}

pub struct Instruction(Instr, Status);

impl Instruction {
    pub const fn new(instr: Instr, status: Status) -> Self {
        Instruction(instr, status)
    }
    
    pub fn execute(&self) {
        self.0.execute(self.1)
    }
}