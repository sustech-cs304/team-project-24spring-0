use std::sync::{Condvar, Mutex};

use super::helper::FakeMiddleware;
use crate::{
    interface::{
        assembler::{AssembleResult, Instruction, InstructionSet},
        simulator::{FakeMiddlewareTrait, Simulator},
    },
    modules::riscv::basic::interface::parser::{ParserInstSet, RV32IInstruction, RISCV},
    simulator::simulator::RISCVSimulator,
    types::middleware_types::{AssemblerConfig, MemoryReturnRange},
    utility::ptr::Ptr,
};

#[test]
fn test() {
    let sim = RISCVSimulator::new("1");
    let sim_ptr = Ptr::new(&sim);
    let sim = sim_ptr.as_mut();
    let mut mid = FakeMiddleware {
        input: None,
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
        data: vec![0x13, 0x00, 0x00, 0x00],
        instruction: vec![
            InstructionSet {
                line_number: 0,
                instruction: Instruction::<RISCV> {
                    operation: <RISCV as ParserInstSet>::Operator::RV32I(RV32IInstruction::Add),
                    operands: vec![0, 0, 0],
                },
                address: 0,
                code: 0,
                basic: String::new(),
            };
            5
        ],
    };
    sim.run().unwrap_err();
    sim.debug().unwrap_err();
    sim.step().unwrap_err();
    sim.undo().unwrap_err();
    sim.resume().unwrap_err();
    sim.stop().unwrap_err();
    sim.load_inst(inst).unwrap();
    sim.debug().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
    sim.reset().unwrap();
    sim.step().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
    sim.undo().unwrap();
    sim.resume().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
    sim.set_breakpoint(1).unwrap();
    sim.set_breakpoint(999).unwrap_err();
    sim.remove_breakpoint(1).unwrap();
    sim.remove_breakpoint(999).unwrap_err();
    assert_eq!(sim.get_filepath(), "1");
    sim.set_memory_return_range(MemoryReturnRange { start: 0, len: 4 })
        .unwrap();
    assert_eq!(
        sim.get_memory_return_range(),
        MemoryReturnRange { start: 0, len: 4 }
    );
    sim.get_raw_inst();
    let config: AssemblerConfig = Default::default();
    sim.set_memory_return_range(MemoryReturnRange {
        start: config.dot_text_base_address,
        len: 4,
    })
    .unwrap();
    sim.get_memory();
    sim.set_memory_return_range(MemoryReturnRange {
        start: config.dot_text_base_address,
        len: 8,
    })
    .unwrap();
    sim.get_memory();
    sim.set_memory_return_range(MemoryReturnRange { start: 1, len: 1 })
        .unwrap_err();
    sim.update_config(&config).unwrap();
}
