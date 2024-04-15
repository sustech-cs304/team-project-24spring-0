use super::super::interface::parser::*;
use super::label::LabelData;
use super::lexer::{LexerIter, RISCVOpToken, RISCVToken, Symbol};
use super::oplist::{RISCVExpectToken, RISCVImmediateType, RISCVOpdSetAim, RISCVOpdSetAimOpd};
use super::r#macro::MacroData;
use crate::utility::ptr::Ptr;
use logos::Logos;
use std::collections::BTreeMap;

pub struct RISCVParser {
    macro_list: BTreeMap<String, MacroData>,
    label_list: BTreeMap<String, LabelData>,
}

impl Parser<ropey::Rope, RISCV> for RISCVParser {
    fn parse(&mut self, code: &ropey::Rope) -> Result<ParserResult<RISCV>, Vec<ParserError>> {
        self.init();
        let code_str = code.to_string();
        let mut _status = RISCVParserStatus::new(&code_str);
        let status_ptr = Ptr::new(&_status);
        let status = status_ptr.as_mut();

        while let Some(token) = status.iter.next()? {
            self.parse_token(status_ptr, token)?;
        }
        self.dispose_label_list()?;
        Ok(_status.result)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub(super) enum RISCVDataType {
    Byte,
    Half,
    Word,
    #[default]
    Dword,
    Float,
    Double,
    Ascii,
    Asciz,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub(super) enum RISCVSegment {
    Data(RISCVDataType),
    #[default]
    Text,
}

pub(super) struct RISCVParserStatus<'a> {
    segment: RISCVSegment,
    iter: LexerIter<'a>,
    macro_def: Option<MacroData>,
    label_def: Option<String>,
    data_seg_size: usize,
    result: ParserResult<RISCV>,
}

impl RISCVParserStatus<'_> {
    pub(super) fn new(code: &String) -> RISCVParserStatus {
        RISCVParserStatus {
            segment: RISCVSegment::Text,
            iter: LexerIter {
                raw: RISCVToken::lexer(code.as_str()),
            },
            macro_def: None,
            label_def: None,
            data_seg_size: 0,
            result: ParserResult {
                data: Vec::new(),
                text: Vec::new(),
            },
        }
    }
}

use RISCVExpectToken::*;
use RISCVImmediateType::*;

macro_rules! load_data_helper {
    ($label_list:expr, $status:expr, $vec:expr) => {
        if MAX_DATA_SIZE - $status.data_seg_size < $vec.len() {
            Err($status
                .iter
                .get_error("data segment size exceed max limit 0xfffff".to_string()))
        } else {
            $status.data_seg_size += $vec.len();
            let label_pos: ParserRISCVLabel;
            let now_len = $status.result.data.len();
            if let Some(ParserResultData::Data(chunk)) = $status.result.data.last_mut() {
                if chunk.len() < DATA_CHUNK_RECOMMEND_SIZE {
                    label_pos = ParserRISCVLabel::Data((now_len - 1, chunk.len()));
                    chunk.extend($vec);
                } else {
                    label_pos = ParserRISCVLabel::Data((now_len, 0));
                    $status
                        .result
                        .data
                        .push(ParserResultData::Data(Vec::from($vec)));
                }
            } else {
                label_pos = ParserRISCVLabel::Data((now_len, 0));
                $status
                    .result
                    .data
                    .push(ParserResultData::Data(Vec::from($vec)));
            }
            if let Some(label_name) = &$status.label_def {
                $label_list.get_mut(label_name).unwrap().def = Some(label_pos);
                $status.label_def = None;
            }
            Ok(())
        }
    };
}

macro_rules! load_data_helper_int {
    ($label_list:expr, $status:expr, $data:expr, $ti:ty, $tu:ty) => {
        if let RISCVToken::ImmediateInt(val) = $data {
            if *val >= <$ti>::MIN as i128 && *val <= <$tu>::MAX as i128 {
                let data = (*val as $tu).to_le_bytes();
                load_data_helper!($label_list, $status, data)
            } else {
                Err($status.iter.get_error("data out of range".to_string()))
            }
        } else {
            Err($status.iter.get_error("requires integer".to_string()))
        }
    };
}

macro_rules! load_data_helper_float {
    ($label_list:expr, $status:expr, $data:expr, $t:ty) => {
        if let RISCVToken::ImmediateFloat(val) = $data {
            let data = (*val as $t).to_le_bytes();
            load_data_helper!($label_list, $status, data)
        } else {
            Err($status.iter.get_error("requires float".to_string()))
        }
    };
}

macro_rules! load_data_helper_string {
    ($label_list:expr, $status:expr, $data:expr, $push_zero:expr) => {
        if let RISCVToken::ImmediateString(val) = $data {
            let mut data = Vec::from(val.as_bytes());
            if $push_zero {
                data.push(0);
            }
            load_data_helper!($label_list, $status, data)
        } else {
            Err($status.iter.get_error("requires string".to_string()))
        }
    };
}

impl RISCVParser {
    pub fn new() -> Self {
        RISCVParser {
            macro_list: BTreeMap::new(),
            label_list: BTreeMap::new(),
        }
    }

