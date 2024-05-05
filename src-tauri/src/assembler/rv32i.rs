use crate::assembler::basic::{
    BOpcode, IOpcode, ImmediateFormatter, JOpcode, Opcode, PackedInstruction, ROpcode, SOpcode,
    UOpcode,
};
use crate::assembler::riscv::*;

pub struct RV32I {}

impl RV32I {
    pub fn fence(
        fm: Fence,
        pred: Fence,
        succ: Fence,
        rs1: Register,
        rd: Register,
    ) -> PackedInstruction {
        let fm: u32 = fm.into();
        let pred: u32 = pred.into();
        let succ: u32 = succ.into();
        let imm: u32 = (fm << 8) | (pred << 4) | succ;
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

    pub fn fencei() -> PackedInstruction {
        (IOpcode::FENCE as u32 | 0b0000000000000000000100000000000).into()
    }

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

    pub fn ecall() -> PackedInstruction {
        (IOpcode::Environment as u32).into()
    }

    pub fn ebreak() -> PackedInstruction {
        (IOpcode::Environment as u32 | 0b00000000000100000000000000000000).into()
    }

    crate::rinstimpl!(Shamt, slli, 0b0000000, 0b001, shamt);
    crate::rinstimpl!(Shamt, srai, 0b0100000, 0b101, shamt);
    crate::rinstimpl!(Shamt, srli, 0b0000000, 0b101, shamt);
    crate::rinstimpl!(ALUReg, add, 0b0000000, 0b000, rs2);
    crate::rinstimpl!(ALUReg, and, 0b0000000, 0b111, rs2);
    crate::rinstimpl!(ALUReg, or, 0b0000000, 0b110, rs2);
    crate::rinstimpl!(ALUReg, sll, 0b0000000, 0b001, rs2);
    crate::rinstimpl!(ALUReg, slt, 0b0000000, 0b010, rs2);
    crate::rinstimpl!(ALUReg, sltu, 0b0000000, 0b011, rs2);
    crate::rinstimpl!(ALUReg, sra, 0b0100000, 0b101, rs2);
    crate::rinstimpl!(ALUReg, srl, 0b0000000, 0b101, rs2);
    crate::rinstimpl!(ALUReg, sub, 0b0100000, 0b000, rs2);
    crate::rinstimpl!(ALUReg, xor, 0b0000000, 0b100, rs2);

    crate::iinstimpl!(JALR, jalr, 0b000);
    crate::iinstimpl!(Load, lb, 0b000);
    crate::iinstimpl!(Load, lbu, 0b100);
    crate::iinstimpl!(Load, lh, 0b001);
    crate::iinstimpl!(Load, lhu, 0b101);
    crate::iinstimpl!(Load, lw, 0b010);
    crate::iinstimpl!(ALUImm, addi, 0b000);
    crate::iinstimpl!(ALUImm, andi, 0b111);
    crate::iinstimpl!(ALUImm, ori, 0b110);
    crate::iinstimpl!(ALUImm, slti, 0b010);
    crate::iinstimpl!(ALUImm, sltiu, 0b011);
    crate::iinstimpl!(ALUImm, xori, 0b100);
    crate::iinstimpl!(Environment, csrrc, 0b011);
    crate::iinstimpl!(Environment, csrrci, 0b111);
    crate::iinstimpl!(Environment, csrrs, 0b010);
    crate::iinstimpl!(Environment, csrrsi, 0b110);
    crate::iinstimpl!(Environment, csrrw, 0b001);
    crate::iinstimpl!(Environment, csrrwi, 0b101);

    crate::sinstimpl!(Store, sb, 0b000);
    crate::sinstimpl!(Store, sh, 0b001);
    crate::sinstimpl!(Store, sw, 0b010);

    crate::binstimpl!(Branch, beq, 0b000);
    crate::binstimpl!(Branch, bge, 0b101);
    crate::binstimpl!(Branch, bgeu, 0b111);
    crate::binstimpl!(Branch, blt, 0b100);
    crate::binstimpl!(Branch, bltu, 0b110);
    crate::binstimpl!(Branch, bne, 0b001);

    crate::uinstimpl!(AUIPC, auipc);
    crate::uinstimpl!(LUI, lui);
}
