use crate::assembler::rv32i::{Immediate12, Immediate20, Register, RV32I};
use crate::interface::assembler::{AssembleError, Assembler, Instruction, Memory, Operand};
use crate::interface::parser::{ParserInstSet, ParserResult};
use crate::modules::riscv::basic::interface::parser::*;
use crate::modules::riscv::rv32i::constants::*;
use ux::{u12, u20, u5};
const MAIN: i32 = 0x00400000;
const DATA: i32 = 0x10010000;
const MAX_RELATIVE_OFFSET: i32 = 0b0111_1111_1111_1111_1111;
const MIN_RELATIVE_OFFSET: i32 = -0b1000_0000_0000_0000_0000;

macro_rules! modify_label {
    ($label:ident, $line:expr, $imm:ident, $start:ident) => {
        match $label {
            ParserRISCVImmediate::Imm(imm) => {
                $imm = *imm;
            }
            ParserRISCVImmediate::Lbl((label, handler)) => {
                let mut address: RISCVImmediate = u32::from(*label) as i32;
                let mut line_addr: RISCVImmediate =
                    u32::from(ParserRISCVLabel::Text($start)) as i32;
                match handler {
                    ParserRISCVLabelHandler::Low => $imm = get_32u_low(address),
                    ParserRISCVLabelHandler::High => $imm = get_32u_high(address),
                    ParserRISCVLabelHandler::DeltaHigh => {
                        $imm = get_32u_high(address) - get_32u_high(line_addr)
                    }
                    ParserRISCVLabelHandler::DeltaMinusOneLow => {
                        line_addr = line_addr - 4;
                        $imm = get_32u_low(address) - get_32u_low(line_addr)
                    }
                }
            }
        }
    };
}

macro_rules! extract_opds {
    ($inst:expr, R, $rd:ident, $rs1:ident, $rs2:ident) => {
        if let [ParserRISCVInstOpd::Reg(rd), ParserRISCVInstOpd::Reg(rs1), ParserRISCVInstOpd::Reg(rs2)] = &$inst[..] {
            $rd = u32::from(*rd);
            $rs1 = u32::from(*rs1);
            $rs2 = u32::from(*rs2);
        }
    };
    ($inst:expr, I, $rd:ident, $rs1:ident, $imm:ident, $start:ident) => {
        if let [ParserRISCVInstOpd::Reg(rd), ParserRISCVInstOpd::Reg(rs1), ParserRISCVInstOpd::Imm(imm)] = &$inst.opd[..] {
            $rd = u32::from(*rd);
            $rs1 = u32::from(*rs1);
            modify_label!(imm, $inst.line, $imm, $start);
        }
    };
    ($inst:expr, S, $rs1:ident, $rs2:ident, $imm:ident, $start:ident) => {
        if let [ParserRISCVInstOpd::Reg(rs1), ParserRISCVInstOpd::Imm(imm), ParserRISCVInstOpd::Reg(rs2)] = &$inst.opd[..] {
            $rs2 = u32::from(*rs1);
            $rs1 = u32::from(*rs2);
            modify_label!(imm, $inst.line, $imm, $start);
        }
    };
    ($inst:expr, J, $rd:ident, $imm:ident, $start:ident, $error:ident) => {
        if let [ParserRISCVInstOpd::Reg(rd), ParserRISCVInstOpd::Lbl(label)] = &$inst.opd[..] {
            $rd = u32::from(*rd);
            let mut label_addr = u32::from(*label);
            let current_pc = u32::from(ParserRISCVLabel::Text($start));
            let jump = label_addr.wrapping_sub(current_pc) as i32;
            if jump > MAX_RELATIVE_OFFSET || jump < MIN_RELATIVE_OFFSET {
                $error.push(AssembleError{line: $start, msg: "Jump offset exceeds 20-bit signed integer range!".to_string()})
            } else {
                $imm = jump;
            }
        }
    };
    ($inst:expr, B, $rd:ident, $rs1:ident, $imm:ident, $start:ident, $error:ident) => {
        if let [ParserRISCVInstOpd::Reg(rd), ParserRISCVInstOpd::Reg(rs1), ParserRISCVInstOpd::Lbl(label)] = &$inst.opd[..] {
            $rd = u32::from(*rd);
            $rs1 = u32::from(*rs1);
            let mut label_addr = u32::from(*label);
            let current_pc = u32::from(ParserRISCVLabel::Text($start));
            let jump = label_addr.wrapping_sub(current_pc) as i32;
            if jump > MAX_RELATIVE_OFFSET || jump < MIN_RELATIVE_OFFSET {
                $error.push(AssembleError{line: $start, msg: "Branch offset exceeds 20-bit signed integer range!".to_string()})
            } else {
                $imm = jump;
            }
        }
    };
    ($inst:expr, U, $rd:ident, $imm:ident, $start:ident) => {
        if let [ParserRISCVInstOpd::Reg(rd), ParserRISCVInstOpd::Imm(imm)] = &$inst.opd[..] {
            $rd = u32::from(*rd);
            modify_label!(imm, $inst.line, $imm, $start);
        }
    };
}

