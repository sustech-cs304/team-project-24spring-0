use super::super::basic::interface::parser::{ParserRISCVInstOpTrait, ParserRISCVRegisterTrait};
use lazy_static::lazy_static;
use strum::{EnumIter, EnumString, IntoEnumIterator};
use strum_macros::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter, EnumString, Display)]
pub enum RV32IRegister {
    #[strum(to_string = "zero", serialize = "zero", serialize = "x0")]
    Zero,
    #[strum(to_string = "ra", serialize = "ra", serialize = "x1")]
    Ra,
    #[strum(to_string = "sp", serialize = "sp", serialize = "x2")]
    Sp,
    #[strum(to_string = "gp", serialize = "gp", serialize = "x3")]
    Gp,
    #[strum(to_string = "tp", serialize = "tp", serialize = "x4")]
    Tp,
    #[strum(to_string = "t0", serialize = "t0", serialize = "x5")]
    T0,
    #[strum(to_string = "t1", serialize = "t1", serialize = "x6")]
    T1,
    #[strum(to_string = "t2", serialize = "t2", serialize = "x7")]
    T2,
    #[strum(to_string = "s0", serialize = "s0", serialize = "fp", serialize = "x8")]
    S0,
    #[strum(to_string = "s1", serialize = "s1", serialize = "x9")]
    S1,
    #[strum(to_string = "a0", serialize = "a0", serialize = "x10")]
    A0,
    #[strum(to_string = "a1", serialize = "a1", serialize = "x11")]
    A1,
    #[strum(to_string = "a2", serialize = "a2", serialize = "x12")]
    A2,
    #[strum(to_string = "a3", serialize = "a3", serialize = "x13")]
    A3,
    #[strum(to_string = "a4", serialize = "a4", serialize = "x14")]
    A4,
    #[strum(to_string = "a5", serialize = "a5", serialize = "x15")]
    A5,
    #[strum(to_string = "a6", serialize = "a6", serialize = "x16")]
    A6,
    #[strum(to_string = "a7", serialize = "a7", serialize = "x17")]
    A7,
    #[strum(to_string = "s2", serialize = "s2", serialize = "x18")]
    S2,
    #[strum(to_string = "s3", serialize = "s3", serialize = "x19")]
    S3,
    #[strum(to_string = "s4", serialize = "s4", serialize = "x20")]
    S4,
    #[strum(to_string = "s5", serialize = "s5", serialize = "x21")]
    S5,
    #[strum(to_string = "s6", serialize = "s6", serialize = "x22")]
    S6,
    #[strum(to_string = "s7", serialize = "s7", serialize = "x23")]
    S7,
    #[strum(to_string = "s8", serialize = "s8", serialize = "x24")]
    S8,
    #[strum(to_string = "s9", serialize = "s9", serialize = "x25")]
    S9,
    #[strum(to_string = "s10", serialize = "s10", serialize = "x26")]
    S10,
    #[strum(to_string = "s11", serialize = "s11", serialize = "x27")]
    S11,
    #[strum(to_string = "t3", serialize = "t3", serialize = "x28")]
    T3,
    #[strum(to_string = "t4", serialize = "t4", serialize = "x29")]
    T4,
    #[strum(to_string = "t5", serialize = "t5", serialize = "x30")]
    T5,
    #[strum(to_string = "t6", serialize = "t6", serialize = "x31")]
    T6,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter, EnumString, strum_macros::IntoStaticStr,
)]
#[strum(serialize_all = "snake_case")]
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

pub static RV32I_REGISTER_VALID_NAME: [&'static str; 65] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6", "fp", "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11",
    "x12", "x13", "x14", "x15", "x16", "x17", "x18", "x19", "x20", "x21", "x22", "x23", "x24",
    "x25", "x26", "x27", "x28", "x29", "x30", "x31",
];

lazy_static! {
    pub static ref RV32I_REGISTER_DEFAULT_NAME: Vec<(RV32IRegister, String)> = {
        RV32IRegister::iter()
            .map(|reg| (reg, reg.to_string()))
            .collect()
    };
}

impl From<RV32IRegister> for &'static str {
    fn from(value: RV32IRegister) -> Self {
        for reg in RV32I_REGISTER_DEFAULT_NAME.iter() {
            if reg.0 == value {
                return reg.1.as_str();
            }
        }
        unreachable!();
    }
}

impl ParserRISCVRegisterTrait for RV32IRegister {}

impl ParserRISCVInstOpTrait for RV32IInstruction {}
