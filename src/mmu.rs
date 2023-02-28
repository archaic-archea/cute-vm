use crate::{INT_CONTROLLER, OUT_PUT_READY};

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
        if (index <= self.io_max) && (index >= self.io_base) {
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
                _ => {
                    INT_CONTROLLER.lock().unwrap().gen_int(2, true);
                    println!("Invaled IO read address: 0x{:x}", index);
                    0
                }
            }
        } else if (index <= self.memory_max) && (index >= self.memory_base) {
            unsafe {
                crate::MEM.vm_read_u16((index - self.memory_base) as usize)
            }
        } else {
            INT_CONTROLLER.lock().unwrap().gen_int(0, true);
            println!("Unknown memory address: 0x{:x}", index);
            0
        }
    }

    pub fn read_u32(&self, index: u32) -> u32 {
        if (index <= self.io_max) && (index >= self.io_base) {
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
                _ => {
                    INT_CONTROLLER.lock().unwrap().gen_int(2, true);
                    println!("Invaled IO read address: 0x{:x}", index);
                    0
                }
            }
        } else if (index <= self.memory_max) && (index >= self.memory_base) {
            unsafe {
                crate::MEM.vm_read_u32((index - self.memory_base) as usize)
            }
        } else {
            INT_CONTROLLER.lock().unwrap().gen_int(0, true);
            println!("Unknown memory address: 0x{:x}", index);
            0
        }
    }

    pub fn write_u16(&self, index: u32, num: u16) {
        if (index <= self.io_max) && (index >= self.io_base) {
            match index {
                0x00 => {
                    unsafe {
                        crate::PRIMARY_STACK.lock().unwrap().set_pos(num as u32)
                    }
                },
                0x04 => {
                    unsafe {
                        crate::RETURN_STACK.lock().unwrap().set_pos(num as u32)
                    }
                },
                0x08 => {
                    unsafe {
                        crate::PRIMARY_STACK.lock().unwrap().set_offset(num)
                    }
                },
                0xA => {
                    unsafe {
                        crate::RETURN_STACK.lock().unwrap().set_offset(num)
                    }
                },
                0x0C => {
                    INT_CONTROLLER.lock().unwrap().gen_int(0, false);
                },
                0x100 => {
                    unsafe {
                        use std::sync::atomic::Ordering;
                        
                        log::info!("Giving data to output device");
                        crate::IO_SEND.send_data(num as u8).expect("Failed to send to io");
                        OUT_PUT_READY.store(false, Ordering::Relaxed);
                    }
                },
                0x300 => {
                    log::info!("Writing jump for SIC");
                    crate::INT_CONTROLLER.lock().unwrap().jmp = num as u32
                },
                _ => {
                    INT_CONTROLLER.lock().unwrap().gen_int(3, true);
                    log::warn!("Invaled IO write address: 0x{:x}", index)
                }
            }
        } else if (index <= self.memory_max) && (index >= self.memory_base) {
            unsafe {
                crate::MEM.vm_write_u16((index - self.memory_base) as usize , num)
            }
        } else {
            INT_CONTROLLER.lock().unwrap().gen_int(1, true);
            println!("Unknown memory address: 0x{:x}", index);
        }
    }

    pub fn write_u32(&self, index: u32, num: u32) {
        if (index <= self.io_max) && (index >= self.io_base) {
            match index {
                0x00 => {
                    unsafe {
                        crate::PRIMARY_STACK.lock().unwrap().set_pos(num)
                    }
                },
                0x04 => {
                    unsafe {
                        crate::RETURN_STACK.lock().unwrap().set_pos(num)
                    }
                },
                0x08 => {
                    unsafe {
                        crate::PRIMARY_STACK.lock().unwrap().set_offset((num >> 16) as u16);
                        crate::RETURN_STACK.lock().unwrap().set_offset(num as u16);
                    }
                },
                0x0C => {
                    INT_CONTROLLER.lock().unwrap().gen_int(0, false);
                },
                0x100 => {
                    unsafe {
                        log::info!("Giving data to output device");
                        crate::IO_SEND.send_data(num as u8).expect("Failed to send to io");
                    }
                },
                0x300 => {
                    log::info!("Writing jump for SIC");
                    crate::INT_CONTROLLER.lock().unwrap().jmp = num
                },
                _ => {
                    INT_CONTROLLER.lock().unwrap().gen_int(3, true);
                    log::warn!("Invaled IO write address: 0x{:x}", index)
                }
            }
        } else if (index <= self.memory_max) && (index >= self.memory_base) {
            unsafe {
                crate::MEM.vm_write_u32((index - self.memory_base) as usize , num)
            }
        } else {
            INT_CONTROLLER.lock().unwrap().gen_int(1, true);
            println!("Unknown memory address: 0x{:x}", index);
        }
    }
}