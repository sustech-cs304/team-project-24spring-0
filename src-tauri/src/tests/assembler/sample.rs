use crate::assembler::assemble::*;
use crate::interface::assembler::{AssembleError, Assembler, Instruction};
use crate::modules::riscv::basic::interface::parser::*;

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
    let mut q = RiscVAssembler::new();
    let res = q.dump(p.parse(rope.clone().to_string()));
    match res {
        Ok(res) => {
            println!("{:?}", res.data);
            println!("Data.length: {}", res.data.len());
            println!("{:?}", res.text);
            println!("Text.length: {}", res.text.len());
        }
        Err(err) => {
            for e in err {
                println!("{}", e.to_string());
            }
        }
    }
    let mut ast = p.parse(rope.clone().to_string()).unwrap();
    for (index, element) in ast.text.iter().enumerate() {
        let res = q.assemble(element, index);
        match res {
            Ok(res) => {
                print!("{:3}: {:?} ", index, res.op);
                for ins in res.ins {
                    print!("{}", ins.to_string());
                }
                println!();
            }
            Err(err) => {
                for e in err {
                    println!("{}", e.to_string());
                }
            }
        }
    }
}
