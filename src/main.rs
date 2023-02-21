use cute_vm::{instructions::{Status, Instr}, PRIMARY_STACK, instr_ptr, RETURN_STACK};

fn main() {
    cute_vm::init();
    println!("Initialized");

    let flags = Status::NONE;
    println!("Set flags");

    PRIMARY_STACK.lock().unwrap().push(0xFFFF_FFFF, flags | Status::SHORT);
    println!("Pushed to stack\n");

    println!("Instr ptr: 0x{:x}", instr_ptr());

    println!("Primary {:#?}", PRIMARY_STACK.lock().unwrap());
    println!("Return {:#?}", RETURN_STACK.lock().unwrap());
    Instr::Jsr.execute(flags);
    println!("Instr ptr: 0x{:x}", instr_ptr());

    println!("Primary {:#?}", PRIMARY_STACK.lock().unwrap());
    println!("Return {:#?}", RETURN_STACK.lock().unwrap());
}