use super::super::interface::parser::{ParserRISCVCsr, ParserRISCVRegister};
use super::oplist::RISCVOpdSet;
use crate::interface::parser::{ParserError, Pos};
use logos::Logos;
use std::fmt::Display;

static EXTENSION: [LexerExtension; 1] = [LexerExtension {
    name: "rv32i",
    parse_op: super::super::super::rv32i::parser::lexer::parse_op,
    parse_reg: super::super::super::rv32i::parser::lexer::parse_reg,
}];

pub struct LexerExtension {
    pub name: &'static str,
    pub parse_op: fn(&str) -> Option<RISCVOpToken>,
    pub parse_reg: fn(&str) -> Option<ParserRISCVRegister>,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub enum LexingError {
    NumberParseError,
    #[default]
    Other,
}

pub enum Symbol<'a> {
    Label(&'a str),
    Op(RISCVOpToken),
    Reg(ParserRISCVRegister),
}

pub trait RISCVOpTokenTrait {
    fn get_opd_set(&self) -> &Vec<RISCVOpdSet>;
}
pub type RISCVOpToken = &'static dyn RISCVOpTokenTrait;

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
                    Ok(token) => match token {
                        RISCVToken::Newline => continue,
                        _ => return Ok(Some(token)),
                    },
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

fn dispatch_symbol(token: &str) -> Symbol {
    for ext in &EXTENSION {
        if let Some(reg) = (ext.parse_reg)(token) {
            return Symbol::Reg(reg);
        }
        if let Some(op) = (ext.parse_op)(token) {
            return Symbol::Op(op);
        }
    }
    Symbol::Label(token)
}

#[derive(Logos)]
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
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| dispatch_symbol(lex.slice()))]
    Symbol(Symbol<'a>),
    #[regex(r"%[a-zA-Z_][a-zA-Z0-9_]*")]
    MacroParameter(&'a str),
    Csr(ParserRISCVCsr),
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

impl Display for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexingError::NumberParseError => write!(f, "Number parse error"),
            LexingError::Other => write!(f, "unrecognized character"),
        }
    }
}
