use crate::interface::assembler::{Instruction, Operand};
use crate::modules::riscv::rv32i::constants::{RISCVImmediate, RV32IInstruction, RV32IRegister};
use crate::simulator::cpu;

#[test]
fn test_R() {
    let mut cpu = cpu::CPU::new(16);
    cpu.execute(Instruction {
        op: RV32IInstruction::Addi,
        ins: vec![
            Operand::Reg(RV32IRegister::A1),
            Operand::Reg(RV32IRegister::Zero),
            Operand::Operator(20),
        ],
    })
    .unwrap();
    cpu.execute(Instruction {
        op: RV32IInstruction::Addi,
        ins: vec![
            Operand::Reg(RV32IRegister::A2),
            Operand::Reg(RV32IRegister::Zero),
            Operand::Operator(3),
        ],
    })
    .unwrap();
    cpu.execute(Instruction {
        op: RV32IInstruction::Add,
        ins: vec![
            Operand::Reg(RV32IRegister::S1),
            Operand::Reg(RV32IRegister::A1),
            Operand::Reg(RV32IRegister::A2),
        ],
    })
    .unwrap();
    cpu.execute(Instruction {
        op: RV32IInstruction::Sub,
        ins: vec![
            Operand::Reg(RV32IRegister::S2),
            Operand::Reg(RV32IRegister::S1),
            Operand::Reg(RV32IRegister::A1),
        ],
    })
    .unwrap();
    cpu.execute(Instruction {
        op: RV32IInstruction::Xor,
        ins: vec![
            Operand::Reg(RV32IRegister::S3),
            Operand::Reg(RV32IRegister::S2),
            Operand::Reg(RV32IRegister::S1),
        ],
    })
    .unwrap();
    cpu.execute(Instruction {
        op: RV32IInstruction::Or,
        ins: vec![
            Operand::Reg(RV32IRegister::S4),
            Operand::Reg(RV32IRegister::S2),
            Operand::Reg(RV32IRegister::S1),
        ],
    })
    .unwrap();
    cpu.execute(Instruction {
        op: RV32IInstruction::And,
        ins: vec![
            Operand::Reg(RV32IRegister::S5),
            Operand::Reg(RV32IRegister::S2),
            Operand::Reg(RV32IRegister::S1),
        ],
    })
    .unwrap();
    cpu.execute(Instruction {
        op: RV32IInstruction::Sll,
        ins: vec![
            Operand::Reg(RV32IRegister::S6),
            Operand::Reg(RV32IRegister::S2),
            Operand::Reg(RV32IRegister::S1),
        ],
    })
    .unwrap();
    cpu.execute(Instruction {
        op: RV32IInstruction::Srl,
        ins: vec![
            Operand::Reg(RV32IRegister::S7),
            Operand::Reg(RV32IRegister::A1),
            Operand::Reg(RV32IRegister::A2),
        ],
    })
    .unwrap();
    cpu.execute(Instruction {
        op: RV32IInstruction::Sra,
        ins: vec![
            Operand::Reg(RV32IRegister::S8),
            Operand::Reg(RV32IRegister::A1),
            Operand::Reg(RV32IRegister::A2),
        ],
    })
    .unwrap();
    cpu.execute(Instruction {
        op: RV32IInstruction::Slt,
        ins: vec![
            Operand::Reg(RV32IRegister::S9),
            Operand::Reg(RV32IRegister::A2),
            Operand::Reg(RV32IRegister::A1),
        ],
    })
    .unwrap();
    cpu.execute(Instruction {
        op: RV32IInstruction::Sltu,
        ins: vec![
            Operand::Reg(RV32IRegister::A3),
            Operand::Reg(RV32IRegister::A2),
            Operand::Reg(RV32IRegister::A1),
        ],
    })
    .unwrap();
    cpu.print_registers();
}

