use crate::assembler::basic::{
    BOpcode, IOpcode, ImmediateFormatter, JOpcode, Opcode, PackedInstruction, ROpcode, SOpcode,
    UOpcode, R4Opcode
};
use crate::assembler::riscv::*;

pub struct RV32F {}

impl RV32F{
    crate::rinstimpl!(Float, fadds, 0b0000000, 0b010, rs2);
    crate::rinstimpl!(Float, fsubs, 0b0000100, 0b010, rs2);
    crate::rinstimpl!(Float, fmuls, 0b0001000, 0b010, rs2);
    crate::rinstimpl!(Float, fdivs, 0b0001100, 0b010, rs2);
    crate::rinstimpl!(Float, fsqrts, 0b0001100, 0b010, rs2);
    crate::rinstimpl!(Float, fsgnjs, 0b0010000, 0b000, rs2);
    crate::rinstimpl!(Float, fsgnjns, 0b0010000, 0b001, rs2);
    crate::rinstimpl!(Float, fsgnjxs, 0b0010000, 0b010, rs2);
    crate::rinstimpl!(Float, fmins, 0b0010100, 0b000, rs2);
    crate::rinstimpl!(Float, fmaxs, 0b0010100, 0b001, rs2);
    crate::rinstimpl!(Float, fcvtws, 0b1100000, 0b000, rs2);
    crate::rinstimpl!(Float, fcvtwus, 0b1100000, 0b000, rs2);
    crate::rinstimpl!(Float, fmvxw, 0b1110000, 0b000, rs2);
    crate::rinstimpl!(Float, feqs, 0b1010000, 0b010, rs2);
    crate::rinstimpl!(Float, flts, 0b1010000, 0b001, rs2);
    crate::rinstimpl!(Float, fles, 0b1010000, 0b000, rs2);
    crate::rinstimpl!(Float, fclasss, 0b1110000, 0b001, rs2);
    crate::rinstimpl!(Float, fcvtsw, 0b1101000, 0b000, rs2);
    crate::rinstimpl!(Float, fcvtswu, 0b1101000, 0b000, rs2);
    crate::rinstimpl!(Float, fmvwx, 0b1111000, 0b000, rs2);
    crate::r4instimpl!(FMA, fmadds, 0b000, rs2, rs3);
    crate::r4instimpl!(FMS, fmsubs, 0b000, rs2, rs3);
    crate::r4instimpl!(FNS, fnmadds, 0b000, rs2, rs3);
    crate::r4instimpl!(FNA, fnmsubs, 0b000, rs2, rs3);
    crate::iinstimpl!(Float, flw, 0b010);
    crate::sinstimpl!(Float, fsw, 0b010);
}