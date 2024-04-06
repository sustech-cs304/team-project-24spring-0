use ux::{u12, u20, u3, u4, u5};

use crate::assembler::basic::{
    BOpcode, IOpcode, ImmediateFormatter, JOpcode, Opcode, PackedInstruction, ROpcode, SOpcode,
    UOpcode,
};

pub struct Register(u5);

impl Into<u5> for Register {
    fn into(self) -> u5 {
        self.0
    }
}
pub struct Immediate12(u12);
pub struct Immediate20(u20);

impl From<u32> for Immediate20 {
    fn from(i: u32) -> Self {
        Self(i.try_into().unwrap())
    }
}
impl From<u32> for Immediate12 {
    fn from(i: u32) -> Self {
        Self(i.try_into().unwrap())
    }
}
impl From<u32> for Register {
    fn from(i: u32) -> Self {
        Self(i.try_into().unwrap())
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

pub struct RV32I {}

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

// TODO: B and J Type Instructions

impl RV32I {
    uinstimpl!(LUI, lui);
    uinstimpl!(AUIPC, auipc);

    pub fn jal(imm: Immediate20, rd: Register) -> PackedInstruction {
        let imm: u32 = Into::<u32>::into(imm).into();
        JOpcode::JAL
            .builder()
            .imm20((imm >> 20).try_into().unwrap())
            .imm19_12(((imm >> 12) & 0b11111111).try_into().unwrap())
            .imm11(((imm >> 11) & 1).try_into().unwrap())
            .imm10_1(((imm >> 1) & 0b1111111111).try_into().unwrap())
            .rd(rd.into())
            .build()
            .unwrap()
            .into()
    }

    iinstimpl!(JALR, jalr, 0b000);

    binstimpl!(Branch, beq, 0b000);
    binstimpl!(Branch, bne, 0b001);
    binstimpl!(Branch, blt, 0b100);
    binstimpl!(Branch, bge, 0b101);
    binstimpl!(Branch, bltu, 0b110);
    binstimpl!(Branch, bgeu, 0b111);

    iinstimpl!(Load, lb, 0b000);
    iinstimpl!(Load, lh, 0b001);
    iinstimpl!(Load, lw, 0b010);
    iinstimpl!(Load, lbu, 0b100);
    iinstimpl!(Load, lhu, 0b101);

    sinstimpl!(Store, sb, 0b000);
    sinstimpl!(Store, sh, 0b001);
    sinstimpl!(Store, sw, 0b010);

    iinstimpl!(ALUImm, addi, 0b000);
    iinstimpl!(ALUImm, slti, 0b010);
    iinstimpl!(ALUImm, sltiu, 0b011);
    iinstimpl!(ALUImm, xori, 0b100);
    iinstimpl!(ALUImm, ori, 0b110);
    iinstimpl!(ALUImm, andi, 0b111);

    rinstimpl!(Shift, slli, 0b0000000, 0b001, shamt);
    rinstimpl!(Shift, srli, 0b0000000, 0b101, shamt);
    rinstimpl!(Shift, srai, 0b0100000, 0b101, shamt);

    rinstimpl!(ALUReg, add, 0b0000000, 0b000, rs2);
    rinstimpl!(ALUReg, sub, 0b0100000, 0b000, rs2);
    rinstimpl!(ALUReg, sll, 0b0000000, 0b001, rs2);
    rinstimpl!(ALUReg, slt, 0b0000000, 0b010, rs2);
    rinstimpl!(ALUReg, sltu, 0b0000000, 0b011, rs2);
    rinstimpl!(ALUReg, xor, 0b0000000, 0b100, rs2);
    rinstimpl!(ALUReg, srl, 0b0000000, 0b101, rs2);
    rinstimpl!(ALUReg, sra, 0b0100000, 0b101, rs2);
    rinstimpl!(ALUReg, or, 0b0000000, 0b110, rs2);
    rinstimpl!(ALUReg, and, 0b0000000, 0b111, rs2);

    pub fn fence(fm: u5, pred: u4, succ: u3, rs1: Register, rd: Register) -> PackedInstruction {
        let fm: u32 = fm.into();
        let pred: u32 = pred.into();
        let succ: u32 = succ.into();
        let imm: u32 = (fm << 7) | (pred << 3) | succ;
        IOpcode::FENCE
            .builder()
            .imm(imm.try_into().unwrap())
            .rs1(rs1.into())
            .funct3((0b000 as u32).try_into().unwrap())
            .rd(rd.into())
            .build()
            .unwrap()
            .into()
    }

    pub fn ecall() -> PackedInstruction {
        (IOpcode::Environment as u32).into()
    }

    pub fn ebreak() -> PackedInstruction {
        (IOpcode::Environment as u32 | 0b00000000000100000000000000000000).into()
    }
}

#[cfg(test)]
mod rv32i_tests {
    use crate::assembler::rv32i::RV32I;

    #[test]
    fn test_lui() {
        assert_eq!(
            0x000231b7,
            Into::<u32>::into(RV32I::lui(0x23.into(), 3.into()))
        );
    }

    #[test]
    fn test_jal() {
        assert_eq!(
            0b00101010101110101010001001101111,
            Into::<u32>::into(RV32I::jal(0xAAAAA.into(), 0x4.into()))
        );
    }

    #[test]
    fn test_beq() {
        assert_eq!(
            0b00101010001000011000010111100011,
            Into::<u32>::into(RV32I::beq(0b101010101010.into(), 0x2.into(), 0x3.into()))
        );
    }

    #[test]
    fn test_lb() {
        assert_eq!(
            0b10101010101000010000000110000011,
            Into::<u32>::into(RV32I::lb(0xAAA.into(), 0x2.into(), 0x3.into()))
        );
    }

    #[test]
    fn test_sb() {
        assert_eq!(
            0b10101010001000011000010100100011,
            Into::<u32>::into(RV32I::sb(0xAAA.into(), 0x2.into(), 0x3.into()))
        )
    }

    #[test]
    fn test_addi() {
        assert_eq!(
            0b01111010101000010000000110010011,
            Into::<u32>::into(RV32I::addi(0x7AA.into(), 0x2.into(), 0x3.into()))
        );
    }

    #[test]
    fn test_slli() {
        assert_eq!(
            0b101000010001000110010011,
            Into::<u32>::into(RV32I::slli(0xA.into(), 0x2.into(), 0x3.into()))
        );
    }
}
