const LOW19: i32 = 0x7ffff;
const LOW18: i32 = 0x7ffff;
const LOW11: i32 = 0x7ff;
const LOW6: i32 = 0x3f;
const LOW5: i32 = 0x1f;

// Naturally Aligned CPU Load/Store Instructions
const LB: i32 = 0x20;
const LBU: i32 = 0x24;
const LD: i32 = 0x37;
const LH: i32 = 0x21;
const LHU: i32 = 0x25;
const LW: i32 = 0x23;
const LWU: i32 = 0x27;
const SB: i32 = 0x28;
const SD: i32 = 0x3f;
const SH: i32 = 0x29;
const SW: i32 = 0x2b;

// PC-relative Loads
const PCREL: i32 = 0x3b;
const LWPC: i32 = 0x1;
const LWUPC: i32 = 0x2;
const LDPC: i32 = 0x6;

// ALU Instructions with 16-bit Immediate Operand
const ADDIU: i32 = 0x09;
const ANDI: i32 = 0x0c;
const DADDIU: i32 = 0x19;
const LUI: i32 = 0x0f;
const ORI: i32 = 0x0d;
const SLTI: i32 = 0x0a;
const SLTIU: i32 = 0x0b;
const XORI: i32 = 0x0e;

// Three-Operand ALU Instructions
const ADD: i32 = 0x20;
const ADDU: i32 = 0x21;
const AND: i32 = 0x24;
const DADD: i32 = 0x2c;
const DADDU: i32 = 0x2d;
const DSUB: i32 = 0x2e;
const DSUBU: i32 = 0x2f;
const NOR: i32 = 0x27;
const OR: i32 = 0x25;
const SLT: i32 = 0x2a;
const SLTU: i32 = 0x2b;
const SUB: i32 = 0x22;
const SUBU: i32 = 0x23;
const XOR: i32 = 0x26;

// Two-Operand ALU Instructions
const CLO: i32 = 0x51;
const CLZ: i32 = 0x50;
const DCLO: i32 = 0x53;
const DCLZ: i32 = 0x52;

// Shift Instructions
const BSHFL: i32 = 0x20;
const ALIGN: i32 = 0x2;
const DBSHFL: i32 = 0x24;
const DALIGN: i32 = 0x1;
const BITSWAP: i32 = 0x00;
const DBITSWAP: i32 = 0x00;
const DSRL: i32 = 0x3a;
const DSRL32: i32 = 0x3e;
const DSRLV: i32 = 0x16;
const DSLL: i32 = 0x38;
const DSLL32: i32 = 0x3c;
const DSLLV: i32 = 0x14;
const DSRA: i32 = 0x3b;
const DSRA32: i32 = 0x3f;
const DSRAV: i32 = 0x17;
const SRL: i32 = 0x02;
const SRLV: i32 = 0x06;
const SLL: i32 = 0x00;
const SLLV: i32 = 0x04;
const SRA: i32 = 0x03;
const SRAV: i32 = 0x07;

// Same-Width Multiply and Divide Instructions
const MUL: i32 = 0x02;
const SOP30: i32 = 0x18;
const MUH: i32 = 0x03;
const MULU: i32 = 0x02;
const MUHU: i32 = 0x03;
const SOP31: i32 = 0x19;
const DMUL: i32 = 0x02;
const DMUH: i32 = 0x03;
const SOP34: i32 = 0x1c;
const DMULU: i32 = 0x02;
const DMUHU: i32 = 0x03;
const SOP35: i32 = 0x1d;
const DIV: i32 = 0x02;
const MOD: i32 = 0x03;
const SOP32: i32 = 0x1a;
const DIVU: i32 = 0x02;
const MODU: i32 = 0x03;
const SOP33: i32 = 0x1b;
const DDIV: i32 = 0x02;
const DMOD: i32 = 0x03;
const SOP36: i32 = 0x1e;
const DDIVU: i32 = 0x02;
const DMODU: i32 = 0x03;
const SOP37: i32 = 0x1f;

// Release 6 Compact Branch and Jump Instructions
const BC: i32 = 0x32;
const BALC: i32 = 0x3a;
const POP66: i32 = 0x36;
const JIC: i32 = 0;
const POP76: i32 = 0x3e;
const JIALC: i32 = 0;
const POP26: i32 = 0x16;
const POP27: i32 = 0x17;
const POP06: i32 = 0x06;
const POP07: i32 = 0x07;
const POP10: i32 = 0x08;
const POP30: i32 = 0x18;

// Delayed Branch Instructions
const J: i32 = 0x02;
const JAL: i32 = 0x03;
const JALR: i32 = 0x09;
const BEQ: i32 = 0x04;

// Special Constants
const SPECIAL3: i32 = 0x1f;
const BREAK: i32 = 0x0d;
const OPCODE: i32 = 26;
const RS: i32 = 21;
const RT: i32 = 16;
const RD: i32 = 11;

