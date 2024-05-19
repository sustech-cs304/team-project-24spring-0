use crate::interface::assembler::{Instruction, Operand};
use crate::modules::riscv::rv32i::constants::{RV32IInstruction, RV32IRegister};

pub struct CPU {
    pub memory: Vec<u8>,
    pub registers: [u32; 32],
    pub pc: u32,
    pub instructions: Vec<Instruction>,
}

impl CPU {
    pub fn new(mem_size: usize) -> Self {
        CPU {
            memory: vec![0; mem_size * 1024],
            registers: [0; 32],
            pc: 0,
            instructions: Vec::new(),
        }
    }

    pub fn print_registers(&mut self) {
        let mut output = String::from("");
        let abi = [
            "zero", " ra ", " sp ", " gp ", " tp ", " t0 ", " t1 ", " t2 ", " s0 ", " s1 ", " a0 ",
            " a1 ", " a2 ", " a3 ", " a4 ", " a5 ", " a6 ", " a7 ", " s2 ", " s3 ", " s4 ", " s5 ",
            " s6 ", " s7 ", " s8 ", " s9 ", " s10", " s11", " t3 ", " t4 ", " t5 ", " t6 ",
        ];
        for i in (0..32).step_by(4) {
            output = format!(
                "{}\n{}",
                output,
                format!(
                    "x{:02}({})={:>#18x} x{:02}({})={:>#18x} x{:02}({})={:>#18x} x{:02}({})={:>#18x}",
                    i,
                    abi[i],
                    self.registers[i],
                    i + 1,
                    abi[i + 1],
                    self.registers[i + 1],
                    i + 2,
                    abi[i + 2],
                    self.registers[i + 2],
                    i + 3,
                    abi[i + 3],
                    self.registers[i + 3],
                )
            );
        }
        println!("{}", output);
    }

    pub fn print_memory(&mut self, rs1: usize, imm: i32) {
        let index = (self.registers[rs1] + CPU::sign_extend(imm as u32, 12)) as usize;
        let mut output = String::from("");
        for i in index..index + 4 {
            output = format!(
                "{}\n{}",
                output,
                format!("memory[{:>#18x}]={:>#18x}", i, self.memory[i],)
            );
        }
        println!("{}", output);
    }

    fn load_inst(&mut self, ir: Instruction) -> Result<bool, ()> {
        self.instructions.push(ir);
        Ok(true)
    }

