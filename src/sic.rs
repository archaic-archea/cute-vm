use crate::MEM;

#[derive(Debug)]
pub struct Sic {
    pub jmp: u32,
    pub cause: u32,
    pub return_addr: u32,
}

impl Sic {
    pub const fn new() -> Self {
        Self {
            jmp: 0,
            cause: 0,
            return_addr: 0
        }
    }

    /// Stores the location to jump to for an interrupt
    pub fn jmp(&self) {
        unsafe {
            MEM.write_u32(0x200, self.jmp);
        }
    }

    pub fn store_ret(&mut self) {
        self.return_addr = crate::instr_ptr() as u32;
    }

    pub fn gen_int(&mut self, cause: u32, exception: bool) {
        let cause = 0x7FFFFFFF & cause;
        let excep_store = (exception as u32) << 31;

        let store = cause | excep_store;
        
        self.cause = store;

        *crate::INTERRUPT.lock().unwrap() = true;
    }
}