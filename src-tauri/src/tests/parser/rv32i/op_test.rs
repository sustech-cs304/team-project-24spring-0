use ParserRISCVInstOpd::*;
use ParserRISCVLabel::*;
use ParserRISCVLabelHandler::*;
use RV32IInstruction::*;
use RV32IRegister::*;

use crate::modules::riscv::basic::interface::parser::*;

macro_rules! reg {
    () => {
        Reg(A0.into())
    };
    ($reg:ident) => {
        Reg($reg.into())
    };
}

macro_rules! imm {
    () => {
        Imm(ParserRISCVImmediate::Imm(1))
    };
    ($imm:expr) => {
        Imm(ParserRISCVImmediate::Imm($imm))
    };
}

macro_rules! lbl {
    () => {
        Lbl(Text(0))
    };
}

macro_rules! lbl_low {
    () => {
        Imm(ParserRISCVImmediate::Lbl((Text(0), Low)))
    };
}

macro_rules! lbl_high {
    () => {
        Imm(ParserRISCVImmediate::Lbl((Text(0), High)))
    };
}

macro_rules! lbl_delta_high {
    () => {
        Imm(ParserRISCVImmediate::Lbl((Text(0), DeltaHigh)))
    };
}

macro_rules! lbl_delta_low {
    () => {
        Imm(ParserRISCVImmediate::Lbl((Text(0), DeltaMinusOneLow)))
    };
}

macro_rules! expect_helper {
    ($op:expr, $($opd:expr),*) => {
        ParserResult::<RISCV> {
            data: vec![],
            text: vec![ParserResultText::Text(ParserInst::<RISCV> {
                line: 0,
                op: $op.into(),
                opd: vec![$($opd),*],
            })],
        }
    };
}

macro_rules! test {
    ($expect:expr, $code:expr, $parser:expr) => {
        let res = $parser.parse(&$code.to_string()).unwrap();
        assert_eq!(res, $expect);
    };
}

macro_rules! test_load_mem {
    ($inst:expr, $name:expr, $parser:expr) => {
        let expect = expect_helper!($inst, reg!(), imm!(), reg!());
        test!(expect, concat!($name, " a0, 1(a0)"), $parser);
        let expect = expect_helper!($inst, reg!(), imm!(0), reg!());
        test!(expect, concat!($name, " a0, (a0)"), $parser);
        let expect = expect_helper!($inst, reg!(), imm!(), reg!(Zero));
        test!(expect, concat!($name, " a0, 1"), $parser);
        let expect = ParserResult::<RISCV> {
            data: vec![],
            text: vec![
                ParserResultText::Text(ParserInst::<RISCV> {
                    line: 0,
                    op: Lui.into(),
                    opd: vec![reg!(), imm!(0x1000)],
                }),
                ParserResultText::Text(ParserInst::<RISCV> {
                    line: 0,
                    op: $inst.into(),
                    opd: vec![reg!(), imm!(-1), reg!()],
                }),
            ],
        };
        test!(expect, concat!($name, " a0, 0xffffff"), $parser);
        let expect = ParserResult::<RISCV> {
            data: vec![],
            text: vec![
                ParserResultText::Text(ParserInst::<RISCV> {
                    line: 0,
                    op: Auipc.into(),
                    opd: vec![reg!(), lbl_delta_high!()],
                }),
                ParserResultText::Text(ParserInst::<RISCV> {
                    line: 0,
                    op: $inst.into(),
                    opd: vec![reg!(), lbl_delta_low!(), reg!()],
                }),
            ],
        };
        test!(expect, concat!("a:", $name, " a0, a"), $parser);
    };
}

macro_rules! test_store_mem {
    ($inst:expr, $name:expr, $parser:expr) => {
        let expect = expect_helper!($inst, reg!(), imm!(), reg!());
        test!(expect, concat!($name, " a0, 1(a0)"), $parser);
        let expect = expect_helper!($inst, reg!(), imm!(0), reg!());
        test!(expect, concat!($name, " a0, (a0)"), $parser);
        let expect = expect_helper!($inst, reg!(), imm!(), reg!(Zero));
        test!(expect, concat!($name, " a0, 1"), $parser);
        let expect = ParserResult::<RISCV> {
            data: vec![],
            text: vec![
                ParserResultText::Text(ParserInst::<RISCV> {
                    line: 0,
                    op: Lui.into(),
                    opd: vec![reg!(A1), imm!(0x1000)],
                }),
                ParserResultText::Text(ParserInst::<RISCV> {
                    line: 0,
                    op: $inst.into(),
                    opd: vec![reg!(A0), imm!(-1), reg!(A1)],
                }),
            ],
        };
        test!(expect, concat!($name, " a0, 0xffffff, a1"), $parser);
        let expect = ParserResult::<RISCV> {
            data: vec![],
            text: vec![
                ParserResultText::Text(ParserInst::<RISCV> {
                    line: 0,
                    op: Auipc.into(),
                    opd: vec![reg!(A1), lbl_delta_high!()],
                }),
                ParserResultText::Text(ParserInst::<RISCV> {
                    line: 0,
                    op: $inst.into(),
                    opd: vec![reg!(A0), lbl_delta_low!(), reg!(A1)],
                }),
            ],
        };
        test!(expect, concat!("a:", $name, " a0, a, a1"), $parser);
    };
}

