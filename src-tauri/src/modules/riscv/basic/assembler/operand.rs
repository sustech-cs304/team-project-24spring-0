use crate::modules::riscv::basic::interface::parser::{ParserRISCVImmediate, ParserRISCVLabel};

impl From<ParserRISCVImmediate> for i32 {
    fn from(imm: ParserRISCVImmediate) -> Self {
        match imm {
            ParserRISCVImmediate::Imm(imm) => imm,
            ParserRISCVImmediate::Lbl((label, handler)) => u32::from(label) as i32,
        }
    }
}

impl From<ParserRISCVLabel> for u32 {
    fn from(label: ParserRISCVLabel) -> Self {
        match label {
            ParserRISCVLabel::Text(pos) => pos as u32 * 4,
            ParserRISCVLabel::Data(pos) => pos as u32,
            ParserRISCVLabel::Unknown(_) => 0,
        }
    }
}
