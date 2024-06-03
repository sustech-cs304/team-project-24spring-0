use ux::{u12, u20, u5};

use crate::{
    interface::{
        assembler::{
            AssembleResult,
            Assembler,
            AssemblyError,
            Instruction,
            InstructionSet,
            Memory,
        },
        parser::ParserResult,
    },
    modules::riscv::{
        basic::{
            assembler::riscv::{Immediate12, Immediate20, Register},
            interface::parser::*,
        },
        rv32i::assembler::rv32i::RV32I,
    },
    types::middleware_types::AssemblerConfig,
};
const MAX_RELATIVE_OFFSET: i32 = 0b0111_1111_1111_1111_1111;
const MIN_RELATIVE_OFFSET: i32 = -0b1000_0000_0000_0000_0000;

macro_rules! modify_label {
    ($self:ident, $label:ident, $line:expr, $imm:ident, $start:ident) => {
        match $label {
            ParserRISCVImmediate::Imm(imm) => {
                $imm = *imm;
            }
            ParserRISCVImmediate::Lbl((label, handler)) => {
                let address: u32 = u32::from(*label) + $self.data;
                let mut line_addr: u32 = u32::from(ParserRISCVLabel::Text($start)) + $self.main;
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
    ($self:ident, $inst:expr, I, $rd:ident, $rs1:ident, $imm:ident, $start:ident) => {
        if let [ParserRISCVInstOpd::Reg(rd), ParserRISCVInstOpd::Reg(rs1), ParserRISCVInstOpd::Imm(imm)] = &$inst.opd[..] {
            $rd = u32::from(*rd);
            $rs1 = u32::from(*rs1);
            modify_label!($self, imm, $inst.line, $imm, $start);
        }
    };
    ($self:ident, $inst:expr, S, $rs1:ident, $rs2:ident, $imm:ident, $start:ident) => {
        if let [ParserRISCVInstOpd::Reg(rs1), ParserRISCVInstOpd::Imm(imm), ParserRISCVInstOpd::Reg(rs2)] = &$inst.opd[..] {
            $rs2 = u32::from(*rs1);
            $rs1 = u32::from(*rs2);
            modify_label!($self, imm, $inst.line, $imm, $start);
        }
    };
    ($inst:expr, J, $rd:ident, $imm:ident, $start:ident, $error:ident) => {
        if let [ParserRISCVInstOpd::Reg(rd), ParserRISCVInstOpd::Lbl(label)] = &$inst.opd[..] {
            $rd = u32::from(*rd);
            let label_addr = u32::from(*label);
            let current_pc = u32::from(ParserRISCVLabel::Text($start));
            let jump = label_addr.wrapping_sub(current_pc) as i32;
            if jump > MAX_RELATIVE_OFFSET || jump < MIN_RELATIVE_OFFSET {
                $error.push(AssemblyError{line: $inst.line, msg: "Jump offset exceeds 20-bit signed integer range!".to_string()})
            } else {
                $imm = jump;
            }
        }
    };
    ($inst:expr, B, $rd:ident, $rs1:ident, $imm:ident, $start:ident, $error:ident) => {
        if let [ParserRISCVInstOpd::Reg(rd), ParserRISCVInstOpd::Reg(rs1), ParserRISCVInstOpd::Lbl(label)] = &$inst.opd[..] {
            $rd = u32::from(*rd);
            $rs1 = u32::from(*rs1);
            let label_addr = u32::from(*label);
            let current_pc = u32::from(ParserRISCVLabel::Text($start));
            let jump = label_addr.wrapping_sub(current_pc) as i32;
            if jump > MAX_RELATIVE_OFFSET || jump < MIN_RELATIVE_OFFSET {
                $error.push(AssemblyError{line: $inst.line, msg: "Branch offset exceeds 20-bit signed integer range!".to_string()})
            } else {
                $imm = jump;
            }
        }
    };
    ($self:ident, $inst:expr, U, $rd:ident, $imm:ident, $start:ident) => {
        if let [ParserRISCVInstOpd::Reg(rd), ParserRISCVInstOpd::Imm(imm)] = &$inst.opd[..] {
            $rd = u32::from(*rd);
            modify_label!($self, imm, $inst.line, $imm, $start);
        }
    };
    ($inst:expr, R4, $rd:ident, $rs1:ident, $rs2:ident, $rs3:ident) => {
        if let [ParserRISCVInstOpd::Reg(rd), ParserRISCVInstOpd::Reg(rs1), ParserRISCVInstOpd::Reg(rs2), ParserRISCVInstOpd::Reg(rs3)] = &$inst[..] {
            $rd = u32::from(*rd);
            $rs1 = u32::from(*rs1);
            $rs2 = u32::from(*rs2);
            $rs3 = u32::from(*rs3);
        }
    };
}

pub struct RiscVAssembler {
    main: u32,
    data: u32,
}

impl RiscVAssembler {
    pub fn new() -> Self {
        let config = AssemblerConfig::default();
        RiscVAssembler {
            main: config.dot_text_base_address as u32,
            data: config.dot_data_base_address as u32,
        }
    }
}

impl Assembler<RISCV> for RiscVAssembler {
    fn assemble(
        &mut self,
        ast: ParserResult<RISCV>,
    ) -> Result<AssembleResult<RISCV>, Vec<AssemblyError>> {
        let mut results: Vec<InstructionSet<RISCV>> = Vec::new();
        let mut error: Vec<AssemblyError> = Vec::new();
        for (index, element) in ast.text.iter().enumerate() {
            let mut result = Instruction::new(ParserRISCVInstOp::from(RV32IInstruction::Add));
            let mut line = InstructionSet::new(Instruction::new(ParserRISCVInstOp::from(
                RV32IInstruction::Add,
            )));
            match element {
                ParserResultText::Text(inst) => {
                    line.line_number = inst.line as u64;
                    match inst.op {
                        ParserRISCVInstOp::RV32I(ins) => {
                            result.operation = ParserRISCVInstOp::from(ins);
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
                                    result.operands = Vec::from([
                                        rd as RISCVImmediate,
                                        rs1 as RISCVImmediate,
                                        rs2 as RISCVImmediate,
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
                                    extract_opds!(self, inst, I, rd, rs1, imm, index);
                                    result.operands = Vec::from([
                                        rd as RISCVImmediate,
                                        rs1 as RISCVImmediate,
                                        imm,
                                    ]);
                                }
                                RV32IInstruction::Sb
                                | RV32IInstruction::Sh
                                | RV32IInstruction::Lw
                                | RV32IInstruction::Sw => {
                                    extract_opds!(self, inst, S, rs1, rs2, imm, index);
                                    result.operands = Vec::from([
                                        rs2 as RISCVImmediate,
                                        imm,
                                        rs1 as RISCVImmediate,
                                    ]);
                                }
                                RV32IInstruction::Jal => {
                                    extract_opds!(inst, J, rd, imm, index, error);
                                    result.operands = Vec::from([rd as RISCVImmediate, imm]);
                                }
                                RV32IInstruction::Beq
                                | RV32IInstruction::Bge
                                | RV32IInstruction::Bgeu
                                | RV32IInstruction::Blt
                                | RV32IInstruction::Bltu
                                | RV32IInstruction::Bne => {
                                    extract_opds!(inst, B, rs1, rs2, imm, index, error);
                                    result.operands = Vec::from([
                                        rs1 as RISCVImmediate,
                                        rs2 as RISCVImmediate,
                                        imm,
                                    ]);
                                }
                                RV32IInstruction::Auipc | RV32IInstruction::Lui => {
                                    extract_opds!(self, inst, U, rd, imm, index);
                                    result.operands = Vec::from([rd as RISCVImmediate, imm]);
                                }
                                RV32IInstruction::Fence => {
                                    if let [ParserRISCVInstOpd::Imm(imm1), ParserRISCVInstOpd::Imm(imm2)] =
                                        inst.opd[..]
                                    {
                                        let imm1 = i32::from(imm1);
                                        let imm2 = i32::from(imm2);
                                        result.operands = Vec::from([imm1, imm2]);
                                    }
                                }
                            }
                        }
                        ParserRISCVInstOp::RV32F(fins) => {
                            result.operation = ParserRISCVInstOp::from(fins);
                            let mut rd: u32 = 0;
                            let mut rs1: u32 = 0;
                            let mut rs2: u32 = 0;
                            let mut rs3: u32 = 0;
                            let mut imm: i32 = 0;
                            match fins {
                                RV32FInstruction::FaddS
                                | RV32FInstruction::FclassS
                                | RV32FInstruction::FcvtSW
                                | RV32FInstruction::FcvtSWu
                                | RV32FInstruction::FcvtWS
                                | RV32FInstruction::FcvtWuS
                                | RV32FInstruction::FdivS
                                | RV32FInstruction::FeqS
                                | RV32FInstruction::FleS
                                | RV32FInstruction::FltS
                                | RV32FInstruction::FmaxS
                                | RV32FInstruction::FminS
                                | RV32FInstruction::FmulS
                                | RV32FInstruction::FmvSX
                                | RV32FInstruction::FmvXS
                                | RV32FInstruction::FsgnjS
                                | RV32FInstruction::FsgnjnS
                                | RV32FInstruction::FsgnjxS
                                | RV32FInstruction::FsqrtS
                                | RV32FInstruction::FsubS => {
                                    extract_opds!(inst.opd, R, rd, rs1, rs2);
                                    result.operands = Vec::from([
                                        rd as RISCVImmediate,
                                        rs1 as RISCVImmediate,
                                        rs2 as RISCVImmediate,
                                    ]);
                                }
                                RV32FInstruction::FmaddS
                                | RV32FInstruction::FmsubS
                                | RV32FInstruction::FnmaddS
                                | RV32FInstruction::FnmsubS => {
                                    extract_opds!(inst.opd, R4, rd, rs1, rs2, rs3);
                                    result.operands = Vec::from([
                                        rd as RISCVImmediate,
                                        rs1 as RISCVImmediate,
                                        rs2 as RISCVImmediate,
                                        rs3 as RISCVImmediate,
                                    ]);
                                }
                                RV32FInstruction::Flw => {
                                    extract_opds!(self, inst, I, rd, rs1, imm, index);
                                    result.operands = Vec::from([
                                        rd as RISCVImmediate,
                                        rs1 as RISCVImmediate,
                                        imm,
                                    ]);
                                }
                                RV32FInstruction::Fsw => {
                                    extract_opds!(self, inst, S, rs1, rs2, imm, index);
                                    result.operands = Vec::from([
                                        imm,
                                        rs1 as RISCVImmediate,
                                        rs2 as RISCVImmediate,
                                    ]);
                                }
                            }
                        }
                    }
                }
                ParserResultText::Align(_) => {}
            }
            match process_code(self, index, element) {
                Ok(result) => {
                    line.code = result.code;
                    line.basic = result.basic;
                }
                Err(err) => error.extend(err),
            };
            line.address = self.main + index as u32 * 4;
            let old_operation = line.instruction.operation;
            let new_instruction = std::mem::replace(&mut line.instruction, result);
            results.push(line);
        }
        if error.is_empty() {
            Ok(AssembleResult {
                data: ast.data,
                instruction: results,
            })
        } else {
            Err(error)
        }
    }

    fn update_config(&mut self, config: &AssemblerConfig) {
        self.main = config.dot_text_base_address as u32;
        self.data = config.dot_data_base_address as u32;
    }

    fn dump(&mut self, ast: ParserResult<RISCV>) -> Result<Memory, Vec<AssemblyError>> {
        let data = ast.data;
        let text = ast.text;
        let mut data_segment = Vec::new();
        let mut text_segment = Vec::new();
        let mut error: Vec<AssemblyError> = Vec::new();
        for chunk in data.chunks(4) {
            let mut line = String::new();
            for e in chunk.iter().rev() {
                line.push_str(&format!("{:08b}", e));
            }
            data_segment.push(line);
        }
        let remaining_size = 1024 - data_segment.len() % 1024;
        for _ in 0..remaining_size {
            data_segment.push(String::from(format!("{:032b}", 0)));
        }
        for (index, element) in text.iter().enumerate() {
            match process_code(self, index, element) {
                Ok(line) => {
                    let binary_string = format!("{:032b}", line.code);
                    text_segment.push(binary_string);
                }
                Err(err) => error.extend(err),
            };
        }
        let data_content = data_segment.join("\n");
        let text_content = text_segment.join("\n");
        if error.is_empty() {
            Ok(Memory {
                data: data_content,
                text: text_content,
            })
        } else {
            Err(error)
        }
    }
}

pub struct ProcessResult {
    pub code: u32,
    pub basic: String,
}

fn process_code(
    assembler: &RiscVAssembler,
    index: usize,
    element: &ParserResultText<RISCV>,
) -> Result<ProcessResult, Vec<AssemblyError>> {
    let mut line: u32 = 0;
    let mut basic = String::new();
    let mut error: Vec<AssemblyError> = Vec::new();
    match element {
        ParserResultText::Text(inst) => match inst.op {
            ParserRISCVInstOp::RV32I(ins) => {
                let mut rd: u32 = 0;
                let mut rs1: u32 = 0;
                let mut rs2: u32 = 0;
                let mut imm: i32 = 0;
                let mut pred: i32 = 0;
                let mut succ: i32 = 0;
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
                        extract_opds!(assembler, inst, I, rd, rs1, imm, index);
                    }
                    RV32IInstruction::Sb
                    | RV32IInstruction::Sh
                    | RV32IInstruction::Lw
                    | RV32IInstruction::Sw => {
                        extract_opds!(assembler, inst, S, rs1, rs2, imm, index);
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
                        extract_opds!(assembler, inst, U, rd, imm, index);
                    }
                    RV32IInstruction::Fence => {
                        if let [ParserRISCVInstOpd::Imm(imm1), ParserRISCVInstOpd::Imm(imm2)] =
                            inst.opd[..]
                        {
                            pred = i32::from(imm1);
                            succ = i32::from(imm2);
                            let imm1u: u32 = pred as u32;
                            let imm2u: u32 = succ as u32;
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
                    Register(u5::try_from(((!((-imm) as u8) & 0x1F) + 1) & 0x1F).unwrap())
                };
                let imm_u12 = if imm >= 0 {
                    Immediate12(u12::try_from(imm as u16 & 0xFFF).unwrap())
                } else {
                    Immediate12(u12::try_from((!((-imm) as u16) & 0xFFF) + 1).unwrap())
                };
                let imm_u20 = if imm >= 0 {
                    Immediate20(u20::try_from(imm as u32 & 0xFFFFF).unwrap())
                } else {
                    Immediate20(u20::try_from((!((-imm) as u32) & 0xFFFFF) + 1).unwrap())
                };
                basic = format_instruction(inst.op, rd, rs1, rs2, imm, pred, succ);
                match ins {
                    RV32IInstruction::Add => {
                        line = Into::<u32>::into(RV32I::add(rs2.into(), rs1.into(), rd.into()));
                    }
                    RV32IInstruction::Addi => {
                        line = Into::<u32>::into(RV32I::addi(imm_u12.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::And => {
                        line = Into::<u32>::into(RV32I::and(rs2.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::Andi => {
                        line = Into::<u32>::into(RV32I::andi(imm_u12.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::Auipc => {
                        line = Into::<u32>::into(RV32I::auipc(imm_u20.into(), rd.into()))
                    }
                    RV32IInstruction::Beq => {
                        line = Into::<u32>::into(RV32I::beq(imm_u12.into(), rs2.into(), rs1.into()))
                    }
                    RV32IInstruction::Bge => {
                        line = Into::<u32>::into(RV32I::bge(imm_u12.into(), rs2.into(), rs1.into()))
                    }
                    RV32IInstruction::Bgeu => {
                        line =
                            Into::<u32>::into(RV32I::bgeu(imm_u12.into(), rs2.into(), rs1.into()))
                    }
                    RV32IInstruction::Blt => {
                        line = Into::<u32>::into(RV32I::blt(imm_u12.into(), rs2.into(), rs1.into()))
                    }
                    RV32IInstruction::Bltu => {
                        line =
                            Into::<u32>::into(RV32I::bltu(imm_u12.into(), rs2.into(), rs1.into()))
                    }
                    RV32IInstruction::Bne => {
                        line = Into::<u32>::into(RV32I::bne(imm_u12.into(), rs2.into(), rs1.into()))
                    }
                    RV32IInstruction::Csrrc => {
                        line =
                            Into::<u32>::into(RV32I::csrrc(imm_u12.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::Csrrci => {
                        line =
                            Into::<u32>::into(RV32I::csrrci(imm_u12.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::Csrrs => {
                        line =
                            Into::<u32>::into(RV32I::csrrs(imm_u12.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::Csrrsi => {
                        line =
                            Into::<u32>::into(RV32I::csrrsi(imm_u12.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::Csrrw => {
                        line =
                            Into::<u32>::into(RV32I::csrrw(imm_u12.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::Csrrwi => {
                        line =
                            Into::<u32>::into(RV32I::csrrwi(imm_u12.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::Ebreak => line = Into::<u32>::into(RV32I::ebreak()),
                    RV32IInstruction::Ecall => line = Into::<u32>::into(RV32I::ecall()),
                    RV32IInstruction::Fence => {}
                    RV32IInstruction::FenceI => line = Into::<u32>::into(RV32I::fencei()),
                    RV32IInstruction::Jal => {
                        line = Into::<u32>::into(RV32I::jal(imm_u20.into(), rd.into()))
                    }
                    RV32IInstruction::Jalr => {
                        line = Into::<u32>::into(RV32I::jalr(imm_u12.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::Lb => {
                        line = Into::<u32>::into(RV32I::lb(imm_u12.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::Lbu => {
                        line = Into::<u32>::into(RV32I::lbu(imm_u12.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::Lh => {
                        line = Into::<u32>::into(RV32I::lh(imm_u12.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::Lhu => {
                        line = Into::<u32>::into(RV32I::lhu(imm_u12.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::Lui => {
                        line = Into::<u32>::into(RV32I::lui(imm_u20.into(), rd.into()))
                    }
                    RV32IInstruction::Lw => {
                        line = Into::<u32>::into(RV32I::lw(imm_u12.into(), rs1.into(), rs2.into()))
                    }
                    RV32IInstruction::Or => {
                        line = Into::<u32>::into(RV32I::or(rs2.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::Ori => {
                        line = Into::<u32>::into(RV32I::ori(imm_u12.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::Sb => {
                        line = Into::<u32>::into(RV32I::sb(imm_u12.into(), rs2.into(), rs1.into()))
                    }
                    RV32IInstruction::Sh => {
                        line = Into::<u32>::into(RV32I::sh(imm_u12.into(), rs2.into(), rs1.into()))
                    }
                    RV32IInstruction::Sll => {
                        line = Into::<u32>::into(RV32I::sll(rs2.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::Slli => {
                        line = Into::<u32>::into(RV32I::slli(shamt.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::Slt => {
                        line = Into::<u32>::into(RV32I::slt(rs2.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::Slti => {
                        line = Into::<u32>::into(RV32I::slti(imm_u12.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::Sltiu => {
                        line =
                            Into::<u32>::into(RV32I::sltiu(imm_u12.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::Sltu => {
                        line = Into::<u32>::into(RV32I::sltu(rs2.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::Sra => {
                        line = Into::<u32>::into(RV32I::sra(rs2.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::Srai => {
                        line = Into::<u32>::into(RV32I::srai(shamt.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::Srl => {
                        line = Into::<u32>::into(RV32I::srl(rs2.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::Srli => {
                        line = Into::<u32>::into(RV32I::srli(shamt.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::Sub => {
                        line = Into::<u32>::into(RV32I::sub(rs2.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::Sw => {
                        line = Into::<u32>::into(RV32I::sw(imm_u12.into(), rs2.into(), rs1.into()))
                    }
                    RV32IInstruction::Xor => {
                        line = Into::<u32>::into(RV32I::xor(rs2.into(), rs1.into(), rd.into()))
                    }
                    RV32IInstruction::Xori => {
                        line = Into::<u32>::into(RV32I::xori(imm_u12.into(), rs1.into(), rd.into()))
                    }
                }
            }
            ParserRISCVInstOp::RV32F(..) => {}
        },
        ParserResultText::Align(..) => {}
    }
    if error.is_empty() {
        Ok(ProcessResult { code: line, basic })
    } else {
        Err(error)
    }
}

fn format_instruction(
    instruction: ParserRISCVInstOp,
    rd: u32,
    rs1: u32,
    rs2: u32,
    imm: i32,
    imm1: i32,
    imm2: i32,
) -> String {
    match instruction {
        ParserRISCVInstOp::RV32I(ins) => match ins {
            RV32IInstruction::Add
            | RV32IInstruction::And
            | RV32IInstruction::Or
            | RV32IInstruction::Sltu
            | RV32IInstruction::Sll
            | RV32IInstruction::Slt
            | RV32IInstruction::Sra
            | RV32IInstruction::Srl
            | RV32IInstruction::Sub
            | RV32IInstruction::Xor => format!(
                "{} x{},x{},x{}",
                Into::<&'static str>::into(ins),
                rd,
                rs1,
                rs2
            ),
            RV32IInstruction::Addi
            | RV32IInstruction::Andi
            | RV32IInstruction::Ori
            | RV32IInstruction::Slti
            | RV32IInstruction::Sltiu
            | RV32IInstruction::Xori
            | RV32IInstruction::Slli
            | RV32IInstruction::Srai
            | RV32IInstruction::Srli
            | RV32IInstruction::Jalr => format!(
                "{} x{},x{},{}",
                Into::<&'static str>::into(ins),
                rd,
                rs1,
                imm
            ),
            RV32IInstruction::Csrrc
            | RV32IInstruction::Csrrci
            | RV32IInstruction::Csrrs
            | RV32IInstruction::Csrrsi
            | RV32IInstruction::Csrrw
            | RV32IInstruction::Csrrwi => format!(
                "{} x{},{},x{}",
                Into::<&'static str>::into(ins),
                rd,
                imm,
                rs1
            ),
            RV32IInstruction::Lb
            | RV32IInstruction::Lbu
            | RV32IInstruction::Lh
            | RV32IInstruction::Lhu => format!(
                "{} x{},{}(x{})",
                Into::<&'static str>::into(ins),
                rd,
                imm,
                rs1
            ),
            RV32IInstruction::FenceI | RV32IInstruction::Ebreak | RV32IInstruction::Ecall => {
                format!("{}", Into::<&'static str>::into(ins))
            }
            RV32IInstruction::Sb
            | RV32IInstruction::Sh
            | RV32IInstruction::Lw
            | RV32IInstruction::Sw => format!(
                "{} x{},{}(x{})",
                Into::<&'static str>::into(ins),
                rs2,
                imm,
                rs1
            ),
            RV32IInstruction::Jal => format!("{} x{},{}", Into::<&'static str>::into(ins), rd, imm),
            RV32IInstruction::Beq
            | RV32IInstruction::Bge
            | RV32IInstruction::Bgeu
            | RV32IInstruction::Blt
            | RV32IInstruction::Bltu
            | RV32IInstruction::Bne => format!(
                "{} x{},x{},{}",
                Into::<&'static str>::into(ins),
                rs1,
                rs2,
                imm
            ),
            RV32IInstruction::Auipc | RV32IInstruction::Lui => {
                format!("{} x{},{}", Into::<&'static str>::into(ins), rd, imm)
            }
            RV32IInstruction::Fence => {
                format!("{} {},{}", Into::<&'static str>::into(ins), imm1, imm2)
            }
        },
        ParserRISCVInstOp::RV32F(..) => String::new(),
    }
}
