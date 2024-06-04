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
type DataChange = (usize, Vec<u8>);

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

struct FakeMiddleware {
    pub input: Option<String>,
    pub input_res: Option<Result<(), String>>,
    pub output: Option<String>,
    pub sim_ptr: Ptr<RISCVSimulator>,
    pub cv: (Condvar, Mutex<()>),
}

impl FakeMiddlewareTrait for FakeMiddleware {
    fn request_input(&mut self) {
        std::thread::sleep(std::time::Duration::from_millis(100));
        self.input_res = Some(
            self.sim_ptr
                .as_mut()
                .syscall_input(self.input.as_ref().unwrap()),
        );
    }

    fn output(&mut self, output: &str) {
        self.output = Some(output.to_string());
    }

    fn update(&mut self, res: crate::types::middleware_types::Optional) {
        self.cv.0.notify_one();
    }
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
                        operands: vec![reg as Opd, Zero as Opd, val as Opd],
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
                        operands: opd!(),
                    },
                    address: 0,
                    code: 0,
                    basic: String::new(),
                };
                ebreak_placeholder
            ]);
            println!("{}", instruction.len());
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
    sim.run().unwrap();
    drop(mid.cv.0.wait(mid.cv.1.lock().unwrap()));
    assert_eq!(sim.get_pc_idx(), expect.pc_idx);
    assert_eq!(sim.get_register(), &expect_reg);
    if let Some((idx, val)) = &expect.data_change {
        sim.set_memory_return_range(MemoryReturnRange {
            start: CONFIG.dot_data_base_address + *idx as u64,
            len: val.len() as u64,
        })
        .unwrap();
    }
    if let Some(output) = expect.output {
        assert_eq!(mid.output.as_ref().unwrap(), &output);
    }
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

#[test]
fn test() {
    test_helper_only_reg(
        Add,
        opd![A0, A1, A2],
        vec![(A1, 123), (A2, 234)],
        (A0, 123 + 234),
    );
    test_helper_only_reg(Addi, opd![A0, A1, 234], vec![(A1, 123)], (A0, 123 + 234));

    test_helper_jump(Jal, opd![A0, 8], vec![], 3, Some(A0), 3);
    test_helper_jump(
        Jalr,
        opd![A0, A1, 8],
        vec![(A1, CONFIG.dot_text_base_address as u32 + 4)],
        3,
        Some(A0),
        3,
    );
}
