use std::fs;

use crate::{
    interface::assembler::Assembler,
    modules::riscv::basic::{assembler::assembler::RiscVAssembler, interface::parser::*},
};

#[test]
pub fn test_assembler() {
    let mut p = RISCVParser::new(&vec![RISCVExtension::RV32I]);
    let rope = ropey::Rope::from_str(
        "
        .data
n:
        .word   10
a:
        .word   416
        .word   8956
        .word   8764
        .word   1654
        .word   8654
        .word   6853478
        .word   8904
        .word   -408
        .word   -5
        .word   656
        .text
main:
        addi    sp,sp,-48
        sw      s0,44(sp)
        addi    s0,sp,48
        sw      a0,-36(s0)
        sw      a1,-40(s0)
        sw      zero,-20(s0)
        j       L2
L6:
        sw      zero,-24(s0)
        j       L3
L5:
        la      a5,a
        lw      a4,0(a5)
        fence   1,1
        lw      a5,-24(s0)
        slli    a5,a5,2
        add     a5,a4,a5
        lw      a4,0(a5)
        lw      a5,-24(s0)
        addi    a5,a5,1
        lw      a3,n
        ble     a4,a3,L4
        la      a4,a
        lw      a5,-24(s0)
        slli    a5,a5,2
        add     a5,a4,a5
        lw      a5,0(a5)
        sw      a5,-28(s0)
        lw      a5,-24(s0)
        addi    a5,a5,1
        la      a4,a
        slli    a5,a5,2
        add     a5,a4,a5
        lw      a4,0(a5)
        la      a5,a
        lw      a4,-24(s0)
        slli    a4,a4,2
        add     a4,a5,a4
        sw      a4,0(a5)
        lw      a5,-24(s0)
        addi    a5,a5,1
        la      a4,a
        slli    a5,a5,2
        add     a5,a4,a5
        lw      a4,-28(s0)
        sw      a4,0(a5)
L4:
        lw      a5,-24(s0)
        addi    a5,a5,1
        sw      a5,-24(s0)
L3:
        lw      a5,n
        addi    a5,a5,-1
        lw      a4,-24(s0)
        blt     a4,a5,L5
        lw      a5,-20(s0)
        addi    a5,a5,1
        sw      a5,-20(s0)
L2:
        lw      a5,n
        lw      a4,-20(s0)
        blt     a4,a5,L6
        lw      s0,44(sp)
        addi    sp,sp,48
        jalr    ra
",
    );
    let mut riscv_assembler = RiscVAssembler::new();
    let parse_result = p.parse(rope.clone().to_string());
    match parse_result {
        Ok(res) => {
            let dump_result = riscv_assembler.dump(res);
            match dump_result {
                Ok(res) => {
                    println!("{:?}", res.data);
                    println!("Data.length: {}", res.data.len());
                    println!("{:?}", res.text);
                    println!("Text.length: {}", res.text.len());
                    if let Err(e) = fs::write("output.txt", res.text) {
                        eprintln!("Error writing to file: {}", e);
                    } else {
                        println!("String has been written to output.txt successfully!");
                    }
                }
                Err(err) => {
                    for e in err {
                        println!("{}", e.to_string());
                    }
                }
            }
        }
        Err(err) => {
            for e in err {
                println!("{}", e.to_string());
            }
        }
    }
    let ast = p.parse(rope.clone().to_string()).unwrap();
    let assembled_result = riscv_assembler.assemble(ast);
    match assembled_result {
        Ok(res) => {
            for data in res.data {
                println!("0x{:08x}", data);
            }
            for instruction in res.instruction {
                println!("{}", instruction.to_string());
            }
        }
        Err(err) => {
            for e in err {
                println!("{}", e.to_string());
            }
        }
    }
}