#[test]
fn test_B() {
    let mut cpu = cpu::CPU::new(16);

    cpu.execute(Instruction {
        op: RV32IInstruction::Addi,
        ins: vec![
            Operand::Reg(RV32IRegister::A1),
            Operand::Reg(RV32IRegister::Zero),
            Operand::Operator(19),
        ],
    })
    .unwrap();
    cpu.execute(Instruction {
        op: RV32IInstruction::Addi,
        ins: vec![
            Operand::Reg(RV32IRegister::A2),
            Operand::Reg(RV32IRegister::Zero),
            Operand::Operator(5),
        ],
    })
    .unwrap();
    cpu.execute(Instruction {
        op: RV32IInstruction::Addi,
        ins: vec![
            Operand::Reg(RV32IRegister::A3),
            Operand::Reg(RV32IRegister::Zero),
            Operand::Operator(5),
        ],
    })
    .unwrap();

    cpu.execute(Instruction {
        op: RV32IInstruction::Beq,
        ins: vec![
            Operand::Reg(RV32IRegister::A2),
            Operand::Reg(RV32IRegister::A1),
            Operand::Operator(3),
        ],
    })
    .unwrap();
    cpu.execute(Instruction {
        op: RV32IInstruction::Bne,
        ins: vec![
            Operand::Reg(RV32IRegister::A1),
            Operand::Reg(RV32IRegister::A1),
            Operand::Operator(3),
        ],
    })
    .unwrap();
    cpu.execute(Instruction {
        op: RV32IInstruction::Blt,
        ins: vec![
            Operand::Reg(RV32IRegister::A1),
            Operand::Reg(RV32IRegister::A2),
            Operand::Operator(3),
        ],
    })
    .unwrap();
    cpu.execute(Instruction {
        op: RV32IInstruction::Bge,
        ins: vec![
            Operand::Reg(RV32IRegister::A1),
            Operand::Reg(RV32IRegister::A2),
            Operand::Operator(3),
        ],
    })
    .unwrap();
    cpu.execute(Instruction {
        op: RV32IInstruction::Bltu,
        ins: vec![
            Operand::Reg(RV32IRegister::A1),
            Operand::Reg(RV32IRegister::A2),
            Operand::Operator(3),
        ],
    })
    .unwrap();
    cpu.execute(Instruction {
        op: RV32IInstruction::Bgeu,
        ins: vec![
            Operand::Reg(RV32IRegister::A1),
            Operand::Reg(RV32IRegister::A2),
            Operand::Operator(3),
        ],
    })
    .unwrap();
    cpu.print_registers();
}

#[test]
fn test_J() {
    let mut cpu = cpu::CPU::new(16);

    cpu.execute(Instruction {
        op: RV32IInstruction::Addi,
        ins: vec![
            Operand::Reg(RV32IRegister::A1),
            Operand::Reg(RV32IRegister::Zero),
            Operand::Operator(19),
        ],
    })
    .unwrap();
    cpu.execute(Instruction {
        op: RV32IInstruction::Addi,
        ins: vec![
            Operand::Reg(RV32IRegister::A2),
            Operand::Reg(RV32IRegister::Zero),
            Operand::Operator(5),
        ],
    })
    .unwrap();
    cpu.execute(Instruction {
        op: RV32IInstruction::Jal,
        ins: vec![Operand::Reg(RV32IRegister::A3), Operand::Operator(3)],
    })
    .unwrap();
    cpu.print_registers();
}

#[test]
fn test_I1() {
    let mut cpu = cpu::CPU::new(16);

    cpu.execute(Instruction {
        op: RV32IInstruction::Addi,
        ins: vec![
            Operand::Reg(RV32IRegister::A1),
            Operand::Reg(RV32IRegister::Zero),
            Operand::Operator(19),
        ],
    })
    .unwrap();

    cpu.execute(Instruction {
        op: RV32IInstruction::Addi,
        ins: vec![
            Operand::Reg(RV32IRegister::S2),
            Operand::Reg(RV32IRegister::Zero),
            Operand::Operator(6),
        ],
    })
    .unwrap();

    cpu.execute(Instruction {
        op: RV32IInstruction::Xori,
        ins: vec![
            Operand::Reg(RV32IRegister::A2),
            Operand::Reg(RV32IRegister::S2),
            Operand::Operator(6),
        ],
    })
    .unwrap();

    cpu.execute(Instruction {
        op: RV32IInstruction::Ori,
        ins: vec![
            Operand::Reg(RV32IRegister::A3),
            Operand::Reg(RV32IRegister::S2),
            Operand::Operator(6),
        ],
    })
    .unwrap();

    cpu.execute(Instruction {
        op: RV32IInstruction::Andi,
        ins: vec![
            Operand::Reg(RV32IRegister::A4),
            Operand::Reg(RV32IRegister::S2),
            Operand::Operator(30),
        ],
    })
    .unwrap();

    cpu.execute(Instruction {
        op: RV32IInstruction::Slti,
        ins: vec![
            Operand::Reg(RV32IRegister::A5),
            Operand::Reg(RV32IRegister::S2),
            Operand::Operator(45),
        ],
    })
    .unwrap();

    cpu.execute(Instruction {
        op: RV32IInstruction::Sltiu,
        ins: vec![
            Operand::Reg(RV32IRegister::A6),
            Operand::Reg(RV32IRegister::S2),
            Operand::Operator(25),
        ],
    })
    .unwrap();

    cpu.execute(Instruction {
        op: RV32IInstruction::Slli,
        ins: vec![
            Operand::Reg(RV32IRegister::A7),
            Operand::Reg(RV32IRegister::S2),
            Operand::Operator(15),
        ],
    })
    .unwrap();

    cpu.execute(Instruction {
        op: RV32IInstruction::Srli,
        ins: vec![
            Operand::Reg(RV32IRegister::S8),
            Operand::Reg(RV32IRegister::S2),
            Operand::Operator(11),
        ],
    })
    .unwrap();

    cpu.execute(Instruction {
        op: RV32IInstruction::Srai,
        ins: vec![
            Operand::Reg(RV32IRegister::S9),
            Operand::Reg(RV32IRegister::S2),
            Operand::Operator(5),
        ],
    })
    .unwrap();

    cpu.execute(Instruction {
        op: RV32IInstruction::Jalr,
        ins: vec![
            Operand::Reg(RV32IRegister::S10),
            Operand::Reg(RV32IRegister::S2),
            Operand::Operator(6),
        ],
    })
    .unwrap();
    cpu.print_registers();
}

