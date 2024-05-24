use std::collections::VecDeque;

use crate::{
    interface::{
        assembler::{AssembleResult, Instruction, InstructionSet, Operand},
        simulator::SimulatesError,
    },
    modules::riscv::{
        basic::interface::parser::{ParserRISCVInstOp, ParserRISCVRegister, RISCV},
        middleware::backend_api::{self, syscall_output_print},
        rv32i::{
            assembler::rv32i::RV32I,
            constants::{RV32IInstruction, RV32IRegister},
        },
    },
    types::middleware_types::AssemblerConfig,
};

#[derive(Clone)]
pub struct CPUState {
    pub memory: Vec<u8>,
    pub registers: [u32; 32],
    pub pc: u32, // point to the NEXT instruction to be executed
}

pub struct CPU {
    pub state: CPUState,
    pub address_config: AssemblerConfig,
    pub data_segment: Vec<u32>,
    pub text_segment: Vec<u32>,
    pub instruction_set: Vec<InstructionSet<RISCV>>,
    pub breakpoints: Vec<u32>,
    pub undo_stack: VecDeque<CPUState>,
}

impl CPU {
    pub fn new(mem_size: usize, assembler_config: &AssemblerConfig, pathname: &str) -> Self {
        CPU {
            state: CPUState {
                memory: vec![0; mem_size],
                registers: [0; 32],
                pc: 0,
            },
            // update config
            address_config: assembler_config.clone(),
            data_segment: Vec::new(),
            text_segment: Vec::new(),
            instruction_set: Vec::new(),
            breakpoints: Vec::new(),
            undo_stack: VecDeque::new(),
        }
    }

    pub fn load_inst(&mut self, assemble_result: &AssembleResult<RISCV>) {
        self.state.registers[2] = self.address_config.stack_pointer_sp;
        self.state.pc = self.address_config.dot_text_base_address;

        let data_base_address = self.address_config.dot_data_base_address;
        let text_base_address = self.address_config.dot_text_base_address;

        // self.load_data_segment(data_segment);
        // self.load_text_segment(text_segment);

        // self.data_segment = assemble_result.data.clone();
        // self.instruction_set = assemble_result.instruction.clone();
    }

    pub fn run(&mut self) -> Result<(), SimulatesError> {
        if self.state.pc == self.address_config.dot_text_base_address {
            self.reset();
        }
        loop {
            let mut result = self.step();
            match result {
                Ok(()) => continue,
                _ => return result,
            }
        }
        Ok(())
    }

