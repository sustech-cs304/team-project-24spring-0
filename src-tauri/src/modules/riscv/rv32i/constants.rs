use super::super::basic::interface::parser::{ParserRISCVInstOpTrait, ParserRISCVRegisterTrait};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum RV32IRegister {
    Zero,
    Ra,
    Sp,
    Gp,
    Tp,
    T0,
    T1,
    T2,
    S0,
    S1,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    T3,
    T4,
    T5,
    T6,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum RV32IInstruction {
    Add,
    Addi,
    And,
    Andi,
    Auipc,
    Beq,
    Bge,
    Bgeu,
    Blt,
    Bltu,
    Bne,
    Csrrc,
    Csrrci,
    Csrrs,
    Csrrsi,
    Csrrw,
    Csrrwi,
    Ebreak,
    Ecall,
    Fence,
    FenceI,
    Jal,
    Jalr,
    Lb,
    Lbu,
    Lh,
    Lhu,
    Lui,
    Lw,
    Or,
    Ori,
    Sb,
    Sh,
    Sll,
    Slli,
    Slt,
    Slti,
    Sltiu,
    Sltu,
    Sra,
    Srai,
    Srl,
    Srli,
    Sub,
    Sw,
    Xor,
    Xori,
}

pub type RISCVImmediate = i32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RISCVCsr {}

pub struct ParseRV32IRegisterError;

impl FromStr for RV32IRegister {
    type Err = ParseRV32IRegisterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "zero" | "x0" => Ok(RV32IRegister::Zero),
            "ra" | "x1" => Ok(RV32IRegister::Ra),
            "sp" | "x2" => Ok(RV32IRegister::Sp),
            "gp" | "x3" => Ok(RV32IRegister::Gp),
            "tp" | "x4" => Ok(RV32IRegister::Tp),
            "t0" | "x5" => Ok(RV32IRegister::T0),
            "t1" | "x6" => Ok(RV32IRegister::T1),
            "t2" | "x7" => Ok(RV32IRegister::T2),
            "s0" | "x8" => Ok(RV32IRegister::S0),
            "s1" | "x9" => Ok(RV32IRegister::S1),
            "a0" | "x10" => Ok(RV32IRegister::A0),
            "a1" | "x11" => Ok(RV32IRegister::A1),
            "a2" | "x12" => Ok(RV32IRegister::A2),
            "a3" | "x13" => Ok(RV32IRegister::A3),
            "a4" | "x14" => Ok(RV32IRegister::A4),
            "a5" | "x15" => Ok(RV32IRegister::A5),
            "a6" | "x16" => Ok(RV32IRegister::A6),
            "a7" | "x17" => Ok(RV32IRegister::A7),
            "s2" | "x18" => Ok(RV32IRegister::S2),
            "s3" | "x19" => Ok(RV32IRegister::S3),
            "s4" | "x20" => Ok(RV32IRegister::S4),
            "s5" | "x21" => Ok(RV32IRegister::S5),
            "s6" | "x22" => Ok(RV32IRegister::S6),
            "s7" | "x23" => Ok(RV32IRegister::S7),
            "s8" | "x24" => Ok(RV32IRegister::S8),
            "s9" | "x25" => Ok(RV32IRegister::S9),
            "s10" | "x26" => Ok(RV32IRegister::S10),
            "s11" | "x27" => Ok(RV32IRegister::S11),
            "t3" | "x28" => Ok(RV32IRegister::T3),
            "t4" | "x29" => Ok(RV32IRegister::T4),
            "t5" | "x30" => Ok(RV32IRegister::T5),
            "t6" | "x31" => Ok(RV32IRegister::T6),
            _ => Err(ParseRV32IRegisterError),
        }
    }
}

impl ParserRISCVRegisterTrait for RV32IRegister {
    fn get_name(&self) -> &'static str {
        RV32I_REGISTER_NAME.get(self).unwrap()
    }
}

impl ParserRISCVInstOpTrait for RV32IInstruction {
    fn get_name(&self) -> &'static str {
        RV32I_INSTRUCTION_NAME.get(self).unwrap()
    }
}

lazy_static! {
    static ref RV32I_REGISTER_NAME: HashMap<RV32IRegister, String> =
        HashMap::from_iter(RV32IRegister::iter().map(|reg| (reg, format!("{:?}", reg))));
}

lazy_static! {
    static ref RV32I_INSTRUCTION_NAME: HashMap<RV32IInstruction, String> =
        HashMap::from_iter(RV32IInstruction::iter().map(|inst| (inst, format!("{:?}", inst))));
}
