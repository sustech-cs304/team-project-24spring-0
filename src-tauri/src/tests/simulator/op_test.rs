use std::sync::{Condvar, Mutex};

use crate::{
    interface::{
        assembler::{AssembleResult, Instruction, InstructionSet, Operand},
        simulator::{FakeMiddlewareTrait, Simulator},
    },
    modules::riscv::{
        basic::interface::parser::{ParserInstSet, RISCV},
        rv32i::constants::{RV32IInstruction, RV32IRegister},
    },
    simulator::simulator::RISCVSimulator,
    tests::simulator::helper::FakeMiddleware,
    types::middleware_types::{AssemblerConfig, MemoryReturnRange},
    utility::ptr::Ptr,
};

static CONFIG: Lazy<AssemblerConfig> = Lazy::new(|| Default::default());

type Op = RV32IInstruction;
type Opd = Operand<RISCV>;
type Reg = RV32IRegister;

use once_cell::sync::Lazy;
use RV32IInstruction::*;
use RV32IRegister::*;

type RegChange = (Reg, u32);
type DataChange = (usize, Vec<u32>);

/// - `pc_idx`: aim pc_idx
/// - `reg_change`: expect register change
/// - `data_change`: expect data change (index, value)
/// - `output`: expect output for syscall
struct Expect {
    pc_idx: Option<usize>,
    reg_change: Option<RegChange>,
    data_change: Option<DataChange>,
    output: Option<String>,
}

macro_rules! opd {
    ($($x:expr),*) => {
        vec![$($x as Opd),*]
    };
}

/// Basic helper function
/// - `op`: operator
/// - `opd`: operand list (can be generated through `opd!`)
/// - `reg`: initial register value
/// - `data`: initial data value
/// - `input`: input for syscall (can be None)
/// - `expect`: expect result
/// - `ebreak_placeholder`: how many ebreak inserted after this instruction (to
///   stop the simulator and make the jump address a valid instruction address)
fn test_helper(
    op: Op,
    opd: Vec<Opd>,
    reg: Vec<(Reg, u32)>,
    data: Vec<u8>,
    input: Option<String>,
    expect: Expect,
    ebreak_placeholder: usize,
) {
    let sim = RISCVSimulator::new("");
    let sim_ptr = Ptr::new(&sim);
    let sim = sim_ptr.as_mut();
    let mut mid = FakeMiddleware {
        input,
        input_res: None,
        output: None,
        sim_ptr,
        cv: (Condvar::new(), Mutex::new(())),
        success: false,
    };
    let mid_ptr = Ptr::new(&mid);
    let mid = mid_ptr.as_mut();
    unsafe {
        sim.set_fake_middleware(Some(std::mem::transmute::<_, &'static mut _>(
            mid_ptr.as_mut() as &mut dyn FakeMiddlewareTrait,
        )));
    }
    let inst = AssembleResult {
        data,
        instruction: {
            let mut instruction = reg
                .iter()
                .map(|&(reg, val)| InstructionSet {
                    line_number: 0,
                    instruction: Instruction::<RISCV> {
                        operation: <RISCV as ParserInstSet>::Operator::RV32I(Addi),
                        operands: vec![reg as Opd, A2 as Opd, val as Opd],
                    },
                    address: 0,
                    code: 0,
                    basic: String::new(),
                })
                .collect::<Vec<_>>();
            instruction.push(InstructionSet {
                line_number: 0,
                instruction: Instruction::<RISCV> {
                    operation: <RISCV as ParserInstSet>::Operator::RV32I(op),
                    operands: opd,
                },
                address: 0,
                code: 0,
                basic: String::new(),
            });
            instruction.append(&mut vec![
                InstructionSet {
                    line_number: 0,
                    instruction: Instruction::<RISCV> {
                        operation: <RISCV as ParserInstSet>::Operator::RV32I(Ebreak),
                        operands: opd![],
                    },
                    address: 0,
                    code: 0,
                    basic: String::new(),
                };
                ebreak_placeholder
            ]);
            instruction
        },
    };
    sim.load_inst(inst).unwrap();
    let mut expect_reg = sim.get_register().to_vec();
    for &(reg, val) in &reg {
        expect_reg[reg as usize] = val;
    }
    if let &Some((reg, val)) = &expect.reg_change {
        expect_reg[reg as usize] = val;
    }
    let thread_sim = sim_ptr.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(10));
        thread_sim.as_mut().run().unwrap();
    });
    drop(mid.cv.0.wait(mid.cv.1.lock().unwrap()));
    assert_eq!(sim.get_pc_idx(), expect.pc_idx);
    assert_eq!(sim.get_register(), &expect_reg);
    if let Some((idx, val)) = &expect.data_change {
        sim.set_memory_return_range(MemoryReturnRange {
            start: CONFIG.dot_data_base_address + *idx as u64,
            len: 4 * val.len() as u64,
        })
        .unwrap();
        assert_eq!(&sim.get_memory(), val);
    }
    if let Some(output) = expect.output {
        assert_eq!(mid.output.as_ref().unwrap(), &output);
    }
    assert!(mid.success);
}

