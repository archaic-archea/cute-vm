use cute_vm::{instructions::{Status, Instr}, PRIMARY_STACK, RETURN_STACK};

fn main() {
    cute_vm::init();
    println!("Initialized");

    let flags = Status::null();
    println!("Set flags");

    PRIMARY_STACK.lock().unwrap().push(0xff, flags);
    PRIMARY_STACK.lock().unwrap().push(0xfe, flags);
    PRIMARY_STACK.lock().unwrap().push(0xfd, flags);
    PRIMARY_STACK.lock().unwrap().push(0xfc, flags);
    println!("Pushed to stack");

    println!("{}\n", PRIMARY_STACK.lock().unwrap());

    Instr::Over.execute(flags);

    println!("{}\n", PRIMARY_STACK.lock().unwrap());

    Instr::Dup.execute(flags);

    println!("Primary {}", PRIMARY_STACK.lock().unwrap());
    println!("Secondary {}", RETURN_STACK.lock().unwrap());
}