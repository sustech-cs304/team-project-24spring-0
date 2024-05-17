use once_cell::sync::Lazy;

use super::super::super::basic::interface::parser::ParserRISCVInstOp;
use super::super::super::basic::parser::oplist::*;
use super::super::constants::{RV32FInstruction, RV32FRegister};
use super::lexer::RV32FOpToken;
use crate::utility::enum_map::build_map_mut_data;

pub use super::super::super::basic::parser::oplist::RISCVOpdSet;

use RV32FRegister::*;

pub static OP_LIST: Lazy<Vec<Vec<RISCVOpdSet>>> = Lazy::new(|| {
    let mut op_def = [
        (RV32FOpToken::FaddS, vec![]),
        (RV32FOpToken::FclassS, vec![]),
        (RV32FOpToken::FcvtSW, vec![]),
        (RV32FOpToken::FcvtSWu, vec![]),
        (RV32FOpToken::FcvtWS, vec![]),
        (RV32FOpToken::FcvtWuS, vec![]),
        (RV32FOpToken::FdivS, vec![]),
        (RV32FOpToken::FeqS, vec![]),
        (RV32FOpToken::FleS, vec![]),
        (RV32FOpToken::FltS, vec![]),
        (RV32FOpToken::Flw, vec![]),
        (RV32FOpToken::FmaddS, vec![]),
        (RV32FOpToken::FmaxS, vec![]),
        (RV32FOpToken::FminS, vec![]),
        (RV32FOpToken::FmsubS, vec![]),
        (RV32FOpToken::FmulS, vec![]),
        (RV32FOpToken::FmvSX, vec![]),
        (RV32FOpToken::FmvXS, vec![]),
        (RV32FOpToken::FnmaddS, vec![]),
        (RV32FOpToken::FnmsubS, vec![]),
        (RV32FOpToken::FsgnjS, vec![]),
        (RV32FOpToken::FsgnjnS, vec![]),
        (RV32FOpToken::FsgnjxS, vec![]),
        (RV32FOpToken::FsqrtS, vec![]),
        (RV32FOpToken::FsubS, vec![]),
        (RV32FOpToken::Fsw, vec![]),
        (RV32FOpToken::FabsS, vec![]),
        (RV32FOpToken::FgeS, vec![]),
        (RV32FOpToken::FgtS, vec![]),
        (RV32FOpToken::FmvS, vec![]),
        (RV32FOpToken::FmvWX, vec![]),
        (RV32FOpToken::FmvXW, vec![]),
        (RV32FOpToken::FnegS, vec![]),
        // (RV32FOpToken::FaddD, vec![]),
        // (RV32FOpToken::FclassD, vec![]),
        // (RV32FOpToken::FcvtDS, vec![]),
        // (RV32FOpToken::FcvtDW, vec![]),
        // (RV32FOpToken::FcvtDWu, vec![]),
        // (RV32FOpToken::FcvtSD, vec![]),
        // (RV32FOpToken::FcvtWD, vec![]),
        // (RV32FOpToken::FcvtWuD, vec![]),
        // (RV32FOpToken::FdivD, vec![]),
        // (RV32FOpToken::FeqD, vec![]),
        // (RV32FOpToken::Fld, vec![]),
        // (RV32FOpToken::FleD, vec![]),
        // (RV32FOpToken::FltD, vec![]),
        // (RV32FOpToken::FmaddD, vec![]),
        // (RV32FOpToken::FmaxD, vec![]),
        // (RV32FOpToken::FminD, vec![]),
        // (RV32FOpToken::FmsubD, vec![]),
        // (RV32FOpToken::FmulD, vec![]),
        // (RV32FOpToken::FnmaddD, vec![]),
        // (RV32FOpToken::FnmsubD, vec![]),
        // (RV32FOpToken::Fsd, vec![]),
        // (RV32FOpToken::FsgnjD, vec![]),
        // (RV32FOpToken::FsgnjnD, vec![]),
        // (RV32FOpToken::FsgnjxD, vec![]),
        // (RV32FOpToken::FsqrtD, vec![]),
        // (RV32FOpToken::FsubD, vec![]),
        // (RV32FOpToken::FabsD, vec![]),
        // (RV32FOpToken::FgeD, vec![]),
        // (RV32FOpToken::FgtD, vec![]),
        // (RV32FOpToken::FmvD, vec![]),
        // (RV32FOpToken::FnegD, vec![]),
        (RV32FOpToken::Frcsr, vec![]),
        (RV32FOpToken::Frflags, vec![]),
        (RV32FOpToken::Frrm, vec![]),
        (RV32FOpToken::Frsr, vec![]),
        (RV32FOpToken::Fsflags, vec![]),
        (RV32FOpToken::Fsrm, vec![]),
        (RV32FOpToken::Fsrr, vec![]),
    ];
    build_map_mut_data(&mut op_def, |def| (def.0, std::mem::take(&mut def.1)))
});