#[test]
pub fn test() {
    let mut parser = RISCVParser::new(&vec![RISCVExtension::RV32I]);

    let expect = expect_helper!(Add, reg!(), reg!(), reg!());
    test!(expect, "add a0, a0, a0", parser);

    let expect = expect_helper!(Addi, reg!(), reg!(), imm!());
    test!(expect, "addi a0, a0, 1", parser);
    let expect = expect_helper!(Addi, reg!(), reg!(), lbl_low!());
    test!(expect, "a: addi a0, a0, a", parser);

    let expect = expect_helper!(And, reg!(), reg!(), reg!());
    test!(expect, "and a0, a0, a0", parser);

    let expect = expect_helper!(Andi, reg!(), reg!(), imm!());
    test!(expect, "andi a0, a0, 1", parser);

    let expect = expect_helper!(Auipc, reg!(), imm!());
    test!(expect, "auipc a0, 1", parser);

    let expect = expect_helper!(Beq, reg!(), reg!(), lbl!());
    test!(expect, "a: beq a0, a0, a", parser);

    let expect = expect_helper!(Bge, reg!(), reg!(), lbl!());
    test!(expect, "a: bge a0, a0, a", parser);

    let expect = expect_helper!(Bgeu, reg!(), reg!(), lbl!());
    test!(expect, "a: bgeu a0, a0, a", parser);

    let expect = expect_helper!(Blt, reg!(), reg!(), lbl!());
    test!(expect, "a: blt a0, a0, a", parser);

    let expect = expect_helper!(Bltu, reg!(), reg!(), lbl!());
    test!(expect, "a: bltu a0, a0, a", parser);

    let expect = expect_helper!(Bne, reg!(), reg!(), lbl!());
    test!(expect, "a: bne a0, a0, a", parser);

    // no csr test

    let expect = expect_helper!(Ebreak,);
    test!(expect, "ebreak", parser);

    let expect = expect_helper!(Ecall,);
    test!(expect, "ecall", parser);

    // let expect = expect_helper!(Fence, imm!(), imm!());
    // test!(expect, "fence", parser);

    // let expect = expect_helper!(FenceI,);
    // test!(expect, "fence.i", parser);

    let expect = expect_helper!(Jal, reg!(Ra), lbl!());
    test!(expect, "a: jal a", parser);
    let expect = expect_helper!(Jal, reg!(), lbl!());
    test!(expect, "a: jal a0, a", parser);

    let expect = expect_helper!(Jalr, reg!(), reg!(), imm!());
    test!(expect, "jalr a0, a0, 1", parser);
    let expect = expect_helper!(Jalr, reg!(Ra), reg!(), imm!(0));
    test!(expect, "jalr a0", parser);
    let expect = expect_helper!(Jalr, reg!(Ra), reg!(), imm!());
    test!(expect, "jalr a0, 1", parser);
    let expect = expect_helper!(Jalr, reg!(), reg!(), imm!());
    test!(expect, "jalr a0, 1(a0)", parser);

    test_load_mem!(Lb, "lb", parser);
    test_load_mem!(Lbu, "lbu", parser);
    test_load_mem!(Lh, "lh", parser);
    test_load_mem!(Lhu, "lhu", parser);

    let expect = expect_helper!(Lui, reg!(), imm!());
    test!(expect, "lui a0, 1", parser);
    let expect = expect_helper!(Lui, reg!(), lbl_high!());
    test!(expect, "a: lui a0, a", parser);

    test_load_mem!(Lw, "lw", parser);

    let expect = expect_helper!(Or, reg!(), reg!(), reg!());
    test!(expect, "or a0, a0, a0", parser);

    let expect = expect_helper!(Ori, reg!(), reg!(), imm!());
    test!(expect, "ori a0, a0, 1", parser);

    test_store_mem!(Sb, "sb", parser);
    test_store_mem!(Sh, "sh", parser);

    let expect = expect_helper!(Sll, reg!(), reg!(), reg!());
    test!(expect, "sll a0, a0, a0", parser);

    let expect = expect_helper!(Slli, reg!(), reg!(), imm!());
    test!(expect, "slli a0, a0, 1", parser);

    let expect = expect_helper!(Slt, reg!(), reg!(), reg!());
    test!(expect, "slt a0, a0, a0", parser);

    let expect = expect_helper!(Slti, reg!(), reg!(), imm!());
    test!(expect, "slti a0, a0, 1", parser);

    let expect = expect_helper!(Sltiu, reg!(), reg!(), imm!());
    test!(expect, "sltiu a0, a0, 1", parser);

    let expect = expect_helper!(Sltu, reg!(), reg!(), reg!());
    test!(expect, "sltu a0, a0, a0", parser);

    let expect = expect_helper!(Sra, reg!(), reg!(), reg!());
    test!(expect, "sra a0, a0, a0", parser);

    let expect = expect_helper!(Srai, reg!(), reg!(), imm!());
    test!(expect, "srai a0, a0, 1", parser);

    let expect = expect_helper!(Srl, reg!(), reg!(), reg!());
    test!(expect, "srl a0, a0, a0", parser);

    let expect = expect_helper!(Srli, reg!(), reg!(), imm!());
    test!(expect, "srli a0, a0, 1", parser);

    let expect = expect_helper!(Sub, reg!(), reg!(), reg!());
    test!(expect, "sub a0, a0, a0", parser);

    test_store_mem!(Sw, "sw", parser);

    let expect = expect_helper!(Xor, reg!(), reg!(), reg!());
    test!(expect, "xor a0, a0, a0", parser);

    let expect = expect_helper!(Xori, reg!(), reg!(), imm!());
    test!(expect, "xori a0, a0, 1", parser);

    let expect = expect_helper!(Jal, reg!(Zero), lbl!());
    test!(expect, "a: b a", parser);

    let expect = expect_helper!(Beq, reg!(), reg!(Zero), lbl!());
    test!(expect, "a: beqz a0, a", parser);

    let expect = expect_helper!(Bge, reg!(), reg!(Zero), lbl!());
    test!(expect, "a: bgez a0, a", parser);

    let expect = expect_helper!(Blt, reg!(A1), reg!(A0), lbl!());
    test!(expect, "a: bgt a0, a1, a", parser);

    let expect = expect_helper!(Bltu, reg!(A1), reg!(A0), lbl!());
    test!(expect, "a: bgtu a0, a1, a", parser);

    let expect = expect_helper!(Blt, reg!(Zero), reg!(), lbl!());
    test!(expect, "a: bgtz a0, a", parser);

    let expect = expect_helper!(Bge, reg!(A1), reg!(A0), lbl!());
    test!(expect, "a: ble a0, a1, a", parser);

    let expect = expect_helper!(Bgeu, reg!(A1), reg!(A0), lbl!());
    test!(expect, "a: bleu a0, a1, a", parser);

    let expect = expect_helper!(Bge, reg!(Zero), reg!(), lbl!());
    test!(expect, "a: blez a0, a", parser);

    let expect = expect_helper!(Blt, reg!(), reg!(Zero), lbl!());
    test!(expect, "a: bltz a0, a", parser);

    let expect = expect_helper!(Bne, reg!(), reg!(Zero), lbl!());
    test!(expect, "a: bnez a0, a", parser);

    let expect = ParserResult::<RISCV> {
        data: vec![],
        text: vec![
            ParserResultText::Text(ParserInst::<RISCV> {
                line: 0,
                op: Auipc.into(),
                opd: vec![reg!(T1), lbl_delta_high!()],
            }),
            ParserResultText::Text(ParserInst::<RISCV> {
                line: 0,
                op: Jalr.into(),
                opd: vec![reg!(Ra), reg!(T1), lbl_delta_low!()],
            }),
        ],
    };
    test!(expect, "a: call a", parser);

    // no csr test

    let expect = expect_helper!(Jal, reg!(Zero), lbl!());
    test!(expect, "a: j a", parser);

    let expect = expect_helper!(Jalr, reg!(Zero), reg!(), imm!(0));
    test!(expect, "jr a0", parser);
    let expect = expect_helper!(Jalr, reg!(Zero), reg!(), imm!());
    test!(expect, "jr a0, 1", parser);

    let expect = ParserResult::<RISCV> {
        data: vec![],
        text: vec![
            ParserResultText::Text(ParserInst::<RISCV> {
                line: 0,
                op: Auipc.into(),
                opd: vec![reg!(), lbl_delta_high!()],
            }),
            ParserResultText::Text(ParserInst::<RISCV> {
                line: 0,
                op: Addi.into(),
                opd: vec![reg!(), reg!(), lbl_delta_low!()],
            }),
        ],
    };
    test!(expect, "a: la a0, a", parser);

    let expect = expect_helper!(Addi, reg!(), reg!(Zero), imm!());
    test!(expect, "li a0, 1", parser);
    let expect = ParserResult::<RISCV> {
        data: vec![],
        text: vec![
            ParserResultText::Text(ParserInst::<RISCV> {
                line: 0,
                op: Lui.into(),
                opd: vec![reg!(), imm!(0x1000)],
            }),
            ParserResultText::Text(ParserInst::<RISCV> {
                line: 0,
                op: Addi.into(),
                opd: vec![reg!(), reg!(), imm!(-1)],
            }),
        ],
    };
    test!(expect, "li a0, 0xffffff", parser);

    let expect = expect_helper!(Add, reg!(A0), reg!(Zero), reg!(A1));
    test!(expect, "mv a0, a1", parser);

    let expect = expect_helper!(Sub, reg!(A0), reg!(Zero), reg!(A1));
    test!(expect, "neg a0, a1", parser);

    let expect = expect_helper!(Addi, reg!(Zero), reg!(Zero), imm!(0));
    test!(expect, "nop", parser);

    let expect = expect_helper!(Xori, reg!(A0), reg!(A1), imm!(-1));
    test!(expect, "not a0, a1", parser);

    let expect = expect_helper!(Jalr, reg!(Zero), reg!(Ra), imm!(0));
    test!(expect, "ret", parser);

    let expect = expect_helper!(Sltiu, reg!(A0), reg!(A1), imm!(1));
    test!(expect, "seqz a0, a1", parser);

    let expect = ParserResult::<RISCV> {
        data: vec![],
        text: vec![
            ParserResultText::Text(ParserInst::<RISCV> {
                line: 0,
                op: Slli.into(),
                opd: vec![reg!(A0), reg!(A1), imm!(24)],
            }),
            ParserResultText::Text(ParserInst::<RISCV> {
                line: 0,
                op: Srai.into(),
                opd: vec![reg!(A0), reg!(A0), imm!(24)],
            }),
        ],
    };
    test!(expect, "sext.b a0, a1", parser);

    let expect = ParserResult::<RISCV> {
        data: vec![],
        text: vec![
            ParserResultText::Text(ParserInst::<RISCV> {
                line: 0,
                op: Slli.into(),
                opd: vec![reg!(A0), reg!(A1), imm!(16)],
            }),
            ParserResultText::Text(ParserInst::<RISCV> {
                line: 0,
                op: Srai.into(),
                opd: vec![reg!(A0), reg!(A0), imm!(16)],
            }),
        ],
    };
    test!(expect, "sext.h a0, a1", parser);

    let expect = expect_helper!(Slt, reg!(A0), reg!(A2), reg!(A1));
    test!(expect, "sgt a0, a1, a2", parser);

    let expect = expect_helper!(Sltu, reg!(A0), reg!(A2), reg!(A1));
    test!(expect, "sgtu a0, a1, a2", parser);

    let expect = expect_helper!(Slt, reg!(A0), reg!(Zero), reg!(A1));
    test!(expect, "sgtz a0, a1", parser);

    let expect = expect_helper!(Slt, reg!(A0), reg!(A1), reg!(Zero));
    test!(expect, "sltz a0, a1", parser);

    let expect = expect_helper!(Sltu, reg!(A0), reg!(Zero), reg!(A1));
    test!(expect, "snez a0, a1", parser);

    let expect = ParserResult::<RISCV> {
        data: vec![],
        text: vec![
            ParserResultText::Text(ParserInst::<RISCV> {
                line: 0,
                op: Auipc.into(),
                opd: vec![reg!(T1), lbl_delta_high!()],
            }),
            ParserResultText::Text(ParserInst::<RISCV> {
                line: 0,
                op: Jalr.into(),
                opd: vec![reg!(Zero), reg!(T1), lbl_delta_low!()],
            }),
        ],
    };
    test!(expect, "a: tail a", parser);

    let expect = ParserResult::<RISCV> {
        data: vec![],
        text: vec![
            ParserResultText::Text(ParserInst::<RISCV> {
                line: 0,
                op: Slli.into(),
                opd: vec![reg!(A0), reg!(A1), imm!(24)],
            }),
            ParserResultText::Text(ParserInst::<RISCV> {
                line: 0,
                op: Srli.into(),
                opd: vec![reg!(A0), reg!(A0), imm!(24)],
            }),
        ],
    };
    test!(expect, "zext.b a0, a1", parser);

    let expect = ParserResult::<RISCV> {
        data: vec![],
        text: vec![
            ParserResultText::Text(ParserInst::<RISCV> {
                line: 0,
                op: Slli.into(),
                opd: vec![reg!(A0), reg!(A1), imm!(16)],
            }),
            ParserResultText::Text(ParserInst::<RISCV> {
                line: 0,
                op: Srli.into(),
                opd: vec![reg!(A0), reg!(A0), imm!(16)],
            }),
        ],
    };
    test!(expect, "zext.h a0, a1", parser);
}