    pub fn execute(&mut self, instruction: Instruction) -> Result<(), ()> {
        let opcode = instruction.op;
        let mut imm: i32 = 0;
        let mut rd: usize = 0;
        let mut rs1: usize = 0;
        let mut rs2: usize = 0;
        let mut temp0: Option<usize> = None;
        let mut temp1: Option<usize> = None;
        let mut temp2: Option<usize> = None;
        let mut imm_temp: Option<i32> = None;
        for operand in instruction.ins {
            match operand {
                Operand::Reg(reg) => {
                    let reg = self.register_name_to_u32(reg);
                    if temp0.is_none() {
                        temp0 = Some(reg as usize);
                    } else if temp1.is_none() {
                        temp1 = Some(reg as usize);
                    } else if temp2.is_none() {
                        temp2 = Some(reg as usize);
                    }
                }
                Operand::Operator(operator) => {
                    imm_temp = Some(operator as i32);
                }
            }
        }

        match opcode {
            0x03 => {
                // imm[11:0] = inst[31:20]
                let imm = ((inst as i32 as i64) >> 20) as u64;
                let addr = self.regs[rs1].wrapping_add(imm);
                match funct3 {
                    0x0 => {
                        // lb
                        let val = self.load(addr, 8)?;
                        self.regs[rd] = val as i8 as i64 as u64;
                    }
                    0x1 => {
                        // lh
                        let val = self.load(addr, 16)?;
                        self.regs[rd] = val as i16 as i64 as u64;
                    }
                    0x2 => {
                        // lw
                        let val = self.load(addr, 32)?;
                        self.regs[rd] = val as i32 as i64 as u64;
                    }
                    0x3 => {
                        // ld
                        let val = self.load(addr, 64)?;
                        self.regs[rd] = val;
                    }
                    0x4 => {
                        // lbu
                        let val = self.load(addr, 8)?;
                        self.regs[rd] = val;
                    }
                    0x5 => {
                        // lhu
                        let val = self.load(addr, 16)?;
                        self.regs[rd] = val;
                    }
                    0x6 => {
                        // lwu
                        let val = self.load(addr, 32)?;
                        self.regs[rd] = val;
                    }
                    _ => {
                        println!(
                            "not implemented yet: opcode {:#x} funct3 {:#x}",
                            opcode, funct3
                        );
                        return Err(());
                    }
                }
            }
            0x13 => {
                // imm[11:0] = inst[31:20]
                let imm = ((inst & 0xfff00000) as i32 as i64 >> 20) as u64;
                // "The shift amount is encoded in the lower 6 bits of the I-immediate field for
                // RV64I."
                let shamt = (imm & 0x3f) as u32;
                match funct3 {
                    0x0 => {
                        // addi
                        self.regs[rd] = self.regs[rs1].wrapping_add(imm);
                    }
                    0x1 => {
                        // slli
                        self.regs[rd] = self.regs[rs1] << shamt;
                    }
                    0x2 => {
                        // slti
                        self.regs[rd] = if (self.regs[rs1] as i64) < (imm as i64) {
                            1
                        } else {
                            0
                        };
                    }
                    0x3 => {
                        // sltiu
                        self.regs[rd] = if self.regs[rs1] < imm { 1 } else { 0 };
                    }
                    0x4 => {
                        // xori
                        self.regs[rd] = self.regs[rs1] ^ imm;
                    }
                    0x5 => {
                        match funct7 >> 1 {
                            // srli
                            0x00 => self.regs[rd] = self.regs[rs1].wrapping_shr(shamt),
                            // srai
                            0x10 => {
                                self.regs[rd] = (self.regs[rs1] as i64).wrapping_shr(shamt) as u64
                            }
                            _ => {}
                        }
                    }
                    0x6 => self.regs[rd] = self.regs[rs1] | imm, // ori
                    0x7 => self.regs[rd] = self.regs[rs1] & imm, // andi
                    _ => {}
                }
            }
            0x17 => {
                // auipc
                let imm = (inst & 0xfffff000) as i32 as i64 as u64;
                self.regs[rd] = self.pc.wrapping_add(imm).wrapping_sub(4);
            }
            0x1b => {
                let imm = ((inst as i32 as i64) >> 20) as u64;
                // "SLLIW, SRLIW, and SRAIW encodings with imm[5] ̸= 0 are reserved."
                let shamt = (imm & 0x1f) as u32;
                match funct3 {
                    0x0 => {
                        // addiw
                        self.regs[rd] = self.regs[rs1].wrapping_add(imm) as i32 as i64 as u64;
                    }
                    0x1 => {
                        // slliw
                        self.regs[rd] = self.regs[rs1].wrapping_shl(shamt) as i32 as i64 as u64;
                    }
                    0x5 => {
                        match funct7 {
                            0x00 => {
                                // srliw
                                self.regs[rd] = (self.regs[rs1] as u32).wrapping_shr(shamt) as i32
                                    as i64 as u64;
                            }
                            0x20 => {
                                // sraiw
                                self.regs[rd] =
                                    (self.regs[rs1] as i32).wrapping_shr(shamt) as i64 as u64;
                            }
                            _ => {
                                println!(
                                    "not implemented yet: opcode {:#x} funct7 {:#x}",
                                    opcode, funct7
                                );
                                return Err(());
                            }
                        }
                    }
                    _ => {
                        println!(
                            "not implemented yet: opcode {:#x} funct3 {:#x}",
                            opcode, funct3
                        );
                        return Err(());
                    }
                }
            }
            0x23 => {
                // imm[11:5|4:0] = inst[31:25|11:7]
                let imm = (((inst & 0xfe000000) as i32 as i64 >> 20) as u64) | ((inst >> 7) & 0x1f);
                let addr = self.regs[rs1].wrapping_add(imm);
                match funct3 {
                    0x0 => self.store(addr, 8, self.regs[rs2])?,  // sb
                    0x1 => self.store(addr, 16, self.regs[rs2])?, // sh
                    0x2 => self.store(addr, 32, self.regs[rs2])?, // sw
                    0x3 => self.store(addr, 64, self.regs[rs2])?, // sd
                    _ => {}
                }
            }
            RV32IInstruction::Jal => {
                self.registers[rd] = self.pc + 4;
                self.pc = (self.pc as i32 + imm) as u32;
            }
            RV32IInstruction::Addi => {
                self.registers[rd] = (self.registers[rs1] as i32).wrapping_add(imm as i32) as u32;
            }
            RV32IInstruction::Xori => {
                self.registers[rd] = ((self.registers[rs1] as i32) ^ (imm as i32)) as u32;
            }
            RV32IInstruction::Ori => {
                self.registers[rd] = ((self.registers[rs1] as i32) | (imm as i32)) as u32;
            }
            RV32IInstruction::Andi => {
                self.registers[rd] = ((self.registers[rs1] as i32) & (imm as i32)) as u32;
            }
            RV32IInstruction::Slti => {
                self.registers[rd] = if (self.registers[rs1] as i32) < (imm as i32) {
                    1
                } else {
                    0
                };
            }
            RV32IInstruction::Sltiu => {
                self.registers[rd] = if self.registers[rs1] < (imm as u32) {
                    1
                } else {
                    0
                };
            }
            RV32IInstruction::Slli => {
                let shamt = imm & 0b11111;
                self.registers[rd] = self.registers[rs1] << shamt;
            }
            RV32IInstruction::Srli => {
                let shamt = imm & 0b11111;
                self.registers[rd] = self.registers[rs1] >> shamt;
            }
            RV32IInstruction::Srai => {
                let shamt = imm & 0b11111;
                self.registers[rd] =
                    CPU::sign_extend(self.registers[rs1] >> shamt, 32 - shamt as u32);
            }
            RV32IInstruction::Jalr => {
                let t = self.pc;
                self.pc = ((self.registers[rs1] as i32 + imm) as u32) & !1;
                // set lsb to 0
                self.registers[rd] = t + 4;
            }
            RV32IInstruction::Lb => {
                let index = (self.registers[rs1] + CPU::sign_extend(imm as u32, 12)) as usize;
                self.registers[rd] = CPU::sign_extend(self.memory[index] as u32, 8);
                self.print_memory(rs1, imm);
            }
            RV32IInstruction::Lh => {
                let index = (self.registers[rs1] + CPU::sign_extend(imm as u32, 12)) as usize;
                let half_word = self.memory[index] as u32 | (self.memory[index + 1] as u32) << 8;
                self.registers[rd] = CPU::sign_extend(half_word as u32, 16);
                self.print_memory(rs1, imm);
            }
            RV32IInstruction::Lw => {
                let index = (self.registers[rs1] + CPU::sign_extend(imm as u32, 12)) as usize;

                self.registers[rd] = self.memory[index] as u32
                    | ((self.memory[index + 1]) as u32) << 8
                    | ((self.memory[index + 2]) as u32) << 16
                    | ((self.memory[index + 3]) as u32) << 24;
                self.print_memory(rs1, imm);
            }
            RV32IInstruction::Lbu => {
                let index = (self.registers[rs1] + CPU::sign_extend(imm as u32, 12)) as usize;
                self.registers[rd] = self.memory[index] as u32;
                self.print_memory(rs1, imm);
            }
            RV32IInstruction::Lhu => {
                let index = (self.registers[rs1] + CPU::sign_extend(imm as u32, 12)) as usize;

                self.registers[rd] =
                    self.memory[index] as u32 | (self.memory[index + 1] as u32) << 8;
                self.print_memory(rs1, imm);
            }
            RV32IInstruction::Sb => {
                let index = (self.registers[rs1] + CPU::sign_extend(imm as u32, 12)) as usize;
                self.memory[index] = (self.registers[rs2] & 0xff) as u8;
                self.print_memory(rs1, imm);
            }
            RV32IInstruction::Sh => {
                let index = (self.registers[rs1] + CPU::sign_extend(imm as u32, 12)) as usize;
                self.memory[index] = (self.registers[rs2] & 0xff) as u8;
                self.memory[index + 1] = ((self.registers[rs2] >> 8) & 0xff) as u8;
                self.print_memory(rs1, imm);
            }
            RV32IInstruction::Sw => {
                let index = (self.registers[rs1] + CPU::sign_extend(imm as u32, 12)) as usize;
                self.memory[index] = (self.registers[rs2] & 0xff) as u8;
                self.memory[index + 1] = (self.registers[rs2] >> 8 & 0xff) as u8;
                self.memory[index + 2] = (self.registers[rs2] >> 16 & 0xff) as u8;
                self.memory[index + 3] = (self.registers[rs2] >> 24 & 0xff) as u8;
                self.print_memory(rs1, imm);
            }
            RV32IInstruction::Lui => {
                self.registers[rd] = (imm << 12) as u32;
            }
            RV32IInstruction::Auipc => {
                self.registers[rd] = (self.pc as i32 + imm << 12) as u32;
            }
            RV32IInstruction::Ecall => {
                self.registers[rd] = self.pc;
                self.pc = (self.pc as i32 + imm) as u32;
            }
            RV32IInstruction::Ebreak => {
                self.registers[rd] = self.pc;
                self.pc = (self.pc as i32 + imm) as u32;
            }
            _ => {
                return Err(());
            }
        }
        return Ok(());
    }