    fn init(&mut self) {
        self.macro_list.clear();
        self.label_list.clear();
    }

    fn in_bound_int(token: &RISCVToken, min: i128, max: i128) -> bool {
        match token {
            RISCVToken::ImmediateInt(val) => *val >= min && *val <= max,
            _ => false,
        }
    }

    fn load_to_result(
        result: &mut ParserResult<RISCV>,
        line: usize,
        stash_opd: &Vec<Option<ParserRISCVInstOpd>>,
        aim_basic: &RISCVOpdSetAim,
    ) {
        let mut inst = ParserInst::<RISCV> {
            line,
            op: aim_basic.op,
            opd: Vec::new(),
        };
        for aim_opd in &aim_basic.opds {
            match aim_opd {
                RISCVOpdSetAimOpd::Idx(idx) => {
                    inst.opd.push((idx.handler)(stash_opd[idx.idx].unwrap()))
                }
                RISCVOpdSetAimOpd::Val(val) => inst.opd.push(val.clone()),
            }
        }
        result.text.push(ParserResultText::Text(inst));
    }

    fn update_label_ref(
        result: &mut ParserResult<RISCV>,
        label_list: &mut BTreeMap<String, LabelData>,
        aim_basic: &RISCVOpdSetAim,
        stash_opd: &Vec<Option<ParserRISCVInstOpd>>,
        stash_label_name: &Vec<String>,
    ) {
        // get last added instruction
        if let ParserResultText::Text(inst) = result.text.last_mut().unwrap() {
            for (basic_opd_idx, aim_opd) in aim_basic.opds.iter().enumerate() {
                if let RISCVOpdSetAimOpd::Idx(stash_opd_idx) = aim_opd {
                    if let Some(ParserRISCVInstOpd::Lbl(_)) = stash_opd[stash_opd_idx.idx] {
                        if let ParserRISCVInstOpd::Lbl(lbl) = &inst.opd[basic_opd_idx] {
                            label_list
                                .get_mut(&stash_label_name[stash_opd_idx.idx])
                                .unwrap()
                                .refs
                                .push(Ptr::new(lbl));
                        }
                    }
                }
            }
        }
    }

    fn set_data_seg(
        status: &mut RISCVParserStatus,
        data_type: RISCVDataType,
    ) -> Result<(), Vec<ParserError>> {
        if status.segment == RISCVSegment::Text {
            return Err(status
                .iter
                .get_error("invalid directive in text segment".to_string()));
        }
        status.segment = RISCVSegment::Data(data_type);
        Ok(())
    }

    fn load_data(
        label_list: &mut BTreeMap<String, LabelData>,
        status: &mut RISCVParserStatus,
        data: &RISCVToken,
    ) -> Result<(), Vec<ParserError>> {
        match status.segment {
            RISCVSegment::Data(data_type) => match data_type {
                RISCVDataType::Byte => load_data_helper_int!(label_list, status, data, i8, u8),
                RISCVDataType::Half => load_data_helper_int!(label_list, status, data, i16, u16),
                RISCVDataType::Word => load_data_helper_int!(label_list, status, data, i32, u32),
                RISCVDataType::Dword => load_data_helper_int!(label_list, status, data, i64, u64),
                RISCVDataType::Float => load_data_helper_float!(label_list, status, data, f32),
                RISCVDataType::Double => load_data_helper_float!(label_list, status, data, f64),
                RISCVDataType::Ascii => load_data_helper_string!(label_list, status, data, false),
                RISCVDataType::Asciz => load_data_helper_string!(label_list, status, data, true),
            },
            _ => Err(status
                .iter
                .get_error("requires in data segment".to_string())),
        }
    }