/// Test operator not need data section and not jump
/// - `op`: operator
/// - `opd`: operand list (can be generated through `opd!`)
/// - `reg`: initial register value
/// - `change`: changes that the instruction makes
fn test_helper_only_reg(op: Op, opd: Vec<Opd>, reg: Vec<(Reg, u32)>, change: RegChange) {
    test_helper(
        op,
        opd,
        reg,
        vec![],
        None,
        Expect {
            pc_idx: None,
            reg_change: Some(change),
            data_change: None,
            output: None,
        },
        0,
    );
}

/// Test operator such as `beq`, `jal`.
/// - `op`: operator
/// - `opd`: operand list (can be generated through `opd!`)
/// - `reg`: initial register value
/// - `pc_idx`: aim pc_idx
/// - `store_pc`: where pc+4 stored in (can be None)
/// - `ebreak_placeholder`: how many ebreak inserted after this instruction (to
///   stop the simulator and make the jump address a valid instruction address)
fn test_helper_jump(
    op: Op,
    opd: Vec<Opd>,
    reg: Vec<(Reg, u32)>,
    pc_idx: usize,
    store_pc: Option<Reg>,
    ebreak_placeholder: usize,
) {
    let reg_len = reg.len();
    test_helper(
        op,
        opd,
        reg,
        vec![],
        None,
        Expect {
            pc_idx: Some(reg_len + pc_idx),
            reg_change: {
                match store_pc {
                    Some(store_pc) => Some((
                        store_pc,
                        CONFIG.dot_text_base_address as u32 + 4 * reg_len as u32 + 4,
                    )),
                    None => None,
                }
            },
            data_change: None,
            output: None,
        },
        ebreak_placeholder,
    );
}

fn test_helper_load(op: Op, offset: u32, data: Vec<u8>, expect_val: u32) {
    test_helper(
        op,
        opd![A0, offset, A1],
        vec![(A1, CONFIG.dot_data_base_address as u32)],
        data,
        None,
        Expect {
            pc_idx: None,
            reg_change: Some((A0, expect_val)),
            data_change: None,
            output: None,
        },
        0,
    );
}

fn test_helper_store(op: Op, val: u32, expect_data: Vec<u32>) {
    test_helper(
        op,
        opd![A0, 0, A1],
        vec![(A0, val), (A1, CONFIG.dot_data_base_address as u32)],
        vec![],
        None,
        Expect {
            pc_idx: None,
            reg_change: None,
            data_change: Some((0, expect_data)),
            output: None,
        },
        0,
    );
}

