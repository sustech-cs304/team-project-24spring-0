use crate::interface::assembler::Operand;
use crate::modules::riscv::basic::assembler::assembler::{DATA, MAIN};
use crate::modules::riscv::basic::interface::parser::{
    ParserRISCVImmediate, ParserRISCVLabel, ParserRISCVRegister, DATA_CHUNK_RECOMMEND_SIZE, RISCV,
};
use crate::modules::riscv::rv32f::constants::RV32FRegister;
use crate::modules::riscv::rv32i::constants::RV32IRegister;

impl From<u32> for Operand<RISCV> {
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
            32 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F0)),
            33 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F1)),
            34 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F2)),
            35 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F3)),
            36 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F4)),
            37 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F5)),
            38 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F6)),
            39 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F7)),
            40 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F8)),
            41 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F9)),
            42 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F10)),
            43 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F11)),
            44 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F12)),
            45 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F13)),
            46 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F14)),
            47 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F15)),
            48 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F16)),
            49 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F17)),
            50 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F18)),
            51 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F19)),
            52 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F20)),
            53 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F21)),
            54 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F22)),
            55 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F23)),
            56 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F24)),
            57 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F25)),
            58 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F26)),
            59 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F27)),
            60 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F28)),
            61 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F29)),
            62 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F30)),
            63 => Operand::Reg(ParserRISCVRegister::from(RV32FRegister::F31)),
            _ => panic!("No such register!"),
        }
    }
}

impl From<i32> for Operand<RISCV> {
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
