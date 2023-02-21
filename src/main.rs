use cute_vm::{instructions::{Status, Instr}, PRIMARY_STACK, RETURN_STACK};

fn main() {
    cute_vm::init();
    println!("Initialized");

    let flags = Status::SHORT;
    println!("Set flags");

    PRIMARY_STACK.lock().unwrap().push(0xffff, flags);
    PRIMARY_STACK.lock().unwrap().push(0xfffe, flags);
    PRIMARY_STACK.lock().unwrap().push(0xfffd, flags);
    PRIMARY_STACK.lock().unwrap().push(0xfffc, flags);
    println!("Pushed to stack");

    println!("{:#?}\n", PRIMARY_STACK.lock().unwrap());

    Instr::Over.execute(flags);

    println!("{:#?}\n", PRIMARY_STACK.lock().unwrap());

    Instr::Dup.execute(flags);

    println!("Primary {:#?}", PRIMARY_STACK.lock().unwrap());
    println!("Secondary {:#?}", RETURN_STACK.lock().unwrap());
}