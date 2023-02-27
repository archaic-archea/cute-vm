/* 
| opcode | Instr  | Stack desc                | Desc                            |
| ------ | ------ | ------------------------- | ------------------------------- |
| 0b0000 | `nop`  | ( -- )                    | Does nothing                    |
| 0b0001 | `lit`  | ( -- x )                  | push the next byte to the stack |
| 0b0010 | `dup`  | ( x -- x x )              | duplicate the top of stack      |
| 0b0011 | `over` | ( x y z -- z x y )        | standard stack over             |
| 0b0100 | `str`  | ( addr -- value )         | write data into memory          |
| 0b0101 | `load` | ( value addr -- )         | load data from memory           |
| 0b0110 | `push` | ( value -- )              | write to other stack            |
| 0b0111 | `drop` | ( value -- )              | Delete a value permanently      |
| 0b1000 | `jsr`  | ( addr -- ) [ -- retaddr] | jump to the address             |
| 0b1001 | `cmp`  | ( val2 val1 -- )          | compare values                  |

| 0b1111 | `halt` |                           | Halt the machine                |
*/

use num_derive::FromPrimitive;
use crate::{MEM, pop, push, instr_ptr, set_instr_ptr, offset_instr_ptr, MMU};

#[derive(FromPrimitive, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Instr {
    Nop,
    Lit,
    Dup,
    Over,
    Str,
    Load,
    Push,
    Drop,
    Jsr,
    Cmp,
    Halt
}

bitflags::bitflags! {
    pub struct Status: u8 {
        const KEEP = 0b1; // 0x1
        const RETURN = 0b10; // 0x2
        const SHORT = 0b100; // 0x4
        const IF_EQUAL = 0b1000; // 0x8
        const IF_GREATER = 0b10000; // 0x10
        const IF_LESS = 0b100000; // 0x20
        const _RESERVED1 = 0b1000000; // 0x40
        const _RESERVED2 = 0b10000000; // 0x80
        const NONE = 0;
    }

    pub struct ConditionRegister: u16 {
        const EQUAL = 0b1;
        const GREATER = 0b10;
        const LESS = 0b100;
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
            9 => Self::Cmp,
            0xff => Self::Halt,
            _ => panic!("Invalid instruction 0b{:b} at address 0x{:x}", byte, instr_ptr())
        }
    }

    pub fn execute(&self, flags: Status) {
        let instr_ptr = instr_ptr();

        //println!("Instruction {:?}\nIP 0x{:x}", self, instr_ptr);
        match self {
            Instr::Nop => (),
            Instr::Lit => {
                let data: u32;
                if flags.contains(Status::SHORT) {
                    data = MMU.lock().unwrap().read_u32((instr_ptr + 2) as u32);
                } else {
                    data = MMU.lock().unwrap().read_u16((instr_ptr + 2) as u32) as u32;
                }

                println!("Loading immediate: 0x{:x} from 0x{:x}", data, instr_ptr + 2);

                crate::push(data as u32, flags);
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
                let store_addr = pop(flags | Status::SHORT);
                let data = pop(flags);

                println!("Storing 0x{:x} at address 0x{:x}", data, store_addr);

                match flags.contains(Status::SHORT) {
                    true => {
                        MMU.lock().unwrap().write_u32(store_addr, data);
                    },
                    false => {
                        MMU.lock().unwrap().write_u16(store_addr, data as u16);
                    }
                }
            },
            Instr::Load => {
                let store_addr = pop(flags | Status::SHORT);

                match flags.contains(Status::SHORT) {
                    true => {
                        push(MMU.lock().unwrap().read_u32(store_addr), flags);
                    },
                    false => {
                        push(MMU.lock().unwrap().read_u16(store_addr) as u32, flags);
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
                let old_ptr = (instr_ptr + 2) as u32;

                if flags.contains(Status::RETURN) {
                    let flag = Status::SHORT;
                    push(old_ptr, flag);

                    set_instr_ptr(pop(flag | Status::RETURN))
                } else {
                    let flag = Status::RETURN | Status::SHORT;
                    push(old_ptr, flag);

                    set_instr_ptr(pop(flag - Status::RETURN))
                }
            },
            Instr::Cmp => {
                let mut condition_register = ConditionRegister::empty();

                let val1 = pop(flags);
                let val2 = pop(flags);

                let order = val1.cmp(&val2);
                println!("{val1} is {:?} compared to {val2}", order);

                if val1 == val2 {
                    condition_register |= ConditionRegister::EQUAL;
                }
                if val1 < val2 {
                    condition_register |= ConditionRegister::LESS;
                }
                if val1 > val2 {
                    condition_register |= ConditionRegister::GREATER;
                }

                condition_register.write();
            },
            Instr::Halt => {
                println!("VM Halting");
                std::process::exit(0);
            }
        }
    }
}

#[derive(Debug)]
pub struct Instruction(Instr, Status);

impl Instruction {
    pub const fn new(instr: Instr, status: Status) -> Self {
        Instruction(instr, status)
    }
    
    pub fn execute(&self) {
        let conditions = ConditionRegister::read();

        let flags = self.1;

        if flags.contains(Status::IF_EQUAL) && conditions.contains(ConditionRegister::EQUAL) {
            println!("Executing {:?}", self);
            self.0.execute(self.1);
        } else if flags.contains(Status::IF_GREATER) && conditions.contains(ConditionRegister::GREATER) {
            println!("Executing {:?}", self);
            self.0.execute(self.1);
        } else if flags.contains(Status::IF_LESS) && conditions.contains(ConditionRegister::LESS) {
            println!("Executing {:?}", self);
            self.0.execute(self.1);
        } else if !(flags.contains(Status::IF_GREATER) | flags.contains(Status::IF_LESS) | flags.contains(Status::IF_EQUAL)) {
            println!("Executing {:?}", self);
            self.0.execute(self.1);
        }

        if self.0 != Instr::Jsr {
            offset_instr_ptr(2);
        }

        if self.0 == Instr::Lit {
            if self.1.contains(Status::SHORT) {
                offset_instr_ptr(4);
            } else {
                offset_instr_ptr(2);
            }
        }
    }
}

impl ConditionRegister {
    pub fn read() -> Self {
        let state = unsafe {
            MEM.read_u16(0x204)
        };

        Self::from_bits(state).unwrap()
    }

    pub fn add(&self) {
        let start = unsafe {
            MEM.read_u16(0x204)
        };

        let mask = start | self.bits();

        unsafe {
            MEM.write_u16(0x204, mask)
        }
    }

    pub fn clear(&self) {
        let start = unsafe {
            MEM.read_u16(0x204)
        };

        let mask = start & !self.bits();

        unsafe {
            MEM.write_u16(0x204, mask)
        }
    }

    pub fn reset() {
        unsafe {
            MEM.write_u16(0x204, 0);
        }
    }

    pub fn write(&self) {
        let state = self.bits();

        unsafe {
            MEM.write_u16(0x204, state);
        }
    }
}