mod cpu;
mod memory;

pub struct Computer {
    cpus: Vec<cpu::Cpu>,
    memory: memory::Memory,
}

pub fn new(cpus: u64, memory: u64) -> Computer {
    let mut com = Computer {
        cpus: Vec::new(),
        memory: memory::new(memory, cpus),
    };
    for i in 0..cpus {
        com.cpus.push(cpu::new(i));
    }
    com
}

impl Computer {
    pub fn step(&mut self) {
        for cpu in self.cpus.iter_mut() {
            cpu.step(&mut self.memory)
        }
    }
}