pub struct RiscVAssembler;

impl RiscVAssembler {
    pub fn new() -> Self {
        RiscVAssembler
    }
}

impl From<ParserRISCVRegister> for u32 {
    fn from(register: ParserRISCVRegister) -> Self {
        match register {
            ParserRISCVRegister::RV32I(rv32i_reg) => u32::from(rv32i_reg),
            ParserRISCVRegister::RV32F(rv32f_reg) => 0,
        }
    }
}

impl From<RV32IRegister> for u32 {
    fn from(register: RV32IRegister) -> Self {
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

impl From<u32> for Operand {
    fn from(opd: u32) -> Self {
        match opd {
            0 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::Zero)),
            1 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::Ra)),
            2 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::Sp)),
            3 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::Gp)),
            4 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::Tp)),
            5 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::T0)),
            6 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::T1)),
            7 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::T2)),
            8 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::S0)),
            9 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::S1)),
            10 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::A0)),
            11 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::A1)),
            12 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::A2)),
            13 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::A3)),
            14 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::A4)),
            15 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::A5)),
            16 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::A6)),
            17 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::A7)),
            18 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::S2)),
            19 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::S3)),
            20 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::S4)),
            21 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::S5)),
            22 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::S6)),
            23 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::S7)),
            24 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::S8)),
            25 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::S9)),
            26 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::S10)),
            27 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::S11)),
            28 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::T3)),
            29 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::T4)),
            30 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::T5)),
            31 => Operand::Reg(ParserRISCVRegister::from(RV32IRegister::T6)),
            _ => panic!("No such register!"),
        }
    }
}

impl From<i32> for Operand {
    fn from(imm: i32) -> Self {
        Operand::Operator(imm)
    }
}

impl From<ParserRISCVImmediate> for i32 {
    fn from(imm: ParserRISCVImmediate) -> Self {
        match imm {
            ParserRISCVImmediate::Imm(imm) => imm,
            ParserRISCVImmediate::Lbl((label, handler)) => u32::from(label) as i32,
        }
    }
}

impl From<ParserRISCVLabel> for u32 {
    fn from(label: ParserRISCVLabel) -> Self {
        match label {
            ParserRISCVLabel::Text(pos) => MAIN as u32 + pos as u32 * 4,
            ParserRISCVLabel::Data((num, pos)) => {
                DATA as u32 + num as u32 * DATA_CHUNK_RECOMMEND_SIZE as u32 + pos as u32
            }
            ParserRISCVLabel::Unknown(_) => 0,
        }
    }
}