    fn parse_op(
        &mut self,
        status_ptr: Ptr<RISCVParserStatus>,
        op: &dyn RISCVOpToken,
    ) -> Result<(), Vec<ParserError>> {
        let status = status_ptr.as_mut();

        if status.segment != RISCVSegment::Text {
            return Err(status
                .iter
                .get_error("operator in data segment".to_string()));
        }

        let token_sets = op.get_opd_set();
        let token_set_len = token_sets.len();

        if token_set_len == 0 {
            // return Ok(());
            return Err(status.iter.get_error("operator not impl".to_string()));
        }

        let mut token_set_state = vec![1u8; token_set_len]; // 0:failed, 1:active, 2:success
        let mut token_idx = 0;
        let op_char_pos = status.iter.pos();
        let mut stash_opd = Vec::<Option<ParserRISCVInstOpd>>::new();
        let mut stash_label_name = Vec::<String>::new();
        let now_line = status.iter.line();
        let mut term_by = 0; // 0:no next 1:newline 2:no valid set

        for token_set in token_sets {
            if token_set.tokens.is_empty() {
                token_set_state[token_idx] = 2;
            }
        }

        while let Some(token) = status.iter.next()? {
            if matches!(token, RISCVToken::Newline) {
                term_by = 1;
                break;
            }
            let mut rest = token_set_len;
            // check if the token is valid for still valid operand set
            for i in 0..token_set_len {
                token_set_state[i] &= 1;
                if token_set_state[i] == 0 {
                    rest -= 1;
                    continue;
                }
                let type_fit: bool;
                match token_sets[i].tokens[token_idx] {
                    Comma => type_fit = matches!(token, RISCVToken::Comma),
                    LParen => type_fit = matches!(token, RISCVToken::LParen),
                    RParen => type_fit = matches!(token, RISCVToken::RParen),
                    Reg => type_fit = matches!(token, RISCVToken::Register(_)),
                    Csr => type_fit = matches!(token, RISCVToken::Csr(_)),
                    Imm(imm_t) => match imm_t {
                        U4 => type_fit = Self::in_bound_int(&token, 0, 0xf),
                        U5 => type_fit = Self::in_bound_int(&token, 0, 0x1f),
                        U12 => type_fit = Self::in_bound_int(&token, 0, 0xfff),
                        U20 => type_fit = Self::in_bound_int(&token, 0, 0xf_ffff),
                        U32 => type_fit = Self::in_bound_int(&token, 0, 0xffff_ffff),
                        U64 => type_fit = Self::in_bound_int(&token, 0, u64::MAX as i128),
                        I12 => type_fit = Self::in_bound_int(&token, -0x800, 0x7ff),
                        I20 => type_fit = Self::in_bound_int(&token, -0x8_0000, 0x7_ffff),
                        I32 => type_fit = Self::in_bound_int(&token, -0x8000_0000, 0x7fff_ffff),
                        I64 => {
                            type_fit = Self::in_bound_int(&token, i64::MIN as i128, i64::MAX as i128)
                        }
                    },
                    Lbl => type_fit = matches!(token, RISCVToken::Symbol(Symbol::Label(_))),
                }
                if !type_fit {
                    token_set_state[i] = 0;
                    rest -= 1;
                } else if token_sets[i].tokens.len() == token_idx + 1 {
                    // the operand set is complete
                    token_set_state[i] = 2;
                }
            }
            // if no valid operand set, return error
            if rest == 0 {
                term_by = 2;
                break;
            }
            // stash operand
            match token {
                RISCVToken::Register(reg) => {
                    stash_opd.push(Some(ParserRISCVInstOpd::Reg(reg)));
                    stash_label_name.push(String::new());
                }
                RISCVToken::ImmediateInt(val) => {
                    stash_opd.push(Some(ParserRISCVInstOpd::Imm(RISCVImmediate::Int(val))));
                    stash_label_name.push(String::new());
                }
                RISCVToken::Symbol(Symbol::Label(lbl)) => {
                    stash_opd.push(Some(ParserRISCVInstOpd::Lbl(ParserRISCVLabel::Unknown(
                        status_ptr.as_ref().iter.pos(),
                    ))));
                    stash_label_name.push(lbl.to_string());
                }
                _ => {
                    stash_opd.push(None);
                    stash_label_name.push(String::new());
                }
            }
            token_idx += 1;
        }
        match term_by {
            1 => {
                let mut success_set_idx = None;
                // find the first success set
                for idx in 0..token_set_len {
                    if token_set_state[idx] == 2 {
                        success_set_idx = Some(idx);
                        break;
                    }
                }
                if let Some(success_set_idx) = success_set_idx {
                    let success_set = &token_sets[success_set_idx];
                    // create label in label_list if not exists
                    for label_name in &stash_label_name {
                        if label_name.is_empty() {
                            continue;
                        }
                        if !self.label_list.contains_key(label_name) {
                            self.label_list.insert(
                                label_name.clone(),
                                LabelData {
                                    name: label_name.clone(),
                                    def: None,
                                    refs: Vec::new(),
                                },
                            );
                        }
                    }
                    // check if a label_def exists
                    if let Some(label_name) = &status.label_def {
                        self.label_list.get_mut(label_name).unwrap().def =
                            Some(ParserRISCVLabel::Text(status.result.text.len()));
                        status.label_def = None;
                    }
                    // add basic instruction to status.result
                    for aim_basic in &success_set.aim_basics {
                        // add instruction
                        Self::load_to_result(&mut status.result, now_line, &stash_opd, aim_basic);
                        // update label_list if has label
                        Self::update_label_ref(
                            &mut status.result,
                            &mut self.label_list,
                            aim_basic,
                            &stash_opd,
                            &stash_label_name,
                        );
                    }
                    Ok(())
                } else {
                    let mut msg = vec!["unmatched operands.\ncandidates are:"];
                    for opd_set in token_sets {
                        msg.push("\n");
                        msg.push(&opd_set.hint);
                    }
                    Err(vec![ParserError {
                        pos: op_char_pos,
                        msg: msg.concat(),
                    }])
                }
            }
            2 => {
                let mut msg = vec!["unmatched operands.\ncandidates are:"];
                for opd_set in token_sets {
                    msg.push("\n");
                    msg.push(&opd_set.hint);
                }
                Err(vec![ParserError {
                    pos: op_char_pos,
                    msg: msg.concat(),
                }])
            }
            _ => Err(status.iter.get_error("too few operands".to_string())),
        }
    }

