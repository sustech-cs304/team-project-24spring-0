use super::super::interface::parser::*;
use super::label::LabelData;
use super::lexer::{LexerIter, RISCVToken};
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RISCVImmediateType {
    U12,
    U20,
    U32,
    U64,
    I12,
    I20,
    I32,
    I64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RISCVExpectToken {
    Comma,
    LParen,
    RParen,
    Reg,
    Imm(RISCVImmediateType),
    Lbl,
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

    fn parse_op(
        &mut self,
        status_ptr: Ptr<RISCVParserStatus>,
        op: ParserRISCVInstOp,
        // (description, token set)
        token_sets: Vec<(&str, Vec<RISCVExpectToken>)>,
    ) -> Result<(), ParserError> {
        let status = status_ptr.as_mut();

        if status.segment != RISCVSegment::Text {
            return Err(status
                .iter
                .get_error("operator in data segment".to_string()));
        }

        let token_set_len = token_sets.len();
        let mut token_set_valid = vec![true; token_sets.len()];
        let mut token_idx = 0;
        let op_char_pos = status.iter.pos();
        let mut result = ParserInst::<RISCV> {
            line: status.iter.line(),
            op,
            opd: Vec::new(),
        };

        while let Some(token) = status.iter.next()? {
            // operand
            let mut rest = token_set_len;
            let mut success = false;
            // check if the token is valid for still valid operand set
            for i in 0..token_set_len {
                if !token_set_valid[i] {
                    rest -= 1;
                    continue;
                }
                let type_fit: bool;
                match token_sets[i].1[token_idx] {
                    Comma => type_fit = matches!(token, RISCVToken::Comma),
                    LParen => type_fit = matches!(token, RISCVToken::LParen),
                    RParen => type_fit = matches!(token, RISCVToken::RParen),
                    Reg => type_fit = matches!(token, RISCVToken::Register(_)),
                    Imm(imm_t) => match imm_t {
                        U12 => type_fit = RISCVParser::in_bound_unsigned(token, 0xfff),
                        U20 => type_fit = RISCVParser::in_bound_unsigned(token, 0xf_ffff),
                        U32 => type_fit = RISCVParser::in_bound_unsigned(token, 0xffff_ffff),
                        U64 => type_fit = RISCVParser::in_bound_unsigned(token, u64::MAX),
                        I12 => type_fit = RISCVParser::in_bound_int(token, -0x800, 0x7ff),
                        I20 => type_fit = RISCVParser::in_bound_int(token, -0x8_0000, 0x7_ffff),
                        I32 => {
                            type_fit = RISCVParser::in_bound_int(token, -0x8000_0000, 0x7fff_ffff)
                        }
                        I64 => type_fit = RISCVParser::in_bound_int(token, i64::MIN, i64::MAX),
                    },
                    Lbl => type_fit = matches!(token, RISCVToken::Label(_)),
                }
                if type_fit {
                    // the operand set is complete
                    if token_sets[i].1.len() == token_idx + 1 {
                        success = true;
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
                for opd_set in &token_sets {
                    msg.push("\n");
                    msg.push(opd_set.0);
                }
                return Err(ParserError {
                    pos: op_char_pos,
                    msg: msg.concat(),
                });
            }
            // add to result
            match token {
                RISCVToken::Register(reg) => {
                    result.opd.push(ParserRISCVInstOpd::Reg(reg));
                }
                RISCVToken::ImmediateInt(val) => {
                    result
                        .opd
                        .push(ParserRISCVInstOpd::Imm(RISCVImmediate::Int(val)));
                }
                RISCVToken::Label(lbl) => {
                    result.opd.push(self.lbl_placeholder);
                    let ref_ptr = Ptr::new(result.opd.last().unwrap());
                    let label_name = lbl.to_string();
                    if let Some(label) = self.label_list.get_mut(&label_name) {
                        label.refs.push(ref_ptr);
                    } else {
                        self.label_list.insert(
                            label_name.clone(),
                            LabelData {
                                name: label_name,
                                def: None,
                                refs: vec![ref_ptr],
                            },
                        );
                    }
                }
                RISCVToken::MacroParameter(par) => {
                    // TODO: implement macro parameter
                }
                _ => {}
            }
            // find complete operand set
            if success {
                status.result.text.push(ParserResultText::Text(result));
                if let Some(label_def) = status.label_def {
                    label_def.as_mut().def = Some(Ptr::new(&status.result.text.last().unwrap()));
                    status.label_def = None;
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
            RISCVToken::Add => self.parse_op(
                status_ptr,
                RISCVInstruction::Add,
                vec![(
                    "add t1, t2, t3 (t1 = t2 + t3)",
                    vec![Reg, Comma, Reg, Comma, Reg],
                )],
            )?,
            RISCVToken::Addi => self.parse_op(
                status_ptr,
                RISCVInstruction::Addi,
                vec![
                    (
                        "addi t1, t2, -100 (t1 = t2 - 100(i12))",
                        vec![Reg, Comma, Reg, Comma, Imm(I12)],
                    ),
                    (
                        "addi t1, t2, label (t1 = t2 - label(i12))",
                        vec![Reg, Comma, Reg, Comma, Lbl],
                    ),
                ],
            )?,
            RISCVToken::And => self.parse_op(
                status_ptr,
                RISCVInstruction::And,
                vec![(
                    "and t1, t2, t3 (t1 = t2 & t3)",
                    vec![Reg, Comma, Reg, Comma, Reg],
                )],
            )?,
            RISCVToken::Andi => {
                // Do something when encountering andi
            }
            RISCVToken::Auipc => {
                // Do something when encountering auipc
            }
            RISCVToken::Beq => {
                // Do something when encountering beq
            }
            RISCVToken::Bge => {
                // Do something when encountering bge
            }
            RISCVToken::Bgeu => {
                // Do something when encountering bgeu
            }
            RISCVToken::Blt => {
                // Do something when encountering blt
            }
            RISCVToken::Bltu => {
                // Do something when encountering bltu
            }
            RISCVToken::Bne => {
                // Do something when encountering bne
            }
            RISCVToken::Csrrc => {
                // Do something when encountering csrrc
            }
            RISCVToken::Csrrci => {
                // Do something when encountering csrrci
            }
            RISCVToken::Csrrs => {
                // Do something when encountering csrrs
            }
            RISCVToken::Csrrsi => {
                // Do something when encountering csrrsi
            }
            RISCVToken::Csrrw => {
                // Do something when encountering csrrw
            }
            RISCVToken::Csrrwi => {
                // Do something when encountering csrrwi
            }
            RISCVToken::Div => {
                // Do something when encountering div
            }
            RISCVToken::Divu => {
                // Do something when encountering divu
            }
            RISCVToken::Ebreak => {
                // Do something when encountering ebreak
            }
            RISCVToken::Ecall => {
                // Do something when encountering ecall
            }
            RISCVToken::FaddD => {
                // Do something when encountering fadd.d
            }
            RISCVToken::FaddS => {
                // Do something when encountering fadd.s
            }
            RISCVToken::FclassD => {
                // Do something when encountering fclass.d
            }
            RISCVToken::FclassS => {
                // Do something when encountering fclass.s
            }
            RISCVToken::FcvtDS => {
                // Do something when encountering fcvt.d.s
            }
            RISCVToken::FcvtDW => {
                // Do something when encountering fcvt.d.w
            }
            RISCVToken::FcvtDWu => {
                // Do something when encountering fcvt.d.wu
            }
            RISCVToken::FcvtSD => {
                // Do something when encountering fcvt.s.d
            }
            RISCVToken::FcvtSW => {
                // Do something when encountering fcvt.s.w
            }
            RISCVToken::FcvtSWu => {
                // Do something when encountering fcvt.s.wu
            }
            RISCVToken::FcvtWD => {
                // Do something when encountering fcvt.w.d
            }
            RISCVToken::FcvtWS => {
                // Do something when encountering fcvt.w.s
            }
            RISCVToken::FcvtWuD => {
                // Do something when encountering fcvt.wu.d
            }
            RISCVToken::FcvtWuS => {
                // Do something when encountering fcvt.wu.s
            }
            RISCVToken::FdivD => {
                // Do something when encountering fdiv.d
            }
            RISCVToken::FdivS => {
                // Do something when encountering fdiv.s
            }
            RISCVToken::Fence => {
                // Do something when encountering fence
            }
            RISCVToken::FenceI => {
                // Do something when encountering fence.i
            }
            RISCVToken::FeqD => {
                // Do something when encountering feq.d
            }
            RISCVToken::FeqS => {
                // Do something when encountering feq.s
            }
            RISCVToken::Fld => {
                // Do something when encountering fld
            }
            RISCVToken::FleD => {
                // Do something when encountering fle.d
            }
            RISCVToken::FleS => {
                // Do something when encountering fle.s
            }
            RISCVToken::FltD => {
                // Do something when encountering flt.d
            }
            RISCVToken::FltS => {
                // Do something when encountering flt.s
            }
            RISCVToken::Flw => {
                // Do something when encountering flw
            }
            RISCVToken::FmaddD => {
                // Do something when encountering fmadd.d
            }
            RISCVToken::FmaddS => {
                // Do something when encountering fmadd.s
            }
            RISCVToken::FmaxD => {
                // Do something when encountering fmax.d
            }
            RISCVToken::FmaxS => {
                // Do something when encountering fmax.s
            }
            RISCVToken::FminD => {
                // Do something when encountering fmin.d
            }
            RISCVToken::FminS => {
                // Do something when encountering fmin.s
            }
            RISCVToken::FmsubD => {
                // Do something when encountering fmsub.d
            }
            RISCVToken::FmsubS => {
                // Do something when encountering fmsub.s
            }
            RISCVToken::FmulD => {
                // Do something when encountering fmul.d
            }
            RISCVToken::FmulS => {
                // Do something when encountering fmul.s
            }
            RISCVToken::FmvSX => {
                // Do something when encountering fmv.s.x
            }
            RISCVToken::FmvXS => {
                // Do something when encountering fmv.x.s
            }
            RISCVToken::FnmaddD => {
                // Do something when encountering fnmadd.d
            }
            RISCVToken::FnmaddS => {
                // Do something when encountering fnmadd.s
            }
            RISCVToken::FnmsubD => {
                // Do something when encountering fnmsub.d
            }
            RISCVToken::FnmsubS => {
                // Do something when encountering fnmsub.s
            }
            RISCVToken::Fsd => {
                // Do something when encountering fsd
            }
            RISCVToken::FsgnjD => {
                // Do something when encountering fsgnj.d
            }
            RISCVToken::FsgnjS => {
                // Do something when encountering fsgnj.s
            }
            RISCVToken::FsgnjnD => {
                // Do something when encountering fsgnjn.d
            }
            RISCVToken::FsgnjnS => {
                // Do something when encountering fsgnjn.s
            }
            RISCVToken::FsgnjxD => {
                // Do something when encountering fsgnjx.d
            }
            RISCVToken::FsgnjxS => {
                // Do something when encountering fsgnjx.s
            }
            RISCVToken::FsqrtD => {
                // Do something when encountering fsqrt.d
            }
            RISCVToken::FsqrtS => {
                // Do something when encountering fsqrt.s
            }
            RISCVToken::FsubD => {
                // Do something when encountering fsub.d
            }
            RISCVToken::FsubS => {
                // Do something when encountering fsub.s
            }
            RISCVToken::Fsw => {
                // Do something when encountering fsw
            }
            RISCVToken::Jal => {
                // Do something when encountering jal
            }
            RISCVToken::Jalr => {
                // Do something when encountering jalr
            }
            RISCVToken::Lb => {
                // Do something when encountering lb
            }
            RISCVToken::Lbu => {
                // Do something when encountering lbu
            }
            RISCVToken::Lh => {
                // Do something when encountering lh
            }
            RISCVToken::Lhu => {
                // Do something when encountering lhu
            }
            RISCVToken::Lui => {
                // Do something when encountering lui
            }
            RISCVToken::Lw => {
                // Do something when encountering lw
            }
            RISCVToken::Mul => {
                // Do something when encountering mul
            }
            RISCVToken::Mulh => {
                // Do something when encountering mulh
            }
            RISCVToken::Mulhsu => {
                // Do something when encountering mulhsu
            }
            RISCVToken::Mulhu => {
                // Do something when encountering mulhu
            }
            RISCVToken::Or => {
                // Do something when encountering or
            }
            RISCVToken::Ori => {
                // Do something when encountering ori
            }
            RISCVToken::Rem => {
                // Do something when encountering rem
            }
            RISCVToken::Remu => {
                // Do something when encountering remu
            }
            RISCVToken::Sb => {
                // Do something when encountering sb
            }
            RISCVToken::Sh => {
                // Do something when encountering sh
            }
            RISCVToken::Sll => {
                // Do something when encountering sll
            }
            RISCVToken::Slli => {
                // Do something when encountering slli
            }
            RISCVToken::Slt => {
                // Do something when encountering slt
            }
            RISCVToken::Slti => {
                // Do something when encountering slti
            }
            RISCVToken::Sltiu => {
                // Do something when encountering sltiu
            }
            RISCVToken::Sltu => {
                // Do something when encountering sltu
            }
            RISCVToken::Sra => {
                // Do something when encountering sra
            }
            RISCVToken::Srai => {
                // Do something when encountering srai
            }
            RISCVToken::Srl => {
                // Do something when encountering srl
            }
            RISCVToken::Srli => {
                // Do something when encountering srli
            }
            RISCVToken::Sub => {
                // Do something when encountering sub
            }
            RISCVToken::Sw => {
                // Do something when encountering sw
            }
            RISCVToken::Uret => {
                // Do something when encountering uret
            }
            RISCVToken::Wfi => {
                // Do something when encountering wfi
            }
            RISCVToken::Xor => {
                // Do something when encountering xor
            }
            RISCVToken::Xori => {
                // Do something when encountering xori
            }
            RISCVToken::B => {
                // Do something when encountering b
            }
            RISCVToken::Beqz => {
                // Do something when encountering beqz
            }
            RISCVToken::Bgez => {
                // Do something when encountering bgez
            }
            RISCVToken::Bgt => {
                // Do something when encountering bgt
            }
            RISCVToken::Bgtu => {
                // Do something when encountering bgtu
            }
            RISCVToken::Bgtz => {
                // Do something when encountering bgtz
            }
            RISCVToken::Ble => {
                // Do something when encountering ble
            }
            RISCVToken::Bleu => {
                // Do something when encountering bleu
            }
            RISCVToken::Blez => {
                // Do something when encountering blez
            }
            RISCVToken::Bltz => {
                // Do something when encountering bltz
            }
            RISCVToken::Bnez => {
                // Do something when encountering bnez
            }
            RISCVToken::Call => {
                // Do something when encountering call
            }
            RISCVToken::Csrc => {
                // Do something when encountering csrc
            }
            RISCVToken::Csrci => {
                // Do something when encountering csrci
            }
            RISCVToken::Csrr => {
                // Do something when encountering csrr
            }
            RISCVToken::Csrs => {
                // Do something when encountering csrs
            }
            RISCVToken::Csrsi => {
                // Do something when encountering csrsi
            }
            RISCVToken::Csrw => {
                // Do something when encountering csrw
            }
            RISCVToken::Csrwi => {
                // Do something when encountering csrwi
            }
            RISCVToken::FabsD => {
                // Do something when encountering fabs.d
            }
            RISCVToken::FabsS => {
                // Do something when encountering fabs.s
            }
            RISCVToken::FgeD => {
                // Do something when encountering fge.d
            }
            RISCVToken::FgeS => {
                // Do something when encountering fge.s
            }
            RISCVToken::FgtD => {
                // Do something when encountering fgt.d
            }
            RISCVToken::FgtS => {
                // Do something when encountering fgt.s
            }
            RISCVToken::FmvD => {
                // Do something when encountering fmv.d
            }
            RISCVToken::FmvS => {
                // Do something when encountering fmv.s
            }
            RISCVToken::FmvWX => {
                // Do something when encountering fmv.w.x
            }
            RISCVToken::FmvXW => {
                // Do something when encountering fmv.x.w
            }
            RISCVToken::FnegD => {
                // Do something when encountering fneg.d
            }
            RISCVToken::FnegS => {
                // Do something when encountering fneg.s
            }
            RISCVToken::Frcsr => {
                // Do something when encountering frcsr
            }
            RISCVToken::Frflags => {
                // Do something when encountering frflags
            }
            RISCVToken::Frrm => {
                // Do something when encountering frrm
            }
            RISCVToken::Frsr => {
                // Do something when encountering frsr
            }
            RISCVToken::Fsflags => {
                // Do something when encountering fsflags
            }
            RISCVToken::Fsrm => {
                // Do something when encountering fsrm
            }
            RISCVToken::Fsrr => {
                // Do something when encountering fsrr
            }
            RISCVToken::J => {
                // Do something when encountering j
            }
            RISCVToken::Jr => {
                // Do something when encountering jr
            }
            RISCVToken::La => {
                // Do something when encountering la
            }
            RISCVToken::Li => {
                // Do something when encountering li
            }
            RISCVToken::Mv => {
                // Do something when encountering mv
            }
            RISCVToken::Neg => {
                // Do something when encountering neg
            }
            RISCVToken::Nop => {
                // Do something when encountering nop
            }
            RISCVToken::Not => {
                // Do something when encountering not
            }
            RISCVToken::Rdcycle => {
                // Do something when encountering rdcycle
            }
            RISCVToken::Rdcycleh => {
                // Do something when encountering rdcycleh
            }
            RISCVToken::Rdinstret => {
                // Do something when encountering rdinstret
            }
            RISCVToken::Rdinstreth => {
                // Do something when encountering rdinstreth
            }
            RISCVToken::Rdtime => {
                // Do something when encountering rdtime
            }
            RISCVToken::Rdtimeh => {
                // Do something when encountering rdtimeh
            }
            RISCVToken::Ret => {
                // Do something when encountering ret
            }
            RISCVToken::Seqz => {
                // Do something when encountering seqz
            }
            RISCVToken::SextB => {
                // Do something when encountering sext.b
            }
            RISCVToken::SextH => {
                // Do something when encountering sext.h
            }
            RISCVToken::Sgt => {
                // Do something when encountering sgt
            }
            RISCVToken::Sgtu => {
                // Do something when encountering sgtu
            }
            RISCVToken::Sgtz => {
                // Do something when encountering sgtz
            }
            RISCVToken::Sltz => {
                // Do something when encountering sltz
            }
            RISCVToken::Snez => {
                // Do something when encountering snez
            }
            RISCVToken::Tail => {
                // Do something when encountering tail
            }
            RISCVToken::ZextB => {
                // Do something when encountering zext.b
            }
            RISCVToken::ZextH => {
                // Do something when encountering zext.h
            }
        }
        Ok(())
    }
}