    fn sign_extend(data: u32, size: u32) -> u32 {
        assert!(size > 0 && size <= 32);
        (((data << (32 - size)) as i32) >> (32 - size)) as u32
    }

    // pub fn load(&mut self, addr: u32, size: u32) -> Result<u32, ()> {
    //     self.dram.load(addr, size)
    // }

    // pub fn store(&mut self, addr: u32, size: u32, value: u32) -> Result<(), ()> {
    //     self.dram.store(addr, size, value)
    // }
    fn register_name_to_u32(&mut self, register: RV32IRegister) -> u32 {
        match register {
            RV32IRegister::Zero => 0,
            RV32IRegister::Ra => 1,
            RV32IRegister::Sp => 2,
            RV32IRegister::Gp => 3,
            RV32IRegister::Tp => 4,
            RV32IRegister::T0 => 5,
            RV32IRegister::T1 => 6,
            RV32IRegister::T2 => 7,
            RV32IRegister::S0 => 8,
            RV32IRegister::S1 => 9,
            RV32IRegister::A0 => 10,
            RV32IRegister::A1 => 11,
            RV32IRegister::A2 => 12,
            RV32IRegister::A3 => 13,
            RV32IRegister::A4 => 14,
            RV32IRegister::A5 => 15,
            RV32IRegister::A6 => 16,
            RV32IRegister::A7 => 17,
            RV32IRegister::S2 => 18,
            RV32IRegister::S3 => 19,
            RV32IRegister::S4 => 20,
            RV32IRegister::S5 => 21,
            RV32IRegister::S6 => 22,
            RV32IRegister::S7 => 23,
            RV32IRegister::S8 => 24,
            RV32IRegister::S9 => 25,
            RV32IRegister::S10 => 26,
            RV32IRegister::S11 => 27,
            RV32IRegister::T3 => 28,
            RV32IRegister::T4 => 29,
            RV32IRegister::T5 => 30,
            RV32IRegister::T6 => 31,
        }
    }
    
