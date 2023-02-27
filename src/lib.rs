extern crate alloc;

pub mod stack;
pub mod memory;
pub mod instructions;
pub mod sic;

use self::{memory::Memory, stack::Stack};


use std::sync::Mutex;
pub static mut MEM: Memory = Memory::null();
pub static PRIMARY_STACK: Mutex<Stack> = Mutex::new(Stack::new(0xff));
pub static RETURN_STACK: Mutex<Stack> = Mutex::new(Stack::new(0x1ff));
pub static INTERRUPT: Mutex<bool> = Mutex::new(false);
pub static INT_CONTROLLER: Mutex<sic::Sic> = Mutex::new(sic::Sic::new(0x300));

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

pub fn set_instr_ptr(ip: u32) {
    assert!(ip & 0b1 == 0, "Instruction pointer unaligned");

    unsafe {
        MEM.write_u32(0x200, ip);
    }
}

pub fn offset_instr_ptr(offset: isize) {
    assert!(offset & 0b1 == 0, "Instruction pointer unaligned");
    let ip = instr_ptr() as isize;

    set_instr_ptr((ip + offset) as u32);
}

pub fn instr() -> instructions::Instruction {
    let binary = unsafe {MEM.read_u16(instr_ptr())}.to_le_bytes();

    let instr = instructions::Instr::from_byte(binary[0]);
    let flag = instructions::Status::from_bits(binary[1]).unwrap();

    instructions::Instruction::new(instr, flag)
}

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
        MEM.write_u32(0x200, 0x600);
    }

    let file_path = std::path::Path::new(&args.file);
    let file = std::fs::read(file_path).expect("Error reading binary");

    assert!(file.len() & 0b1 == 0, "File length is not aligned properly");

    let base = 0x600;
    let mut offset = 0;
    for data in file.iter() {
        unsafe {
            MEM[base + offset] = *data;
        }

        offset += 1;
    }

}

use clap::Parser;
use instructions::Status;
#[derive(Parser,Default,Debug)]
#[clap(author="Lilly, & Arc", version, about="A simple stack machine")]
struct Args {
    #[clap(short, long)]
    memory_size: Option<u32>,

    #[clap(short, long)]
    file: String
}

pub fn store_ret() {
    INT_CONTROLLER.lock().unwrap().store_ret();
}

pub fn int_jmp() {
    INT_CONTROLLER.lock().unwrap().jmp();
}