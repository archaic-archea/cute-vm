extern crate alloc;

pub mod stack;
pub mod memory;
pub mod instructions;

use self::{memory::Memory, stack::Stack};


use std::sync::Mutex;
pub static mut MEM: Memory = Memory::null();

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
    let args = Args::parse();

    unsafe {
        let memory = args.memory_size.unwrap_or_else(|| {0xFFFF});

        if memory < 0x402 {
            panic!("Not enough memory provided for stack and instruction pointer");
        }

        MEM = Memory::new(memory as usize);
    }
}

use clap::Parser;
#[derive(Parser,Default,Debug)]
#[clap(author="Lilly, & Arc", version, about="A simple stack machine")]
struct Args {
    #[clap(short, long)]
    memory_size: Option<u16>,
}