use cute_vm::{INTERRUPT, store_ret, int_jmp, OUT_PUT_READY, HALTED};
use std::sync::atomic::Ordering;

fn main() {
    cute_vm::init();
    println!("Initialized");

    while !HALTED.load(Ordering::Relaxed) {
        if *INTERRUPT.lock().unwrap() {
            log::info!("Interrupt generated");
            //println!("{:x?}", INT_CONTROLLER.lock().unwrap());
            store_ret();
            int_jmp();

            *INTERRUPT.lock().unwrap() = false;
        }

        log::debug!("Instruction: {:?}", cute_vm::instr());

        //println!("Instr ptr: 0x{:x}", cute_vm::instr_ptr());
        let instruction = cute_vm::instr();

        //println!("Running instruction {:?} at 0x{:x}", instruction, cute_vm::instr_ptr());
        instruction.execute();

        // Make sure all IO devices are ready before stopping
        while !io_ready() {}
        //std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn io_ready() -> bool {
    if OUT_PUT_READY.load(Ordering::Relaxed) {
        return true;
    }
    false
}