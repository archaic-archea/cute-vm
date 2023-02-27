pub struct MMU {
    io_base: u32,
    io_max: u32,
    memory_base: u32,
    memory_max: u32
}

impl MMU {
    pub const fn new(io_base: u32, io_max: u32, memory_base: u32, memory_max: u32) -> Self {
        Self { io_base, io_max, memory_base, memory_max }
    }

    pub fn read_u16(&self, index: u32) -> u16 {
        if (index < self.io_max) && (index > self.io_base) {
            match index {
                0x300 => {
                    crate::INT_CONTROLLER.lock().unwrap().jmp as u16
                },
                0x304 => {
                    crate::INT_CONTROLLER.lock().unwrap().cause as u16
                },
                0x308 => {
                    crate::INT_CONTROLLER.lock().unwrap().return_addr as u16
                },
                _ => panic!("Invaled IO write address: 0x{:x}", index)
            }
        } else if (index < self.memory_base) && (index > self.memory_max) {
            unsafe {
                crate::MEM.vm_read_u16((index - self.memory_base) as usize)
            }
        } else {
            panic!("Unknown memory address: 0x{:x}", index);
        }
    }

    pub fn read_u32(&self, index: u32) -> u32 {
        if (index < self.io_max) && (index > self.io_base) {
            match index {
                0x300 => {
                    crate::INT_CONTROLLER.lock().unwrap().jmp
                },
                0x304 => {
                    crate::INT_CONTROLLER.lock().unwrap().cause
                },
                0x308 => {
                    crate::INT_CONTROLLER.lock().unwrap().return_addr
                },
                _ => panic!("Invaled IO write address: 0x{:x}", index)
            }
        } else if (index < self.memory_base) && (index > self.memory_max) {
            unsafe {
                crate::MEM.vm_read_u32((index - self.memory_base) as usize)
            }
        } else {
            panic!("Unknown memory address: 0x{:x}", index);
        }
    }

    pub fn write_u16(&self, index: u32, num: u16) {
        if (index < self.io_max) && (index > self.io_base) {
            match index {
                0x300 => {
                    crate::INT_CONTROLLER.lock().unwrap().jmp = num as u32
                },
                _ => panic!("Invaled IO write address: 0x{:x}", index)
            }
        } else if (index < self.memory_base) && (index > self.memory_max) {
            unsafe {
                crate::MEM.vm_write_u16((index - self.memory_base) as usize , num)
            }
        } else {
            panic!("Unknown memory address: 0x{:x}", index);
        }
    }

    pub fn write_u32(&self, index: u32, num: u32) {
        if (index < self.io_max) && (index > self.io_base) {
            match index {
                0x300 => {
                    crate::INT_CONTROLLER.lock().unwrap().jmp = num
                },
                _ => panic!("Invaled IO write address: 0x{:x}", index)
            }
        } else if (index < self.memory_base) && (index > self.memory_max) {
            unsafe {
                crate::MEM.vm_write_u32((index - self.memory_base) as usize , num)
            }
        } else {
            panic!("Unknown memory address: 0x{:x}", index);
        }
    }
}