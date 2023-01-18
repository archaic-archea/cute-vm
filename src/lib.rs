extern crate alloc;

pub mod stack;
pub mod memory;

use self::{memory::Memory, stack::Stack};


pub static mut MEM: Memory = Memory::null();

use std::sync::Mutex;
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


/// Initialize memory
/// TODO: Add custom memory sizes
pub fn init() {
    unsafe {
        MEM = Memory::new(0xffff);
    }
}