use super::super::super::basic::parser::lexer::RISCVOpToken;
use super::oplist::OP_LIST;
use strum::EnumIter;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter, strum_macros::IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum RV32FOpToken {
    FaddS,
    FclassS,
    FcvtSW,
    FcvtSWu,
    FcvtWS,
    FcvtWuS,
    FdivS,
    FeqS,
    FleS,
    FltS,
    Flw,
    FmaddS,
    FmaxS,
    FminS,
    FmsubS,
    FmulS,
    FmvSX,
    FmvXS,
    FnmaddS,
    FnmsubS,
    FsgnjS,
    FsgnjnS,
    FsgnjxS,
    FsqrtS,
    FsubS,
    Fsw,
    FabsS,
    FgeS,
    FgtS,
    FmvS,
    FmvWX,
    FmvXW,
    FnegS,
    // FaddD,
    // FclassD,
    // FcvtDS,
    // FcvtDW,
    // FcvtDWu,
    // FcvtSD,
    // FcvtWD,
    // FcvtWuD,
    // FdivD,
    // FeqD,
    // Fld,
    // FleD,
    // FltD,
    // FmaddD,
    // FmaxD,
    // FminD,
    // FmsubD,
    // FmulD,
    // FnmaddD,
    // FnmsubD,
    // Fsd,
    // FsgnjD,
    // FsgnjnD,
    // FsgnjxD,
    // FsqrtD,
    // FsubD,
    // FabsD,
    // FgeD,
    // FgtD,
    // FmvD,
    // FnegD,
    Frcsr,
    Frflags,
    Frrm,
    Frsr,
    Fsflags,
    Fsrm,
    Fsrr,
}

impl RV32FOpToken {
    pub fn name(&self) -> String {
        Into::<&'static str>::into(self).replace("_", ".")
    }
}

impl From<RV32FOpToken> for RISCVOpToken {
    fn from(op: RV32FOpToken) -> RISCVOpToken {
        unsafe {
            RISCVOpToken {
                val: *std::mem::transmute::<&_, &u8>(&op),
                get_opd_set: |v| &OP_LIST[v as usize],
            }
        }
    }
}