impl Assembler<RISCV> for RiscVAssembler {
    fn assemble(
        &mut self,
        inst: &ParserResultText<RISCV>,
        index: usize,
    ) -> Result<Instruction, Vec<AssembleError>> {
        let mut result = Instruction::new();
        let mut error: Vec<AssembleError> = Vec::new();
        match inst {
            ParserResultText::Text(inst) => match inst.op {
                ParserRISCVInstOp::RV32I(ins) => {
                    result.op = ParserRISCVInstOp::from(ins);
                    let mut rd: u32 = 0;
                    let mut rs1: u32 = 0;
                    let mut rs2: u32 = 0;
                    let mut imm: i32 = 0;
                    match ins {
                        RV32IInstruction::Add
                        | RV32IInstruction::And
                        | RV32IInstruction::Or
                        | RV32IInstruction::Sltu
                        | RV32IInstruction::Sll
                        | RV32IInstruction::Slt
                        | RV32IInstruction::Sra
                        | RV32IInstruction::Srl
                        | RV32IInstruction::Sub
                        | RV32IInstruction::Xor => {
                            extract_opds!(inst.opd, R, rd, rs1, rs2);
                            result.ins = Vec::from([
                                Operand::from(rd),
                                Operand::from(rs1),
                                Operand::from(rs2),
                            ]);
                        }
                        RV32IInstruction::Addi
                        | RV32IInstruction::Andi
                        | RV32IInstruction::Ori
                        | RV32IInstruction::Slti
                        | RV32IInstruction::Sltiu
                        | RV32IInstruction::Xori
                        | RV32IInstruction::Slli
                        | RV32IInstruction::Srai
                        | RV32IInstruction::Srli
                        | RV32IInstruction::Jalr
                        | RV32IInstruction::Lb
                        | RV32IInstruction::Lbu
                        | RV32IInstruction::Lh
                        | RV32IInstruction::Lhu
                        | RV32IInstruction::Ebreak
                        | RV32IInstruction::FenceI
                        | RV32IInstruction::Csrrc
                        | RV32IInstruction::Csrrci
                        | RV32IInstruction::Csrrs
                        | RV32IInstruction::Csrrsi
                        | RV32IInstruction::Csrrw
                        | RV32IInstruction::Csrrwi
                        | RV32IInstruction::Ecall => {
                            extract_opds!(inst, I, rd, rs1, imm, index);
                            result.ins = Vec::from([
                                Operand::from(rd),
                                Operand::from(rs1),
                                Operand::from(imm),
                            ]);
                        }
                        RV32IInstruction::Sb
                        | RV32IInstruction::Sh
                        | RV32IInstruction::Lw
                        | RV32IInstruction::Sw => {
                            extract_opds!(inst, S, rs1, rs2, imm, index);
                            result.ins = Vec::from([
                                Operand::from(imm),
                                Operand::from(rs1),
                                Operand::from(rs2),
                            ]);
                        }
                        RV32IInstruction::Jal => {
                            extract_opds!(inst, J, rd, imm, index, error);
                            result.ins = Vec::from([Operand::from(rd), Operand::from(imm)]);
                        }
                        RV32IInstruction::Beq
                        | RV32IInstruction::Bge
                        | RV32IInstruction::Bgeu
                        | RV32IInstruction::Blt
                        | RV32IInstruction::Bltu
                        | RV32IInstruction::Bne => {
                            extract_opds!(inst, B, rs1, rs2, imm, index, error);
                            result.ins = Vec::from([
                                Operand::from(rs1),
                                Operand::from(rs2),
                                Operand::from(imm),
                            ]);
                        }
                        RV32IInstruction::Auipc | RV32IInstruction::Lui => {
                            extract_opds!(inst, U, rd, imm, index);
                            result.ins = Vec::from([Operand::from(rd), Operand::from(imm)]);
                        }
                        RV32IInstruction::Fence => {
                            if let [ParserRISCVInstOpd::Imm(imm1), ParserRISCVInstOpd::Imm(imm2)] =
                                inst.opd[..]
                            {
                                let imm1 = i32::from(imm1);
                                let imm2 = i32::from(imm2);
                                result.ins = Vec::from([Operand::from(imm1), Operand::from(imm2)]);
                            }
                        }
                    }
                }
                ParserRISCVInstOp::RV32F(fins) => {
                    result.op = ParserRISCVInstOp::from(fins);
                }
            },
            ParserResultText::Align(_) => {}
        }
        // for (index, element) in ast.text.iter().enumerate(){
        //     match element {
        //         ParserResultText::Text(inst) => {
        //         }
        //         ParserResultText::Align(_) => {}
        //     }
        // }
        Ok(result)
    }

