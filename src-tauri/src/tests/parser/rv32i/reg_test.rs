use crate::modules::riscv::basic::interface::parser::*;
use crate::modules::riscv::rv32i::constants::*;
use std::str::FromStr;

static VALID_REG_NAME: [&'static str; 65] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6", "fp", "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11",
    "x12", "x13", "x14", "x15", "x16", "x17", "x18", "x19", "x20", "x21", "x22", "x23", "x24",
    "x25", "x26", "x27", "x28", "x29", "x30", "x31",
];

#[test]
pub fn test() {
    let mut parser = RISCVParser::new(&vec![RISCVExtension::RV32I]);
    for i in 0..VALID_REG_NAME.len() {
        for j in 0..VALID_REG_NAME.len() {
            for k in 0..VALID_REG_NAME.len() {
                let expect = ParserResult::<RISCV> {
                    data: vec![],
                    text: vec![ParserResultText::Text(ParserInst::<RISCV> {
                        line: 0,
                        op: RV32IInstruction::Add.into(),
                        opd: vec![
                            ParserRISCVInstOpd::Reg(
                                RV32IRegister::from_str(VALID_REG_NAME[i]).unwrap().into(),
                            ),
                            ParserRISCVInstOpd::Reg(
                                RV32IRegister::from_str(VALID_REG_NAME[j]).unwrap().into(),
                            ),
                            ParserRISCVInstOpd::Reg(
                                RV32IRegister::from_str(VALID_REG_NAME[k]).unwrap().into(),
                            ),
                        ],
                    })],
                };

                let res = parser
                    .parse(format!(
                        "add {}, {}, {}",
                        VALID_REG_NAME[i], VALID_REG_NAME[j], VALID_REG_NAME[k]
                    ))
                    .unwrap();

                assert_eq!(res, expect);
            }
        }
    }
}