#[test]
fn test_I2() {
    let mut cpu = cpu::CPU::new(16);

    cpu.execute(Instruction {
        op: RV32IInstruction::Addi,
        ins: vec![
            Operand::Reg(RV32IRegister::A1),
            Operand::Reg(RV32IRegister::Zero),
            Operand::Operator(20),
        ],
    })
    .unwrap();

    cpu.execute(Instruction {
        op: RV32IInstruction::Sw,
        ins: vec![
            Operand::Reg(RV32IRegister::S1),
            Operand::Reg(RV32IRegister::A1),
            Operand::Operator(6),
        ],
    })
    .unwrap();

    cpu.execute(Instruction {
        op: RV32IInstruction::Lb,
        ins: vec![
            Operand::Reg(RV32IRegister::A1),
            Operand::Reg(RV32IRegister::S1),
            Operand::Operator(6),
        ],
    })
    .unwrap();

    cpu.execute(Instruction {
        op: RV32IInstruction::Lh,
        ins: vec![
            Operand::Reg(RV32IRegister::A2),
            Operand::Reg(RV32IRegister::S1),
            Operand::Operator(5),
        ],
    })
    .unwrap();

    cpu.execute(Instruction {
        op: RV32IInstruction::Lw,
        ins: vec![
            Operand::Reg(RV32IRegister::A3),
            Operand::Reg(RV32IRegister::S1),
            Operand::Operator(4),
        ],
    })
    .unwrap();

    cpu.execute(Instruction {
        op: RV32IInstruction::Lbu,
        ins: vec![
            Operand::Reg(RV32IRegister::A4),
            Operand::Reg(RV32IRegister::S1),
            Operand::Operator(5),
        ],
    })
    .unwrap();

    cpu.execute(Instruction {
        op: RV32IInstruction::Lhu,
        ins: vec![
            Operand::Reg(RV32IRegister::A5),
            Operand::Reg(RV32IRegister::S1),
            Operand::Operator(6),
        ],
    })
    .unwrap();

    cpu.print_registers();
}

#[test]
fn test_S() {
    let mut cpu = cpu::CPU::new(16);

    cpu.execute(Instruction {
        op: RV32IInstruction::Addi,
        ins: vec![
            Operand::Reg(RV32IRegister::A1),
            Operand::Reg(RV32IRegister::Zero),
            Operand::Operator(19),
        ],
    })
    .unwrap();

    cpu.execute(Instruction {
        op: RV32IInstruction::Addi,
        ins: vec![
            Operand::Reg(RV32IRegister::A3),
            Operand::Reg(RV32IRegister::Zero),
            Operand::Operator(5),
        ],
    })
    .unwrap();

    cpu.execute(Instruction {
        op: RV32IInstruction::Sb,
        ins: vec![
            Operand::Reg(RV32IRegister::S1),
            Operand::Reg(RV32IRegister::A1),
            Operand::Operator(5),
        ],
    })
    .unwrap();

    cpu.execute(Instruction {
        op: RV32IInstruction::Sh,
        ins: vec![
            Operand::Reg(RV32IRegister::A3),
            Operand::Reg(RV32IRegister::A1),
            Operand::Operator(6),
        ],
    })
    .unwrap();

    cpu.execute(Instruction {
        op: RV32IInstruction::Sw,
        ins: vec![
            Operand::Reg(RV32IRegister::S5),
            Operand::Reg(RV32IRegister::A1),
            Operand::Operator(6),
        ],
    })
    .unwrap();

    cpu.print_registers();
}

#[test]
fn test_U() {
    let mut cpu = cpu::CPU::new(16);

    cpu.execute(Instruction {
        op: RV32IInstruction::Lui,
        ins: vec![Operand::Reg(RV32IRegister::A1), Operand::Operator(19)],
    })
    .unwrap();

    cpu.execute(Instruction {
        op: RV32IInstruction::Auipc,
        ins: vec![Operand::Reg(RV32IRegister::A2), Operand::Operator(3)],
    })
    .unwrap();

    cpu.print_registers();
}
