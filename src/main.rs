use cute_vm::{instructions::{Status, Instr}, PRIMARY_STACK, push};

fn main() {
    cute_vm::init();
    println!("Initialized");

    let flags = vec![];
    println!("Set flags");

    let prim_stack = &mut PRIMARY_STACK.lock().unwrap();

    prim_stack.push(0xff, &flags);
    println!("Pushed to stack");

    println!("{}", prim_stack);
    println!("Printed stack");
}