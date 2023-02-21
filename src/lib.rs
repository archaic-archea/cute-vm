extern crate alloc;

pub mod stack;
pub mod memory;
pub mod instructions;

use self::{memory::Memory, stack::Stack};


use std::sync::Mutex;
pub static mut MEM: Memory = Memory::null();
pub static PRIMARY_STACK: Mutex<Stack> = Mutex::new(Stack::new(0xff));
pub static RETURN_STACK: Mutex<Stack> = Mutex::new(Stack::new(0x1ff));

pub fn push(data: u32, flags: Status) {
    if flags.contains(Status::RETURN) {
        RETURN_STACK.lock().unwrap().push(data, flags);
    } else {
        PRIMARY_STACK.lock().unwrap().push(data, flags);
    }
}

pub fn pop(flags: Status) -> u32 {
    if flags.contains(Status::RETURN) {
        return RETURN_STACK.lock().unwrap().pop(flags);
    } else {
        return PRIMARY_STACK.lock().unwrap().pop(flags);
    }
}

pub fn copy(index: usize, flags: Status) -> u32 {
    if flags.contains(Status::RETURN) {
        return RETURN_STACK.lock().unwrap().copy(index, flags);
    } else {
        return PRIMARY_STACK.lock().unwrap().copy(index, flags);
    }
}

pub fn top(ret_stack: bool) -> usize {
    if ret_stack {
        return RETURN_STACK.lock().unwrap().top();
    } else {
        return PRIMARY_STACK.lock().unwrap().top();
    }
}

pub fn instr_ptr() -> usize {
    unsafe {
        MEM.read_u32(0x200) as usize
    }
}

pub fn instr() -> instructions::Instr {todo!()}

/// Initialize memory
/// TODO: Add custom memory sizes
pub fn init() {
    let args = Args::parse();

    let memory = args.memory_size.unwrap_or(0xFFFF);

    if memory < 0x204 {
        panic!("Not enough memory provided for stack and instruction pointer");
    }

    unsafe {
        MEM = Memory::new(memory as usize);
        MEM.write_u16(0x200, 0x204);
    }
}

use clap::Parser;
use instructions::Status;
#[derive(Parser,Default,Debug)]
#[clap(author="Lilly, & Arc", version, about="A simple stack machine")]
struct Args {
    #[clap(short, long)]
    memory_size: Option<u32>,
}