use std::io::Write;

use super::super::parser::parser::RISCVSymbolList;

pub use super::super::super::rv32f::constants::*;
pub use super::super::super::rv32i::constants::*;
pub use super::super::parser::parser::RISCVParser;
pub use crate::interface::parser::*;

pub const MAX_DATA_SIZE: usize = 0xf_ffff;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RISCV;

pub enum RISCVExtension {
    RV32I,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParserRISCVInstOp {
    RV32I(RV32IInstruction),
    RV32F(RV32FInstruction),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParserRISCVRegister {
    RV32I(RV32IRegister),
    RV32F(RV32FRegister),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParserRISCVImmediate {
    Imm(RISCVImmediate),
    Lbl((ParserRISCVLabel, ParserRISCVLabelHandler)),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParserRISCVCsr {
    RV32I(RV32ICsr),
    RV32F(RV32FCsr),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParserRISCVInstOpd {
    Reg(ParserRISCVRegister),
    Imm(ParserRISCVImmediate),
    Lbl(ParserRISCVLabel),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParserRISCVLabel {
    Text(usize),  // ParserResult<RISCV>::text[usize]
    Data(usize),  // ParserResult<RISCV>::data[usize]
    Unknown(Pos), // the label position in the code (mustn't exist in the output)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParserRISCVLabelHandler {
    Low,
    High,
    DeltaHigh,
    DeltaMinusOneLow,
}

impl ParserInstSet for RISCV {
    type Operator = ParserRISCVInstOp;
    type Operand = ParserRISCVInstOpd;
}

// ------------------------- Implementations -------------------------

impl RISCVExtension {
    pub fn get_symbol_parser(&self) -> &RISCVSymbolList {
        match self {
            RISCVExtension::RV32I => &super::super::super::rv32i::parser::parser::RV32I_SYMBOL_LIST,
        }
    }

    #[cfg(debug_assertions)]
    #[allow(dead_code)]
    pub fn export(&self, folder: &str) -> std::io::Result<()> {
        match self {
            RISCVExtension::RV32I => super::super::super::rv32i::parser::parser::export(folder),
        }
    }
}

#[cfg(debug_assertions)]
#[allow(dead_code)]
pub fn export_pair<T, KFn, VFn, K, W>(
    pairs: &[T],
    key_fn: KFn,
    val_fn: VFn,
    prefix: [&str; 2],
    output: &mut std::io::BufWriter<W>,
) -> std::io::Result<()>
where
    KFn: Fn(&T) -> K,
    VFn: Fn(&T, &mut std::io::BufWriter<W>) -> std::io::Result<()>,
    K: std::fmt::Display,
    W: std::io::Write,
{
    if pairs.is_empty() {
        output.write("{}".as_bytes())?;
    } else {
        output.write("{\n".as_bytes())?;
        for data in &pairs[0..pairs.len() - 1] {
            let key = key_fn(data);
            output.write(format!("{}\"{}\": ", prefix[1], key).as_bytes())?;
            val_fn(data, output)?;
            output.write(",\n".as_bytes())?;
        }
        {
            let data = &pairs[pairs.len() - 1];
            let key = key_fn(data);
            output.write(format!("{}\"{}\": ", prefix[1], key).as_bytes())?;
            val_fn(data, output)?;
            output.write("\n".as_bytes())?;
        }
        output.write(format!("{}}}", prefix[0]).as_bytes())?;
    }
    Ok(())
}

#[cfg(debug_assertions)]
#[allow(dead_code)]
pub fn export_list<T, F, V, W>(
    list: &[T],
    val_fn: F,
    prefix: [&str; 2],
    output: &mut std::io::BufWriter<W>,
) -> std::io::Result<()>
where
    F: Fn(&T) -> std::io::Result<V>,
    V: std::fmt::Display,
    W: std::io::Write,
{
    if list.is_empty() {
        output.write("[]".as_bytes())?;
    } else {
        output.write("[\n".as_bytes())?;
        for data in &list[0..list.len() - 1] {
            let val = val_fn(data)?;
            output.write(format!("{}\"{}\",\n", prefix[1], val).as_bytes())?;
        }
        {
            let data = &list[list.len() - 1];
            let val = val_fn(data)?;
            output.write(format!("{}\"{}\"\n", prefix[1], val).as_bytes())?;
        }
        output.write(format!("{}]", prefix[0]).as_bytes())?;
    }
    Ok(())
}