    pub fn debug(&mut self) -> Result<(), SimulatesError> {
        if self.state.pc == self.address_config.dot_text_base_address {
            self.reset();
        }
        loop {
            if self.breakpoints.contains(&self.state.pc) {
                break;
            }
            let mut result = self.step();
            match result {
                Ok(()) => continue,
                _ => return result,
            }
        }
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), SimulatesError> {
        let pc_copy = self.state.pc;
        if pc_copy > self.address_config.text_limit_address {
            return Err(SimulatesError {
                address: pc_copy,
                msg: "PC overflow.".to_string(),
            });
        }
        let inst = &self.instruction_set
            [(pc_copy - self.address_config.dot_text_base_address) as usize]
            .instruction;

        let opcode = &inst.operation;
        let mut imm: i32 = 0;
        let mut rd: usize = 0;
        let mut rs1: usize = 0;
        let mut rs2: usize = 0;
        let mut temp0: Option<usize> = None;
        let mut temp1: Option<usize> = None;
        let mut temp2: Option<usize> = None;
        let mut imm_temp: Option<i32> = None;
        for operand in inst.operands.clone() {
            match operand {
                Operand::Reg(reg) => {
                    let reg = u32::from(operand);
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
            ParserRISCVInstOp::RV32I(rv32i_inst) => {
                match rv32i_inst {
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
                        return Err(SimulatesError {
                            address: self.state.pc,
                            msg: "Invalid Instruction".to_string(),
                        });
                    }
                }
            }
            _ => {}
        }

        match opcode {
            ParserRISCVInstOp::RV32I(rv32i_inst) => {
                match rv32i_inst {
                    RV32IInstruction::Add => {
                        self.state.registers[rd] =
                            self.state.registers[rs1] + self.state.registers[rs2];
                    }
                    RV32IInstruction::Sub => {
                        self.state.registers[rd] =
                            self.state.registers[rs1] - self.state.registers[rs2];
                    }
                    RV32IInstruction::Xor => {
                        self.state.registers[rd] =
                            self.state.registers[rs1] ^ self.state.registers[rs2];
                    }
                    RV32IInstruction::Or => {
                        self.state.registers[rd] =
                            self.state.registers[rs1] | self.state.registers[rs2];
                    }
                    RV32IInstruction::And => {
                        self.state.registers[rd] =
                            self.state.registers[rs1] & self.state.registers[rs2];
                    }
                    RV32IInstruction::Sll => {
                        self.state.registers[rd] =
                            self.state.registers[rs1] << self.state.registers[rs2];
                    }
                    RV32IInstruction::Srl => {
                        self.state.registers[rd] =
                            self.state.registers[rs1] >> self.state.registers[rs2];
                    }
                    RV32IInstruction::Sra => {
                        self.state.registers[rd] = ((self.state.registers[rs1] as i32)
                            >> self.state.registers[rs2])
                            as u32;
                    }
                    RV32IInstruction::Slt => {
                        self.state.registers[rd] = if (self.state.registers[rs1] as i32)
                            < (self.state.registers[rs2] as i32)
                        {
                            1
                        } else {
                            0
                        };
                    }
                    RV32IInstruction::Sltu => {
                        self.state.registers[rd] =
                            if self.state.registers[rs1] < self.state.registers[rs2] {
                                1
                            } else {
                                0
                            };
                    }
                    RV32IInstruction::Beq => {
                        if self.state.registers[rs1] == self.state.registers[rs2] {
                            self.state.pc = (self.state.pc as i32 + imm) as u32;
                            // self.state.pc += 4 in previous instruction
                        }
                    }
                    RV32IInstruction::Bne => {
                        if self.state.registers[rs1] != self.state.registers[rs2] {
                            self.state.pc = (self.state.pc as i32 + imm) as u32;
                        }
                    }
                    RV32IInstruction::Blt => {
                        if (self.state.registers[rs1] as i32) < (self.state.registers[rs2] as i32) {
                            self.state.pc = (self.state.pc as i32 + imm) as u32;
                        }
                    }
                    RV32IInstruction::Bge => {
                        if (self.state.registers[rs1] as i32) >= (self.state.registers[rs2] as i32)
                        {
                            self.state.pc = (self.state.pc as i32 + imm) as u32;
                        }
                    }
                    RV32IInstruction::Bltu => {
                        if self.state.registers[rs1] < self.state.registers[rs2] {
                            self.state.pc = (self.state.pc as i32 + imm) as u32;
                        }
                    }
                    RV32IInstruction::Bgeu => {
                        if self.state.registers[rs1] >= self.state.registers[rs2] {
                            self.state.pc = (self.state.pc as i32 + imm) as u32;
                        }
                    }
                    RV32IInstruction::Jal => {
                        self.state.registers[rd] = self.state.pc + 4;
                        self.state.pc = (self.state.pc as i32 + imm) as u32;
                    }
                    RV32IInstruction::Addi => {
                        self.state.registers[rd] =
                            (self.state.registers[rs1] as i32).wrapping_add(imm as i32) as u32;
                    }
                    RV32IInstruction::Xori => {
                        self.state.registers[rd] =
                            ((self.state.registers[rs1] as i32) ^ (imm as i32)) as u32;
                    }
                    RV32IInstruction::Ori => {
                        self.state.registers[rd] =
                            ((self.state.registers[rs1] as i32) | (imm as i32)) as u32;
                    }
                    RV32IInstruction::Andi => {
                        self.state.registers[rd] =
                            ((self.state.registers[rs1] as i32) & (imm as i32)) as u32;
                    }
                    RV32IInstruction::Slti => {
                        self.state.registers[rd] =
                            if (self.state.registers[rs1] as i32) < (imm as i32) {
                                1
                            } else {
                                0
                            };
                    }
                    RV32IInstruction::Sltiu => {
                        self.state.registers[rd] = if self.state.registers[rs1] < (imm as u32) {
                            1
                        } else {
                            0
                        };
                    }
                    RV32IInstruction::Slli => {
                        let shamt = imm & 0b11111;
                        self.state.registers[rd] = self.state.registers[rs1] << shamt;
                    }
                    RV32IInstruction::Srli => {
                        let shamt = imm & 0b11111;
                        self.state.registers[rd] = self.state.registers[rs1] >> shamt;
                    }
                    RV32IInstruction::Srai => {
                        let shamt = imm & 0b11111;
                        self.state.registers[rd] =
                            CPU::sign_extend(self.state.registers[rs1] >> shamt, 32 - shamt as u32);
                    }
                    RV32IInstruction::Jalr => {
                        let t = self.state.pc;
                        self.state.pc = ((self.state.registers[rs1] as i32 + imm) as u32) & !1;
                        // set lsb to 0
                        self.state.registers[rd] = t + 4;
                    }
                    RV32IInstruction::Lb => {
                        let index =
                            (self.state.registers[rs1] + CPU::sign_extend(imm as u32, 12)) as usize;
                        self.state.registers[rd] =
                            CPU::sign_extend(self.state.memory[index] as u32, 8);
                        self.print_memory(rs1, imm);
                    }
                    RV32IInstruction::Lh => {
                        let index =
                            (self.state.registers[rs1] + CPU::sign_extend(imm as u32, 12)) as usize;
                        let half_word = self.state.memory[index] as u32
                            | (self.state.memory[index + 1] as u32) << 8;
                        self.state.registers[rd] = CPU::sign_extend(half_word as u32, 16);
                        self.print_memory(rs1, imm);
                    }
                    RV32IInstruction::Lw => {
                        let index =
                            (self.state.registers[rs1] + CPU::sign_extend(imm as u32, 12)) as usize;

                        self.state.registers[rd] = self.state.memory[index] as u32
                            | ((self.state.memory[index + 1]) as u32) << 8
                            | ((self.state.memory[index + 2]) as u32) << 16
                            | ((self.state.memory[index + 3]) as u32) << 24;
                        self.print_memory(rs1, imm);
                    }
                    RV32IInstruction::Lbu => {
                        let index =
                            (self.state.registers[rs1] + CPU::sign_extend(imm as u32, 12)) as usize;
                        self.state.registers[rd] = self.state.memory[index] as u32;
                        self.print_memory(rs1, imm);
                    }
                    RV32IInstruction::Lhu => {
                        let index =
                            (self.state.registers[rs1] + CPU::sign_extend(imm as u32, 12)) as usize;

                        self.state.registers[rd] = self.state.memory[index] as u32
                            | (self.state.memory[index + 1] as u32) << 8;
                        self.print_memory(rs1, imm);
                    }
                    RV32IInstruction::Sb => {
                        let index =
                            (self.state.registers[rs1] + CPU::sign_extend(imm as u32, 12)) as usize;
                        self.state.memory[index] = (self.state.registers[rs2] & 0xff) as u8;
                        self.print_memory(rs1, imm);
                    }
                    RV32IInstruction::Sh => {
                        let index =
                            (self.state.registers[rs1] + CPU::sign_extend(imm as u32, 12)) as usize;
                        self.state.memory[index] = (self.state.registers[rs2] & 0xff) as u8;
                        self.state.memory[index + 1] =
                            ((self.state.registers[rs2] >> 8) & 0xff) as u8;
                        self.print_memory(rs1, imm);
                    }
                    RV32IInstruction::Sw => {
                        let index =
                            (self.state.registers[rs1] + CPU::sign_extend(imm as u32, 12)) as usize;
                        self.state.memory[index] = (self.state.registers[rs2] & 0xff) as u8;
                        self.state.memory[index + 1] =
                            (self.state.registers[rs2] >> 8 & 0xff) as u8;
                        self.state.memory[index + 2] =
                            (self.state.registers[rs2] >> 16 & 0xff) as u8;
                        self.state.memory[index + 3] =
                            (self.state.registers[rs2] >> 24 & 0xff) as u8;
                        self.print_memory(rs1, imm);
                    }
                    RV32IInstruction::Lui => {
                        self.state.registers[rd] = (imm << 12) as u32;
                    }
                    RV32IInstruction::Auipc => {
                        self.state.registers[rd] = (self.state.pc as i32 + imm << 12) as u32;
                    }
                    RV32IInstruction::Ecall => {
                        match self.state.registers[17] {
                            1 => {
                                // print integer
                                println!("{}", self.state.registers[RV32IRegister::A0 as usize]);
                                let output =
                                    &self.state.registers[RV32IRegister::A0 as usize].to_string();
                                syscall_output_print("pathname", output);
                            }
                            4 => {
                                // print string
                                let mut index =
                                    self.state.registers[RV32IRegister::A0 as usize] as usize;
                                while self.state.memory[index] != 0 {
                                    print!("{}", self.state.memory[index] as char);
                                    index += 1;
                                }
                            }
                            5 => {
                                // read integer
                                let mut input = String::new();
                                std::io::stdin().read_line(&mut input).unwrap();
                                self.state.registers[RV32IRegister::A0 as usize] =
                                    input.trim().parse().unwrap();
                            }
                            8 => {
                                // read string
                                let mut input = String::new();
                                std::io::stdin().read_line(&mut input).unwrap();
                                let mut index =
                                    self.state.registers[RV32IRegister::A0 as usize] as usize;
                                for c in input.chars() {
                                    self.state.memory[index] = c as u8;
                                    index += 1;
                                }
                                self.state.memory[index] = 0;
                            }
                            10 => {
                                // exit
                                return Ok(());
                            }
                            11 => {
                                // print character
                                print!(
                                    "{}",
                                    self.state.registers[RV32IRegister::A0 as usize] as u8 as char
                                );
                            }
                            12 => {
                                // read character
                                let mut input = String::new();
                                std::io::stdin().read_line(&mut input).unwrap();
                                self.state.registers[RV32IRegister::A0 as usize] =
                                    input.chars().next().unwrap() as u32;
                            }
                            34 => {
                                // Prints an integer (in hexdecimal format left-padded
                                // with zeroes)
                                print!("{:08x}", self.state.registers[RV32IRegister::A0 as usize]);
                            }
                            35 => {
                                // Prints an integer (in binary format left-padded with
                                // zeroes)
                                print!("{:032b}", self.state.registers[RV32IRegister::A0 as usize]);
                            }
                            36 => {
                                // Prints an integer (in decimal format)
                                print!("{}", self.state.registers[RV32IRegister::A0 as usize]);
                            }
                            _ => {
                                // invalid syscall
                                return Err(SimulatesError {
                                    address: self.state.pc,
                                    msg: "invalid syscall".to_string(),
                                });
                            }
                        }
                        match self.state.registers[17] {
                            10 => {
                                // `exit` syscall
                                // NOTE: NOT error
                                return Err(SimulatesError {
                                    address: self.state.pc,
                                    msg: "program is finished running (0)".to_string(),
                                });
                            }
                            _ => {}
                        }
                    }
                    RV32IInstruction::Ebreak => {}
                    _ => {
                        return Err(SimulatesError {
                            address: self.state.pc,
                            msg: "Invalid Instruction".to_string(),
                        })
                    }
                }
            }
            _ => {
                return Err(SimulatesError {
                    address: self.state.pc,
                    msg: "Invalid Instruction".to_string(),
                })
            }
        }
        return Ok(());
    }

    pub fn reset(&mut self) {
        self.state.registers.fill(0);
        self.state.registers[2] = self.address_config.stack_pointer_sp;
        self.state.pc = self.address_config.dot_text_base_address;
        self.state.memory.fill(0);
        // let data_segment_copy = self.data_segment.clone();
        // let text_segment_copy = self.text_segment.clone();
    }

    fn undo(&mut self) -> Result<(), SimulatesError> {
        if let Some(state) = self.undo_stack.pop_front() {
            self.state = state;
            return Ok(());
        }
        return Err(SimulatesError {
            address: self.state.pc,
            msg: "No more undo steps.".to_string(),
        });
    }

    pub fn set_breakpoint(&mut self, line_number: u32) -> Result<(), SimulatesError> {
        if !self.breakpoints.contains(&line_number) {
            self.breakpoints.push(line_number);
        }
        return Ok(());
    }

    pub fn remove_breakpoint(&mut self, line_number: u32) -> Result<(), SimulatesError> {
        if let Some(index) = self.breakpoints.iter().position(|&x| x == line_number) {
            self.breakpoints.remove(index);
        }
        return Ok(());
    }

    pub fn load_data_segment(&mut self, data_segment: &Vec<u8>) {
        self.state.memory[self.address_config.dot_data_base_address as usize
            ..(self.address_config.dot_data_base_address as usize + data_segment.len())]
            .copy_from_slice(&data_segment);
    }

    pub fn load_text_segment(&mut self, text_segment: &Vec<u8>) {
        self.state.memory[self.address_config.dot_text_base_address as usize
            ..(self.address_config.dot_text_base_address as usize + text_segment.len())]
            .copy_from_slice(&text_segment);
    }

    fn sign_extend(data: u32, size: u32) -> u32 {
        assert!(size > 0 && size <= 32);
        (((data << (32 - size)) as i32) >> (32 - size)) as u32
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
                    self.state.registers[i],
                    i + 1,
                    abi[i + 1],
                    self.state.registers[i + 1],
                    i + 2,
                    abi[i + 2],
                    self.state.registers[i + 2],
                    i + 3,
                    abi[i + 3],
                    self.state.registers[i + 3],
                )
            );
        }
        println!("{}", output);
    }

    pub fn print_memory(&mut self, rs1: usize, imm: i32) {
        let index = (self.state.registers[rs1] + CPU::sign_extend(imm as u32, 12)) as usize;
        let mut output = String::from("");
        for i in index..index + 4 {
            output = format!(
                "{}\n{}",
                output,
                format!("memory[{:>#18x}]={:>#18x}", i, self.state.memory[i],)
            );
        }
        println!("{}", output);
    }
}
