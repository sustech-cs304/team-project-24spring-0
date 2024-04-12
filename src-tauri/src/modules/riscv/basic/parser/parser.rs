use super::super::interface::parser::*;
use super::label::LabelData;
use super::lexer::{LexerIter, RISCVOpToken, RISCVToken};
use super::oplist::{RISCVExpectToken, RISCVImmediateType, RISCVOpdSetAim, OP_LIST};
use super::r#macro::MacroData;
use crate::utility::ptr::Ptr;
use logos::Logos;
use std::collections::BTreeMap;

pub struct RISCVParser {
    macro_list: BTreeMap<String, MacroData>,
    label_list: BTreeMap<String, LabelData>,
    lbl_placeholder: ParserRISCVInstOpd,
}

impl Parser<ropey::Rope, RISCV> for RISCVParser {
    fn parse(&mut self, code: &ropey::Rope) -> Result<ParserResult<RISCV>, ParserError> {
        self.init();
        let code_str = code.to_string();
        let mut _status = RISCVParserStatus {
            segment: RISCVSegment::Text,
            iter: LexerIter {
                raw: RISCVToken::lexer(code_str.as_str()),
            },
            macro_def: None,
            label_def: None,
            result: ParserResult {
                data: Vec::new(),
                text: Vec::new(),
            },
        };
        let status_ptr = Ptr::new(&_status);
        let status = status_ptr.as_mut();

        while let Some(token) = status.iter.next()? {
            self.parse_token(status_ptr, token)?;
        }
        Ok(_status.result)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RISCVDataType {
    Byte,
    Half,
    Word,
    Dword,
    Float,
    Double,
    String,
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
    label_def: Option<Ptr<LabelData>>,
    result: ParserResult<RISCV>,
}

use RISCVExpectToken::*;
use RISCVImmediateType::*;

impl RISCVParser {
    pub fn new() -> Self {
        RISCVParser {
            macro_list: BTreeMap::new(),
            label_list: BTreeMap::new(),
            lbl_placeholder: ParserRISCVInstOpd::Lbl(Ptr::new(&ParserInst::<RISCV> {
                line: 0,
                op: RISCVInstruction::Add,
                opd: Vec::new(),
            })),
        }
    }

    fn init(&mut self) {
        self.macro_list = BTreeMap::new();
        self.label_list = BTreeMap::new();
        self.lbl_placeholder = ParserRISCVInstOpd::Lbl(Ptr::new(&ParserInst::<RISCV> {
            line: 0,
            op: RISCVInstruction::Add,
            opd: Vec::new(),
        }));
    }

    fn in_bound_int(token: RISCVToken, min: i64, max: i64) -> bool {
        match token {
            RISCVToken::ImmediateInt(val) => val >= min as i128 && val <= max as i128,
            _ => false,
        }
    }

    fn in_bound_unsigned(token: RISCVToken, max: u64) -> bool {
        match token {
            RISCVToken::ImmediateInt(val) => val <= max as i128,
            _ => false,
        }
    }

    fn in_bound_float(token: RISCVToken, min: f64, max: f64) -> bool {
        match token {
            RISCVToken::ImmediateFloat(val) => val >= min && val <= max,
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
        for idx in &aim_basic.opds_idx {
            inst.opd.push(stash_opd[*idx].unwrap());
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
            for (basic_opd_idx, stash_opd_idx) in aim_basic.opds_idx.iter().enumerate() {
                if let Some(ParserRISCVInstOpd::Lbl(lbl)) = stash_opd[*stash_opd_idx] {
                    label_list
                        .get_mut(&stash_label_name[*stash_opd_idx])
                        .unwrap()
                        .refs
                        .push(Ptr::new(&inst.opd[basic_opd_idx]));
                }
            }
        }
    }

    fn parse_op(
        &mut self,
        status_ptr: Ptr<RISCVParserStatus>,
        op: RISCVOpToken,
    ) -> Result<(), ParserError> {
        let status = status_ptr.as_mut();

        if status.segment != RISCVSegment::Text {
            return Err(status
                .iter
                .get_error("operator in data segment".to_string()));
        }

        let token_sets = OP_LIST.get(&op).unwrap();
        let token_set_len = token_sets.len();

        if token_set_len == 0 {
            return Ok(());
            // return Err(status.iter.get_error("operator not impl".to_string()));
        }

        let mut token_set_valid = vec![true; token_set_len];
        let mut token_idx = 0;
        let op_char_pos = status.iter.pos();
        let mut stash_opd = Vec::<Option<ParserRISCVInstOpd>>::new();
        let mut stash_label_name = Vec::<String>::new();
        let now_line = status.iter.line();

        while let Some(token) = status.iter.next()? {
            // operand
            let mut rest = token_set_len;
            let mut success_set_idx: Option<usize> = None;
            // check if the token is valid for still valid operand set
            for i in 0..token_set_len {
                if !token_set_valid[i] {
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
                        U4 => type_fit = Self::in_bound_unsigned(token, 0xf),
                        U5 => type_fit = Self::in_bound_unsigned(token, 0x1f),
                        U12 => type_fit = Self::in_bound_unsigned(token, 0xfff),
                        U20 => type_fit = Self::in_bound_unsigned(token, 0xf_ffff),
                        U32 => type_fit = Self::in_bound_unsigned(token, 0xffff_ffff),
                        U64 => type_fit = Self::in_bound_unsigned(token, u64::MAX),
                        I12 => type_fit = Self::in_bound_int(token, -0x800, 0x7ff),
                        I20 => type_fit = Self::in_bound_int(token, -0x8_0000, 0x7_ffff),
                        I32 => type_fit = Self::in_bound_int(token, -0x8000_0000, 0x7fff_ffff),
                        I64 => type_fit = Self::in_bound_int(token, i64::MIN, i64::MAX),
                    },
                    Lbl => type_fit = matches!(token, RISCVToken::Label(_)),
                }
                if type_fit {
                    // the operand set is complete
                    if token_sets[i].tokens.len() == token_idx + 1 {
                        success_set_idx = Some(i);
                        break;
                    }
                } else {
                    token_set_valid[i] = false;
                    rest -= 1;
                }
            }
            // if no valid operand set, return error
            if rest == 0 {
                let mut msg = vec!["unmatched operands.\ncandidates are:"];
                for opd_set in token_sets {
                    msg.push("\n");
                    msg.push(opd_set.hint);
                }
                return Err(ParserError {
                    pos: op_char_pos,
                    msg: msg.concat(),
                });
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
                RISCVToken::Label(lbl) => {
                    stash_opd.push(Some(self.lbl_placeholder));
                    stash_label_name.push(lbl.to_string());
                }
                _ => {
                    stash_opd.push(None);
                    stash_label_name.push(String::new());
                }
            }
            // find complete operand set
            if let Some(idx) = success_set_idx {
                let success_set = &token_sets[idx];
                // create label in label_list if not exists
                for label_name in &stash_label_name {
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
                // add first basic instruction to status.result and check if a label_def exists
                {
                    // add instruction
                    Self::load_to_result(
                        &mut status.result,
                        now_line,
                        &stash_opd,
                        &success_set.aim_basics[0],
                    );
                    // check if a label_def exists
                    if let Some(label_def) = status.label_def {
                        label_def.as_mut().def =
                            Some(Ptr::new(&status.result.text.last().unwrap()));
                        status.label_def = None;
                    }
                    // update label_list if has label
                    Self::update_label_ref(
                        &mut status.result,
                        &mut self.label_list,
                        &success_set.aim_basics[0],
                        &stash_opd,
                        &stash_label_name,
                    );
                }
                // add rest basic instruction to status.result
                for aim_basic in &success_set.aim_basics[1..] {
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
                return Ok(());
            }
            token_idx += 1;
        }

        Err(status.iter.get_error("too few operands".to_string()))
    }

    fn parse_token(
        &mut self,
        status_ptr: Ptr<RISCVParserStatus>,
        token: RISCVToken,
    ) -> Result<(), ParserError> {
        match token {
            RISCVToken::Comma => {
                // Do something when encountering a comma
            }
            RISCVToken::Newline => {
                // Do something when encountering a newline
            }
            RISCVToken::Colon => {
                // Do something when encountering a colon
            }
            RISCVToken::LParen => {
                // Do something when encountering a left parenthesis
            }
            RISCVToken::RParen => {
                // Do something when encountering a right parenthesis
            }
            RISCVToken::ImmediateInt(value) => {
                // Do something when encountering an immediate integer, access value with `value`
            }
            RISCVToken::ImmediateFloat(value) => {
                // Do something when encountering an immediate float, access value with `value`
            }
            RISCVToken::ImmediateString(value) => {
                // Do something when encountering an immediate string, access value with `value`
            }
            RISCVToken::Label(value) => {
                // Do something when encountering a label, access value with `value`
            }
            RISCVToken::MacroParameter(value) => {
                // Do something when encountering a macro parameter, access value with `value`
            }
            RISCVToken::Register(register) => {
                // Do something when encountering a register, access register type with `register`
            }
            RISCVToken::Csr(csr) => {
                // Do something when encountering a csr
            }
            RISCVToken::Align => {
                // Do something when encountering .align
            }
            RISCVToken::Ascii => {
                // Do something when encountering .ascii
            }
            RISCVToken::Asciz => {
                // Do something when encountering .asciz
            }
            RISCVToken::Byte => {
                // Do something when encountering .byte
            }
            RISCVToken::Data => {
                // Do something when encountering .data
            }
            RISCVToken::Double => {
                // Do something when encountering .double
            }
            RISCVToken::Dword => {
                // Do something when encountering .dword
            }
            RISCVToken::EndMacro => {
                // Do something when encountering .end_macro
            }
            RISCVToken::Eqv => {
                // Do something when encountering .eqv
            }
            RISCVToken::Extern => {
                // Do something when encountering .extern
            }
            RISCVToken::Float => {
                // Do something when encountering .float
            }
            RISCVToken::Global => {
                // Do something when encountering .global
            }
            RISCVToken::Half => {
                // Do something when encountering .half
            }
            RISCVToken::Include => {
                // Do something when encountering .include
            }
            RISCVToken::MacroDef => {
                // Do something when encountering a macro definition
            }
            RISCVToken::Macro => {
                // Do something when encountering .macro
            }
            RISCVToken::Section => {
                // Do something when encountering .section
            }
            RISCVToken::Space => {
                // Do something when encountering .space
            }
            RISCVToken::String => {
                // Do something when encountering .string
            }
            RISCVToken::Text => {
                // Do something when encountering .text
            }
            RISCVToken::Word => {
                // Do something when encountering .word
            }
            RISCVToken::UnknownDirective(value) => {
                // Do something when encountering an unknown directive, access value with `value`
            }
            RISCVToken::Operator(op) => {
                self.parse_op(status_ptr, op)?;
            }
        }
        Ok(())
    }
}
