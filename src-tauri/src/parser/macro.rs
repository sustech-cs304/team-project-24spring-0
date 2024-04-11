use super::parser::RISCVSegment;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MacroParaType {
    Register,
    Immediate,
    Label,
    None,
}

#[derive(Clone, Debug)]
pub struct MacroData {
    pub name: String,
    pub para: Vec<(String, MacroParaType)>,
    pub ret_seg: RISCVSegment,
}

impl PartialEq for MacroData {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for MacroData {}

impl PartialOrd for MacroData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MacroData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}
