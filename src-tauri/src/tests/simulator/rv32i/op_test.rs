use crate::{
    interface::{assembler::*, simulator::Simulator},
    modules::riscv::{
        basic::interface::parser::{ParserRISCVInstOp, RISCV},
        rv32i::constants::{RISCVImmediate, RV32IInstruction},
    },
    simulator::simulator::*,
};

macro_rules! instruction_set {
    ($line_number:expr, $instruction:expr, $address:expr, $code:expr, $basic:expr) => {
        InstructionSet::<RISCV> {
            line_number: $line_number,
            instruction: Instruction::<RISCV> {
                operation: $instruction.operation,
                operands: $instruction.operands,
            },
            address: $address,
            code: $code,
            basic: $basic.to_string(),
        }
    };
}

macro_rules! instruction {
    ($op:expr, $rd:expr, $rs1:expr, $imm:expr) => {
        Instruction::<RISCV> {
            operation: $op,
            operands: Vec::from([$rd, $rs1, $imm]),
        }
    };
    ($op:expr, $rd:expr, $rs1:expr) => {
        Instruction::<RISCV> {
            operation: $op,
            operands: vec![$rd as RISCVImmediate, $rs1 as RISCVImmediate],
        }
    };
    ($op:expr) => {
        Instruction::<RISCV> {
            operation: $op,
            operands: vec![],
        }
    };
}

macro_rules! test {
    ($simulator:ident,$expect:ident) => {
        let out = $simulator.step();
        match out {
            Ok(()) => {}
            Err(error) => {
                println!("Error: {}", error);
                assert!(false);
            }
        }
        assert_eq!($simulator.get_register(), $expect);
    };
}

macro_rules! test_op {
    ($instruction:ident) => {
        let data = vec![0x00, 0x00, 0x00, 0x00];
        let assemble_result = AssembleResult {
            data: data,
            instruction: $instruction,
        };

        let mut simulator = RISCVSimulator::new("");
        let _ = simulator.load_inst(assemble_result);
        let mut expect = vec![0; 32];
        expect[2] = 0x7ffffffc;

        test!(simulator, expect);
    };
}

