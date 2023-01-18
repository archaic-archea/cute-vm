use std::alloc::Layout;

use cute_vm::{memory::Memory, stack::Stack};

lazy_static::lazy_static!(
    pub static ref MEMORY: Memory = {
        let memory = Memory::new();

        memory
    };
);

lazy_static::lazy_static!(
    pub static ref PRIMARY_STACK: Stack = {
        let stack = Stack::new(*MEMORY, 0xff);

        stack
    };
);

lazy_static::lazy_static!(
    pub static ref RETURn_STACK: Stack = {
        let stack = Stack::new(*MEMORY, 0xff * 0x2);

        stack
    };
);


fn main() {
}
