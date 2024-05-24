use crate::{
    interface::assembler::Operand,
    modules::riscv::{
        basic::interface::parser::{ParserRISCVRegister, RISCV},
        rv32i::constants::RV32IRegister,
    },
};

impl From<Operand<RISCV>> for u32 {
    fn from(opd: Operand<RISCV>) -> Self {
        match opd {
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::Zero)) => 0,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::Ra)) => 1,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::Sp)) => 2,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::Gp)) => 3,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::Tp)) => 4,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::T0)) => 5,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::T1)) => 6,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::T2)) => 7,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::S0)) => 8,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::S1)) => 9,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::A0)) => 10,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::A1)) => 11,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::A2)) => 12,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::A3)) => 13,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::A4)) => 14,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::A5)) => 15,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::A6)) => 16,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::A7)) => 17,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::S2)) => 18,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::S3)) => 19,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::S4)) => 20,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::S5)) => 21,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::S6)) => 22,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::S7)) => 23,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::S8)) => 24,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::S9)) => 25,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::S10)) => 26,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::S11)) => 27,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::T3)) => 28,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::T4)) => 29,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::T5)) => 30,
            Operand::Reg(ParserRISCVRegister::RV32I(RV32IRegister::T6)) => 31,
            _ => panic!("Operator not supported"),
        }
    }
}
