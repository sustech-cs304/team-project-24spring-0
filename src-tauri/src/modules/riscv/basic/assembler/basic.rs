use derive_builder::Builder;
use ux::{u1, u10, u12, u20, u3, u4, u5, u6, u7};

macro_rules! all_into_scope {
    ($self:ident, $($vars:ident)+) => {
        $(
            let $vars = $self.$vars;
        )+
    };
}

macro_rules! all_into {
    ($type:ty, $($vars:ident)+) => {
        $(
            let $vars : $type = $vars.into();
        )+
    };
}

pub trait ImmediateFormatter {
    fn immediate(&mut self, imm: u12) -> &mut Self;
}

#[derive(Default, Builder)]
#[builder(public)]
pub struct RInstruction {
    funct7: u7,
    rs2: u5,
    rs1: u5,
    funct3: u3,
    rd: u5,
    opcode: u7,
}

impl Into<PackedInstruction> for RInstruction {
    fn into(self) -> PackedInstruction {
        all_into_scope!(self, funct7 rs2 rs1 funct3 rd opcode);
        all_into! {u32, funct7 rs2 rs1 funct3 rd opcode }
        PackedInstruction(
            (funct7 << 25) + (rs2 << 20) + (rs1 << 15) + (funct3 << 12) + (rd << 7) + opcode,
        )
    }
}

#[derive(Default, Builder)]
#[builder(public)]
pub struct R4Instruction {
    rs3: u5,
    rs2: u5,
    rs1: u5,
    funct3: u3,
    rd: u5,
    opcode: u7,
}

impl Into<PackedInstruction> for R4Instruction {
    fn into(self) -> PackedInstruction {
        all_into_scope!(self, rs3 rs2 rs1 funct3 rd opcode);
        all_into! {u32, rs3 rs2 rs1 funct3 rd opcode }
        PackedInstruction(
            (rs3 << 27) + (rs2 << 20) + (rs1 << 15) + (funct3 << 12) + (rd << 7) + opcode,
        )
    }
}

#[derive(Default, Builder)]
#[builder(public)]
pub struct IInstruction {
    // I-type
    imm: u12,
    rs1: u5,
    funct3: u3,
    rd: u5,
    opcode: u7,
}

impl Into<PackedInstruction> for IInstruction {
    fn into(self) -> PackedInstruction {
        all_into_scope!(self, imm  rs1 funct3 rd opcode);
        all_into! {u32, imm  rs1 funct3 rd opcode }
        PackedInstruction((imm << 20) + (rs1 << 15) + (funct3 << 12) + (rd << 7) + opcode)
    }
}

#[derive(Default, Builder)]
#[builder(public)]
pub struct SInstruction {
    imm11_5: u7,
    rs2: u5,
    rs1: u5,
    funct3: u3,
    imm4_0: u5,
    opcode: u7,
}

impl ImmediateFormatter for SInstructionBuilder {
    fn immediate(&mut self, imm: u12) -> &mut Self {
        let imm: u32 = imm.into();
        let imm11_5: u7 = ((imm >> 5) & 0b1111111).try_into().unwrap();
        let imm4_0: u5 = (imm & 0b11111).try_into().unwrap();
        self.imm11_5(imm11_5).imm4_0(imm4_0)
    }
}

impl Into<PackedInstruction> for SInstruction {
    fn into(self) -> PackedInstruction {
        all_into_scope!(self, imm11_5 rs2 rs1 funct3 imm4_0 opcode);
        all_into! {u32, imm11_5 rs2 rs1 funct3 imm4_0 opcode}
        PackedInstruction(
            (imm11_5 << 25) + (rs2 << 20) + (rs1 << 15) + (funct3 << 12) + (imm4_0 << 7) + opcode,
        )
    }
}

#[derive(Default, Builder)]
#[builder(public)]
pub struct BInstruction {
    imm12: u1,
    imm10_5: u6,
    rs2: u5,
    rs1: u5,
    funct3: u3,
    imm4_1: u4,
    imm11: u1,
    opcode: u7,
}

impl ImmediateFormatter for BInstructionBuilder {
    fn immediate(&mut self, imm: u12) -> &mut Self {
        let imm: u32 = imm.into();
        let imm12: u1 = (imm >> 11).try_into().unwrap();
        let imm10_5: u6 = ((imm >> 5) & 0b111111).try_into().unwrap();
        let imm4_1: u4 = ((imm >> 1) & 0b1111).try_into().unwrap();
        let imm11: u1 = ((imm >> 11) & 0b1).try_into().unwrap();
        self.imm12(imm12)
            .imm10_5(imm10_5)
            .imm4_1(imm4_1)
            .imm11(imm11)
    }
}

