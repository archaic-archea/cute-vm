use cute_vm::{INTERRUPT, store_ret, int_jmp};

fn main() {
    cute_vm::init();
    println!("Initialized");

    loop {
        if *INTERRUPT.lock().unwrap() {
            println!("Interrupt generated");
            //println!("{:x?}", INT_CONTROLLER.lock().unwrap());
            store_ret();
            int_jmp();

            *INTERRUPT.lock().unwrap() = false;
        }

        //println!("Instr ptr: 0x{:x}", cute_vm::instr_ptr());
        let instruction = cute_vm::instr();

        //println!("Running instruction {:?} at 0x{:x}", instruction, cute_vm::instr_ptr());
        instruction.execute();

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}