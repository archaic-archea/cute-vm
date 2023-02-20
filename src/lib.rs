extern crate alloc;

pub mod stack;
pub mod memory;
pub mod instructions;

use self::{memory::Memory, stack::Stack};


use std::sync::Mutex;
pub static mut MEM: Memory = Memory::null();
pub static PRIMARY_STACK: Mutex<Stack> = Mutex::new(Stack::new(0xff));
pub static RETURN_STACK: Mutex<Stack> = Mutex::new(Stack::new(0x1ff));

pub fn push(data: u32, flags: &Vec<Status>) {
    if flags.contains(&Status::Return) {
        RETURN_STACK.lock().unwrap().push(data, &flags);
    } else {
        PRIMARY_STACK.lock().unwrap().push(data, &flags);
    }
}

pub fn pop(flags: &Vec<Status>) -> u32 {
    if flags.contains(&Status::Return) {
        return RETURN_STACK.lock().unwrap().pop(&flags);
    } else {
        return PRIMARY_STACK.lock().unwrap().pop(&flags);
    }
}

pub fn copy(index: usize, flags: &Vec<Status>) -> u32 {
    if flags.contains(&Status::Return) {
        return RETURN_STACK.lock().unwrap().copy(index, &flags);
    } else {
        return PRIMARY_STACK.lock().unwrap().copy(index, &flags);
    }
}

pub fn top(ret_stack: bool) -> usize {
    if ret_stack {
        return RETURN_STACK.lock().unwrap().top();
    } else {
        return PRIMARY_STACK.lock().unwrap().top();
    }
}

/// Initialize memory
/// TODO: Add custom memory sizes
pub fn init() {
    let args = Args::parse();

    let memory = args.memory_size.unwrap_or_else(|| {0xFFFF});

    if memory < 0x402 {
        panic!("Not enough memory provided for stack and instruction pointer");
    }

    unsafe {
        MEM = Memory::new(memory as usize);
    }
}

use clap::Parser;
use instructions::Status;
#[derive(Parser,Default,Debug)]
#[clap(author="Lilly, & Arc", version, about="A simple stack machine")]
struct Args {
    #[clap(short, long)]
    memory_size: Option<u16>,
}