impl Into<PackedInstruction> for BInstruction {
    fn into(self) -> PackedInstruction {
        all_into_scope!(self, imm12 imm10_5 rs2 rs1 funct3 imm4_1 imm11 opcode);
        all_into! { u32, imm12 imm10_5 rs2 rs1 funct3 imm4_1 imm11 opcode }
        PackedInstruction(
            (imm12 << 31)
                + (imm10_5 << 25)
                + (rs2 << 20)
                + (rs1 << 15)
                + (funct3 << 12)
                + (imm4_1 << 8)
                + (imm11 << 7)
                + opcode,
        )
    }
}

#[derive(Default, Builder)]
#[builder(public)]
pub struct UInstruction {
    imm: u20,
    rd: u5,
    opcode: u7,
}

impl Into<PackedInstruction> for UInstruction {
    fn into(self) -> PackedInstruction {
        all_into_scope!(self, imm rd opcode);
        all_into! { u32, imm rd opcode }
        PackedInstruction((imm << 12) + (rd << 7) + opcode)
    }
}

#[derive(Default, Builder)]
#[builder(public)]
pub struct JInstruction {
    imm20: u1,
    imm10_1: u10,
    imm11: u1,
    imm19_12: u8,
    rd: u5,
    opcode: u7,
}

impl Into<PackedInstruction> for JInstruction {
    fn into(self) -> PackedInstruction {
        all_into_scope!(self, imm20 imm10_1 imm11 imm19_12 rd opcode);
        all_into! { u32, imm20 imm10_1 imm11 imm19_12 rd opcode }
        PackedInstruction(
            (imm20 << 31) + (imm10_1 << 21) + (imm11 << 20) + (imm19_12 << 12) + (rd << 7) + opcode,
        )
    }
}

pub trait Opcode<T>: Sized {
    fn builder(self) -> T;
} // 7 bits long

#[repr(u8)]
pub enum ROpcode {
    Shamt = 0b0010011,  // Slli, Srai, Srli
    ALUReg = 0b0110011, // Add, And, Or, Sll, Slt, Sltu, Sra, Srl, Sub, Xor
    Float = 0b1010011,
}

#[repr(u8)]
pub enum R4Opcode {
    FMA = 0b1000011,
    FMS = 0b1000111,
    FNS = 0b1001011,
    FNA = 0b1001111,
}

#[repr(u8)]
pub enum IOpcode {
    JALR = 0b1100111,        //Jalr
    Load = 0b0000011,        // Lb, Lbu, Lh, Lhu, Lw
    ALUImm = 0b0010011,      // Addi, Andi, Ori, Slti, Sltiu, Xori
    FENCE = 0b0001111,       // Fence, FenceI
    Environment = 0b1110011, // Csrrc, Csrrci, Csrrs, Csrrsi, Csrrw, Csrrwi. Ebreak, Ecall
    Float = 0b0000111,
}

#[repr(u8)]
pub enum SOpcode {
    Store = 0b100011, // Sb, Sh, Sw
    Float = 0b0100111,
}

#[repr(u8)]
pub enum BOpcode {
    Branch = 0b1100011, // Beq, Bge, Bgeu, Blt, Bltu, Bne
}

#[repr(u8)]
pub enum UOpcode {
    AUIPC = 0b0010111, // Auipc
    LUI = 0b0110111,   // Lui
}

#[repr(u8)]
pub enum JOpcode {
    JAL = 0b1101111, // Jal
}

macro_rules! implopcode {
    ($builder:ident, $opcode:ident) => {
        impl Opcode<$builder> for $opcode {
            fn builder(self) -> $builder {
                $builder {
                    opcode: Some((self as u8).try_into().unwrap()),
                    ..$builder::default()
                }
            }
        }
    };
}

implopcode!(RInstructionBuilder, ROpcode);
implopcode!(R4InstructionBuilder, R4Opcode);
implopcode!(IInstructionBuilder, IOpcode);
implopcode!(SInstructionBuilder, SOpcode);
implopcode!(BInstructionBuilder, BOpcode);
implopcode!(UInstructionBuilder, UOpcode);
implopcode!(JInstructionBuilder, JOpcode);

macro_rules! implinto {
    ($tr:ident) => {
        impl Into<u32> for $tr {
            fn into(self) -> u32 {
                return self as u32;
            }
        }
    };
}
implinto!(ROpcode);
implinto!(IOpcode);
implinto!(SOpcode);
implinto!(BOpcode);
implinto!(UOpcode);
implinto!(JOpcode);

#[derive(Copy, Clone, Debug)]
pub struct PackedInstruction(u32);

impl From<PackedInstruction> for u32 {
    fn from(p: PackedInstruction) -> Self {
        p.0
    }
}

impl From<u32> for PackedInstruction {
    fn from(u: u32) -> Self {
        Self(u)
    }
}
// Refer to github.com/dotjulia/riscv_emitter, and RISC-V tutorial
