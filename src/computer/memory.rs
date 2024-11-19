pub struct MemoryManagementUnit {
    base: u64,
    limit: u64,
}

pub struct Memory {
    memory: Vec<u8>,
    mmus: Vec<MemoryManagementUnit>,
}

pub fn new(size: u64, mmus: u64) -> Memory {
    let mut mem = Memory {
        memory: Vec::with_capacity(size as usize),
        mmus: Vec::new(),
    };
    for _ in 0..mmus {
        mem.mmus.push(MemoryManagementUnit {
            base: 0,
            limit: 0,
        });
    }
    mem
}

impl Memory {
    pub fn translate_address(&mut self,
                             cpu_id: u64,
                             address: u64) -> Option<u64> {
        if address > self.mmus[cpu_id as usize].limit {
            None
        } else {
            Some(address + self.mmus[cpu_id as usize].base)
        }
    }

    pub fn read(&mut self, address: u64, size: u64) -> Option<u64> {
        if (address + size - 1) as usize > self.memory.len() {
            None
        } else {
            let mut value: u64 = 0;
            for i in 0..size {
                value |= (self.memory[(address + i) as usize] as u64)
                            << (i * 8);
            }
            Some(value)
        }
    }

    pub fn write(&mut self, address: u64, value: u64, size: u64) -> bool {
        if (address + size - 1) as usize > self.memory.len() {
            false
        } else {
            for i in 0..size {
                self.memory[(address + i) as usize] =
                    (value >> (i * 8)) as u8;
            }
            true
        }
    }

    pub fn read_dword(&mut self, address: u64) -> Option<u64> {
        self.read(address, 8)
    }

    pub fn write_dword(&mut self, address: u64, value: u64) -> bool {
        self.write(address, value, 8)
    }

    pub fn read_word(&mut self, address: u64) -> Option<u32> {
        match self.read(address, 4) {
            None => None,
            Some(value) => Some(value as u32),
        }
    }

    pub fn write_word(&mut self, address: u64, value: u32) -> bool {
        self.write(address, value as u64, 4)
    }

    pub fn read_halfword(&mut self, address: u64) -> Option<u16> {
        match self.read(address, 2) {
            None => None,
            Some(value) => Some(value as u16),
        }
    }

    pub fn write_halfword(&mut self, address: u64, value: u16) -> bool {
        self.write(address, value as u64, 2)
    }

    pub fn read_byte(&mut self, address: u64) -> Option<u8> {
        match self.read(address, 1) {
            None => None,
            Some(value) => Some(value as u8),
        }
    }

    pub fn write_byte(&mut self, address: u64, value: u8) -> bool {
        self.write(address, value as u64, 1)
    }

    pub fn read_instruction(&mut self, address: u64) -> Option<u32> {
        if (address + 3) as usize > self.memory.len() {
            None
        } else {
            let mut value: u32 = 0;
            for i in 0..4 {
                value |= (self.memory[(address + i) as usize] as u32)
                            << (24 - (i * 8));
            }
            Some(value)
        }
    }
}
