use cute_vm::{INTERRUPT, store_ret, int_jmp};

fn main() {
    cute_vm::init();
    println!("Initialized");

    loop {
        if *INTERRUPT.lock().unwrap() {
            println!("Interrupt generated");
            store_ret();
            int_jmp();

            *INTERRUPT.lock().unwrap() = false;
        }

        let instruction = cute_vm::instr();

        //println!("Running instruction {:?} at 0x{:x}", instruction, cute_vm::instr_ptr());

        instruction.execute();
    }
}