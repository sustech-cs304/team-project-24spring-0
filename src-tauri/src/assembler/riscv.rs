use ux::{u1, u10, u12, u20, u3, u4, u5, u6, u7};

pub struct Register(pub u5);
pub struct Fence(u4);

impl Into<u5> for Register {
    fn into(self) -> u5 {
        self.0
    }
}
pub struct Immediate12(pub u12);
pub struct Immediate20(pub u20);

impl From<u32> for Immediate20 {
    fn from(i: u32) -> Self {
        if i > 1048575 {
            panic!("Value out of range for U12 type");
        }
        Immediate20(u20::try_from(i as u32).unwrap())
    }
}
impl From<u32> for Immediate12 {
    fn from(i: u32) -> Self {
        if i > 4095 {
            panic!("Value out of range for U12 type");
        }
        Immediate12(u12::try_from(i as u16).unwrap())
    }
}
impl From<u32> for Register {
    fn from(i: u32) -> Self {
        if i > 31 {
            panic!("Value out of range for U5 type");
        }
        Register(u5::try_from(i as u8).unwrap())
    }
}

impl From<u32> for Fence {
    fn from(i: u32) -> Self {
        if i > 15 {
            panic!("Value out of range for U5 type");
        }
        Fence(u4::try_from(i as u8).unwrap())
    }
}
impl Into<u32> for Fence {
    fn into(self) -> u32 {
        u32::from(self.0)
    }
}

impl Into<u32> for Immediate20 {
    fn into(self) -> u32 {
        self.0.into()
    }
}

impl Into<u12> for Immediate12 {
    fn into(self) -> u12 {
        self.0
    }
}

impl Into<u20> for Immediate20 {
    fn into(self) -> u20 {
        self.0
    }
}

#[macro_export]
macro_rules! rinstimpl {
    ($name:ident, $func_name:ident, $funct7:literal, $funct3:literal, $rs2name:ident) => {
        pub fn $func_name($rs2name: Register, rs1: Register, rd: Register) -> PackedInstruction {
            ROpcode::$name
                .builder()
                .funct7(($funct7 as u32).try_into().unwrap())
                .rs2($rs2name.into())
                .rs1(rs1.into())
                .funct3(($funct3 as u32).try_into().unwrap())
                .rd(rd.into())
                .build()
                .unwrap()
                .into()
        }
    };
}

#[macro_export]
macro_rules! r4instimpl {
    ($name:ident, $func_name:ident, $funct3:literal, $rs2name:ident, $rs3name:ident) => {
        pub fn $func_name($rs3name: Register, $rs2name: Register, rs1: Register, rd: Register) -> PackedInstruction {
            R4Opcode::$name
                .builder()
                .rs3($rs3name.into())
                .rs2($rs2name.into())
                .rs1(rs1.into())
                .funct3(($funct3 as u32).try_into().unwrap())
                .rd(rd.into())
                .build()
                .unwrap()
                .into()
        }
    };
}

#[macro_export]
macro_rules! iinstimpl {
    ($name:ident, $func_name:ident, $funct3:literal) => {
        pub fn $func_name(imm: Immediate12, rs1: Register, rd: Register) -> PackedInstruction {
            IOpcode::$name
                .builder()
                .imm(imm.into())
                .rs1(rs1.into())
                .rd(rd.into())
                .funct3(($funct3 as u32).try_into().unwrap())
                .build()
                .unwrap()
                .into()
        }
    };
}

#[macro_export]
macro_rules! uinstimpl {
    ($name:ident, $func_name:ident) => {
        pub fn $func_name(imm: Immediate20, rd: Register) -> PackedInstruction {
            UOpcode::$name
                .builder()
                .imm(imm.into())
                .rd(rd.into())
                .build()
                .unwrap()
                .into()
        }
    };
}

#[macro_export]
macro_rules! sinstimpl {
    ($name:ident, $func_name:ident, $funct3:literal) => {
        pub fn $func_name(imm: Immediate12, rs2: Register, rs1: Register) -> PackedInstruction {
            SOpcode::$name
                .builder()
                .immediate(imm.into())
                .rs1(rs1.into())
                .rs2(rs2.into())
                .funct3(($funct3 as u32).try_into().unwrap())
                .build()
                .unwrap()
                .into()
        }
    };
}

#[macro_export]
macro_rules! binstimpl {
    ($name:ident, $func_name:ident, $funct3:literal) => {
        pub fn $func_name(imm: Immediate12, rs2: Register, rs1: Register) -> PackedInstruction {
            BOpcode::$name
                .builder()
                .immediate(imm.into())
                .rs2(rs2.into())
                .rs1(rs1.into())
                .funct3(($funct3 as u32).try_into().unwrap())
                .build()
                .unwrap()
                .into()
        }
    };
}

pub use rinstimpl;
pub use r4instimpl;
pub use iinstimpl;
pub use uinstimpl;
pub use sinstimpl;
pub use binstimpl;
