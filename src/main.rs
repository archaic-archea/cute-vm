fn main() {
    cute_vm::init();
    println!("Initialized");

    loop {
        let instruction = cute_vm::instr();

        instruction.execute();
    }
}