use crate::MEM;

pub struct Sic {
    base: u32,
    jmp_offset: u32,
    cause_offset: u32,
    return_offset: u32,
}

impl Sic {
    pub const fn new(base: u32) -> Self {
        Self {
            base,
            jmp_offset: 0,
            cause_offset: 4,
            return_offset: 8
        }
    }

    /// Stores the location to jump to for an interrupt
    pub fn jmp(&self) {
        unsafe {
            let jump_loc = MEM.read_u32((self.base + self.jmp_offset) as usize);

            MEM.write_u32(0x200, jump_loc);
        }
    }

    pub fn store_ret(&self) {
        let store_addr = self.base + self.return_offset;

        unsafe {
            MEM.write_u32(store_addr as usize, crate::instr_ptr() as u32);
        }
    }

    pub fn gen_int(&self, cause: u32, exception: bool) {
        let cause = 0x7FFFFFFF & cause;
        let excep_store = (exception as u32) << 31;

        let store = cause | excep_store;

        let address = self.base + self.cause_offset;

        unsafe {
            crate::MEM.write_u32(address as usize, store);
        }

        *crate::INTERRUPT.lock().unwrap() = true;
    }
}