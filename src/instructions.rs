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

use num_derive::FromPrimitive;

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
    }
}

impl Instr {
    pub fn read_instr() -> Instr {
        let instr_ptr = unsafe {super::MEM.read_u16(0x400)};
        let instr = unsafe {super::MEM.read_u16(instr_ptr as usize)};

        num::FromPrimitive::from_u16(instr).expect("Invalid opcode")
    }

    pub fn execute(&self, flags: Status) {
        let instr_ptr = unsafe {&mut super::MEM.read_u16(0x400)};

        match self {
            Instr::Nop => (),
            Instr::Lit => {
                let data = unsafe {MEM[(*instr_ptr + 2) as usize]};

                let mut tmpflags = Status::null();
                if flags.contains(Status::RETURN) {tmpflags |= Status::RETURN}

                crate::push(data as u32, tmpflags);

                if flags.contains(Status::SHORT) {
                    *instr_ptr += 1;
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

use crate::{MEM, pop, push};