struct Registers {
    registers: [u64; 32],
    pc: u64,
}

pub struct Cpu {
    rf: Registers,
    id: u64,
    syscall: bool,
    exception: bool,
    next_branching: bool,
    branching: bool,
    branch_target: u64,
}

pub fn new(id: u64) -> Cpu {
    Cpu {
        rf: Registers {
            registers: [0; 32],
            pc: 0,
        },
        id,
        syscall: false,
        exception: false,
        next_branching: false,
        branching: false,
        branch_target: 0,
    }
}

impl Cpu {

    // This function contains panics!
    // They should never be hit, but I don't know the MIPS ISA well enough
    // to say that definitively.
    // Reserved instructions will probably panic.
    pub fn step(&mut self, memory: &mut crate::computer::memory::Memory) {
        // If there's a current exception or syscall that hasn't
        // been handled, we just need to stop then and there.
        if self.exception || self.syscall {
            return;
        }

        // Get the real address from the memory's translation unit.
        let pc_address = memory.translate_address(self.id, self.rf.pc);
        if pc_address == None {
            self.exception = true;
            return;
        }

        // Get the actual instruction from memory.
        let instruction = memory.read_instruction(pc_address.unwrap());
        if instruction == None {
            self.exception = true;
            return;
        }

        // Finally, execute the instruction.
        self.execute_instruction(instruction.unwrap(), memory);
    }