#[test]
pub fn test() {
    let instruction: Vec<InstructionSet<RISCV>> = vec![instruction_set!(
        1,
        instruction!(
            ParserRISCVInstOp::from(RV32IInstruction::Add),
            10 as RISCVImmediate,
            10 as RISCVImmediate,
            10 as RISCVImmediate
        ),
        0x00400000,
        0x00a50533,
        "add a0, a0, a0"
    )];
    test_op!(instruction);

    let instruction: Vec<InstructionSet<RISCV>> = vec![instruction_set!(
        1,
        instruction!(
            ParserRISCVInstOp::from(RV32IInstruction::And),
            10 as RISCVImmediate,
            10 as RISCVImmediate,
            10 as RISCVImmediate
        ),
        0x00400004,
        0x00a57533,
        "and a0, a0, a0"
    )];
    test_op!(instruction);

    let instruction: Vec<InstructionSet<RISCV>> = vec![instruction_set!(
        1,
        instruction!(
            ParserRISCVInstOp::from(RV32IInstruction::Or),
            10 as RISCVImmediate,
            10 as RISCVImmediate,
            10 as RISCVImmediate
        ),
        0x00400008,
        0x00c575b3,
        "or a0, a0, a0"
    )];
    test_op!(instruction);

    let instruction: Vec<InstructionSet<RISCV>> = vec![instruction_set!(
        1,
        instruction!(
            ParserRISCVInstOp::from(RV32IInstruction::Sltu),
            10 as RISCVImmediate,
            10 as RISCVImmediate,
            10 as RISCVImmediate
        ),
        0x0040000c,
        0x0015f613,
        "sltu a0, a0, a0"
    )];
    test_op!(instruction);

    let instruction: Vec<InstructionSet<RISCV>> = vec![instruction_set!(
        1,
        instruction!(
            ParserRISCVInstOp::from(RV32IInstruction::Sll),
            10 as RISCVImmediate,
            10 as RISCVImmediate,
            10 as RISCVImmediate
        ),
        0x00400010,
        0x00001517,
        "sll a0, a0, a0"
    )];
    test_op!(instruction);

    let instruction: Vec<InstructionSet<RISCV>> = vec![instruction_set!(
        1,
        instruction!(
            ParserRISCVInstOp::from(RV32IInstruction::Slt),
            10 as RISCVImmediate,
            10 as RISCVImmediate,
            10 as RISCVImmediate
        ),
        0x00400014,
        0x00a50063,
        "slt a0, a0, a0"
    )];
    test_op!(instruction);

    let instruction: Vec<InstructionSet<RISCV>> = vec![instruction_set!(
        1,
        instruction!(
            ParserRISCVInstOp::from(RV32IInstruction::Sra),
            10 as RISCVImmediate,
            10 as RISCVImmediate,
            10 as RISCVImmediate
        ),
        0x00400018,
        0xfea55ee3,
        "sra a0, a0, a0"
    )];
    test_op!(instruction);

    let instruction: Vec<InstructionSet<RISCV>> = vec![instruction_set!(
        1,
        instruction!(
            ParserRISCVInstOp::from(RV32IInstruction::Srl),
            10 as RISCVImmediate,
            10 as RISCVImmediate,
            10 as RISCVImmediate
        ),
        0x0040001c,
        0xfea55fe3,
        "srl a0, a0, a0"
    )];
    test_op!(instruction);

    let instruction: Vec<InstructionSet<RISCV>> = vec![instruction_set!(
        1,
        instruction!(
            ParserRISCVInstOp::from(RV32IInstruction::Sub),
            10 as RISCVImmediate,
            10 as RISCVImmediate,
            10 as RISCVImmediate
        ),
        0x00400020,
        0xfea54ae3,
        "sub a0, a0, a0"
    )];
    test_op!(instruction);

    let instruction: Vec<InstructionSet<RISCV>> = vec![instruction_set!(
        1,
        instruction!(
            ParserRISCVInstOp::from(RV32IInstruction::Xor),
            10 as RISCVImmediate,
            10 as RISCVImmediate,
            10 as RISCVImmediate
        ),
        0x00400024,
        0xfea516e3,
        "xor a0, a0, a0"
    )];
    test_op!(instruction);

    let instruction: Vec<InstructionSet<RISCV>> = vec![instruction_set!(
        1,
        instruction!(
            ParserRISCVInstOp::from(RV32IInstruction::Addi),
            10 as RISCVImmediate,
            10 as RISCVImmediate,
            0 as i32
        ),
        0x00400028,
        0xfea516e3,
        "addi a0, a0, 0"
    )];
    test_op!(instruction);

    let instruction: Vec<InstructionSet<RISCV>> = vec![instruction_set!(
        1,
        instruction!(
            ParserRISCVInstOp::from(RV32IInstruction::Andi),
            10 as RISCVImmediate,
            10 as RISCVImmediate,
            0 as i32
        ),
        0x0040002c,
        0xfe9ff0ef,
        "andi a0, a0, 0"
    )];
    test_op!(instruction);

    let instruction: Vec<InstructionSet<RISCV>> = vec![instruction_set!(
        1,
        instruction!(
            ParserRISCVInstOp::from(RV32IInstruction::Ori),
            10 as RISCVImmediate,
            10 as RISCVImmediate,
            0 as i32
        ),
        0x00400030,
        0x00100683,
        "ori a0, a0, 0"
    )];
    test_op!(instruction);

    let instruction: Vec<InstructionSet<RISCV>> = vec![instruction_set!(
        1,
        instruction!(
            ParserRISCVInstOp::from(RV32IInstruction::Slti),
            10 as RISCVImmediate,
            10 as RISCVImmediate,
            0 as i32
        ),
        0x00400034,
        0x00104683,
        "slti a0, a0, 0"
    )];
    test_op!(instruction);

    let instruction: Vec<InstructionSet<RISCV>> = vec![instruction_set!(
        1,
        instruction!(
            ParserRISCVInstOp::from(RV32IInstruction::Sltiu),
            10 as RISCVImmediate,
            10 as RISCVImmediate,
            0 as i32
        ),
        0x00400038,
        0x00101683,
        "sltiu a0, a0, 0"
    )];
    test_op!(instruction);

    let instruction: Vec<InstructionSet<RISCV>> = vec![instruction_set!(
        1,
        instruction!(
            ParserRISCVInstOp::from(RV32IInstruction::Xori),
            10 as RISCVImmediate,
            10 as RISCVImmediate,
            0 as i32
        ),
        0x0040003c,
        0x00105683,
        "xori a0, a0, 0"
    )];
    test_op!(instruction);

    let instruction: Vec<InstructionSet<RISCV>> = vec![instruction_set!(
        1,
        instruction!(
            ParserRISCVInstOp::from(RV32IInstruction::Slli),
            10 as RISCVImmediate,
            10 as RISCVImmediate,
            0 as i32
        ),
        0x00400040,
        0x0001537,
        "slli a0, a0, 0"
    )];
    test_op!(instruction);

    let instruction: Vec<InstructionSet<RISCV>> = vec![instruction_set!(
        1,
        instruction!(
            ParserRISCVInstOp::from(RV32IInstruction::Srai),
            10 as RISCVImmediate,
            10 as RISCVImmediate,
            0 as i32
        ),
        0x00400044,
        0x00102683,
        "srai a0, a0, 0"
    )];
    test_op!(instruction);

    let instruction: Vec<InstructionSet<RISCV>> = vec![instruction_set!(
        1,
        instruction!(
            ParserRISCVInstOp::from(RV32IInstruction::Bne),
            10 as RISCVImmediate,
            10 as RISCVImmediate,
            0 as i32
        ),
        0x00400048,
        0x00a5e633,
        "bne a0, a0, a"
    )];
    test_op!(instruction);

    let instruction: Vec<InstructionSet<RISCV>> = vec![instruction_set!(
        1,
        instruction!(
            ParserRISCVInstOp::from(RV32IInstruction::Bge),
            10 as RISCVImmediate,
            10 as RISCVImmediate,
            0 as i32
        ),
        0x0040004c,
        0x0066693,
        "bge a0, a0, b"
    )];
    test_op!(instruction);

    let instruction: Vec<InstructionSet<RISCV>> = vec![instruction_set!(
        1,
        instruction!(
            ParserRISCVInstOp::from(RV32IInstruction::Bgeu),
            10 as RISCVImmediate,
            10 as RISCVImmediate,
            0 as i32
        ),
        0x00400050,
        0x00000637,
        "bgeu a0, a0, c"
    )];
    test_op!(instruction);

    let instruction: Vec<InstructionSet<RISCV>> = vec![instruction_set!(
        1,
        instruction!(
            ParserRISCVInstOp::from(RV32IInstruction::Blt),
            10 as RISCVImmediate,
            10 as RISCVImmediate,
            0 as i32
        ),
        0x00400054,
        0x00000637,
        "blt a0, a0, a"
    )];
    test_op!(instruction);

    let instruction: Vec<InstructionSet<RISCV>> = vec![instruction_set!(
        1,
        instruction!(
            ParserRISCVInstOp::from(RV32IInstruction::Jal),
            10 as RISCVImmediate,
            0 as i32
        ),
        0x0040005c,
        0x00000637,
        "jal x1,0"
    )];
    test_op!(instruction);

    let instruction: Vec<InstructionSet<RISCV>> = vec![instruction_set!(
        1,
        instruction!(ParserRISCVInstOp::from(RV32IInstruction::Ebreak)),
        0x00400060,
        0x00000637,
        "ebreak"
    )];
    test_op!(instruction);
}