#[test]
fn test() {
    test_helper_only_reg(
        Add,
        opd![A0, A1, A2],
        vec![(A1, 123), (A2, 234)],
        (A0, 123 + 234),
    );

    test_helper_only_reg(Addi, opd![A0, A1, 234], vec![(A1, 123)], (A0, 123 + 234));
    test_helper_only_reg(Addi, opd![Zero, A1, 234], vec![(A1, 123)], (Zero, 0));

    test_helper_only_reg(
        And,
        opd![A0, A1, A2],
        vec![(A1, 0b1010), (A2, 0b1100)],
        (A0, 0b1000),
    );

    test_helper_only_reg(
        Andi,
        opd![A0, A1, 0b1100],
        vec![(A1, 0b1010)],
        (A0, 0b1010 & 0b1100),
    );

    test_helper_only_reg(
        Auipc,
        opd![A0, 4],
        vec![],
        (A0, CONFIG.dot_text_base_address as u32 + ((4 as u32) << 12)),
    );

    test_helper_jump(Beq, opd![A0, A1, 8], vec![], 3, None, 3);
    test_helper_jump(Beq, opd![A0, A1, 8], vec![(A0, 1)], 2, None, 3);

    test_helper_jump(Bge, opd![A0, A1, 8], vec![], 3, None, 3);
    test_helper_jump(Bge, opd![A0, A1, 8], vec![(A0, -1i32 as u32)], 2, None, 3);

    test_helper_jump(Bgeu, opd![A0, A1, 8], vec![(A0, -1i32 as u32)], 3, None, 3);
    test_helper_jump(Bgeu, opd![A0, A1, 8], vec![(A1, 1)], 2, None, 3);

    test_helper_jump(Blt, opd![A0, A1, 8], vec![(A0, -1i32 as u32)], 3, None, 3);
    test_helper_jump(Blt, opd![A0, A1, 8], vec![], 2, None, 3);

    test_helper_jump(Bltu, opd![A0, A1, 8], vec![(A1, 1)], 3, None, 3);
    test_helper_jump(Bltu, opd![A0, A1, 8], vec![(A0, -1i32 as u32)], 2, None, 3);

    test_helper_jump(Bne, opd![A0, A1, 8], vec![(A1, 1)], 3, None, 3);
    test_helper_jump(Bne, opd![A0, A1, 8], vec![], 2, None, 3);

    test_helper_only_reg(Ebreak, opd![], vec![], (Zero, 0));

    test_helper(
        Ecall,
        opd![],
        vec![(A7, 1), (A0, -1i32 as u32)],
        vec![],
        None,
        Expect {
            pc_idx: None,
            reg_change: None,
            data_change: None,
            output: Some("-1".to_string()),
        },
        0,
    );
    test_helper(
        Ecall,
        opd![],
        vec![(A7, 4), (A0, CONFIG.dot_data_base_address as u32)],
        vec!['a' as u8, 'b' as u8, 'c' as u8, 0],
        None,
        Expect {
            pc_idx: None,
            reg_change: None,
            data_change: None,
            output: Some("abc".to_string()),
        },
        0,
    );
    test_helper(
        Ecall,
        opd![],
        vec![(A7, 5)],
        vec![],
        Some("123456789".to_string()),
        Expect {
            pc_idx: None,
            reg_change: Some((A0, 123456789)),
            data_change: None,
            output: None,
        },
        0,
    );
    test_helper(
        Ecall,
        opd![],
        vec![(A7, 8), (A0, CONFIG.dot_data_base_address as u32), (A1, 3)],
        vec![],
        Some("abcd".to_string()),
        Expect {
            pc_idx: None,
            reg_change: None,
            data_change: Some((
                0,
                vec!['a' as u32 + (('b' as u32) << 8) + (('c' as u32) << 16)],
            )),
            output: None,
        },
        0,
    );
    test_helper(
        Ecall,
        opd![],
        vec![(A7, 10)],
        vec![],
        None,
        Expect {
            pc_idx: Some(1),
            reg_change: None,
            data_change: None,
            output: None,
        },
        2,
    );
    test_helper(
        Ecall,
        opd![],
        vec![(A7, 11), (A0, 0x12345678)],
        vec![],
        None,
        Expect {
            pc_idx: None,
            reg_change: None,
            data_change: None,
            output: Some((0x78u8 as char).to_string()),
        },
        0,
    );
    test_helper(
        Ecall,
        opd![],
        vec![(A7, 12)],
        vec![],
        Some("12345678".to_string()),
        Expect {
            pc_idx: None,
            reg_change: Some((A0, '1' as u32)),
            data_change: None,
            output: None,
        },
        0,
    );
    test_helper(
        Ecall,
        opd![],
        vec![(A7, 34), (A0, 0x123)],
        vec![],
        None,
        Expect {
            pc_idx: None,
            reg_change: None,
            data_change: None,
            output: Some("0x00000123".to_string()),
        },
        0,
    );
    test_helper(
        Ecall,
        opd![],
        vec![(A7, 35), (A0, 0b101100101011)],
        vec![],
        None,
        Expect {
            pc_idx: None,
            reg_change: None,
            data_change: None,
            output: Some("0b00000000000000000000101100101011".to_string()),
        },
        0,
    );
    test_helper(
        Ecall,
        opd![],
        vec![(A7, 36), (A0, -1i32 as u32)],
        vec![],
        None,
        Expect {
            pc_idx: None,
            reg_change: None,
            data_change: None,
            output: Some((-1i32 as u32).to_string()),
        },
        0,
    );

    test_helper_only_reg(Fence, opd![], vec![], (Zero, 0));
    test_helper_only_reg(FenceI, opd![], vec![], (Zero, 0));

    test_helper_jump(Jal, opd![A0, 8], vec![], 3, Some(A0), 3);

    test_helper_jump(
        Jalr,
        opd![A0, A1, 8],
        vec![(A1, CONFIG.dot_text_base_address as u32 + 4)],
        3,
        Some(A0),
        3,
    );

    test_helper_load(Lb, 1, vec![0x0, 0xff, 0x2, 0x3], 0xffffffff);

    test_helper_load(Lbu, 1, vec![0x0, 0xff, 0x2, 0x3], 0xff);

    test_helper_load(Lh, 1, vec![0x0, 0x1, 0xff, 0x3], 0xffffff01);

    test_helper_load(Lhu, 1, vec![0x0, 0x1, 0xff, 0x3], 0xff01);

    test_helper_only_reg(Lui, opd![A0, 0b1100], vec![], (A0, (0b1100 as u32) << 12));

    test_helper_load(Lw, 1, vec![0x0, 0x1, 0x2, 0x3, 0x4, 0x5], 0x04030201);

    test_helper_only_reg(
        Or,
        opd![A0, A1, A2],
        vec![(A1, 0b1010), (A2, 0b1100)],
        (A0, 0b1110),
    );

    test_helper_only_reg(
        Ori,
        opd![A0, A1, 0b1100],
        vec![(A1, 0b1010), (A2, 0b1100)],
        (A0, 0b1110),
    );

    test_helper_only_reg(
        Ori,
        opd![A0, A1, 0b1100],
        vec![(A1, 0b1010), (A2, 0b1100)],
        (A0, 0b1110),
    );

    test_helper_store(Sb, 0x12345678, vec![0x78]);

    test_helper_store(Sh, 0x12345678, vec![0x5678]);

    test_helper_only_reg(
        Sll,
        opd![A0, A1, A2],
        vec![(A1, 0b1010), (A2, 0b1100)],
        (A0, 0b1010 << (0b1100 as u32 & 0x1f)),
    );

    test_helper_only_reg(
        Slli,
        opd![A0, A1, 0b1100],
        vec![(A1, 0b1010), (A2, 0b1100)],
        (A0, 0b1010 << (0b1100 as u32 & 0x1f)),
    );

    test_helper_only_reg(Slt, opd![A0, A1, A2], vec![(A1, -1i32 as u32)], (A0, 1));
    test_helper_only_reg(Slt, opd![A0, A1, A2], vec![], (A0, 0));

    test_helper_only_reg(Slti, opd![A0, A1, 1], vec![], (A0, 1));
    test_helper_only_reg(Slti, opd![A0, A1, -1], vec![], (A0, 0));

    test_helper_only_reg(Sltiu, opd![A0, A1, -1], vec![], (A0, 1));
    test_helper_only_reg(Sltiu, opd![A0, A1, 0], vec![(A1, -1i32 as u32)], (A0, 0));

    test_helper_only_reg(Sltu, opd![A0, A1, A2], vec![(A2, -1i32 as u32)], (A0, 1));
    test_helper_only_reg(Sltu, opd![A0, A1, A2], vec![], (A0, 0));

    test_helper_only_reg(
        Sra,
        opd![A0, A1, A2],
        vec![(A1, 1), (A2, 234)],
        (A0, 1 >> (234 as u32 & 0x1f) as u32),
    );

    test_helper_only_reg(
        Srai,
        opd![A0, A1, 234],
        vec![(A1, 1), (A2, 234)],
        (A0, 1 >> (234 as u32 & 0x1f) as u32),
    );

    test_helper_only_reg(
        Srl,
        opd![A4, A5, A6],
        vec![(A5, 1), (A6, 234)],
        (A0, 1 >> (234 as u32 & 0x1f) as u32),
    );

    test_helper_only_reg(Srli, opd![A4, A5, 2], vec![(A5, 1)], (A4, 1 >> 2));

    test_helper_only_reg(
        Sub,
        opd![S4, S5, S6],
        vec![(S5, 123), (S6, 23)],
        (S4, 123 - 23),
    );

    test_helper_store(Sw, 0x12345678, vec![0x12345678]);

    test_helper_only_reg(
        Xor,
        opd![S4, S5, S6],
        vec![(S5, 123), (S6, 23)],
        (S4, 123 ^ 23),
    );

    test_helper_only_reg(
        Xori,
        opd![S4, S5, 23],
        vec![(S5, 123), (S6, 23)],
        (S4, 123 ^ 23),
    );
}
