use super::super::interface::parser::{ParseRISCVRegisterError, RISCVCsr, RISCVRegister};
use crate::interface::parser::{ParserError, Pos};
use logos::Logos;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Default)]
pub enum LexingError {
    NumberParseError,
    RegisterParseError,
    #[default]
    Other,
}

pub(super) struct LexerIter<'a> {
    pub raw: logos::Lexer<'a, RISCVToken<'a>>,
}

impl LexerIter<'_> {
    #[inline(always)]
    pub fn next<'a>(&'a mut self) -> Result<Option<RISCVToken<'a>>, Vec<ParserError>> {
        match self.raw.next() {
            Some(unit) => match unit {
                Ok(token) => Ok(Some(token)),
                Err(e) => Err(self.get_error(e.to_string())),
            },
            None => Ok(None),
        }
    }

    pub fn next_not_newline<'a>(&'a mut self) -> Result<Option<RISCVToken<'a>>, Vec<ParserError>> {
        loop {
            match self.raw.next() {
                Some(unit) => match unit {
                    Ok(token) => {
                        if token == RISCVToken::Newline {
                            continue;
                        }
                        return Ok(Some(token));
                    }
                    Err(e) => return Err(self.get_error(e.to_string())),
                },
                None => return Ok(None),
            }
        }
    }

    #[inline(always)]
    pub fn get_error<'a>(&mut self, msg: String) -> Vec<ParserError> {
        vec![ParserError {
            pos: self.pos(),
            msg,
        }]
    }

    #[inline(always)]
    pub fn pos(&self) -> Pos {
        if self.raw.span().start < self.raw.extras.1 {
            Pos(self.raw.extras.0, self.raw.span().start)
        } else {
            Pos(self.raw.extras.0, self.raw.span().start - self.raw.extras.1)
        }
    }

    #[inline(always)]
    pub fn line(&self) -> usize {
        self.raw.extras.0
    }
}

