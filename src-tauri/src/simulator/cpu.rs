use crate::interface::assembler::Instruction;
use crate::interface::assembler::Operand;
use crate::simulator::dram::Dram;
use crate::modules::riscv::rv32i::constants::{RV32IInstruction, RV32IRegister};



pub struct CPU {
    dram: Dram,
    pub registers: [u32; 32],
    pub pc: u32,
    pub instructions: Vec<Instruction>,
}

impl CPU {
    pub fn new(mem_size: usize) -> Self {
        CPU {
            dram: Dram::new(mem_size),
            registers: [0; 32],
            pc: 0,
            instructions: Vec::new(),
        }
    }

    fn load_inst(&mut self, ir: Instruction) -> Result<bool, ()> {
        // TODO: Error handling
        self.instructions.push(ir);
        Ok(true)
    }


    pub fn execute(&mut self, instruction:Instruction) -> Result<(), ()> {
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
            RV32IInstruction::Add | RV32IInstruction::Sub | RV32IInstruction::Xor | RV32IInstruction::Or | RV32IInstruction::And | RV32IInstruction::Sll | RV32IInstruction::Srl | RV32IInstruction::Sra | RV32IInstruction::Slt | RV32IInstruction::Sltu => {
                rd = temp0.unwrap();
                rs1 = temp1.unwrap();
                rs2 = temp2.unwrap();
            }
            // B-type
            RV32IInstruction::Beq | RV32IInstruction::Bne | RV32IInstruction::Blt | RV32IInstruction::Bge | RV32IInstruction::Bltu | RV32IInstruction::Bgeu => {
                rs1 = temp0.unwrap();
                rs2 = temp1.unwrap();
                imm = imm_temp.unwrap();
            }
            
            // I-type
            RV32IInstruction::Addi | RV32IInstruction::Slti | RV32IInstruction::Sltiu | RV32IInstruction::Xori | RV32IInstruction::Ori | RV32IInstruction::Andi | RV32IInstruction::Slli | RV32IInstruction::Srli | RV32IInstruction::Srai 
            | RV32IInstruction::Jalr=> {
                rd = temp0.unwrap();
                rs1 = temp1.unwrap();
                imm = imm_temp.unwrap();
            }
            // 
            RV32IInstruction::Lb | RV32IInstruction::Lh | RV32IInstruction::Lw | RV32IInstruction::Lbu | RV32IInstruction::Lhu => {
                rd = temp0.unwrap();
                imm = imm_temp.unwrap();
            }
            // J-type
            RV32IInstruction::Jal => {
                rd = temp0.unwrap();
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
                self.registers[rd] = ((self.registers[rs1] as i32) >> self.registers[rs2])as u32;
            }
            RV32IInstruction::Slt => {
                self.registers[rd] = if (self.registers[rs1] as i32) < (self.registers[rs2] as i32) { 1 } else { 0 };
            }
            RV32IInstruction::Sltu => {
                self.registers[rd] = if self.registers[rs1] < self.registers[rs2] { 1 } else { 0 };
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
                self.registers[rd] = (self.registers[rs1]as i32).wrapping_add(imm as i32) as u32;
            }
            RV32IInstruction::Xori => {
                self.registers[rd] = ((self.registers[rs1]as i32) ^ (imm as i32)) as u32;
            }
            RV32IInstruction::Ori => {
                self.registers[rd] = ((self.registers[rs1]as i32) | (imm as i32)) as u32;
            }
            RV32IInstruction::Andi => {
                self.registers[rd] = ((self.registers[rs1]as i32) & (imm as i32)) as u32;
            }
            RV32IInstruction::Slti => {
                self.registers[rd] = if (self.registers[rs1] as i32) < (imm as i32) { 1 } else { 0 };
            }
            RV32IInstruction::Sltiu => {
                self.registers[rd] = if self.registers[rs1] < (imm as u32) { 1 } else { 0 };
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
                self.registers[rd] = CPU::sign_extend(self.registers[rs1]>>shamt, 32- shamt as u32);

            }
            RV32IInstruction::Lb => {
                let addr = (self.registers[rs1] as i32 + imm) as u32;
                let val = self.load(addr, 8)?;
                self.registers[rd] = val as u32;
            }
            RV32IInstruction::Lh => {
                let addr = (self.registers[rs1] as i32 + imm) as u32;
                let val = self.load(addr, 16)?;
                self.registers[rd] = val as u32;
            }
            RV32IInstruction::Lw => {
                let addr = (self.registers[rs1] as i32 + imm) as u32;
                let val = self.load(addr, 32)?;
                self.registers[rd] = val as u32;
            }
            RV32IInstruction::Lbu => {
                let addr = (self.registers[rs1] as i32 + imm) as u32;
                let val = self.load(addr, 8)?;
                self.registers[rd] = val;
            }
            RV32IInstruction::Lhu => {
                let addr = (self.registers[rs1] as i32 + imm) as u32;
                let val = self.load(addr, 16)?;
                self.registers[rd] = val;
            }
            RV32IInstruction::Jalr => {
                let t = self.pc;
                self.pc = ((self.registers[rs1] as i32 + imm) as u32) & !1;
                // set lsb to 0
                self.registers[rd] = t + 4;
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

    pub fn load(&mut self, addr: u32, size: u32) -> Result<u32, ()> {
        self.dram.load(addr, size)
    }
    
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
}