    fn parse_token(
        &mut self,
        status_ptr: Ptr<RISCVParserStatus>,
        token: RISCVToken,
    ) -> Result<(), Vec<ParserError>> {
        let status = status_ptr.as_mut();
        match token {
            RISCVToken::Comma | RISCVToken::Colon | RISCVToken::LParen | RISCVToken::RParen => {
                Err(status.iter.get_error("unexpected character".to_string()))
            }
            RISCVToken::Newline => Ok(()),
            RISCVToken::ImmediateInt(_)
            | RISCVToken::ImmediateFloat(_)
            | RISCVToken::ImmediateString(_) => {
                Self::load_data(&mut self.label_list, status, &token)
            }
            RISCVToken::Symbol(symbol) => match symbol {
                Symbol::Label(value) => {
                    let pos = status.iter.pos();
                    let next_token = status.iter.next()?;
                    if status.label_def.is_some()
                        || next_token.is_none()
                        || !matches!(next_token.unwrap(), RISCVToken::Colon)
                    {
                        return Err(vec![ParserError {
                            pos,
                            msg: "unrecognized symbol".to_string(),
                        }]);
                    }
                    let label_name = value.to_string();
                    if self.label_list.get(&label_name).is_none() {
                        self.label_list.insert(
                            label_name.clone(),
                            LabelData {
                                name: label_name.clone(),
                                def: None,
                                refs: Vec::new(),
                            },
                        );
                    }
                    status.label_def = Some(label_name);
                    Ok(())
                }
                Symbol::Op(op) => self.parse_op(status_ptr, op),
            },
            RISCVToken::MacroParameter(_) | RISCVToken::Register(_) | RISCVToken::Csr(_) => {
                Err(status.iter.get_error("unexpected symbol".to_string()))
            }
            RISCVToken::Align => {
                let next_token = status.iter.next()?;
                match next_token {
                    Some(RISCVToken::ImmediateInt(val)) => {
                        if val >= 0 && val <= 3 {
                            if status.segment == RISCVSegment::Text {
                                status.result.text.push(ParserResultText::Align(val as u8));
                            } else {
                                status.result.data.push(ParserResultData::Align(val as u8));
                            }
                            Ok(())
                        } else {
                            Err(status.iter.get_error(
                                ".align requires 0(byte), 1(half), 2(word), or 3(double)"
                                    .to_string(),
                            ))
                        }
                    }
                    _ => Err(status.iter.get_error(
                        ".align requires 0(byte), 1(half), 2(word), or 3(double)".to_string(),
                    )),
                }
            }
            RISCVToken::Ascii => Self::set_data_seg(status, RISCVDataType::Ascii),
            RISCVToken::Asciz => Self::set_data_seg(status, RISCVDataType::Asciz),
            RISCVToken::Byte => Self::set_data_seg(status, RISCVDataType::Byte),
            RISCVToken::Data => {
                status.segment = RISCVSegment::Data(RISCVDataType::default());
                Ok(())
            }
            RISCVToken::Double => Self::set_data_seg(status, RISCVDataType::Double),
            RISCVToken::Dword => Self::set_data_seg(status, RISCVDataType::Dword),
            RISCVToken::EndMacro => Ok(()),
            RISCVToken::Eqv => Err(status
                .iter
                .get_error("not implemented directive".to_string())),
            RISCVToken::Extern => Err(status
                .iter
                .get_error("not implemented directive".to_string())),
            RISCVToken::Float => Self::set_data_seg(status, RISCVDataType::Float),
            RISCVToken::Global => Err(status
                .iter
                .get_error("not implemented directive".to_string())),
            RISCVToken::Half => Self::set_data_seg(status, RISCVDataType::Half),
            RISCVToken::Include => Err(status
                .iter
                .get_error("not implemented directive".to_string())),
            RISCVToken::MacroDef => Ok(()),
            RISCVToken::Macro => Err(status.iter.get_error("missing macro name".to_string())),
            RISCVToken::Section => {
                let next_token = status.iter.next()?;
                match next_token {
                    Some(RISCVToken::Symbol(Symbol::Label(_)))
                    | Some(RISCVToken::ImmediateString(_)) => Ok(()),
                    Some(_) => Err(status.iter.get_error("invalid section name".to_string())),
                    None => Err(status.iter.get_error("missing section name".to_string())),
                }
            }
            RISCVToken::Space => {
                if status.segment == RISCVSegment::Text {
                    return Err(status
                        .iter
                        .get_error("invalid directive in text segment".to_string()));
                }
                if let Some(RISCVToken::ImmediateInt(val)) = status.iter.next()? {
                    if val < 0 {
                        return Err(status
                            .iter
                            .get_error(".space requires a non-negative integer".to_string()));
                    } else if ((MAX_DATA_SIZE - status.data_seg_size) as i128) < val {
                        return Err(status
                            .iter
                            .get_error("data segment size exceed max limit 0xfffff".to_string()));
                    } else {
                        status
                            .result
                            .data
                            .push(ParserResultData::Data(vec![0; val as usize]));
                        status.data_seg_size += val as usize;
                        return Ok(());
                    }
                } else {
                    Err(status
                        .iter
                        .get_error(".space requires a non-negative integer".to_string()))
                }
            }
            RISCVToken::String => Self::set_data_seg(status, RISCVDataType::Asciz),
            RISCVToken::Text => {
                status.segment = RISCVSegment::Text;
                Ok(())
            }
            RISCVToken::Word => Self::set_data_seg(status, RISCVDataType::Word),
            RISCVToken::UnknownDirective(_) => {
                Err(status.iter.get_error("unrecognized directive".to_string()))
            }
        }
    }

    fn dispose_label_list(&self) -> Result<(), Vec<ParserError>> {
        for label in self.label_list.values() {
            if let Some(def) = label.def {
                for ref_ptr in &label.refs {
                    *ref_ptr.as_mut() = def;
                }
            } else {
                let mut errors = Vec::<ParserError>::new();
                for ref_ptr in &label.refs {
                    if let ParserRISCVLabel::Unknown(pos) = ref_ptr.as_ref() {
                        errors.push(ParserError {
                            pos: *pos,
                            msg: format!("label {} not found", label.name),
                        });
                    }
                }
                return Err(errors);
            }
        }
        Ok(())
    }
}
