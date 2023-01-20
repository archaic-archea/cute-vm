use cute_vm::{instructions::Status, PRIMARY_STACK};

fn main() {
    cute_vm::init();

    let mut flags = vec![];

    PRIMARY_STACK.lock().unwrap().push(0xffff_ffff, &flags);
    let test = PRIMARY_STACK.lock().unwrap().pop(&flags);
    println!("{:x}", test);

    flags.push(Status::Short);

    PRIMARY_STACK.lock().unwrap().push(0xffff_ffff, &flags);
    let test = PRIMARY_STACK.lock().unwrap().pop(&flags);
    println!("{:x}", test);
    
    flags.push(Status::Keep);

    PRIMARY_STACK.lock().unwrap().push(0xffff_ffff, &flags);
    let test = PRIMARY_STACK.lock().unwrap().pop(&flags);
    println!("{:x}", test);
}