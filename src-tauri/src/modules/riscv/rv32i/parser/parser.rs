use std::{io::Write, str::FromStr};

use once_cell::sync::Lazy;
use strum::VariantArray;

#[cfg(debug_assertions)]
use super::super::super::basic::interface::parser::{export_list, export_pair};
use super::{
    super::super::{
        basic::{
            interface::parser::{ParserRISCVCsr, ParserRISCVInstOp, ParserRISCVRegister},
            parser::{
                lexer::{RISCVOpToken, Symbol},
                parser::RISCVSymbolList,
            },
        },
        rv32i::constants::{RV32ICsr, RV32IInstruction, RV32IRegister, RV32I_REGISTER_VALID_NAME},
    },
    lexer::RV32IOpToken,
};

pub static RV32I_SYMBOL_LIST: Lazy<RISCVSymbolList> = Lazy::new(|| vec![&OP_TOKEN, &REG_TOKEN]);

pub static OP_TOKEN: Lazy<Vec<(&'static str, Symbol<'static>)>> = Lazy::new(|| {
    OP_TOKEN_STASH
        .iter()
        .map(|op| (op.0.as_str(), op.1))
        .collect()
});

pub static REG_TOKEN: Lazy<Vec<(&'static str, Symbol<'static>)>> = Lazy::new(|| {
    RV32I_REGISTER_VALID_NAME
        .iter()
        .map(|reg| {
            (
                *reg,
                Symbol::Reg(RV32IRegister::from_str(reg).unwrap().into()),
            )
        })
        .collect()
});

pub static CSR_TOKEN: Lazy<Vec<(&'static str, Symbol<'static>)>> = Lazy::new(|| {
    RV32ICsr::VARIANTS
        .iter()
        .map(|&csr| (csr.into(), Symbol::Csr(csr.into())))
        .collect()
});

#[cfg(debug_assertions)]
#[allow(dead_code)]
pub fn export(folder: &str) -> std::io::Result<()> {
    let path = format!("{}/rv32i.json", folder);
    let mut file = std::fs::File::create(&path)?;
    let mut output = std::io::BufWriter::new(&mut file);
    let indent = {
        const LEVEL: usize = 5;
        let mut indent: [String; LEVEL] = Default::default();
        for i in 0..LEVEL {
            indent[i] = "    ".repeat(i);
        }
        indent
    };
    output.write(format!("{}{{\n", indent[0]).as_bytes())?;
    output.write(format!("{}\"directive\": ", indent[1]).as_bytes())?;
    export_list(
        &[
            ".align", ".ascii", ".asciz", ".byte", ".data", ".double", ".dword", ".eqv", ".extern",
            ".float", ".global", ".half", ".include", ".section", ".space", ".string", ".text",
            ".word",
        ],
        |&dir| Ok(dir),
        [&indent[1], &indent[2]],
        &mut output,
    )?;
    output.write(",\n".as_bytes())?;
    output.write(format!("{}\"register\": ", indent[1]).as_bytes())?;
    export_pair(
        &RV32I_REGISTER_VALID_NAME,
        |&name| name,
        |&name, output| {
            output.write(format!("\"{}\"", RV32IRegister::from_str(name).unwrap()).as_bytes())?;
            Ok(())
        },
        [&indent[1], &indent[2]],
        &mut output,
    )?;
    output.write(",\n".as_bytes())?;
    output.write(format!("{}\"operator\": ", indent[1]).as_bytes())?;
    export_pair(
        RV32IOpToken::VARIANTS,
        |&op| op.name(),
        |&op, output| {
            export_list(
                Into::<RISCVOpToken>::into(op).get_opd_set().as_slice(),
                |opd_set| Ok(opd_set.hint.clone()),
                [&indent[2], &indent[3]],
                output,
            )
        },
        [&indent[1], &indent[2]],
        &mut output,
    )?;
    output.write("\n".as_bytes())?;
    output.write(format!("{}}}", indent[0]).as_bytes())?;
    Ok(())
}

static OP_TOKEN_STASH: Lazy<Vec<(String, Symbol<'static>)>> = Lazy::new(|| {
    RV32IOpToken::VARIANTS
        .iter()
        .map(|&op| (op.name(), Symbol::Op(op.into())))
        .collect()
});

impl From<RV32IRegister> for ParserRISCVRegister {
    fn from(reg: RV32IRegister) -> Self {
        ParserRISCVRegister::RV32I(reg)
    }
}

impl From<RV32IInstruction> for ParserRISCVInstOp {
    fn from(inst: RV32IInstruction) -> Self {
        ParserRISCVInstOp::RV32I(inst)
    }
}

impl From<RV32ICsr> for ParserRISCVCsr {
    fn from(csr: RV32ICsr) -> Self {
        ParserRISCVCsr::RV32I(csr)
    }
}
