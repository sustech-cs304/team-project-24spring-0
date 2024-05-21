use std::mem;
use std::collections::VecDeque;
use super::instruction::*;
use crate::{
    interface::simulator::SimulatesError,
    modules::riscv::rv32i::constants::RV32IRegister,
    types::middleware_types::AssemblerConfig,
};


#[derive(Clone)]
struct CPUState {
    pub memory: Vec<u8>,
    pub registers: [u32; 32],
    pub pc: u32,
}


pub struct CPU {
    pub state: CPUState,
    pub address_config: AssemblerConfig,
    pub data_segment: Vec<u8>,
    pub text_segment: Vec<u8>,
    pub breakpoints: Vec<u32>,
    undo_stack: VecDeque<CPUState>,
}

impl CPU {
    pub fn new(mem_size: usize, assembler_config: &AssemblerConfig) -> Self {
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
            breakpoints: Vec::new(),
            undo_stack: VecDeque::new(),
        }
    }

    pub fn load_inst(&mut self, data_segment: &Vec<u8>, text_segment: &Vec<u8>) {
        let data_base_address = self.address_config.dot_data_base_address;
        let text_base_address = self.address_config.dot_text_base_address;
    
        // Copy data segment
        self.state.memory[data_base_address as usize..(data_base_address as usize + data_segment.len())]
            .copy_from_slice(data_segment);
    
        // Copy text segment
        self.state.memory[text_base_address as usize..(text_base_address as usize + text_segment.len())]
            .copy_from_slice(text_segment);

        self.data_segment = data_segment.clone();
        self.text_segment = text_segment.clone();
    }

    pub fn run(&mut self) -> Result<(), SimulatesError> {
        self.reset();
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
        self.reset();
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
        let raw_inst = self.fetch();
            let mut inst: SIMInstruction = self.decode(raw_inst);
            let pc_copy = self.state.pc;
            if (self.state.pc as usize) >= self.state.memory.len() {
                return Err(SimulatesError {
                    address: self.state.pc,
                    msg: "PC overflow.".to_string(),
                });
            }
            match self.execute(&mut inst, pc_copy) {
                Ok(()) => {}
                Err(e) => return Err(e),
            }
            match inst.name.as_str() {
                "ecall" => match self.state.registers[17] {
                    93 => {
                        // `exit` syscall
                        // NOTE: NOT error
                        return Err(SimulatesError {
                            address: pc_copy,
                            msg: "Program exited with exit code 0.".to_string(),
                        });
                    }
                    _ => {}
                },
                "unimp" => {
                    return Err(SimulatesError {
                        address: pc_copy,
                        msg: "Reached an unimplemented instruction.".to_string(),
                    });
                }
                _ => {}
            }
        return Ok(());
    }

    pub fn reset(&mut self) {
        self.state.registers.fill(0);
        self.state.registers[2] = self.address_config.stack_pointer_sp;
        self.state.pc = self.address_config.dot_text_base_address;
        self.state.memory.fill(0);
        let data_segment_copy = self.data_segment.clone();
        let text_segment_copy = self.text_segment.clone();
        self.load_inst(&data_segment_copy, &text_segment_copy);
    }

    fn undo(&mut self) -> Result<(), SimulatesError>{
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


    fn fetch(&self) -> u32 {
        let index = self.state.pc as usize;
        self.state.memory[index] as u32
            | ((self.state.memory[index + 1]) as u32) << 8
            | ((self.state.memory[index + 2]) as u32) << 16
            | ((self.state.memory[index + 3]) as u32) << 24
    }

    fn decode(&self, inst: u32) -> SIMInstruction {
        let mut instruction = SIMInstruction::new();
        let opcode = inst & 0b1111111;
        instruction.opcode = opcode;
        match opcode {
            // R Type
            0b0110011 => {
                let rd = ((inst >> 7) & 0b11111) as usize;
                let funct3 = (inst >> 12) & 0b111;
                let rs1 = ((inst >> 15) & 0b11111) as usize;
                let rs2 = ((inst >> 20) & 0b11111) as usize;
                let funct7 = (inst >> 25) & 0b1111111;
                instruction.type_data = InstTypeData::R {
                    rd,
                    funct3,
                    rs1,
                    rs2,
                    funct7,
                };
                instruction.type_name = InstTypeName::R;
            }

            // I Type
            0b0010011 | 0b0000011 | 0b1100111 | 0b1110011 => {
                let rd = ((inst >> 7) & 0b11111) as usize;
                let funct3 = (inst >> 12) & 0b111;
                let rs1 = ((inst >> 15) & 0b11111) as usize;
                let imm = (inst >> 20) & 0b111111111111;
                let imm = CPU::sign_extend(imm, 12);
                instruction.type_data = InstTypeData::I {
                    rd,
                    funct3,
                    rs1,
                    imm,
                };
                instruction.type_name = InstTypeName::I;
            }

            // S Type
            0b0100011 => {
                let imm4_0 = (inst >> 7) & 0b11111;
                let imm11_5 = (inst >> 25) & 0b1111111;
                let imm = ((imm11_5 << 5) | imm4_0) as i32 as u32;
                let imm = CPU::sign_extend(imm, 12);

                let funct3 = (inst >> 12) & 0b111;
                let rs1 = ((inst >> 15) & 0b11111) as usize;
                let rs2 = ((inst >> 20) & 0b11111) as usize;
                instruction.type_data = InstTypeData::S {
                    imm,
                    funct3,
                    rs1,
                    rs2,
                };
                instruction.type_name = InstTypeName::S;
            }

            // B type
            0b1100011 => {
                let imm11 = (inst >> 7) & 0b1;
                let imm4_1 = (inst >> 8) & 0b1111;
                let imm10_5 = (inst >> 25) & 0b111111;
                let imm12 = (inst >> 31) & 0b1;
                let imm = (imm12 << 12) | (imm11 << 11) | (imm10_5 << 5) | (imm4_1 << 1);
                let imm = CPU::sign_extend(imm, 12);

                let funct3 = (inst >> 12) & 0b111;
                let rs1 = ((inst >> 15) & 0b11111) as usize;
                let rs2 = ((inst >> 20) & 0b11111) as usize;
                instruction.type_data = InstTypeData::B {
                    imm,
                    funct3,
                    rs1,
                    rs2,
                };
                instruction.type_name = InstTypeName::B;
            }

            // J type
            0b1101111 => {
                let rd = ((inst >> 7) & 0b11111) as usize;

                let imm19_12 = (inst >> 12) & 0b11111111;
                let imm11 = (inst >> 20) & 0b1;
                let imm10_1 = (inst >> 21) & 0b1111111111;
                let imm20 = (inst >> 31) & 0b1;
                let imm = (imm20 << 20) | (imm19_12 << 12) | (imm11 << 11) | (imm10_1 << 1);
                let imm = CPU::sign_extend(imm, 12);
                instruction.type_data = InstTypeData::J { rd, imm };
                instruction.type_name = InstTypeName::J;
            }

            // U type
            0b0110111 | 0b0010111 => {
                let rd = ((inst >> 7) & 0b11111) as usize;
                let imm = (inst >> 12) & 0b11111111111111111111;
                instruction.type_data = InstTypeData::U { rd, imm };
                instruction.type_name = InstTypeName::U;
            }

            // Fence
            0b0001111 => {
                instruction.type_data = InstTypeData::Fence;
                instruction.type_name = InstTypeName::Fence;
            }

            _ => {
                instruction.type_data = InstTypeData::Unimp;
                instruction.type_name = InstTypeName::Unimp;
            }
        }
        if inst == 0 || inst == 0xc0001073 {
            instruction.type_data = InstTypeData::Unimp;
            instruction.type_name = InstTypeName::Unimp;
        }
        instruction
    }

    fn execute(&mut self, inst: &mut SIMInstruction, pc_copy: u32) -> Result<(), SimulatesError> {
        match inst.type_name {
            InstTypeName::R => {
                if let InstTypeData::R {
                    rd,
                    funct3,
                    funct7,
                    rs1,
                    rs2,
                } = inst.type_data
                {
                    match funct3 {
                        0x0 => match funct7 {
                            0x0 => {
                                inst.name = format!("add     x{},x{},x{}", rd, rs1, rs2);
                                self.state.registers[rd] =
                                    self.state.registers[rs1].wrapping_add(self.state.registers[rs2]);
                            }
                            0x20 => {
                                inst.name = format!("sub     x{},x{},x{}", rd, rs1, rs2);
                                self.state.registers[rd] =
                                    self.state.registers[rs1].wrapping_sub(self.state.registers[rs2]);
                            }
                            _ => {
                                return Err(SimulatesError {
                                    address: pc_copy,
                                    msg: format!("unknown R funct7: {:#09b}", funct7),
                                });
                            }
                        },
                        0x4 => {
                            inst.name = format!("xor     x{},x{},x{}", rd, rs1, rs2);
                            self.state.registers[rd] = self.state.registers[rs1] ^ self.state.registers[rs2];
                        }
                        0x6 => {
                            inst.name = format!("or      x{},x{},x{}", rd, rs1, rs2);
                            self.state.registers[rd] = self.state.registers[rs1] | self.state.registers[rs2];
                        }
                        0x7 => {
                            inst.name = format!("and     x{},x{},x{}", rd, rs1, rs2);
                            self.state.registers[rd] = self.state.registers[rs1] & self.state.registers[rs2];
                        }
                        0x1 => {
                            inst.name = format!("sll     x{},x{},x{}", rd, rs1, rs2);
                            self.state.registers[rd] = self.state.registers[rs1] << self.state.registers[rs2];
                        }
                        0x5 => match funct7 {
                            0x0 => {
                                inst.name = format!("srl     x{},x{},x{}", rd, rs1, rs2);
                                self.state.registers[rd] = self.state.registers[rs1] >> self.state.registers[rs2];
                            }
                            0x20 => {
                                inst.name = format!("sra     x{},x{},x{}", rd, rs1, rs2);
                                self.state.registers[rd] =
                                    ((self.state.registers[rs1] as i32) >> self.state.registers[rs2]) as u32;
                            }
                            _ => {
                                return Err(SimulatesError {
                                    address: pc_copy,
                                    msg: format!("unknown R funct7: {:#09b}", funct7),
                                });
                            }
                        },
                        0x2 => {
                            inst.name = format!("slt     x{},x{},x{}", rd, rs1, rs2);
                            self.state.registers[rd] =
                                if (self.state.registers[rs1] as i32) < (self.state.registers[rs2] as i32) {
                                    1
                                } else {
                                    0
                                }
                        }
                        0x3 => {
                            inst.name = format!("sltu    x{},x{},x{}", rd, rs1, rs2);
                            self.state.registers[rd] = if self.state.registers[rs1] < self.state.registers[rs2] {
                                1
                            } else {
                                0
                            }
                        }
                        _ => {
                            return Err(SimulatesError {
                                address: pc_copy,
                                msg: format!("execute: unimplemented R funct3: {:#05b}", funct3),
                            });
                        }
                    };
                }
            }
            InstTypeName::B => {
                if let InstTypeData::B {
                    imm,
                    funct3,
                    rs1,
                    rs2,
                } = inst.type_data
                {
                    match funct3 {
                        0x0 => {
                            inst.name = format!(
                                "beq     x{},x{},{:08x}",
                                rs1,
                                rs2,
                                (self.state.pc) as i32 + imm as i32
                            );
                            let lhs = self.state.registers[rs1];
                            let rhs = self.state.registers[rs2];
                            if lhs == rhs {
                                self.state.pc = (self.state.pc as i32 + imm as i32) as u32;
                                return Ok(());
                            };
                        }
                        0x1 => {
                            inst.name = format!(
                                "bne     x{},x{},{:08x}",
                                rs1,
                                rs2,
                                (self.state.pc) as i32 + imm as i32
                            );
                            let lhs = self.state.registers[rs1];
                            let rhs = self.state.registers[rs2];
                            if lhs != rhs {
                                self.state.pc = (self.state.pc as i32 + imm as i32) as u32;
                                return Ok(());
                            };
                        }
                        0x4 => {
                            inst.name = format!(
                                "blt     x{},x{},{:08x}",
                                rs1,
                                rs2,
                                (self.state.pc) as i32 + imm as i32
                            );
                            let lhs = self.state.registers[rs1] as i32;
                            let rhs = self.state.registers[rs2] as i32;
                            if lhs < rhs {
                                self.state.pc = (self.state.pc as i32 + imm as i32) as u32;
                                return Ok(());
                            };
                        }
                        0x5 => {
                            inst.name = format!(
                                "bge     x{},x{},{:08x}",
                                rs1,
                                rs2,
                                (self.state.pc) as i32 + imm as i32
                            );
                            let lhs = self.state.registers[rs1] as i32;
                            let rhs = self.state.registers[rs2] as i32;
                            if lhs >= rhs {
                                self.state.pc = (self.state.pc as i32 + imm as i32) as u32;
                                return Ok(());
                            };
                        }
                        0x6 => {
                            inst.name = format!(
                                "bltu    x{},x{},{:08x}",
                                rs1,
                                rs2,
                                (self.state.pc) as i32 + imm as i32
                            );
                            let lhs = self.state.registers[rs1];
                            let rhs = self.state.registers[rs2];
                            if lhs < rhs {
                                self.state.pc = (self.state.pc as i32 + imm as i32) as u32;
                                return Ok(());
                            };
                        }
                        0x7 => {
                            inst.name = format!(
                                "bgeu    x{},x{},{:08x}",
                                rs1,
                                rs2,
                                (self.state.pc) as i32 + imm as i32
                            );
                            let lhs = self.state.registers[rs1];
                            let rhs = self.state.registers[rs2];
                            if lhs >= rhs {
                                self.state.pc = (self.state.pc as i32 + imm as i32) as u32;
                                return Ok(());
                            };
                        }
                        _ => {
                            return Err(SimulatesError {
                                address: pc_copy,
                                msg: format!("execute: unimplemented B funct3: {:#05b}", funct3),
                            });
                        }
                    };
                }
            }
            InstTypeName::J => {
                if let InstTypeData::J { rd, imm } = inst.type_data {
                    match inst.opcode {
                        0b1101111 => {
                            inst.name = format!("jal     x{},{:08x}", rd, imm);
                            self.state.registers[rd] = self.state.pc + 4;
                            self.state.pc = (self.state.pc as i32 + imm as i32) as u32;
                            self.state.registers[0] = 0;
                            return Ok(());
                        }
                        _ => {
                            panic!("execute: unimplemented J opcode: {:#09b}", inst.opcode);
                        }
                    };
                }
            }
            InstTypeName::I => {
                if let InstTypeData::I {
                    rd,
                    funct3,
                    rs1,
                    imm,
                } = inst.type_data
                {
                    match inst.opcode {
                        0b0010011 => match funct3 {
                            0x0 => {
                                inst.name = format!("addi    x{},x{},{}", rd, rs1, imm as i32);
                                self.state.registers[rd] =
                                    (self.state.registers[rs1] as i32).wrapping_add(imm as i32) as u32;

                                if rd == 0 && rs1 == 0 && imm == 0 {
                                    inst.name = String::from("nop");
                                }
                            }
                            0x4 => {
                                inst.name = format!("xori    x{},x{},{}", rd, rs1, imm as i32);
                                self.state.registers[rd] =
                                    ((self.state.registers[rs1] as i32) ^ (imm as i32)) as u32;
                            }
                            0x6 => {
                                inst.name = format!("ori     x{},x{},{}", rd, rs1, imm as i32);
                                self.state.registers[rd] =
                                    ((self.state.registers[rs1] as i32) | (imm as i32)) as u32;
                            }
                            0x7 => {
                                inst.name = format!("andi    x{},x{},{}", rd, rs1, imm as i32);
                                self.state.registers[rd] =
                                    ((self.state.registers[rs1] as i32) & (imm as i32)) as u32;
                            }
                            0x2 => {
                                inst.name = format!("slti    x{},x{},{}", rd, rs1, imm as i32);
                                self.state.registers[rd] = if (self.state.registers[rs1] as i32) < (imm as i32)
                                {
                                    1
                                } else {
                                    0
                                }
                            }
                            0x3 => {
                                inst.name = format!("sltiu   x{},x{},{}", rd, rs1, imm);
                                self.state.registers[rd] = if self.state.registers[rs1] < imm { 1 } else { 0 }
                            }
                            0x1 => {
                                let shamt = imm & 0b11111;
                                inst.name = format!("slli    x{},x{},{:#x}", rd, rs1, shamt);
                                self.state.registers[rd] = self.state.registers[rs1] << shamt;
                            }
                            0x5 => match (imm >> 5) & 0b1111111 {
                                0 => {
                                    let shamt = imm & 0b11111;
                                    inst.name = format!("srli    x{},x{},{:#x}", rd, rs1, shamt);
                                    self.state.registers[rd] = self.state.registers[rs1] >> shamt;
                                }
                                0b0100000 => {
                                    let shamt = imm & 0b11111;
                                    inst.name = format!("srai    x{},x{},{:#x}", rd, rs1, shamt);
                                    self.state.registers[rd] =
                                        CPU::sign_extend(self.state.registers[rs1] >> shamt, 32 - shamt);
                                }
                                _ => {
                                    panic!("should never be here.")
                                }
                            },
                            _ => {
                                panic!("unknown I funct3: {:#05b}", funct3,);
                            }
                        },
                        0b0000011 => match funct3 {
                            0x0 => {
                                inst.name = format!("lb      x{},{}(x{})", rd, imm as i32, rs1);
                                let index =
                                    (self.state.registers[rs1] + CPU::sign_extend(imm, 12)) as usize;
                                self.state.registers[rd] = CPU::sign_extend(self.state.memory[index] as u32, 8);
                            }
                            0x1 => {
                                inst.name = format!("lh      x{},{}(x{})", rd, imm as i32, rs1);
                                let index =
                                    (self.state.registers[rs1] + CPU::sign_extend(imm, 12)) as usize;
                                let half_word = self.state.memory[index] as u32
                                    | (self.state.memory[index + 1] as u32) << 8;
                                self.state.registers[rd] = CPU::sign_extend(half_word as u32, 16);
                            }
                            0x2 => {
                                inst.name = format!("lw      x{},{}(x{})", rd, imm as i32, rs1);
                                let index =
                                    (self.state.registers[rs1] + CPU::sign_extend(imm, 12)) as usize;

                                self.state.registers[rd] = self.state.memory[index] as u32
                                    | ((self.state.memory[index + 1]) as u32) << 8
                                    | ((self.state.memory[index + 2]) as u32) << 16
                                    | ((self.state.memory[index + 3]) as u32) << 24;
                            }
                            0x4 => {
                                inst.name = format!("lbu     x{},{}(x{})", rd, imm, rs1);
                                let index =
                                    (self.state.registers[rs1] + CPU::sign_extend(imm, 12)) as usize;
                                self.state.registers[rd] = self.state.memory[index] as u32;
                            }
                            0x5 => {
                                inst.name = format!("lhu     x{},{}(x{})", rd, imm, rs1);
                                let index =
                                    (self.state.registers[rs1] + CPU::sign_extend(imm, 12)) as usize;

                                self.state.registers[rd] = self.state.memory[index] as u32
                                    | (self.state.memory[index + 1] as u32) << 8;
                            }
                            _ => {
                                panic!("unknown I funct3: {:#05b}", funct3);
                            }
                        },
                        0b1100111 => match funct3 {
                            0x0 => {
                                inst.name = format!("jalr    x{},x{},{:#x}", rd, rs1, imm);
                                let pc_copy = self.state.pc;
                                self.state.pc = self.state.registers[rs1] + CPU::sign_extend(imm, 12);
                                self.state.pc &= !1; // set lsb to 0
                                self.state.registers[rd] = pc_copy + 4;

                                self.state.registers[0] = 0;
                                return Ok(());
                            }
                            _ => {
                                panic!("unknown I funct3: {:#05b}", funct3);
                            }
                        },
                        0b1110011 => match funct3 {
                            0b000 => match imm {
                                0x0 => {
                                    inst.name = String::from("ecall");
                                    let syscall = self.state.registers[RV32IRegister::A7 as usize];
                                    match syscall {
                                        1 => {
                                            // print integer
                                            println!(
                                                "{}",
                                                self.state.registers[RV32IRegister::A0 as usize]
                                            );
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
                                            //syscall_input()
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
                                                self.state.registers[RV32IRegister::A0 as usize] as u8
                                                    as char
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
                                            print!(
                                                "{:08x}",
                                                self.state.registers[RV32IRegister::A0 as usize]
                                            );
                                        }
                                        35 => {
                                            // Prints an integer (in binary format left-padded with
                                            // zeroes)
                                            print!(
                                                "{:032b}",
                                                self.state.registers[RV32IRegister::A0 as usize]
                                            );
                                        }
                                        36 => {
                                            // Prints an integer (in decimal format)
                                            print!(
                                                "{}",
                                                self.state.registers[RV32IRegister::A0 as usize]
                                            );
                                        }
                                        _ => {
                                            panic!("unknown syscall: {}", syscall);
                                        }
                                    }
                                }
                                0x1 => {
                                    inst.name = String::from("ebreak");
                                }
                                0b1100000010 => {
                                    inst.name = String::from("mret");
                                }
                                _ => {
                                    panic!("unknown I imm: {:#014b}", imm)
                                }
                            },
                            0b001 => {
                                inst.name = format!("csrrw   x{},{:#x},x{}", rd, imm, rs1);
                            }
                            0b010 => {
                                inst.name = format!("csrrs   x{},{:#x},x{}", rd, imm, rs1);
                            }
                            0b011 => {
                                inst.name = format!("csrrc   x{},{:#x},x{}", rd, imm, rs1);
                            }
                            0b101 => {
                                inst.name = format!("csrrwi  x{},{:#x},{}", rd, imm, rs1);
                            }
                            0b110 => {
                                inst.name = format!("csrrsi  x{},{:#x},{}", rd, imm, rs1);
                            }
                            0b111 => {
                                inst.name = format!("csrrci  x{},{:#x},{}", rd, imm, rs1);
                            }
                            _ => {
                                panic!("unknown I funct3: {:#05b}", funct3);
                            }
                        },
                        _ => {
                            panic!("unknown I opcode: {:#09b}", inst.opcode);
                        }
                    };
                }
            }
            InstTypeName::S => {
                if let InstTypeData::S {
                    imm,
                    funct3,
                    rs1,
                    rs2,
                } = inst.type_data
                {
                    match funct3 {
                        0x0 => {
                            inst.name = format!("sb      x{},{}(x{})", rs2, imm as i32, rs1);
                            let index = (self.state.registers[rs1] + CPU::sign_extend(imm, 12)) as usize;
                            self.state.memory[index] = (self.state.registers[rs2] & 0xff) as u8;
                        }
                        0x1 => {
                            inst.name = format!("sh      x{},{}(x{})", rs2, imm as i32, rs1);
                            let index = (self.state.registers[rs1] + CPU::sign_extend(imm, 12)) as usize;
                            self.state.memory[index] = (self.state.registers[rs2] & 0xff) as u8;
                            self.state.memory[index + 1] = (self.state.registers[rs2] >> 8 & 0xff) as u8;
                        }
                        0x2 => {
                            inst.name = format!("sw      x{},{}(x{})", rs2, imm as i32, rs1);
                            let index = (self.state.registers[rs1] + CPU::sign_extend(imm, 12)) as usize;
                            self.state.memory[index] = (self.state.registers[rs2] & 0xff) as u8;
                            self.state.memory[index + 1] = (self.state.registers[rs2] >> 8 & 0xff) as u8;
                            self.state.memory[index + 2] = (self.state.registers[rs2] >> 16 & 0xff) as u8;
                            self.state.memory[index + 3] = (self.state.registers[rs2] >> 24 & 0xff) as u8;
                        }
                        _ => {
                            panic!("unknown S funct3: {:#05b}", funct3);
                        }
                    };
                }
            }
            InstTypeName::U => {
                if let InstTypeData::U { rd, imm } = inst.type_data {
                    match inst.opcode {
                        0b0110111 => {
                            inst.name = format!("lui     x{},{:#x}", rd, imm);
                            self.state.registers[rd] = imm << 12;
                        }
                        0b0010111 => {
                            inst.name = format!("auipc   x{},{:#x}", rd, imm);
                            self.state.registers[rd] = self.state.pc + (imm << 12);
                        }
                        _ => {
                            panic!("unknown U opcode: {:#09b}", inst.opcode);
                        }
                    };
                }
            }
            InstTypeName::Fence => inst.name = String::from("fence"),
            InstTypeName::Unimp => inst.name = String::from("unimp"),
        }
        self.undo_stack.push_front(self.state.clone());
        self.state.pc += 4;
        Ok(())
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
