use std::collections::HashMap;

use super::super::super::basic::parser::lexer::RISCVOpToken;
use super::oplist::{RISCVOpdSet, OP_LIST};
use lazy_static::lazy_static;

pub fn op_lexer(op: &str) -> Option<&'static dyn RISCVOpToken> {
    match OP_TOKEN.get(op) {
        Some(op) => Some(op as &dyn RISCVOpToken),
        None => None,
    }
}

lazy_static! {
    static ref OP_TOKEN: HashMap<&'static str, RV32IOpToken> = HashMap::from([
        ("add", RV32IOpToken::Add),
        ("addi", RV32IOpToken::Addi),
        ("and", RV32IOpToken::And),
        ("andi", RV32IOpToken::Andi),
        ("auipc", RV32IOpToken::Auipc),
        ("beq", RV32IOpToken::Beq),
        ("bge", RV32IOpToken::Bge),
        ("bgeu", RV32IOpToken::Bgeu),
        ("blt", RV32IOpToken::Blt),
        ("bltu", RV32IOpToken::Bltu),
        ("bne", RV32IOpToken::Bne),
        ("csrrc", RV32IOpToken::Csrrc),
        ("csrrci", RV32IOpToken::Csrrci),
        ("csrrs", RV32IOpToken::Csrrs),
        ("csrrsi", RV32IOpToken::Csrrsi),
        ("csrrw", RV32IOpToken::Csrrw),
        ("csrrwi", RV32IOpToken::Csrrwi),
        ("div", RV32IOpToken::Div),
        ("divu", RV32IOpToken::Divu),
        ("ebreak", RV32IOpToken::Ebreak),
        ("ecall", RV32IOpToken::Ecall),
        ("jal", RV32IOpToken::Jal),
        ("jalr", RV32IOpToken::Jalr),
        ("lb", RV32IOpToken::Lb),
        ("lbu", RV32IOpToken::Lbu),
        ("lh", RV32IOpToken::Lh),
        ("lhu", RV32IOpToken::Lhu),
        ("lui", RV32IOpToken::Lui),
        ("lw", RV32IOpToken::Lw),
        ("mul", RV32IOpToken::Mul),
        ("mulh", RV32IOpToken::Mulh),
        ("mulhsu", RV32IOpToken::Mulhsu),
        ("mulhu", RV32IOpToken::Mulhu),
        ("or", RV32IOpToken::Or),
        ("ori", RV32IOpToken::Ori),
        ("rem", RV32IOpToken::Rem),
        ("remu", RV32IOpToken::Remu),
        ("sb", RV32IOpToken::Sb),
        ("sh", RV32IOpToken::Sh),
        ("sll", RV32IOpToken::Sll),
        ("slli", RV32IOpToken::Slli),
        ("slt", RV32IOpToken::Slt),
        ("slti", RV32IOpToken::Slti),
        ("sltiu", RV32IOpToken::Sltiu),
        ("sltu", RV32IOpToken::Sltu),
        ("sra", RV32IOpToken::Sra),
        ("srai", RV32IOpToken::Srai),
        ("srl", RV32IOpToken::Srl),
        ("srli", RV32IOpToken::Srli),
        ("sub", RV32IOpToken::Sub),
        ("sw", RV32IOpToken::Sw),
        ("uret", RV32IOpToken::Uret),
        ("wfi", RV32IOpToken::Wfi),
        ("xor", RV32IOpToken::Xor),
        ("xori", RV32IOpToken::Xori),
        ("b", RV32IOpToken::B),
        ("beqz", RV32IOpToken::Beqz),
        ("bgez", RV32IOpToken::Bgez),
        ("bgt", RV32IOpToken::Bgt),
        ("bgtu", RV32IOpToken::Bgtu),
        ("bgtz", RV32IOpToken::Bgtz),
        ("ble", RV32IOpToken::Ble),
        ("bleu", RV32IOpToken::Bleu),
        ("blez", RV32IOpToken::Blez),
        ("bltz", RV32IOpToken::Bltz),
        ("bnez", RV32IOpToken::Bnez),
        ("call", RV32IOpToken::Call),
        ("csrc", RV32IOpToken::Csrc),
        ("csrci", RV32IOpToken::Csrci),
        ("csrr", RV32IOpToken::Csrr),
        ("csrs", RV32IOpToken::Csrs),
        ("csrsi", RV32IOpToken::Csrsi),
        ("csrw", RV32IOpToken::Csrw),
        ("csrwi", RV32IOpToken::Csrwi),
        ("j", RV32IOpToken::J),
        ("jr", RV32IOpToken::Jr),
        ("la", RV32IOpToken::La),
        ("li", RV32IOpToken::Li),
        ("mv", RV32IOpToken::Mv),
        ("neg", RV32IOpToken::Neg),
        ("nop", RV32IOpToken::Nop),
        ("not", RV32IOpToken::Not),
        ("rdcycle", RV32IOpToken::Rdcycle),
        ("rdcycleh", RV32IOpToken::Rdcycleh),
        ("rdinstret", RV32IOpToken::Rdinstret),
        ("rdinstreth", RV32IOpToken::Rdinstreth),
        ("rdtime", RV32IOpToken::Rdtime),
        ("rdtimeh", RV32IOpToken::Rdtimeh),
        ("ret", RV32IOpToken::Ret),
        ("seqz", RV32IOpToken::Seqz),
        ("sext.b", RV32IOpToken::SextB),
        ("sext.h", RV32IOpToken::SextH),
        ("sgt", RV32IOpToken::Sgt),
        ("sgtu", RV32IOpToken::Sgtu),
        ("sgtz", RV32IOpToken::Sgtz),
        ("sltz", RV32IOpToken::Sltz),
        ("snez", RV32IOpToken::Snez),
        ("tail", RV32IOpToken::Tail),
        ("zext.b", RV32IOpToken::ZextB),
        ("zext.h", RV32IOpToken::ZextH),
    ]);
}

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RV32IOpToken {
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

impl RISCVOpToken for RV32IOpToken {
    fn get_opd_set(&self) -> &Vec<RISCVOpdSet> {
        &OP_LIST.get(self).unwrap()
    }
}
