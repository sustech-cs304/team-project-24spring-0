use crate::modules::riscv::basic::interface::parser::*;

impl From<ParserRISCVRegister> for u32 {
    fn from(register: ParserRISCVRegister) -> Self {
        match register {
            ParserRISCVRegister::RV32I(rv32i_reg) => u32::from(rv32i_reg),
            ParserRISCVRegister::RV32F(rv32f_reg) => u32::from(rv32f_reg),
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

impl From<RV32FRegister> for u32 {
    fn from(register: RV32FRegister) -> Self {
        match register {
            RV32FRegister::F0 => 32,
            RV32FRegister::F1 => 33,
            RV32FRegister::F2 => 34,
            RV32FRegister::F3 => 35,
            RV32FRegister::F4 => 36,
            RV32FRegister::F5 => 37,
            RV32FRegister::F6 => 38,
            RV32FRegister::F7 => 39,
            RV32FRegister::F8 => 40,
            RV32FRegister::F9 => 41,
            RV32FRegister::F10 => 42,
            RV32FRegister::F11 => 43,
            RV32FRegister::F12 => 44,
            RV32FRegister::F13 => 45,
            RV32FRegister::F14 => 46,
            RV32FRegister::F15 => 47,
            RV32FRegister::F16 => 48,
            RV32FRegister::F17 => 49,
            RV32FRegister::F18 => 50,
            RV32FRegister::F19 => 51,
            RV32FRegister::F20 => 52,
            RV32FRegister::F21 => 53,
            RV32FRegister::F22 => 54,
            RV32FRegister::F23 => 55,
            RV32FRegister::F24 => 56,
            RV32FRegister::F25 => 57,
            RV32FRegister::F26 => 58,
            RV32FRegister::F27 => 59,
            RV32FRegister::F28 => 60,
            RV32FRegister::F29 => 61,
            RV32FRegister::F30 => 62,
            RV32FRegister::F31 => 63,
        }
    }
}
