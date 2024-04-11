use super::super::interface::parser::{ParseRISCVRegisterError, RISCVRegister};
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
    pub fn next<'a>(&'a mut self) -> Result<Option<RISCVToken<'a>>, ParserError> {
        match self.raw.next() {
            Some(unit) => match unit {
                Ok(token) => Ok(Some(token)),
                Err(e) => Err(self.get_error(e.to_string())),
            },
            None => Ok(None),
        }
    }

    pub fn next_not_newline<'a>(&'a mut self) -> Result<Option<RISCVToken<'a>>, ParserError> {
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
    pub fn get_error<'a>(&mut self, msg: String) -> ParserError {
        ParserError {
            pos: self.pos(),
            msg,
        }
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
pub enum RISCVToken<'a> {
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
    #[token("add", priority = 10)]
    Add,
    #[token("addi", priority = 10)]
    Addi,
    #[token("and", priority = 10)]
    And,
    #[token("andi", priority = 10)]
    Andi,
    #[token("auipc", priority = 10)]
    Auipc,
    #[token("beq", priority = 10)]
    Beq,
    #[token("bge", priority = 10)]
    Bge,
    #[token("bgeu", priority = 10)]
    Bgeu,
    #[token("blt", priority = 10)]
    Blt,
    #[token("bltu", priority = 10)]
    Bltu,
    #[token("bne", priority = 10)]
    Bne,
    #[token("csrrc", priority = 10)]
    Csrrc,
    #[token("csrrci", priority = 10)]
    Csrrci,
    #[token("csrrs", priority = 10)]
    Csrrs,
    #[token("csrrsi", priority = 10)]
    Csrrsi,
    #[token("csrrw", priority = 10)]
    Csrrw,
    #[token("csrrwi", priority = 10)]
    Csrrwi,
    #[token("div", priority = 10)]
    Div,
    #[token("divu", priority = 10)]
    Divu,
    #[token("ebreak", priority = 10)]
    Ebreak,
    #[token("ecall", priority = 10)]
    Ecall,
    #[token("fadd.d", priority = 10)]
    FaddD,
    #[token("fadd.s", priority = 10)]
    FaddS,
    #[token("fclass.d", priority = 10)]
    FclassD,
    #[token("fclass.s", priority = 10)]
    FclassS,
    #[token("fcvt.d.s", priority = 10)]
    FcvtDS,
    #[token("fcvt.d.w", priority = 10)]
    FcvtDW,
    #[token("fcvt.d.wu", priority = 10)]
    FcvtDWu,
    #[token("fcvt.s.d", priority = 10)]
    FcvtSD,
    #[token("fcvt.s.w", priority = 10)]
    FcvtSW,
    #[token("fcvt.s.wu", priority = 10)]
    FcvtSWu,
    #[token("fcvt.w.d", priority = 10)]
    FcvtWD,
    #[token("fcvt.w.s", priority = 10)]
    FcvtWS,
    #[token("fcvt.wu.d", priority = 10)]
    FcvtWuD,
    #[token("fcvt.wu.s", priority = 10)]
    FcvtWuS,
    #[token("fdiv.d", priority = 10)]
    FdivD,
    #[token("fdiv.s", priority = 10)]
    FdivS,
    #[token("fence", priority = 10)]
    Fence,
    #[token("fence.i", priority = 10)]
    FenceI,
    #[token("feq.d", priority = 10)]
    FeqD,
    #[token("feq.s", priority = 10)]
    FeqS,
    #[token("fld", priority = 10)]
    Fld,
    #[token("fle.d", priority = 10)]
    FleD,
    #[token("fle.s", priority = 10)]
    FleS,
    #[token("flt.d", priority = 10)]
    FltD,
    #[token("flt.s", priority = 10)]
    FltS,
    #[token("flw", priority = 10)]
    Flw,
    #[token("fmadd.d", priority = 10)]
    FmaddD,
    #[token("fmadd.s", priority = 10)]
    FmaddS,
    #[token("fmax.d", priority = 10)]
    FmaxD,
    #[token("fmax.s", priority = 10)]
    FmaxS,
    #[token("fmin.d", priority = 10)]
    FminD,
    #[token("fmin.s", priority = 10)]
    FminS,
    #[token("fmsub.d", priority = 10)]
    FmsubD,
    #[token("fmsub.s", priority = 10)]
    FmsubS,
    #[token("fmul.d", priority = 10)]
    FmulD,
    #[token("fmul.s", priority = 10)]
    FmulS,
    #[token("fmv.s.x", priority = 10)]
    FmvSX,
    #[token("fmv.x.s", priority = 10)]
    FmvXS,
    #[token("fnmadd.d", priority = 10)]
    FnmaddD,
    #[token("fnmadd.s", priority = 10)]
    FnmaddS,
    #[token("fnmsub.d", priority = 10)]
    FnmsubD,
    #[token("fnmsub.s", priority = 10)]
    FnmsubS,
    #[token("fsd", priority = 10)]
    Fsd,
    #[token("fsgnj.d", priority = 10)]
    FsgnjD,
    #[token("fsgnj.s", priority = 10)]
    FsgnjS,
    #[token("fsgnjn.d", priority = 10)]
    FsgnjnD,
    #[token("fsgnjn.s", priority = 10)]
    FsgnjnS,
    #[token("fsgnjx.d", priority = 10)]
    FsgnjxD,
    #[token("fsgnjx.s", priority = 10)]
    FsgnjxS,
    #[token("fsqrt.d", priority = 10)]
    FsqrtD,
    #[token("fsqrt.s", priority = 10)]
    FsqrtS,
    #[token("fsub.d", priority = 10)]
    FsubD,
    #[token("fsub.s", priority = 10)]
    FsubS,
    #[token("fsw", priority = 10)]
    Fsw,
    #[token("jal", priority = 10)]
    Jal,
    #[token("jalr", priority = 10)]
    Jalr,
    #[token("lb", priority = 10)]
    Lb,
    #[token("lbu", priority = 10)]
    Lbu,
    #[token("lh", priority = 10)]
    Lh,
    #[token("lhu", priority = 10)]
    Lhu,
    #[token("lui", priority = 10)]
    Lui,
    #[token("lw", priority = 10)]
    Lw,
    #[token("mul", priority = 10)]
    Mul,
    #[token("mulh", priority = 10)]
    Mulh,
    #[token("mulhsu", priority = 10)]
    Mulhsu,
    #[token("mulhu", priority = 10)]
    Mulhu,
    #[token("or", priority = 10)]
    Or,
    #[token("ori", priority = 10)]
    Ori,
    #[token("rem", priority = 10)]
    Rem,
    #[token("remu", priority = 10)]
    Remu,
    #[token("sb", priority = 10)]
    Sb,
    #[token("sh", priority = 10)]
    Sh,
    #[token("sll", priority = 10)]
    Sll,
    #[token("slli", priority = 10)]
    Slli,
    #[token("slt", priority = 10)]
    Slt,
    #[token("slti", priority = 10)]
    Slti,
    #[token("sltiu", priority = 10)]
    Sltiu,
    #[token("sltu", priority = 10)]
    Sltu,
    #[token("sra", priority = 10)]
    Sra,
    #[token("srai", priority = 10)]
    Srai,
    #[token("srl", priority = 10)]
    Srl,
    #[token("srli", priority = 10)]
    Srli,
    #[token("sub", priority = 10)]
    Sub,
    #[token("sw", priority = 10)]
    Sw,
    #[token("uret", priority = 10)]
    Uret,
    #[token("wfi", priority = 10)]
    Wfi,
    #[token("xor", priority = 10)]
    Xor,
    #[token("xori", priority = 10)]
    Xori,
    #[token("b", priority = 10)]
    B,
    #[token("beqz", priority = 10)]
    Beqz,
    #[token("bgez", priority = 10)]
    Bgez,
    #[token("bgt", priority = 10)]
    Bgt,
    #[token("bgtu", priority = 10)]
    Bgtu,
    #[token("bgtz", priority = 10)]
    Bgtz,
    #[token("ble", priority = 10)]
    Ble,
    #[token("bleu", priority = 10)]
    Bleu,
    #[token("blez", priority = 10)]
    Blez,
    #[token("bltz", priority = 10)]
    Bltz,
    #[token("bnez", priority = 10)]
    Bnez,
    #[token("call", priority = 10)]
    Call,
    #[token("csrc", priority = 10)]
    Csrc,
    #[token("csrci", priority = 10)]
    Csrci,
    #[token("csrr", priority = 10)]
    Csrr,
    #[token("csrs", priority = 10)]
    Csrs,
    #[token("csrsi", priority = 10)]
    Csrsi,
    #[token("csrw", priority = 10)]
    Csrw,
    #[token("csrwi", priority = 10)]
    Csrwi,
    #[token("fabs.d", priority = 10)]
    FabsD,
    #[token("fabs.s", priority = 10)]
    FabsS,
    #[token("fge.d", priority = 10)]
    FgeD,
    #[token("fge.s", priority = 10)]
    FgeS,
    #[token("fgt.d", priority = 10)]
    FgtD,
    #[token("fgt.s", priority = 10)]
    FgtS,
    #[token("fmv.d", priority = 10)]
    FmvD,
    #[token("fmv.s", priority = 10)]
    FmvS,
    #[token("fmv.w.x", priority = 10)]
    FmvWX,
    #[token("fmv.x.w", priority = 10)]
    FmvXW,
    #[token("fneg.d", priority = 10)]
    FnegD,
    #[token("fneg.s", priority = 10)]
    FnegS,
    #[token("frcsr", priority = 10)]
    Frcsr,
    #[token("frflags", priority = 10)]
    Frflags,
    #[token("frrm", priority = 10)]
    Frrm,
    #[token("frsr", priority = 10)]
    Frsr,
    #[token("fsflags", priority = 10)]
    Fsflags,
    #[token("fsrm", priority = 10)]
    Fsrm,
    #[token("fsrr", priority = 10)]
    Fsrr,
    #[token("j", priority = 10)]
    J,
    #[token("jr", priority = 10)]
    Jr,
    #[token("la", priority = 10)]
    La,
    #[token("li", priority = 10)]
    Li,
    #[token("mv", priority = 10)]
    Mv,
    #[token("neg", priority = 10)]
    Neg,
    #[token("nop", priority = 10)]
    Nop,
    #[token("not", priority = 10)]
    Not,
    #[token("rdcycle", priority = 10)]
    Rdcycle,
    #[token("rdcycleh", priority = 10)]
    Rdcycleh,
    #[token("rdinstret", priority = 10)]
    Rdinstret,
    #[token("rdinstreth", priority = 10)]
    Rdinstreth,
    #[token("rdtime", priority = 10)]
    Rdtime,
    #[token("rdtimeh", priority = 10)]
    Rdtimeh,
    #[token("ret", priority = 10)]
    Ret,
    #[token("seqz", priority = 10)]
    Seqz,
    #[token("sext.b", priority = 10)]
    SextB,
    #[token("sext.h", priority = 10)]
    SextH,
    #[token("sgt", priority = 10)]
    Sgt,
    #[token("sgtu", priority = 10)]
    Sgtu,
    #[token("sgtz", priority = 10)]
    Sgtz,
    #[token("sltz", priority = 10)]
    Sltz,
    #[token("snez", priority = 10)]
    Snez,
    #[token("tail", priority = 10)]
    Tail,
    #[token("zext.b", priority = 10)]
    ZextB,
    #[token("zext.h", priority = 10)]
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
            LexingError::Other => write!(f, "Other lexing error"),
        }
    }
}
