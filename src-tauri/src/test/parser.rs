use crate::interface::parser::Parser;
use crate::parser::parser::RISCVParser;

pub fn test_parser() {
    let mut p = RISCVParser::new();
    let rope = ropey::Rope::from_str(
        ".macro i (%a)
    add a2,a3,a1
    .end_macro
    .data
    .global a 
    a : .string \"1a\"
    .text
    Li :
        nop
        la a0, aaa
    c :
        i a2
    d :
        # 1341
        addi a0, a1, 10
    .data
    aaa: .string \"11\"
    addi a0, a1, 1
    .text
    addi a0, a1, 1
    .data
    la a0, a
    .word
    12 123 123 12",
    );
    let res = p.parse(&rope);
    println!("{:?}", res);
}