    pub fn execute_enum(&mut self, instruction: Instruction) -> Result<(), ()> {
        let opcode = instruction.op;
        let mut imm: i32 = 0;
        let mut rd: usize = 0;
        let mut rs1: usize = 0;
        let mut rs2: usize = 0;
        let mut temp0: Option<usize> = None;
        let mut temp1: Option<usize> = None;
        let mut temp2: Option<usize> = None;
        let mut imm_temp: Option<i32> = None;
        for operand in instruction.ins {
            match operand {
                Operand::Reg(reg) => {
                    let reg = self.register_name_to_u32(reg);
                    if temp0.is_none() {
                        temp0 = Some(reg as usize);
                    } else if temp1.is_none() {
                        temp1 = Some(reg as usize);
                    } else if temp2.is_none() {
                        temp2 = Some(reg as usize);
                    }
                }
                Operand::Operator(operator) => {
                    imm_temp = Some(operator as i32);
                }
            }
        }

        match opcode {
            // R-type
            RV32IInstruction::Add
            | RV32IInstruction::Sub
            | RV32IInstruction::Xor
            | RV32IInstruction::Or
            | RV32IInstruction::And
            | RV32IInstruction::Sll
            | RV32IInstruction::Srl
            | RV32IInstruction::Sra
            | RV32IInstruction::Slt
            | RV32IInstruction::Sltu => {
                rd = temp0.unwrap();
                rs1 = temp1.unwrap();
                rs2 = temp2.unwrap();
            }
            // B-type
            RV32IInstruction::Beq
            | RV32IInstruction::Bne
            | RV32IInstruction::Blt
            | RV32IInstruction::Bge
            | RV32IInstruction::Bltu
            | RV32IInstruction::Bgeu => {
                rs1 = temp0.unwrap();
                rs2 = temp1.unwrap();
                imm = imm_temp.unwrap();
            }
            // J-type
            RV32IInstruction::Jal => {
                rd = temp0.unwrap();
                imm = imm_temp.unwrap();
            }
            // I-type
            RV32IInstruction::Addi
            | RV32IInstruction::Xori
            | RV32IInstruction::Ori
            | RV32IInstruction::Andi
            | RV32IInstruction::Slti
            | RV32IInstruction::Sltiu
            | RV32IInstruction::Slli
            | RV32IInstruction::Srli
            | RV32IInstruction::Srai
            | RV32IInstruction::Jalr
            | RV32IInstruction::Lb
            | RV32IInstruction::Lh
            | RV32IInstruction::Lw
            | RV32IInstruction::Lbu
            | RV32IInstruction::Lhu => {
                rd = temp0.unwrap();
                rs1 = temp1.unwrap();
                imm = imm_temp.unwrap();
            }

            // S-type
            RV32IInstruction::Sb | RV32IInstruction::Sh | RV32IInstruction::Sw => {
                rs1 = temp0.unwrap();
                rs2 = temp1.unwrap();
                imm = imm_temp.unwrap();
            }
            // U-type
            RV32IInstruction::Lui | RV32IInstruction::Auipc => {
                rd = temp0.unwrap();
                imm = imm_temp.unwrap();
            }
            _ => {
                return Err(());
            }
        }

        match opcode {
            RV32IInstruction::Add => {
                self.registers[rd] = self.registers[rs1] + self.registers[rs2];
            }
            RV32IInstruction::Sub => {
                self.registers[rd] = self.registers[rs1] - self.registers[rs2];
            }
            RV32IInstruction::Xor => {
                self.registers[rd] = self.registers[rs1] ^ self.registers[rs2];
            }
            RV32IInstruction::Or => {
                self.registers[rd] = self.registers[rs1] | self.registers[rs2];
            }
            RV32IInstruction::And => {
                self.registers[rd] = self.registers[rs1] & self.registers[rs2];
            }
            RV32IInstruction::Sll => {
                self.registers[rd] = self.registers[rs1] << self.registers[rs2];
            }
            RV32IInstruction::Srl => {
                self.registers[rd] = self.registers[rs1] >> self.registers[rs2];
            }
            RV32IInstruction::Sra => {
                self.registers[rd] = ((self.registers[rs1] as i32) >> self.registers[rs2]) as u32;
            }
            RV32IInstruction::Slt => {
                self.registers[rd] = if (self.registers[rs1] as i32) < (self.registers[rs2] as i32)
                {
                    1
                } else {
                    0
                };
            }
            RV32IInstruction::Sltu => {
                self.registers[rd] = if self.registers[rs1] < self.registers[rs2] {
                    1
                } else {
                    0
                };
            }
            RV32IInstruction::Beq => {
                if self.registers[rs1] == self.registers[rs2] {
                    self.pc = (self.pc as i32 + imm) as u32;
                    // self.pc += 4 in previous instruction
                }
            }
            RV32IInstruction::Bne => {
                if self.registers[rs1] != self.registers[rs2] {
                    self.pc = (self.pc as i32 + imm) as u32;
                }
            }
            RV32IInstruction::Blt => {
                if (self.registers[rs1] as i32) < (self.registers[rs2] as i32) {
                    self.pc = (self.pc as i32 + imm) as u32;
                }
            }
            RV32IInstruction::Bge => {
                if (self.registers[rs1] as i32) >= (self.registers[rs2] as i32) {
                    self.pc = (self.pc as i32 + imm) as u32;
                }
            }
            RV32IInstruction::Bltu => {
                if self.registers[rs1] < self.registers[rs2] {
                    self.pc = (self.pc as i32 + imm) as u32;
                }
            }
            RV32IInstruction::Bgeu => {
                if self.registers[rs1] >= self.registers[rs2] {
                    self.pc = (self.pc as i32 + imm) as u32;
                }
            }
            RV32IInstruction::Jal => {
                self.registers[rd] = self.pc + 4;
                self.pc = (self.pc as i32 + imm) as u32;
            }
            RV32IInstruction::Addi => {
                self.registers[rd] = (self.registers[rs1] as i32).wrapping_add(imm as i32) as u32;
            }
            RV32IInstruction::Xori => {
                self.registers[rd] = ((self.registers[rs1] as i32) ^ (imm as i32)) as u32;
            }
            RV32IInstruction::Ori => {
                self.registers[rd] = ((self.registers[rs1] as i32) | (imm as i32)) as u32;
            }
            RV32IInstruction::Andi => {
                self.registers[rd] = ((self.registers[rs1] as i32) & (imm as i32)) as u32;
            }
            RV32IInstruction::Slti => {
                self.registers[rd] = if (self.registers[rs1] as i32) < (imm as i32) {
                    1
                } else {
                    0
                };
            }
            RV32IInstruction::Sltiu => {
                self.registers[rd] = if self.registers[rs1] < (imm as u32) {
                    1
                } else {
                    0
                };
            }
            RV32IInstruction::Slli => {
                let shamt = imm & 0b11111;
                self.registers[rd] = self.registers[rs1] << shamt;
            }
            RV32IInstruction::Srli => {
                let shamt = imm & 0b11111;
                self.registers[rd] = self.registers[rs1] >> shamt;
            }
            RV32IInstruction::Srai => {
                let shamt = imm & 0b11111;
                self.registers[rd] =
                    CPU::sign_extend(self.registers[rs1] >> shamt, 32 - shamt as u32);
            }
            RV32IInstruction::Jalr => {
                let t = self.pc;
                self.pc = ((self.registers[rs1] as i32 + imm) as u32) & !1;
                // set lsb to 0
                self.registers[rd] = t + 4;
            }
            RV32IInstruction::Lb => {
                let index = (self.registers[rs1] + CPU::sign_extend(imm as u32, 12)) as usize;
                self.registers[rd] = CPU::sign_extend(self.memory[index] as u32, 8);
                self.print_memory(rs1, imm);
            }
            RV32IInstruction::Lh => {
                let index = (self.registers[rs1] + CPU::sign_extend(imm as u32, 12)) as usize;
                let half_word = self.memory[index] as u32 | (self.memory[index + 1] as u32) << 8;
                self.registers[rd] = CPU::sign_extend(half_word as u32, 16);
                self.print_memory(rs1, imm);
            }
            RV32IInstruction::Lw => {
                let index = (self.registers[rs1] + CPU::sign_extend(imm as u32, 12)) as usize;

                self.registers[rd] = self.memory[index] as u32
                    | ((self.memory[index + 1]) as u32) << 8
                    | ((self.memory[index + 2]) as u32) << 16
                    | ((self.memory[index + 3]) as u32) << 24;
                self.print_memory(rs1, imm);
            }
            RV32IInstruction::Lbu => {
                let index = (self.registers[rs1] + CPU::sign_extend(imm as u32, 12)) as usize;
                self.registers[rd] = self.memory[index] as u32;
                self.print_memory(rs1, imm);
            }
            RV32IInstruction::Lhu => {
                let index = (self.registers[rs1] + CPU::sign_extend(imm as u32, 12)) as usize;

                self.registers[rd] =
                    self.memory[index] as u32 | (self.memory[index + 1] as u32) << 8;
                self.print_memory(rs1, imm);
            }
            RV32IInstruction::Sb => {
                let index = (self.registers[rs1] + CPU::sign_extend(imm as u32, 12)) as usize;
                self.memory[index] = (self.registers[rs2] & 0xff) as u8;
                self.print_memory(rs1, imm);
            }
            RV32IInstruction::Sh => {
                let index = (self.registers[rs1] + CPU::sign_extend(imm as u32, 12)) as usize;
                self.memory[index] = (self.registers[rs2] & 0xff) as u8;
                self.memory[index + 1] = ((self.registers[rs2] >> 8) & 0xff) as u8;
                self.print_memory(rs1, imm);
            }
            RV32IInstruction::Sw => {
                let index = (self.registers[rs1] + CPU::sign_extend(imm as u32, 12)) as usize;
                self.memory[index] = (self.registers[rs2] & 0xff) as u8;
                self.memory[index + 1] = (self.registers[rs2] >> 8 & 0xff) as u8;
                self.memory[index + 2] = (self.registers[rs2] >> 16 & 0xff) as u8;
                self.memory[index + 3] = (self.registers[rs2] >> 24 & 0xff) as u8;
                self.print_memory(rs1, imm);
            }
            RV32IInstruction::Lui => {
                self.registers[rd] = (imm << 12) as u32;
            }
            RV32IInstruction::Auipc => {
                self.registers[rd] = (self.pc as i32 + imm << 12) as u32;
            }
            RV32IInstruction::Ecall => {
                self.registers[rd] = self.pc;
                self.pc = (self.pc as i32 + imm) as u32;
            }
            RV32IInstruction::Ebreak => {
                self.registers[rd] = self.pc;
                self.pc = (self.pc as i32 + imm) as u32;
            }
            _ => {
                return Err(());
            }
        }
        return Ok(());
    }


}
