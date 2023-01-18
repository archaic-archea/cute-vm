extern crate alloc;

pub mod stack;
pub mod memory;

use self::{memory::Memory, stack::Stack};

use std::sync::Mutex;

pub static mut MEM: Option<Memory> = None;

lazy_static::lazy_static!(
    pub static ref PRIMARY_STACK: Mutex<Stack> = {
        let stack = Stack::new(0xff);

        Mutex::new(stack)
    };

    pub static ref RETURN_STACK: Mutex<Stack> = {
        let stack = Stack::new(0x1ff);

        Mutex::new(stack)
    };
);

pub fn init() {
    unsafe {
        MEM = Some(Memory::new(0xffff))
    }
}