    // This function is the source of the panics.
    pub fn execute_instruction(&mut self,
                               instruction: u32,
                               memory: &mut crate::computer::memory::Memory) {
        // We start by pulling the opcode and some commonly used parts of
        // the instruction.
        let opcode: i32 = (instruction >> OPCODE) as i32;

        let rs: usize = (((instruction >> RS) as i32) & LOW5) as usize;
        let rt: usize = (((instruction >> RT) as i32) & LOW5) as usize;
        let rd: usize = (((instruction >> RD) as i32) & LOW5) as usize;

        let imm16: i64 = ( instruction as i16 ) as i64;
        
        // Match on the opcode to find the category of instruction.
        match opcode {
            0 => {
                // If the opcode is 0, we need to fetch the function bits
                // and check those next.
                let function: i32 = (instruction as i32) & LOW6;

                if function == BREAK {

                } else if (instruction as i32 & LOW11) == CLO && rt == 0 {
                    let i: u64 = 0;
                    for i in 0..32 {
                        if ( (self.rf.registers[rs] & (0x1 << (31 - i)))
                                    >> (31 - i)) == 1 {
                            break;
                        }
                    }
                    self.rf.registers[rd] = i;
                } else if (instruction as i32 & LOW11) == CLZ && rt == 0 {
                    let i: u64 = 0;
                    for i in 0..32 {
                        if ( (self.rf.registers[rs] ^ (0x1 << (31 - i)))
                                    >> (31 - i)) & 0x1 == 1 {
                            break;
                        }
                    }
                    self.rf.registers[rd] = i;
                } else if (instruction as i32 & LOW11) == DCLO && rt == 0 {
                    let i: u64 = 0;
                    for i in 0..64 {
                        if ( (self.rf.registers[rs] & (0x1 << (63 - i)))
                                    >> (63 - i)) == 1 {
                            break;
                        }
                    }
                    self.rf.registers[rd] = i;
                } else if (instruction as i32 & LOW11) == DCLZ && rt == 0 {
                    let i: u64 = 0;
                    for i in 0..64 {
                        if ( (self.rf.registers[rs] ^ (0x1 << (63 - i)))
                                    >> (63 - i)) & 0x1 == 1 {
                            break;
                        }
                    }
                    self.rf.registers[rd] = i;
                } else {

                    match function {
                        ADD => {
                            if (self.rf.registers[rs] as i32).checked_add(
                                    self.rf.registers[rt] as i32) == None {
                                self.exception = true;
                                self.rf.pc -= 4;
                            } else {
                                self.rf.registers[rd] = (
                                    self.rf.registers[rs] as i32 +
                                    self.rf.registers[rt] as i32
                                ) as i64 as u64;
                            }
                        },
                        ADDU => {
                            self.rf.registers[rd] = (
                                self.rf.registers[rs] as i64 +
                                self.rf.registers[rt] as i64
                            ) as u64;
                        },
                        AND => {
                            self.rf.registers[rd] = self.rf.registers[rs] &
                                                    self.rf.registers[rt];
                        },
                        DADD => {
                            if (self.rf.registers[rs] as i64).checked_add(
                                    self.rf.registers[rt] as i64) == None {
                                self.exception = true;
                                self.rf.pc -= 4;
                            } else {
                                self.rf.registers[rd] = (
                                    self.rf.registers[rs] as i64 +
                                    self.rf.registers[rt] as i64
                                ) as u64;
                            }
                        },
                        DADDU => {
                            self.rf.registers[rd] = (
                                self.rf.registers[rs] as i64 +
                                self.rf.registers[rt] as i64
                            ) as u64;
                        },
                        DSUB => {
                            if (self.rf.registers[rs] as i64).checked_sub(
                                    self.rf.registers[rt] as i64) == None {
                                self.exception = true;
                                self.rf.pc -= 4;
                            } else {
                                self.rf.registers[rd] = (
                                    self.rf.registers[rs] as i64 -
                                    self.rf.registers[rt] as i64
                                ) as u64;
                            }
                        },
                        DSUBU => {
                            self.rf.registers[rd] = (
                                self.rf.registers[rs] as i64 -
                                self.rf.registers[rt] as i64
                            ) as u64;
                        },
                        NOR => {
                            self.rf.registers[rd] = !(
                                self.rf.registers[rs] |
                                self.rf.registers[rt]
                            );
                        },
                        OR => {
                            self.rf.registers[rd] = self.rf.registers[rs] |
                                                    self.rf.registers[rt];
                        },
                        SLT => {
                            self.rf.registers[rd] = if (self.rf.registers[rs] as i64) <
                                                      (self.rf.registers[rt] as i64)
                                                        { 1 } else { 0 };
                        },
                        SLTU => {
                            self.rf.registers[rd] = if self.rf.registers[rs] <
                                                    self.rf.registers[rt]
                                                        { 1 } else { 0 };
                        },
                        SUB => {
                            if (self.rf.registers[rs] as i32).checked_sub(
                                    self.rf.registers[rt] as i32) == None {
                                self.exception = true;
                                self.rf.pc -= 4;
                            } else {
                                self.rf.registers[rd] = (
                                    self.rf.registers[rs] as i32 -
                                    self.rf.registers[rt] as i32
                                ) as i64 as u64;
                            }
                        },
                        SUBU => {
                            self.rf.registers[rd] = (
                                self.rf.registers[rs] as i32 -
                                self.rf.registers[rt] as i32
                            ) as i64 as u64;
                        },
                        XOR => {
                            self.rf.registers[rd] = self.rf.registers[rs] ^
                                                    self.rf.registers[rt];
                        },
                        DSRL => { // DROTR
                            let sa = (instruction as i32 >> 6) & LOW5;
                            if (instruction >> 21) & 0x1 != 0 {
                                self.rf.registers[rd] =
                                    (self.rf.registers[rt] << (64 - sa)) |
                                    (self.rf.registers[rt] >> sa);
                            } else { // DSRL
                                self.rf.registers[rd] = self.rf.registers[rt]
                                    >> sa;
                            }
                        },
                        DSRL32 => { // DROTR32
                            let sa = ((instruction as i32 >> 6) & LOW5) + 32;
                            if (instruction >> 21) & 0x1 != 0 {
                                self.rf.registers[rd] =
                                    (self.rf.registers[rt] << (64 - sa)) |
                                    (self.rf.registers[rt] >> sa);
                            } else { // DSLR32
                                self.rf.registers[rd] =
                                    self.rf.registers[rt] >> sa;
                            }
                        },
                        DSRLV => { // DROTRV
                            let sa = self.rf.registers[rs];
                            if (instruction >> 6) & 0x1 != 0 {
                                self.rf.registers[rd] =
                                    (self.rf.registers[rt] << (64 - sa)) |
                                    (self.rf.registers[rt] >> sa);
                            } else {
                                self.rf.registers[rd] =
                                    self.rf.registers[rt] >> sa;
                            }
                        },
                        DSLL => {
                            let sa = (instruction as i32 >> 6) & LOW5;
                            if instruction >> 21 == 0 {
                                self.rf.registers[rd] = self.rf.registers[rt]
                                    << sa;
                            } else {
                                panic!("Unknown instruction!");
                            }
                        },
                        DSLL32 => {
                            let sa = (instruction as i32 >> 6) & LOW5;
                            if instruction >> 21 == 0 {
                                self.rf.registers[rd] = self.rf.registers[rt]
                                    << (sa + 32);
                            } else {
                                panic!("Unknown instruction!");
                            }
                        },
                        DSLLV => {
                            if ( instruction >> 6 ) as i32 & LOW5 == 0 {
                                self.rf.registers[rd] = self.rf.registers[rt]
                                    << self.rf.registers[rs];
                            } else {
                                panic!("Unknown instruction!");
                            }
                        },
                        DSRA => {
                            let sa = (instruction as i32 >> 6) & LOW5;
                            if instruction >> 21 == 0 {
                                self.rf.registers[rd] = (self.rf.registers[rt]
                                    as i64 >> sa) as u64;
                            } else {
                                panic!("Unknown instruction!");
                            }
                        },
                        DSRA32 => {
                            let sa = (instruction as i32 >> 6) & LOW5;
                            if instruction >> 21 == 0 {
                                self.rf.registers[rd] = (self.rf.registers[rt]
                                    as i64 >> (sa + 32)) as u64;
                            } else {
                                panic!("Unknown instruction!");
                            }
                        },
                        DSRAV => {
                            if (instruction as i32 >> 6) & LOW5 == 0 {
                                self.rf.registers[rd] = (self.rf.registers[rt]
                                    as i64 >> (self.rf.registers[rs] + 32)
                                    ) as u64;
                            } else {
                                panic!("Unknown instruction!");
                            }
                        },
                        SRL => {
                            let sa = (instruction as i32 >> 6) & LOW5;
                            if (instruction as i32 >> 21) & LOW5 == 0x1 { // ROTR
                                self.rf.registers[rd] = (((self.rf.registers[rt]
                                    << (32 - sa)) as i32) |
                                    ((self.rf.registers[rt] as u32 >> sa) as i32))
                                    as i64 as u64;
                            } else { // SRL
                                self.rf.registers[rd] = (self.rf.registers[rt]
                                    as u32 >> sa) as u64;
                            }
                        },
                        SRLV => {
                            let sa = self.rf.registers[rs] as i32 & LOW5;
                            if (instruction as i32 >> 6) & LOW5 == 0x1 { // ROTRV
                                self.rf.registers[rd] = (((self.rf.registers[rt]
                                    << (32 - sa)) as i32) |
                                    ((self.rf.registers[rt] as u32 >> sa) as i32))
                                    as i64 as u64;
                            } else { // SRLV
                                self.rf.registers[rd] = (self.rf.registers[rt]
                                    as u32 >> sa) as u64;
                            }
                        },
                        SLL => {
                            let sa = (instruction as i32 >> 6) & LOW5;
                            if (instruction as i32 >> 21) & LOW5 == 0x0 {
                                self.rf.registers[rd] = (self.rf.registers[rt]
                                    << sa) as i32 as i64 as u64;
                            } else {
                                panic!("Unknown instruction!");
                            }
                        },
                        SLLV => {
                            let sa = self.rf.registers[rs] as i32 & LOW5;
                            if (instruction as i32 >> 6) & LOW5 == 0x0 {
                                self.rf.registers[rd] = (self.rf.registers[rt]
                                    << sa) as i32 as i64 as u64;
                            } else {
                                panic!("Unknown instruction!");
                            }
                        },
                        SRA => {
                            let sa = (instruction as i32 >> 6) & LOW5;
                            if (instruction as i32 >> 21) & LOW5 == 0x0 {
                                self.rf.registers[rd] = (self.rf.registers[rt]
                                    as i32 >> sa) as i64 as u64;
                            } else {
                                panic!("Unknown instruction!");
                            }
                        },
                        SRAV => {
                            let sa = self.rf.registers[rs] as i32 & LOW5;
                            if (instruction as i32 >> 6) & LOW5 == 0x0 {
                                self.rf.registers[rd] = (self.rf.registers[rt]
                                    as i32 >> sa) as i64 as u64;
                            } else {
                                panic!("Unknown instruction!");
                            }
                        },
                        SOP30 => {
                            if ( instruction as i32 >> 6 ) & LOW5 == MUL {
                                self.rf.registers[rd] = (
                                    self.rf.registers[rs] as i32
                                    * self.rf.registers[rt] as i32)
                                    as i64 as u64;
                            } else if (instruction as i32 >> 6) & LOW5 == MUH {
                                self.rf.registers [rd] = ((
                                    (self.rf.registers[rs] as i32 as i64) *
                                    (self.rf.registers[rt] as i32 as i64)
                                    ) >> 32) as u64;
                            } else {
                                panic!("Unknown instruction!");
                            }
                        },
                        SOP31 => {
                            if ( instruction as i32 >> 6 ) & LOW5 == MULU {
                                self.rf.registers[rd] = (
                                    self.rf.registers[rs] as u32
                                    * self.rf.registers[rt] as u32
                                ) as u64;
                            } else if (instruction as i32 >> 6) & LOW5 == MUHU {
                                self.rf.registers[rd] = (
                                    (self.rf.registers[rs] as u32 as u64)
                                    * (self.rf.registers[rt] as u32 as u64)
                                ) >> 32;
                            } else {
                                panic!("Unknown instruction!");
                            }
                        },
                        SOP34 => {
                            if (instruction as i32 >> 6) & LOW5 == DMUL {
                                self.rf.registers[rd] = (
                                    (self.rf.registers[rs] as i64)
                                    * (self.rf.registers[rt] as i64)
                                ) as u64;
                            } else if (instruction as i32 >> 6) & LOW5 == DMUH {
                                self.rf.registers[rd] = ((
                                    (self.rf.registers[rs] as i128)
                                    * (self.rf.registers[rt] as i128)
                                ) >> 64) as u64;
                            } else {
                                panic!("Unknown instruction!");
                            }
                        },
                        SOP35 => {
                            if (instruction as i32 >> 6) & LOW5 == DMULU {
                                self.rf.registers[rd] = (
                                    (self.rf.registers[rs] as u64)
                                    * (self.rf.registers[rt] as u64)
                                ) as u64;
                            } else if (instruction as i32 >> 6) & LOW5 == DMUHU {
                                self.rf.registers[rd] = ((
                                    (self.rf.registers[rs] as u128)
                                    * (self.rf.registers[rt] as u128)
                                ) >> 64) as u64;
                            } else {
                                panic!("Unknown instruction!");
                            }
                        },
                        SOP32 => {
                            if (instruction as i32 >> 6) & LOW5 == DIV {
                                self.rf.registers[rd] = (
                                    (self.rf.registers[rs] as i32)
                                    / (self.rf.registers[rt] as i32)
                                ) as i64 as u64;
                            } else if (instruction as i32 >> 6) & LOW5 == MOD {
                                self.rf.registers[rd] = (
                                    (self.rf.registers[rs] as i32)
                                    % (self.rf.registers[rt] as i32)
                                ) as i64 as u64;
                            } else {
                                panic!("Unknown instruction!");
                            }
                        },
                        SOP33 => {
                            if (instruction as i32 >> 6) & LOW5 == DIVU {
                                self.rf.registers[rd] = (
                                    (self.rf.registers[rs] as u32)
                                    / (self.rf.registers[rt] as u32)
                                ) as u64;
                            } else if (instruction as i32 >> 6) & LOW5 == MODU {
                                self.rf.registers[rd] = (
                                    (self.rf.registers[rs] as u32)
                                    % (self.rf.registers[rt] as u32)
                                ) as u64;
                            } else {
                                panic!("Unknown instruction!");
                            }
                        },
                        SOP36 => {
                            if (instruction as i32 >> 6) & LOW5 == DDIV {
                                self.rf.registers[rd] = (
                                    (self.rf.registers[rs] as i64)
                                    / (self.rf.registers[rt] as i64)
                                ) as u64;
                            } else if (instruction as i32 >> 6) & LOW5 == DMOD {
                                self.rf.registers[rd] = (
                                    (self.rf.registers[rs] as i64)
                                    % (self.rf.registers[rt] as i64)
                                ) as u64;
                            } else {
                                panic!("Unknown instruction!");
                            }
                        },
                        SOP37 => {
                            if (instruction as i32 >> 6) & LOW5 == DDIVU {
                                self.rf.registers[rd] =
                                    (self.rf.registers[rs])
                                    / (self.rf.registers[rt]);
                            } else if (instruction as i32 >> 6) & LOW5 == DMODU {
                                self.rf.registers[rd] =
                                    (self.rf.registers[rs])
                                    % (self.rf.registers[rt]);
                            } else {
                                panic!("Unknown instruction!");
                            }
                        },
                        JALR => {
                            if rd != 0 {
                                self.rf.registers[rd] = self.rf.pc + 8;
                            }
                            self.rf.pc = self.rf.registers[rs];
                        },
                        _ => panic!("Uncovered type!"),
                    }
                }
            },
            LB | LBU | LD | LH | LHU | LW | LWU | SB | SD | SH | SW => {
                match memory.translate_address(self.id,
                        (self.rf.registers[rs] as i64 + imm16) as u64) {
                    None => {
                        self.exception = true;
                    },
                    Some(address) => {
                        match opcode {
                            LB => {
                                match memory.read_byte(address) {
                                    None => {
                                        self.exception = true;
                                    },
                                    Some(value) => {
                                        self.rf.registers[rt] =
                                            value as i8 as i64 as u64;
                                    }
                                }
                            },
                            LBU => {
                                match memory.read_byte(address) {
                                    None => {
                                        self.exception = true;
                                    },
                                    Some(value) => {
                                        self.rf.registers[rt] = value as u64;
                                    }
                                }
                            },
                            LD => {
                                match memory.read_dword(address) {
                                    None => {
                                        self.exception = true;
                                    },
                                    Some(value) => {
                                        self.rf.registers[rt] =
                                            value;
                                    }
                                }
                            },
                            LH => {
                                match memory.read_halfword(address) {
                                    None => {
                                        self.exception = true;
                                    },
                                    Some(value) => {
                                        self.rf.registers[rt] =
                                            value as i16 as i64 as u64;
                                    }
                                }
                            },
                            LHU => {
                                match memory.read_halfword(address) {
                                    None => {
                                        self.exception = true;
                                    },
                                    Some(value) => {
                                        self.rf.registers[rt] = value as u64;
                                    }
                                }
                            },
                            LW => {
                                match memory.read_word(address) {
                                    None => {
                                        self.exception = true;
                                    },
                                    Some(value) => {
                                        self.rf.registers[rt] =
                                            value as i32 as i64 as u64;
                                    }
                                }
                            },
                            LWU => {
                                match memory.read_word(address) {
                                    None => {
                                        self.exception = true;
                                    },
                                    Some(value) => {
                                        self.rf.registers[rt] = value as u64;
                                    }
                                }
                            },
                            SB => {
                                if !memory.write_byte(address,
                                    self.rf.registers[rt] as u8) {
                                    self.exception = true;
                                }
                            },
                            SD => {
                                if !memory.write_dword(address,
                                    self.rf.registers[rt]) {
                                    self.exception = true;
                                }
                            },
                            SH => {
                                if !memory.write_halfword(address,
                                    self.rf.registers[rt] as u16) {
                                    self.exception = true;
                                }
                            },
                            SW => {
                                if !memory.write_word(address,
                                    self.rf.registers[rt] as u32) {
                                    self.exception = true;
                                }
                            },
                            _ => {
                                panic!("This is impossible!");
                            }
                        }
                    }
                }
            },
            PCREL => {
                if ((instruction >> 19) & 0x3) as i32 == LWPC {
                    let address = self.rf.pc +
                        (((instruction as i64 & LOW19 as i64)
                                << 45) >> 43) as u64;
                    match memory.translate_address(self.id, address) {
                        None => {
                            self.exception = true;
                        }
                        Some(address) => {
                            match memory.read_word(address) {
                                None => {
                                    self.exception = true;
                                },
                                Some(value) => {
                                    self.rf.registers[rs] =
                                        value as i32 as i64 as u64;
                                }
                            }
                        }
                    }
                } else if ((instruction >> 19) & 0x3) as i32 == LWUPC {
                    let address = self.rf.pc +
                        (((instruction as i64 & LOW19 as i64)
                                << 45) >> 43) as u64;
                    match memory.translate_address(self.id, address) {
                        None => {
                            self.exception = true;
                        }
                        Some(address) => {
                            match memory.read_word(address) {
                                None => {
                                    self.exception = true;
                                },
                                Some(value) => {
                                    self.rf.registers[rs] = value as u64;
                                }
                            }
                        }
                    }
                } else if ((instruction >> 18) & 0x7) as i32 == LDPC {
                    let address = self.rf.pc +
                        (((instruction as i64 & LOW18 as i64)
                                << 46) >> 43) as u64;
                    match memory.translate_address(self.id, address) {
                        None => {
                            self.exception = true;
                        }
                        Some(address) => {
                            match memory.read_word(address) {
                                None => {
                                    self.exception = true;
                                },
                                Some(value) => {
                                    self.rf.registers[rs] = value as u64;
                                }
                            }
                        }
                    }
                } else {
                    panic!("Unknown instruction.");
                }
            },
            ADDIU => {
                self.rf.registers[rt] = ((self.rf.registers[rs] as i32) +
                                        (imm16 as i32)) as i64 as u64;
            },
            ANDI => {
                self.rf.registers[rt] = self.rf.registers[rs] & 
                                        (imm16 as u16 as u64);
            },
            DADDIU => {
                self.rf.registers[rt] = ((self.rf.registers[rs] as i64) +
                                        (imm16 as i64)) as u64;
            },
            LUI => {
                self.rf.registers[rt] = (imm16 << 16) as u64;
            },
            ORI => {
                self.rf.registers[rt] = (imm16 as u16 as u64) |
                                        self.rf.registers[rs];
            },
            SLTI => {
                self.rf.registers[rt] = if imm16 < (self.rf.registers[rs] as i64)
                                        { 1 } else { 0 };
            },
            SLTIU => {
                self.rf.registers[rt] = if (imm16 as u64) < self.rf.registers[rs]
                                        { 1 } else { 0 };
            },
            XORI => {
                self.rf.registers[rt] = (imm16 as u16 as u64) ^
                                        self.rf.registers[rs];
            },
            SPECIAL3 => {
                if rs != 0 {
                    match instruction as i32 & LOW6 {
                        BSHFL => {
                            match ((instruction >> 8) & 0x7) as i32 {
                                ALIGN => {
                                    let bp = ((instruction >> 6) & 0x3) as u8;
                                    self.rf.registers[rd] =
                                        (self.rf.registers[rt] << (8 * bp)) |
                                        (self.rf.registers[rs] >> (64 - 8 * bp));
                                    if self.rf.registers[rd] & 0x80000000 != 0 {
                                        self.rf.registers[rd] |= 0x1111111100000000;
                                    } else {
                                        self.rf.registers[rd] &= 0x0000000011111111;
                                    }
                                },
                                _ => {
                                    panic!("Unknown instruction!");
                                }
                            }
                        },
                        DBSHFL => {
                            match ((instruction >> 9) & 0x3) as i32 {
                                DALIGN => {
                                    let bp = ((instruction >> 6) & 0x7) as u8;
                                    self.rf.registers[rd] =
                                        (self.rf.registers[rt] << (8 * bp)) |
                                        (self.rf.registers[rs] >> (64 - 8 * bp));
                                },
                                _ => {
                                    panic!("Unknown instruction!");
                                }
                            }
                        },
                        _ => {
                            panic!("Unknown instruction!");
                        }
                    }
                } else {
                    match instruction as i32 & LOW6 {
                        BSHFL => {
                            if instruction as i32 & LOW5 == BITSWAP {
                                let mut result: u64 = 0;
                                for i in 0..4 {
                                    let byte: u8 =
                                        (self.rf.registers[rt] >> (8 * i))
                                        as u8;
                                    let mut obyte: u8 = 0;
                                    for j in 0..8 {
                                        if byte & (0x1 << j) != 0 {
                                            obyte |= 0x80 >> j;
                                        }
                                    }
                                    result |= (obyte as u64) << (i * 8);
                                }
                                if result & 0x80000000 != 0 {
                                    result |= 0xffffffff00000000;
                                } else {
                                    result &= 0x00000000ffffffff;
                                }
                                self.rf.registers[rd] = result;
                            } else {
                                panic!("Unknown instruction!");
                            }
                        },
                        DBSHFL => {
                            if instruction as i32 & LOW5 == DBITSWAP {
                                let mut result: u64 = 0;
                                for i in 0..8 {
                                    let byte: u8 =
                                        (self.rf.registers[rt] >> (8 * i))
                                        as u8;
                                    let mut obyte: u8 = 0;
                                    for j in 0..8 {
                                        if byte & (0x1 << j) != 0 {
                                            obyte |= 0x80 >> j;
                                        }
                                    }
                                    result |= (obyte as u64) << (i * 8);
                                }
                                self.rf.registers[rd] = result;
                            } else {
                                panic!("Unknown instruction!");
                            }
                        },
                        _ => panic!("Unknown instruction!"),
                    }
                }
            },
            BC => {
                self.rf.pc = ((((instruction as i32) << 6) >> 4) as i64 +
                    (self.rf.pc as i64)) as u64;
            },
            BALC => {
                self.rf.registers[31] = self.rf.pc + 4;
                self.rf.pc = ((((instruction as i32) << 6) >> 4) as i64 +
                    (self.rf.pc as i64)) as u64;
            },
            POP06 => {
                if rs != 0 && rt != 0 && rs != rt { // BGEUC
                    if self.rf.registers[rt] >= self.rf.registers[rs] {
                        self.rf.pc = ((self.rf.pc as i64) +
                            imm16 - 4) as u64;
                    }
                } else if rs != 0 && rs == rt { // BGEZALC
                    if self.rf.registers[rt] as i64 >= 0 {
                        self.rf.registers[31] = self.rf.pc + 4;
                        self.rf.pc = ((self.rf.pc as i64) +
                            imm16 - 4) as u64;
                    }
                } else if rs == 0 && rt != 0 { // BLEZALC
                    if self.rf.registers[rt] as i64 <= 0 {
                        self.rf.registers[31] = self.rf.pc + 4;
                        self.rf.pc = ((self.rf.pc as i64) +
                            imm16 - 4) as u64;
                    }
                } else {
                    panic!("Unknown instruction!");
                }
            },
            POP07 => {
                if rs != 0 && rt != 0 && rs != rt { // BLTUC
                    if self.rf.registers[rt] < self.rf.registers[rs] {
                        self.rf.pc = ((self.rf.pc as i64) +
                            imm16 - 4) as u64;
                    }
                } else if rs != 0 && rs == rt { // BLTZALC
                    if (self.rf.registers[rt] as i64) < 0 {
                        self.rf.registers[31] = self.rf.pc + 4;
                        self.rf.pc = ((self.rf.pc as i64) +
                            imm16 - 4) as u64;
                    }
                } else if rs == 0 && rt != 0 { // BGTZALC
                    if self.rf.registers[rt] as i64 > 0 {
                        self.rf.registers[31] = self.rf.pc + 4;
                        self.rf.pc = ((self.rf.pc as i64) +
                            imm16 - 4) as u64;
                    }
                } else {
                    panic!("Unknown instruction!");
                }
            },
            POP10 => {
                if rs != 0 && rt != 0 && rs < rt { // BEQC
                    if self.rf.registers[rt] == self.rf.registers[rs] {
                        self.rf.pc = ((self.rf.pc as i64) +
                            imm16 - 4) as u64;
                    }
                } else if rs == 0 && rs < rt { // BEQZALC
                    if self.rf.registers[rt] == 0 {
                        self.rf.registers[31] = self.rf.pc + 4;
                        self.rf.pc = ((self.rf.pc as i64) +
                            imm16 - 4) as u64;
                    }
                } else if rs >= rt { // BOVC
                    if (self.rf.registers[rt] as i32)
                        .checked_add(self.rf.registers[rs] as i32) == None {
                        self.rf.pc = ((self.rf.pc as i64) +
                            imm16 - 4) as u64;
                    }
                } else {
                    panic!("Unknown instruction!");
                }
            },
            POP30 => {
                if rs != 0 && rt != 0 && rs < rt { // BNEC
                    if self.rf.registers[rt] != self.rf.registers[rs] {
                        self.rf.pc = ((self.rf.pc as i64) +
                            imm16 - 4) as u64;
                    }
                } else if rs == 0 && rs < rt { // BNEZALC
                    if self.rf.registers[rt] != 0 {
                        self.rf.registers[31] = self.rf.pc + 4;
                        self.rf.pc = ((self.rf.pc as i64) +
                            imm16 - 4) as u64;
                    }
                } else if rs >= rt { // BOVC
                    if (self.rf.registers[rt] as i32)
                        .checked_add(self.rf.registers[rs] as i32) != None {
                        self.rf.pc = ((self.rf.pc as i64) +
                            imm16 - 4) as u64;
                    }
                } else {
                    panic!("Unknown instruction!");
                }
            },
            POP66 => {
                if (instruction as i32 >> 21) & LOW5 == JIC {
                    self.rf.pc = (self.rf.registers[rt] as i64 + imm16 - 4)
                        as u64;
                } else { // BEQZC
                    if self.rf.registers[rs] == 0 {
                        self.rf.pc = ((self.rf.pc as i64) +
                            ((((instruction as i32) << 11) >> 9) as i64 - 4)
                            ) as u64;
                    }
                }
            },
            POP76 => {
                if (instruction as i32 >> 21) & LOW5 == JIALC {
                    self.rf.registers[31] = self.rf.pc + 4;
                    self.rf.pc = (self.rf.registers[rt] as i64 + imm16 - 4)
                        as u64;
                } else { // BNEZC
                    if self.rf.registers[rs] != 0 {
                        self.rf.pc = ((self.rf.pc as i64) +
                            ((((instruction as i32) << 11) >> 9) as i64 - 4)
                            ) as u64;
                    }
                }
            },
            POP26 => {
                if rs == 0 && rt != 0 { // BLEZC
                    if self.rf.registers[rt] as i64 <= 0 {
                        self.rf.pc = ((self.rf.pc as i64) +
                            imm16 - 4) as u64;
                    }
                } else if rs != 0 && rt != 0 && rs == rt { // BGEZC
                    if self.rf.registers[rt] as i64 >= 0 {
                        self.rf.pc = ((self.rf.pc as i64) +
                            imm16 - 4) as u64;
                    }
                } else if rs != 0 && rt != 0 && rs != rt { // BGEC
                    if self.rf.registers[rs] as i64 >=
                        self.rf.registers[rt] as i64 {
                        self.rf.pc = ((self.rf.pc as i64) +
                            imm16 - 4) as u64;
                    }
                } else {
                    panic!("Unknown instruction!");
                }
            },
            POP27 => {
                if rs == 0 && rt != 0 { // BGTZC
                    if self.rf.registers[rt] as i64 > 0 {
                        self.rf.pc = ((self.rf.pc as i64) +
                            imm16 - 4) as u64;
                    }
                } else if rs != 0 && rt != 0 && rs == rt { // BLTZC
                    if (self.rf.registers[rt] as i64) < 0 {
                        self.rf.pc = ((self.rf.pc as i64) +
                            imm16 - 4) as u64;
                    }
                } else if rs != 0 && rt != 0 && rs != rt { // BLTC
                    if (self.rf.registers[rs] as i64) <
                        self.rf.registers[rt] as i64 {
                        self.rf.pc = ((self.rf.pc as i64) +
                            imm16 - 4) as u64;
                    }
                } else {
                    panic!("Unknown instruction!");
                }
            },
            J => {
                self.rf.pc = (((self.rf.pc >> 28) << 28) |
                    (((instruction << 6) >> 4) as u64)) - 4;
            },
            JAL => {
                self.rf.registers[31] = self.rf.pc + 4;
                self.rf.pc = (((self.rf.pc >> 28) << 28) |
                    (((instruction << 6) >> 4) as u64)) - 4;
            },
            BEQ => {
                if self.branching {
                    self.exception = true;
                } else if self.rf.registers[rs] == self.rf.registers[rt] {
                    self.next_branching = true;
                    self.branch_target = (self.rf.pc as i64 + (imm16 << 2))
                                            as u64;
                }
            },
            _ => panic!("Uncovered opcode!"),
        }

        if self.branching {
            self.branching = false;
            self.rf.pc = self.branch_target;
        } else {
            self.rf.pc += 4;
        }

        if self.next_branching {
            self.next_branching = false;
            self.branching = true;
        }
    }
}
