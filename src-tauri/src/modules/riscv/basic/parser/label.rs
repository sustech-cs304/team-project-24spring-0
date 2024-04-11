use super::super::super::basic::interface::parser::*;
use crate::utility::ptr::Ptr;

#[derive(Clone)]
pub(super) struct LabelData {
    pub name: String,
    pub def: Option<Ptr<ParserResultText<RISCV>>>,
    pub refs: Vec<Ptr<ParserRISCVInstOpd>>,
}

// impl PartialEq for LabelData {
//     fn eq(&self, other: &Self) -> bool {
//         self.name == other.name
//     }
// }

// impl Eq for LabelData {}

// impl PartialOrd for LabelData {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl Ord for LabelData {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.name.cmp(&other.name)
//     }
// }
