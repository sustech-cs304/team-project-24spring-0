use lazy_static::lazy_static;
use strum::{EnumIter, EnumString, IntoEnumIterator};
use strum_macros::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter, EnumString, Display)]
pub enum RV32FRegister {
    #[strum(to_string = "f0", serialize = "f0", serialize = "ft0")]
    F0,
    #[strum(to_string = "f1", serialize = "f1", serialize = "ft1")]
    F1,
    #[strum(to_string = "f2", serialize = "f2", serialize = "ft2")]
    F2,
    #[strum(to_string = "f3", serialize = "f3", serialize = "ft3")]
    F3,
    #[strum(to_string = "f4", serialize = "f4", serialize = "ft4")]
    F4,
    #[strum(to_string = "f5", serialize = "f5", serialize = "ft5")]
    F5,
    #[strum(to_string = "f6", serialize = "f6", serialize = "ft6")]
    F6,
    #[strum(to_string = "f7", serialize = "f7", serialize = "ft7")]
    F7,
    #[strum(to_string = "f8", serialize = "f8", serialize = "ft8")]
    F8,
    #[strum(to_string = "f9", serialize = "f9", serialize = "ft9")]
    F9,
    #[strum(to_string = "f10", serialize = "f10", serialize = "ft10")]
    F10,
    #[strum(to_string = "f11", serialize = "f11", serialize = "ft11")]
    F11,
    #[strum(to_string = "f12", serialize = "f12", serialize = "ft12")]
    F12,
    #[strum(to_string = "f13", serialize = "f13", serialize = "ft13")]
    F13,
    #[strum(to_string = "f14", serialize = "f14", serialize = "ft14")]
    F14,
    #[strum(to_string = "f15", serialize = "f15", serialize = "ft15")]
    F15,
    #[strum(to_string = "f16", serialize = "f16", serialize = "ft16")]
    F16,
    #[strum(to_string = "f17", serialize = "f17", serialize = "ft17")]
    F17,
    #[strum(to_string = "f18", serialize = "f18", serialize = "ft18")]
    F18,
    #[strum(to_string = "f19", serialize = "f19", serialize = "ft19")]
    F19,
    #[strum(to_string = "f20", serialize = "f20", serialize = "ft20")]
    F20,
    #[strum(to_string = "f21", serialize = "f21", serialize = "ft21")]
    F21,
    #[strum(to_string = "f22", serialize = "f22", serialize = "ft22")]
    F22,
    #[strum(to_string = "f23", serialize = "f23", serialize = "ft23")]
    F23,
    #[strum(to_string = "f24", serialize = "f24", serialize = "ft24")]
    F24,
    #[strum(to_string = "f25", serialize = "f25", serialize = "ft25")]
    F25,
    #[strum(to_string = "f26", serialize = "f26", serialize = "ft26")]
    F26,
    #[strum(to_string = "f27", serialize = "f27", serialize = "ft27")]
    F27,
    #[strum(to_string = "f28", serialize = "f28", serialize = "ft28")]
    F28,
    #[strum(to_string = "f29", serialize = "f29", serialize = "ft29")]
    F29,
    #[strum(to_string = "f30", serialize = "f30", serialize = "ft30")]
    F30,
    #[strum(to_string = "f31", serialize = "f31", serialize = "ft31")]
    F31,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter, EnumString, strum_macros::IntoStaticStr,
)]
#[strum(serialize_all = "snake_case")]
pub enum RV32FInstruction {
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
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter, EnumString, strum_macros::IntoStaticStr,
)]
pub enum RV32FCsr {}

pub static RV32F_REGISTER_VALID_NAME: [&'static str; 64] = [
    "f0", "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "f10", "f11", "f12", "f13", "f14",
    "f15", "f16", "f17", "f18", "f19", "f20", "f21", "f22", "f23", "f24", "f25", "f26", "f27",
    "f28", "f29", "f30", "f31", "ft0", "ft1", "ft2", "ft3", "ft4", "ft5", "ft6", "ft7", "ft8",
    "ft9", "ft10", "ft11", "ft12", "ft13", "ft14", "ft15", "ft16", "ft17", "ft18", "ft19", "ft20",
    "ft21", "ft22", "ft23", "ft24", "ft25", "ft26", "ft27", "ft28", "ft29", "ft30", "ft31",
];

lazy_static! {
    pub static ref RV32F_REGISTER_DEFAULT_NAME: Vec<(RV32FRegister, String)> = {
        RV32FRegister::iter()
            .map(|reg| (reg, reg.to_string()))
            .collect()
    };
}

impl From<RV32FRegister> for &'static str {
    fn from(value: RV32FRegister) -> Self {
        for reg in RV32F_REGISTER_DEFAULT_NAME.iter() {
            if reg.0 == value {
                return reg.1.as_str();
            }
        }
        unreachable!();
    }
}