    fn dump(
        &mut self,
        ast: Result<ParserResult<RISCV>, Vec<ParserError>>,
    ) -> Result<Memory, Vec<AssembleError>> {
        let mut ast = ast.unwrap();
        let data = ast.data;
        let text = ast.text;
        let mut data_segment = Vec::new();
        let mut text_segment = Vec::new();
        let mut error: Vec<AssembleError> = Vec::new();
        for element in data {
            match element {
                ParserResultData::Data(data) => {
                    for chunk in data.chunks(4) {
                        let mut line = String::new();
                        for e in chunk.iter().rev() {
                            line.push_str(&format!("{:08b}", e));
                        }
                        data_segment.push(line);
                    }
                }
                ParserResultData::Align(power) => {
                    for _ in 0..(DATA + data_segment.len() as i32 * 4) % 2_i32.pow(power as u32) {
                        if let Some(last) = data_segment.last_mut() {
                            if last.len() < 32 {
                                last.insert_str(0, "00");
                            } else {
                                let mut line = String::new();
                                line.push_str("00");
                                data_segment.push(line);
                            }
                        }
                    }
                }
            }
        }
        let mut remaining_size = 1024 - data_segment.len() % 1024;
        for _ in 0..remaining_size {
            data_segment.push(String::from(format!("{:032b}", 0)));
        }
        for (index, element) in text.iter().enumerate() {
            let mut line: u32 = 0;
            // println!("{}", element.to_string());
            match element {
                ParserResultText::Text(inst) => {
                    match inst.op {
                        ParserRISCVInstOp::RV32I(ins) => {
                            let mut rd: u32 = 0;
                            let mut rs1: u32 = 0;
                            let mut rs2: u32 = 0;
                            let mut imm: i32 = 0;
                            match ins {
                                RV32IInstruction::Add
                                | RV32IInstruction::And
                                | RV32IInstruction::Or
                                | RV32IInstruction::Sltu
                                | RV32IInstruction::Sll
                                | RV32IInstruction::Slt
                                | RV32IInstruction::Sra
                                | RV32IInstruction::Srl
                                | RV32IInstruction::Sub
                                | RV32IInstruction::Xor => {
                                    extract_opds!(inst.opd, R, rd, rs1, rs2);
                                }
                                RV32IInstruction::Addi
                                | RV32IInstruction::Andi
                                | RV32IInstruction::Ori
                                | RV32IInstruction::Slti
                                | RV32IInstruction::Sltiu
                                | RV32IInstruction::Xori
                                | RV32IInstruction::Slli
                                | RV32IInstruction::Srai
                                | RV32IInstruction::Srli
                                | RV32IInstruction::Jalr
                                | RV32IInstruction::Lb
                                | RV32IInstruction::Lbu
                                | RV32IInstruction::Lh
                                | RV32IInstruction::Lhu
                                | RV32IInstruction::Ebreak
                                | RV32IInstruction::FenceI
                                | RV32IInstruction::Csrrc
                                | RV32IInstruction::Csrrci
                                | RV32IInstruction::Csrrs
                                | RV32IInstruction::Csrrsi
                                | RV32IInstruction::Csrrw
                                | RV32IInstruction::Csrrwi
                                | RV32IInstruction::Ecall => {
                                    extract_opds!(inst, I, rd, rs1, imm, index);
                                }
                                RV32IInstruction::Sb
                                | RV32IInstruction::Sh
                                | RV32IInstruction::Lw
                                | RV32IInstruction::Sw => {
                                    extract_opds!(inst, S, rs1, rs2, imm, index);
                                }
                                RV32IInstruction::Jal => {
                                    extract_opds!(inst, J, rd, imm, index, error);
                                }
                                RV32IInstruction::Beq
                                | RV32IInstruction::Bge
                                | RV32IInstruction::Bgeu
                                | RV32IInstruction::Blt
                                | RV32IInstruction::Bltu
                                | RV32IInstruction::Bne => {
                                    extract_opds!(inst, B, rs1, rs2, imm, index, error);
                                }
                                RV32IInstruction::Auipc | RV32IInstruction::Lui => {
                                    extract_opds!(inst, U, rd, imm, index);
                                }
                                RV32IInstruction::Fence => {
                                    if let [ParserRISCVInstOpd::Imm(imm1), ParserRISCVInstOpd::Imm(imm2)] =
                                        inst.opd[..]
                                    {
                                        let imm1 = i32::from(imm1);
                                        let imm2 = i32::from(imm2);
                                        let imm1u: u32 = imm1 as u32;
                                        let imm2u: u32 = imm2 as u32;
                                        line = Into::<u32>::into(RV32I::fence(
                                            0x0.into(),
                                            imm1u.into(),
                                            imm2u.into(),
                                            0x0.into(),
                                            0x0.into(),
                                        ));
                                    }
                                }
                            }
                            let shamt = if imm >= 0 {
                                Register(u5::try_from((imm & 0x1F) as u8).unwrap())
                            } else {
                                Register(
                                    u5::try_from(((!((-imm) as u8) & 0x1F) + 1) & 0x1F).unwrap(),
                                )
                            };
                            let imm_u12 = if imm >= 0 {
                                Immediate12(u12::try_from(imm as u16 & 0xFFF).unwrap())
                            } else {
                                Immediate12(u12::try_from((!((-imm) as u16) & 0xFFF) + 1).unwrap())
                            };
                            let imm_u20 = if imm >= 0 {
                                Immediate20(u20::try_from(imm as u32 & 0xFFFFF).unwrap())
                            } else {
                                Immediate20(
                                    u20::try_from((!((-imm) as u32) & 0xFFFFF) + 1).unwrap(),
                                )
                            };
                            match ins {
                                RV32IInstruction::Add => {
                                    line = Into::<u32>::into(RV32I::add(
                                        rs2.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Addi => {
                                    line = Into::<u32>::into(RV32I::addi(
                                        imm_u12.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::And => {
                                    line = Into::<u32>::into(RV32I::and(
                                        rs2.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Andi => {
                                    line = Into::<u32>::into(RV32I::andi(
                                        imm_u12.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Auipc => {
                                    line =
                                        Into::<u32>::into(RV32I::auipc(imm_u20.into(), rd.into()))
                                }
                                RV32IInstruction::Beq => {
                                    line = Into::<u32>::into(RV32I::beq(
                                        imm_u12.into(),
                                        rs2.into(),
                                        rs1.into(),
                                    ))
                                }
                                RV32IInstruction::Bge => {
                                    line = Into::<u32>::into(RV32I::bge(
                                        imm_u12.into(),
                                        rs2.into(),
                                        rs1.into(),
                                    ))
                                }
                                RV32IInstruction::Bgeu => {
                                    line = Into::<u32>::into(RV32I::bgeu(
                                        imm_u12.into(),
                                        rs2.into(),
                                        rs1.into(),
                                    ))
                                }
                                RV32IInstruction::Blt => {
                                    line = Into::<u32>::into(RV32I::blt(
                                        imm_u12.into(),
                                        rs2.into(),
                                        rs1.into(),
                                    ))
                                }
                                RV32IInstruction::Bltu => {
                                    line = Into::<u32>::into(RV32I::bltu(
                                        imm_u12.into(),
                                        rs2.into(),
                                        rs1.into(),
                                    ))
                                }
                                RV32IInstruction::Bne => {
                                    line = Into::<u32>::into(RV32I::bne(
                                        imm_u12.into(),
                                        rs2.into(),
                                        rs1.into(),
                                    ))
                                }
                                RV32IInstruction::Csrrc => {
                                    line = Into::<u32>::into(RV32I::csrrc(
                                        imm_u12.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Csrrci => {
                                    line = Into::<u32>::into(RV32I::csrrci(
                                        imm_u12.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Csrrs => {
                                    line = Into::<u32>::into(RV32I::csrrs(
                                        imm_u12.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Csrrsi => {
                                    line = Into::<u32>::into(RV32I::csrrsi(
                                        imm_u12.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Csrrw => {
                                    line = Into::<u32>::into(RV32I::csrrw(
                                        imm_u12.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Csrrwi => {
                                    line = Into::<u32>::into(RV32I::csrrwi(
                                        imm_u12.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Ebreak => {
                                    line = Into::<u32>::into(RV32I::ebreak())
                                }
                                RV32IInstruction::Ecall => line = Into::<u32>::into(RV32I::ecall()),
                                RV32IInstruction::Fence => {}
                                RV32IInstruction::FenceI => {
                                    line = Into::<u32>::into(RV32I::fencei())
                                }
                                RV32IInstruction::Jal => {
                                    line = Into::<u32>::into(RV32I::jal(imm_u20.into(), rd.into()))
                                }
                                RV32IInstruction::Jalr => {
                                    line = Into::<u32>::into(RV32I::jalr(
                                        imm_u12.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Lb => {
                                    line = Into::<u32>::into(RV32I::lb(
                                        imm_u12.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Lbu => {
                                    line = Into::<u32>::into(RV32I::lbu(
                                        imm_u12.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Lh => {
                                    line = Into::<u32>::into(RV32I::lh(
                                        imm_u12.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Lhu => {
                                    line = Into::<u32>::into(RV32I::lhu(
                                        imm_u12.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Lui => {
                                    line = Into::<u32>::into(RV32I::lui(imm_u20.into(), rd.into()))
                                }
                                RV32IInstruction::Lw => {
                                    line = Into::<u32>::into(RV32I::lw(
                                        imm_u12.into(),
                                        rs1.into(),
                                        rs2.into(),
                                    ))
                                }
                                RV32IInstruction::Or => {
                                    line = Into::<u32>::into(RV32I::or(
                                        rs2.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Ori => {
                                    line = Into::<u32>::into(RV32I::ori(
                                        imm_u12.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Sb => {
                                    line = Into::<u32>::into(RV32I::sb(
                                        imm_u12.into(),
                                        rs2.into(),
                                        rs1.into(),
                                    ))
                                }
                                RV32IInstruction::Sh => {
                                    line = Into::<u32>::into(RV32I::sh(
                                        imm_u12.into(),
                                        rs2.into(),
                                        rs1.into(),
                                    ))
                                }
                                RV32IInstruction::Sll => {
                                    line = Into::<u32>::into(RV32I::sll(
                                        rs2.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Slli => {
                                    line = Into::<u32>::into(RV32I::slli(
                                        shamt.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Slt => {
                                    line = Into::<u32>::into(RV32I::slt(
                                        rs2.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Slti => {
                                    line = Into::<u32>::into(RV32I::slti(
                                        imm_u12.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Sltiu => {
                                    line = Into::<u32>::into(RV32I::sltiu(
                                        imm_u12.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Sltu => {
                                    line = Into::<u32>::into(RV32I::sltu(
                                        rs2.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Sra => {
                                    line = Into::<u32>::into(RV32I::sra(
                                        rs2.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Srai => {
                                    line = Into::<u32>::into(RV32I::srai(
                                        shamt.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Srl => {
                                    line = Into::<u32>::into(RV32I::srl(
                                        rs2.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Srli => {
                                    line = Into::<u32>::into(RV32I::srli(
                                        shamt.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Sub => {
                                    line = Into::<u32>::into(RV32I::sub(
                                        rs2.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Sw => {
                                    line = Into::<u32>::into(RV32I::sw(
                                        imm_u12.into(),
                                        rs2.into(),
                                        rs1.into(),
                                    ))
                                }
                                RV32IInstruction::Xor => {
                                    line = Into::<u32>::into(RV32I::xor(
                                        rs2.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                                RV32IInstruction::Xori => {
                                    line = Into::<u32>::into(RV32I::xori(
                                        imm_u12.into(),
                                        rs1.into(),
                                        rd.into(),
                                    ))
                                }
                            }
                        }
                        ParserRISCVInstOp::RV32F(fins) => {}
                    }
                    let binary_string = format!("{:032b}", line);
                    text_segment.push(binary_string);
                    let hex_string = format!("{:08x}", line);
                }
                ParserResultText::Align(power) => {
                }
            }
        }
        Ok(Memory {
            data: data_segment,
            text: text_segment,
        })
    }
}