#[derive(Logos, Clone, Copy, Debug, PartialEq)]
#[logos(skip r"([ \t\f]+)|(#.*)", error = LexingError, extras = (usize, usize))]
pub(super) enum RISCVToken<'a> {
    #[token(",")]
    Comma,
    #[token("\n", |lex| lex.extras.0 += 1; lex.extras.1 = lex.span().end;)]
    Newline,
    #[token(":")]
    Colon,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[regex(r"-?0x[0-9a-fA-F]+", |lex| i128::from_str_radix(&lex.slice()[2..], 16))]
    #[regex(r"-?[0-9]+", |lex| lex.slice().parse())]
    ImmediateInt(i128),
    #[regex(r"-?[0-9]+\.[0-9]+", |lex| lex.slice().parse())]
    ImmediateFloat(f64),
    #[regex("\"([^\\\\\"]*\\\\.)*\"")]
    ImmediateString(&'a str),
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Label(&'a str),
    #[regex(r"%[a-zA-Z_][a-zA-Z0-9_]*")]
    MacroParameter(&'a str),
    #[token("zero", |lex| lex.slice().parse(), priority = 10)]
    #[token("ra", |lex| lex.slice().parse(), priority = 10)]
    #[token("sp", |lex| lex.slice().parse(), priority = 10)]
    #[token("gp", |lex| lex.slice().parse(), priority = 10)]
    #[token("tp", |lex| lex.slice().parse(), priority = 10)]
    #[regex(r"t[0-6]", |lex| lex.slice().parse(), priority = 10)]
    #[regex(r"s([0-9]|(1[0-1]))", |lex| lex.slice().parse(), priority = 10)]
    #[regex(r"a[0-7]", |lex| lex.slice().parse(), priority = 10)]
    #[regex(r"x(([1-2]?[0-9])|(3[0-1]))", |lex| lex.slice().parse(), priority = 10)]
    Register(RISCVRegister),
    Csr(RISCVCsr),
    #[token(".align", priority = 10)]
    Align,
    #[token(".ascii", priority = 10)]
    Ascii,
    #[token(".asciz", priority = 10)]
    Asciz,
    #[token(".byte", priority = 10)]
    Byte,
    #[token(".data", priority = 10)]
    Data,
    #[token(".double", priority = 10)]
    Double,
    #[token(".dword", priority = 10)]
    Dword,
    #[token(".end_macro", priority = 10)]
    EndMacro,
    #[token(".eqv", priority = 10)]
    Eqv,
    #[token(".extern", priority = 10)]
    Extern,
    #[token(".float", priority = 10)]
    Float,
    #[token(".global", priority = 10)]
    Global,
    #[token(".half", priority = 10)]
    Half,
    #[token(".include", priority = 10)]
    Include,
    #[regex(r"\.macro[ ]+[a-zA-Z_][a-zA-Z0-9_]*[ ]+(\([ ]*%[a-zA-Z_][a-zA-Z0-9_]*([ ]*,[ ]*%[a-zA-Z_][a-zA-Z0-9_]*)*[ ]*\))?", priority = 10)]
    MacroDef,
    #[token(".macro", priority = 10)]
    Macro,
    #[token(".section", priority = 10)]
    Section,
    #[token(".space", priority = 10)]
    Space,
    #[token(".string", priority = 10)]
    String,
    #[token(".text", priority = 10)]
    Text,
    #[token(".word", priority = 10)]
    Word,
    #[regex(r"\.[a-zA-Z_][a-zA-Z0-9_]*")]
    UnknownDirective(&'a str),
    #[token("add", |_| RISCVOpToken::Add, priority = 10)]
    #[token("addi", |_| RISCVOpToken::Addi ,priority = 10)]
    #[token("and", |_| RISCVOpToken::And ,priority = 10)]
    #[token("andi", |_| RISCVOpToken::Andi ,priority = 10)]
    #[token("auipc", |_| RISCVOpToken::Auipc ,priority = 10)]
    #[token("beq", |_| RISCVOpToken::Beq ,priority = 10)]
    #[token("bge", |_| RISCVOpToken::Bge ,priority = 10)]
    #[token("bgeu", |_| RISCVOpToken::Bgeu ,priority = 10)]
    #[token("blt", |_| RISCVOpToken::Blt ,priority = 10)]
    #[token("bltu", |_| RISCVOpToken::Bltu ,priority = 10)]
    #[token("bne", |_| RISCVOpToken::Bne ,priority = 10)]
    #[token("csrrc", |_| RISCVOpToken::Csrrc ,priority = 10)]
    #[token("csrrci", |_| RISCVOpToken::Csrrci ,priority = 10)]
    #[token("csrrs", |_| RISCVOpToken::Csrrs ,priority = 10)]
    #[token("csrrsi", |_| RISCVOpToken::Csrrsi ,priority = 10)]
    #[token("csrrw", |_| RISCVOpToken::Csrrw ,priority = 10)]
    #[token("csrrwi", |_| RISCVOpToken::Csrrwi ,priority = 10)]
    #[token("div", |_| RISCVOpToken::Div ,priority = 10)]
    #[token("divu", |_| RISCVOpToken::Divu ,priority = 10)]
    #[token("ebreak", |_| RISCVOpToken::Ebreak ,priority = 10)]
    #[token("ecall", |_| RISCVOpToken::Ecall ,priority = 10)]
    // #[token("fadd.d", priority = 10)]
    // #[token("fadd.s", priority = 10)]
    // #[token("fclass.d", priority = 10)]
    // #[token("fclass.s", priority = 10)]
    // #[token("fcvt.d.s", priority = 10)]
    // #[token("fcvt.d.w", priority = 10)]
    // #[token("fcvt.d.wu", priority = 10)]
    // #[token("fcvt.s.d", priority = 10)]
    // #[token("fcvt.s.w", priority = 10)]
    // #[token("fcvt.s.wu", priority = 10)]
    // #[token("fcvt.w.d", priority = 10)]
    // #[token("fcvt.w.s", priority = 10)]
    // #[token("fcvt.wu.d", priority = 10)]
    // #[token("fcvt.wu.s", priority = 10)]
    // #[token("fdiv.d", priority = 10)]
    // #[token("fdiv.s", priority = 10)]
    // #[token("fence", priority = 10)]
    // #[token("fence.i", priority = 10)]
    // #[token("feq.d", priority = 10)]
    // #[token("feq.s", priority = 10)]
    // #[token("fld", priority = 10)]
    // #[token("fle.d", priority = 10)]
    // #[token("fle.s", priority = 10)]
    // #[token("flt.d", priority = 10)]
    // #[token("flt.s", priority = 10)]
    // #[token("flw", priority = 10)]
    // #[token("fmadd.d", priority = 10)]
    // #[token("fmadd.s", priority = 10)]
    // #[token("fmax.d", priority = 10)]
    // #[token("fmax.s", priority = 10)]
    // #[token("fmin.d", priority = 10)]
    // #[token("fmin.s", priority = 10)]
    // #[token("fmsub.d", priority = 10)]
    // #[token("fmsub.s", priority = 10)]
    // #[token("fmul.d", priority = 10)]
    // #[token("fmul.s", priority = 10)]
    // #[token("fmv.s.x", priority = 10)]
    // #[token("fmv.x.s", priority = 10)]
    // #[token("fnmadd.d", priority = 10)]
    // #[token("fnmadd.s", priority = 10)]
    // #[token("fnmsub.d", priority = 10)]
    // #[token("fnmsub.s", priority = 10)]
    // #[token("fsd", priority = 10)]
    // #[token("fsgnj.d", priority = 10)]
    // #[token("fsgnj.s", priority = 10)]
    // #[token("fsgnjn.d", priority = 10)]
    // #[token("fsgnjn.s", priority = 10)]
    // #[token("fsgnjx.d", priority = 10)]
    // #[token("fsgnjx.s", priority = 10)]
    // #[token("fsqrt.d", priority = 10)]
    // #[token("fsqrt.s", priority = 10)]
    // #[token("fsub.d", priority = 10)]
    // #[token("fsub.s", priority = 10)]
    // #[token("fsw", priority = 10)]
    #[token("jal", |_| RISCVOpToken::Jal, priority = 10)]
    #[token("jalr", |_| RISCVOpToken::Jalr, priority = 10)]
    #[token("lb", |_| RISCVOpToken::Lb, priority = 10)]
    #[token("lbu", |_| RISCVOpToken::Lbu, priority = 10)]
    #[token("lh", |_| RISCVOpToken::Lh, priority = 10)]
    #[token("lhu", |_| RISCVOpToken::Lhu, priority = 10)]
    #[token("lui", |_| RISCVOpToken::Lui, priority = 10)]
    #[token("lw", |_| RISCVOpToken::Lw, priority = 10)]
    #[token("mul", |_| RISCVOpToken::Mul, priority = 10)]
    #[token("mulh", |_| RISCVOpToken::Mulh, priority = 10)]
    #[token("mulhsu", |_| RISCVOpToken::Mulhsu, priority = 10)]
    #[token("mulhu", |_| RISCVOpToken::Mulhu, priority = 10)]
    #[token("or", |_| RISCVOpToken::Or, priority = 10)]
    #[token("ori", |_| RISCVOpToken::Ori, priority = 10)]
    #[token("rem", |_| RISCVOpToken::Rem, priority = 10)]
    #[token("remu", |_| RISCVOpToken::Remu, priority = 10)]
    #[token("sb", |_| RISCVOpToken::Sb, priority = 10)]
    #[token("sh", |_| RISCVOpToken::Sh, priority = 10)]
    #[token("sll", |_| RISCVOpToken::Sll, priority = 10)]
    #[token("slli", |_| RISCVOpToken::Slli, priority = 10)]
    #[token("slt", |_| RISCVOpToken::Slt, priority = 10)]
    #[token("slti", |_| RISCVOpToken::Slti, priority = 10)]
    #[token("sltiu", |_| RISCVOpToken::Sltiu, priority = 10)]
    #[token("sltu", |_| RISCVOpToken::Sltu, priority = 10)]
    #[token("sra", |_| RISCVOpToken::Sra, priority = 10)]
    #[token("srai", |_| RISCVOpToken::Srai, priority = 10)]
    #[token("srl", |_| RISCVOpToken::Srl, priority = 10)]
    #[token("srli", |_| RISCVOpToken::Srli, priority = 10)]
    #[token("sub", |_| RISCVOpToken::Sub, priority = 10)]
    #[token("sw", |_| RISCVOpToken::Sw, priority = 10)]
    #[token("uret", |_| RISCVOpToken::Uret, priority = 10)]
    #[token("wfi", |_| RISCVOpToken::Wfi, priority = 10)]
    #[token("xor", |_| RISCVOpToken::Xor, priority = 10)]
    #[token("xori", |_| RISCVOpToken::Xori, priority = 10)]
    #[token("b", |_| RISCVOpToken::B, priority = 10)]
    #[token("beqz", |_| RISCVOpToken::Beqz, priority = 10)]
    #[token("bgez", |_| RISCVOpToken::Bgez, priority = 10)]
    #[token("bgt", |_| RISCVOpToken::Bgt, priority = 10)]
    #[token("bgtu", |_| RISCVOpToken::Bgtu, priority = 10)]
    #[token("bgtz", |_| RISCVOpToken::Bgtz, priority = 10)]
    #[token("ble", |_| RISCVOpToken::Ble, priority = 10)]
    #[token("bleu", |_| RISCVOpToken::Bleu, priority = 10)]
    #[token("blez", |_| RISCVOpToken::Blez, priority = 10)]
    #[token("bltz", |_| RISCVOpToken::Bltz, priority = 10)]
    #[token("bnez", |_| RISCVOpToken::Bnez, priority = 10)]
    #[token("call", |_| RISCVOpToken::Call, priority = 10)]
    #[token("csrc", |_| RISCVOpToken::Csrc, priority = 10)]
    #[token("csrci", |_| RISCVOpToken::Csrci, priority = 10)]
    #[token("csrr", |_| RISCVOpToken::Csrr, priority = 10)]
    #[token("csrs", |_| RISCVOpToken::Csrs, priority = 10)]
    #[token("csrsi", |_| RISCVOpToken::Csrsi, priority = 10)]
    #[token("csrw", |_| RISCVOpToken::Csrw, priority = 10)]
    #[token("csrwi", |_| RISCVOpToken::Csrwi, priority = 10)]
    // #[token("fabs.d", |_| RISCVOpToken::FabsD, priority = 10)]
    // #[token("fabs.s", |_| RISCVOpToken::FabsS, priority = 10)]
    // #[token("fge.d", |_| RISCVOpToken::FgeD, priority = 10)]
    // #[token("fge.s", |_| RISCVOpToken::FgeS, priority = 10)]
    // #[token("fgt.d", |_| RISCVOpToken::FgtD, priority = 10)]
    // #[token("fgt.s", |_| RISCVOpToken::FgtS, priority = 10)]
    // #[token("fmv.d", |_| RISCVOpToken::FmvD, priority = 10)]
    // #[token("fmv.s", |_| RISCVOpToken::FmvS, priority = 10)]
    // #[token("fmv.w.x", |_| RISCVOpToken::FmvWX, priority = 10)]
    // #[token("fmv.x.w", |_| RISCVOpToken::FmvXW, priority = 10)]
    // #[token("fneg.d", |_| RISCVOpToken::FnegD, priority = 10)]
    // #[token("fneg.s", |_| RISCVOpToken::FnegS, priority = 10)]
    // #[token("frcsr", |_| RISCVOpToken::Frcsr, priority = 10)]
    // #[token("frflags", |_| RISCVOpToken::Frflags, priority = 10)]
    // #[token("frrm", |_| RISCVOpToken::Frrm, priority = 10)]
    // #[token("frsr", |_| RISCVOpToken::Frsr, priority = 10)]
    // #[token("fsflags", |_| RISCVOpToken::Fsflags, priority = 10)]
    // #[token("fsrm", |_| RISCVOpToken::Fsrm, priority = 10)]
    // #[token("fsrr", |_| RISCVOpToken::Fsrr, priority = 10)]
    #[token("j", |_| RISCVOpToken::J, priority = 10)]
    #[token("jr", |_| RISCVOpToken::Jr, priority = 10)]
    #[token("la", |_| RISCVOpToken::La, priority = 10)]
    #[token("li", |_| RISCVOpToken::Li, priority = 10)]
    #[token("mv", |_| RISCVOpToken::Mv, priority = 10)]
    #[token("neg", |_| RISCVOpToken::Neg, priority = 10)]
    #[token("nop", |_| RISCVOpToken::Nop, priority = 10)]
    #[token("not", |_| RISCVOpToken::Not, priority = 10)]
    #[token("rdcycle", |_| RISCVOpToken::Rdcycle, priority = 10)]
    #[token("rdcycleh", |_| RISCVOpToken::Rdcycleh, priority = 10)]
    #[token("rdinstret", |_| RISCVOpToken::Rdinstret, priority = 10)]
    #[token("rdinstreth", |_| RISCVOpToken::Rdinstreth, priority = 10)]
    #[token("rdtime", |_| RISCVOpToken::Rdtime, priority = 10)]
    #[token("rdtimeh", |_| RISCVOpToken::Rdtimeh, priority = 10)]
    #[token("ret", |_| RISCVOpToken::Ret, priority = 10)]
    #[token("seqz", |_| RISCVOpToken::Seqz, priority = 10)]
    #[token("sext.b", |_| RISCVOpToken::SextB, priority = 10)]
    #[token("sext.h", |_| RISCVOpToken::SextH, priority = 10)]
    #[token("sgt", |_| RISCVOpToken::Sgt, priority = 10)]
    #[token("sgtu", |_| RISCVOpToken::Sgtu, priority = 10)]
    #[token("sgtz", |_| RISCVOpToken::Sgtz, priority = 10)]
    #[token("sltz", |_| RISCVOpToken::Sltz, priority = 10)]
    #[token("snez", |_| RISCVOpToken::Snez, priority = 10)]
    #[token("tail", |_| RISCVOpToken::Tail, priority = 10)]
    #[token("zext.b", |_| RISCVOpToken::ZextB, priority = 10)]
    #[token("zext.h", |_| RISCVOpToken::ZextH, priority = 10)]
    Operator(RISCVOpToken),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum RISCVOpToken {
    Add,
    Addi,
    And,
    Andi,
    Auipc,
    Beq,
    Bge,
    Bgeu,
    Blt,
    Bltu,
    Bne,
    Csrrc,
    Csrrci,
    Csrrs,
    Csrrsi,
    Csrrw,
    Csrrwi,
    Div,
    Divu,
    Ebreak,
    Ecall,
    FaddD,
    FaddS,
    FclassD,
    FclassS,
    FcvtDS,
    FcvtDW,
    FcvtDWu,
    FcvtSD,
    FcvtSW,
    FcvtSWu,
    FcvtWD,
    FcvtWS,
    FcvtWuD,
    FcvtWuS,
    FdivD,
    FdivS,
    Fence,
    FenceI,
    FeqD,
    FeqS,
    Fld,
    FleD,
    FleS,
    FltD,
    FltS,
    Flw,
    FmaddD,
    FmaddS,
    FmaxD,
    FmaxS,
    FminD,
    FminS,
    FmsubD,
    FmsubS,
    FmulD,
    FmulS,
    FmvSX,
    FmvXS,
    FnmaddD,
    FnmaddS,
    FnmsubD,
    FnmsubS,
    Fsd,
    FsgnjD,
    FsgnjS,
    FsgnjnD,
    FsgnjnS,
    FsgnjxD,
    FsgnjxS,
    FsqrtD,
    FsqrtS,
    FsubD,
    FsubS,
    Fsw,
    Jal,
    Jalr,
    Lb,
    Lbu,
    Lh,
    Lhu,
    Lui,
    Lw,
    Mul,
    Mulh,
    Mulhsu,
    Mulhu,
    Or,
    Ori,
    Rem,
    Remu,
    Sb,
    Sh,
    Sll,
    Slli,
    Slt,
    Slti,
    Sltiu,
    Sltu,
    Sra,
    Srai,
    Srl,
    Srli,
    Sub,
    Sw,
    Uret,
    Wfi,
    Xor,
    Xori,
    B,
    Beqz,
    Bgez,
    Bgt,
    Bgtu,
    Bgtz,
    Ble,
    Bleu,
    Blez,
    Bltz,
    Bnez,
    Call,
    Csrc,
    Csrci,
    Csrr,
    Csrs,
    Csrsi,
    Csrw,
    Csrwi,
    FabsD,
    FabsS,
    FgeD,
    FgeS,
    FgtD,
    FgtS,
    FmvD,
    FmvS,
    FmvWX,
    FmvXW,
    FnegD,
    FnegS,
    Frcsr,
    Frflags,
    Frrm,
    Frsr,
    Fsflags,
    Fsrm,
    Fsrr,
    J,
    Jr,
    La,
    Li,
    Mv,
    Neg,
    Nop,
    Not,
    Rdcycle,
    Rdcycleh,
    Rdinstret,
    Rdinstreth,
    Rdtime,
    Rdtimeh,
    Ret,
    Seqz,
    SextB,
    SextH,
    Sgt,
    Sgtu,
    Sgtz,
    Sltz,
    Snez,
    Tail,
    ZextB,
    ZextH,
}

impl From<std::num::ParseIntError> for LexingError {
    fn from(_: std::num::ParseIntError) -> Self {
        LexingError::NumberParseError
    }
}

impl From<std::num::ParseFloatError> for LexingError {
    fn from(_: std::num::ParseFloatError) -> Self {
        LexingError::NumberParseError
    }
}

impl From<ParseRISCVRegisterError> for LexingError {
    fn from(_: ParseRISCVRegisterError) -> Self {
        LexingError::RegisterParseError
    }
}

impl Display for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexingError::NumberParseError => write!(f, "Number parse error"),
            LexingError::RegisterParseError => write!(f, "Register parse error"),
            LexingError::Other => write!(f, "unrecognized character"),
        }
    }
}
