use std::{collections::VecDeque, sync::atomic::AtomicU8};

use strum::VariantArray;

use super::{
    instruction::{InstHandlerArg, INST_HANDLER_MAP},
    memory::Memory,
};
use crate::{
    dprintln,
    interface::{assembler::AssembleResult, simulator::Simulator},
    modules::riscv::{
        basic::interface::parser::{ParserRISCVInstOp, RV32IRegister, RISCV},
        middleware::backend_api::simulator_update,
    },
    types::middleware_types::{AssemblerConfig, MemoryReturnRange, Optional},
    utility::ptr::Ptr,
};

pub const MAX_HISTORY_SIZE: usize = 100;

#[derive(Clone, Copy, PartialEq, Eq, VariantArray)]
pub(super) enum SimulatorStatus {
    Unloaded = 0,
    Loading = 1,
    Running = 2,
    Paused = 3,
    Stopped = 4,
    Stopping = 5,
    Undo = 6,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum WaitStatus {
    Not,
    Int,
    String,
    Char,
}

pub struct RISCVSimulator {
    pub(super) reg: [u32; 32],
    pub(super) pc_idx: usize,
    pub(super) mem: Memory,
    pub(super) conf: AssemblerConfig,
    pub(super) inst: Option<AssembleResult<RISCV>>,
    pub(super) file: String,
    pub(super) wait_input: WaitStatus,
    breakpoints: Vec<bool>,
    debug: bool,
    thread: Option<std::thread::JoinHandle<()>>,
    status: AtomicU8,
    history: VecDeque<History>,
    mem_range: MemoryReturnRange,
}

pub(super) struct History {
    pub reg_idx: i32,
    pub reg_val: u32,
    pub pc_idx: usize,
    pub mem_addr: u32,
    pub mem: [u8; 4],
    pub mem_len: u8,
}

impl RISCVSimulator {
    pub fn new(file: &str) -> Self {
        RISCVSimulator {
            reg: [0; 32],
            pc_idx: 0,
            mem: Memory::new(),
            conf: Default::default(),
            inst: None,
            breakpoints: Vec::new(),
            debug: false,
            wait_input: WaitStatus::Not,
            thread: None,
            status: AtomicU8::new(0),
            file: file.to_string(),
            history: VecDeque::with_capacity(MAX_HISTORY_SIZE),
            mem_range: Default::default(),
        }
    }

    pub(super) fn in_data_segment(&self, addr: u32, len: u32) -> bool {
        let data_start = self.conf.dot_data_base_address as u32;
        let data_end = self.conf.data_segment_limit_address as u32;
        addr >= data_start && addr <= data_end && addr + (len - 1) <= data_end
    }

    pub(super) fn in_stack_segment(&self, addr: u32, len: u32) -> bool {
        let stack_start = self.conf.stack_base_address as u32;
        let stack_end = self.conf.stack_limit_address as u32;
        addr <= stack_start && addr >= stack_end && addr - (len - 1) >= stack_end
    }

    // (start, len)
    pub(super) fn text_range(&self) -> (u32, u32) {
        (
            self.conf.dot_text_base_address as u32,
            (self.inst.as_ref().unwrap().instruction.len() * 4) as u32,
        )
    }

    pub(super) fn to_text_idx(&self, addr: u32) -> Option<usize> {
        let (text_start, text_len) = self.text_range();
        if addr >= text_start && addr - text_start < text_len && addr % 4 == 0 {
            Some(((addr - text_start) / 4) as usize)
        } else {
            None
        }
    }

    pub(super) fn to_text_addr(&self, idx: usize) -> u32 {
        (self.conf.dot_text_base_address as usize + idx * 4) as u32
    }
}

impl Simulator for RISCVSimulator {
    fn load_inst(&mut self, inst: AssembleResult<RISCV>) -> Result<(), String> {
        if !self.cas_status(SimulatorStatus::Unloaded, SimulatorStatus::Loading)
            && !self.cas_status(SimulatorStatus::Stopped, SimulatorStatus::Loading)
        {
            return Err("Simulator is still running".to_string());
        }
        self.breakpoints = vec![false; inst.instruction.len()];
        self.inst = Some(inst);
        self._reset();
        self.update(Optional {
            success: true,
            message: "instruction loaded".to_string(),
        });
        self.set_status(SimulatorStatus::Stopped);
        Ok(())
    }

    fn get_raw_inst(&self) -> &Option<AssembleResult<RISCV>> {
        &self.inst
    }

    fn update_config(&mut self, config: &AssemblerConfig) -> Result<(), String> {
        let old_status;
        if !self.cas_status(SimulatorStatus::Unloaded, SimulatorStatus::Loading) {
            old_status = SimulatorStatus::Unloaded;
        } else if !self.cas_status(SimulatorStatus::Stopped, SimulatorStatus::Loading) {
            old_status = SimulatorStatus::Stopped;
        } else {
            return Err("Simulator is still running".to_string());
        }
        if config.dot_text_base_address % 4 != 0 {
            return Err("Invalid text base address".to_string());
        }
        self.conf = config.clone();
        if old_status == SimulatorStatus::Stopped {
            self._reset();
            self.update(Optional {
                success: true,
                message: "config updated".to_string(),
            });
        }
        self.set_status(old_status);
        Ok(())
    }

    fn run(&mut self) -> Result<(), String> {
        if !self.cas_status(SimulatorStatus::Stopped, SimulatorStatus::Running) {
            return Err("Invalid operation".to_string());
        }
        self.debug = false;
        self._reset();
        self._start(None);
        Ok(())
    }

    fn debug(&mut self) -> Result<(), String> {
        if !self.cas_status(SimulatorStatus::Stopped, SimulatorStatus::Running) {
            return Err("Invalid operation".to_string());
        }
        self.debug = true;
        self._reset();
        self._start(None);
        Ok(())
    }

    fn stop(&mut self) -> Result<(), String> {
        if !self.cas_status(SimulatorStatus::Running, SimulatorStatus::Stopping) {
            return Err("Simulator not running".to_string());
        }
        if let Some(t) = self.thread.take() {
            t.join().unwrap();
        }
        self.set_status(SimulatorStatus::Stopped);
        Ok(())
    }

    fn resume(&mut self) -> Result<(), String> {
        if self.wait_input != WaitStatus::Not {
            return Err("Waiting for input".to_string());
        }
        if !self.cas_status(SimulatorStatus::Paused, SimulatorStatus::Running) {
            return Err("Simulator not paused".to_string());
        }
        self._start(None);
        Ok(())
    }

    fn step(&mut self) -> Result<(), String> {
        if self.pc_idx >= self.inst.as_ref().unwrap().instruction.len() {
            return Err("Simulator finished".to_string());
        }
        if !self.cas_status(SimulatorStatus::Stopped, SimulatorStatus::Running) {
            if !self.cas_status(SimulatorStatus::Paused, SimulatorStatus::Running) {
                return Err("Invalid operation".to_string());
            } else {
                if self.wait_input != WaitStatus::Not {
                    self.set_status(SimulatorStatus::Paused);
                    return Err("Waiting for input".to_string());
                }
            }
        }
        self._start(Some(1));
        Ok(())
    }

    fn reset(&mut self) -> Result<(), String> {
        if !self.cas_status(SimulatorStatus::Stopped, SimulatorStatus::Stopping)
            && !self.cas_status(SimulatorStatus::Paused, SimulatorStatus::Stopping)
        {
            return Err("Invalid operation".to_string());
        }
        self._reset();
        self.set_status(SimulatorStatus::Stopped);
        self.update(Optional {
            success: true,
            message: "reset".to_string(),
        });
        Ok(())
    }

    fn undo(&mut self) -> Result<(), String> {
        if self.history.is_empty() {
            return Err("No history".to_string());
        }
        if self.cas_status(SimulatorStatus::Stopped, SimulatorStatus::Undo) {
            if self.history.is_empty() {
                self.set_status(SimulatorStatus::Stopped);
                return Err("No history".to_string());
            }
        } else if self.cas_status(SimulatorStatus::Paused, SimulatorStatus::Undo) {
            if self.history.is_empty() {
                self.set_status(SimulatorStatus::Paused);
                return Err("No history".to_string());
            }
        } else {
            return Err("Invalid operation".to_string());
        }
        let h = self.history.pop_back().unwrap();
        if h.reg_idx != -1 {
            self.reg[h.reg_idx as usize] = h.reg_val;
        }
        self.pc_idx = h.pc_idx;
        if h.mem_len != 0 {
            self.mem.set_range(h.mem_addr, &h.mem[..h.mem_len as usize]);
        }
        self.set_status(SimulatorStatus::Paused);
        self.update(Optional {
            success: true,
            message: "undo".to_string(),
        });
        Ok(())
    }

    fn set_breakpoint(&mut self, idx: usize) -> Result<(), String> {
        if idx >= self.breakpoints.len() {
            return Err("Invalid breakpoint index".to_string());
        }
        self.breakpoints[idx] = true;
        Ok(())
    }

    fn remove_breakpoint(&mut self, idx: usize) -> Result<(), String> {
        if idx >= self.breakpoints.len() {
            return Err("Invalid breakpoint index".to_string());
        }
        self.breakpoints[idx] = false;
        Ok(())
    }

    fn syscall_input(&mut self, input: &str) -> Result<(), String> {
        match self.wait_input {
            WaitStatus::Not => Err("No input required".to_string()),
            WaitStatus::Int => {
                if let Ok(val) = input.parse::<u32>() {
                    self.reg[RV32IRegister::A0 as usize] = val;
                    self.wait_input = WaitStatus::Not;
                    self.pc_idx += 1;
                    self.resume()
                } else {
                    Err("Invalid input".to_string())
                }
            }
            WaitStatus::String => {
                let addr = self.reg[RV32IRegister::A0 as usize];
                let len = self.reg[RV32IRegister::A1 as usize];
                if !self.in_data_segment(addr, len) {
                    return Err("Invalid memory access".to_string());
                }
                let data = input.as_bytes();
                self.mem.set_range(addr, &data[..len as usize]);
                self.wait_input = WaitStatus::Not;
                self.pc_idx += 1;
                self.resume()
            }
            WaitStatus::Char => {
                self.reg[RV32IRegister::A0 as usize] = input.as_bytes()[0] as u32;
                self.wait_input = WaitStatus::Not;
                self.pc_idx += 1;
                self.resume()
            }
        }
    }

    fn get_register(&self) -> &[u32] {
        &self.reg
    }

    fn get_memory(&self) -> Vec<u32> {
        let start = self.mem_range.start as u32;
        let len = self.mem_range.len as u32;
        if let (Some(begin_idx), Some(end_idx)) =
            (self.to_text_idx(start), self.to_text_idx(start + (len - 4)))
        {
            self.inst.as_ref().unwrap().instruction[begin_idx..=end_idx]
                .iter()
                .map(|inst| inst.code)
                .collect()
        } else if let Some(idx) = self.to_text_idx(start) {
            let text_range = self.text_range();
            let mut res = self.inst.as_ref().unwrap().instruction[idx..]
                .iter()
                .map(|inst| inst.code)
                .collect::<Vec<u32>>();
            res.extend(
                self.mem
                    .get_range(text_range.0 + text_range.1, len - 4 * (res.len() as u32))
                    .chunks(4)
                    .map(|data| u32::from_le_bytes([data[0], data[1], data[2], data[3]])),
            );
            res
        } else if let Some(idx) = self.to_text_idx(start + (len - 4)) {
            let mut res = self
                .mem
                .get_range(start, self.text_range().0 - start)
                .chunks(4)
                .map(|data| u32::from_le_bytes([data[0], data[1], data[2], data[3]]))
                .collect::<Vec<u32>>();
            res.extend(
                self.inst.as_ref().unwrap().instruction[..=idx]
                    .iter()
                    .map(|inst| inst.code),
            );
            res
        } else {
            self.mem
                .get_range(start, len)
                .chunks(4)
                .map(|data| u32::from_le_bytes([data[0], data[1], data[2], data[3]]))
                .collect()
        }
    }

    fn get_pc_idx(&self) -> Option<usize> {
        if self.pc_idx < self.inst.as_ref().unwrap().instruction.len() {
            Some(self.pc_idx)
        } else {
            None
        }
    }

    fn get_filepath(&self) -> &str {
        &self.file
    }

    fn get_memory_return_range(&self) -> MemoryReturnRange {
        self.mem_range
    }

    fn set_memory_return_range(&mut self, range: MemoryReturnRange) -> Result<(), String> {
        if range.start % 4 != 0
            || range.len % 4 != 0
            || range.start > u32::MAX as u64
            || u32::MAX as u64 - range.start < range.len
        {
            Err("Invalid range".to_string())
        } else {
            self.mem_range = range;
            if self.get_status() == SimulatorStatus::Stopped
                || self.get_status() == SimulatorStatus::Paused
            {
                self.update(Optional {
                    success: true,
                    message: "memory return range updated".to_string(),
                });
            }
            Ok(())
        }
    }
}

impl RISCVSimulator {
    fn get_status(&self) -> SimulatorStatus {
        SimulatorStatus::VARIANTS[self.status.load(std::sync::atomic::Ordering::Acquire) as usize]
    }

    fn set_status(&self, status: SimulatorStatus) {
        self.status
            .store(status as u8, std::sync::atomic::Ordering::Release);
    }

    fn cas_status(&self, current: SimulatorStatus, new: SimulatorStatus) -> bool {
        self.status
            .compare_exchange(
                current as u8,
                new as u8,
                std::sync::atomic::Ordering::AcqRel,
                std::sync::atomic::Ordering::Acquire,
            )
            .is_ok()
    }

    fn _step(&mut self) -> Result<SimulatorStatus, String> {
        let inst = &self.inst.as_ref().unwrap().instruction[self.pc_idx].instruction;
        if let ParserRISCVInstOp::RV32I(op) = inst.operation {
            let mut history = History {
                reg_idx: -1,
                reg_val: 0,
                pc_idx: self.pc_idx,
                mem_addr: 0,
                mem: [0; 4],
                mem_len: 0,
            };
            let res = (INST_HANDLER_MAP.get(op))(InstHandlerArg {
                sim: Ptr::new(self),
                args: &inst.operands,
                history: Ptr::new(&history),
            });
            if self.history.len() == MAX_HISTORY_SIZE {
                self.history.pop_front();
            }
            self.history.push_back(history);
            if matches!(res, Ok(SimulatorStatus::Running))
                && self.debug
                && self.pc_idx < self.breakpoints.len()
                && self.breakpoints[self.pc_idx]
            {
                Ok(SimulatorStatus::Paused)
            } else {
                res
            }
        } else {
            unimplemented!(
                "想要更多拓展? https://github.com/sustech-cs304/team-project-24spring-0点亮☆谢谢喵~"
            )
        }
    }

    fn _reset(&mut self) {
        self.reg = [0; 32];
        self.reg[RV32IRegister::Sp as usize] = self.conf.stack_pointer_sp as u32;
        self.pc_idx = 0;
        self.mem.reset();
        self.mem.set_range(
            self.conf.dot_data_base_address as u32,
            self.inst.as_ref().unwrap().data.as_slice(),
        );
        if let Some(t) = self.thread.take() {
            t.join().unwrap();
        }
        self.wait_input = WaitStatus::Not;
    }

    fn _start(&mut self, max_step: Option<usize>) {
        let self_ptr = Ptr::new(self);
        self.thread = Some(std::thread::spawn(move || {
            let mut step = 0;
            let _self = self_ptr.as_mut();
            let max_pc_idx = _self.inst.as_ref().unwrap().instruction.len();
            loop {
                if let Some(max_step) = max_step {
                    if step >= max_step {
                        _self.set_status(SimulatorStatus::Paused);
                        _self.update(Optional {
                            success: true,
                            message: "finished running".to_string(),
                        });
                        break;
                    } else {
                        step += 1;
                    }
                }
                match _self._step() {
                    Ok(SimulatorStatus::Paused) => {
                        _self.set_status(SimulatorStatus::Paused);
                        _self.update(Optional {
                            success: true,
                            message: "paused".to_string(),
                        });
                        break;
                    }
                    Err(e) => {
                        _self.set_status(SimulatorStatus::Stopped);
                        _self.update(Optional {
                            success: false,
                            message: e,
                        });
                        break;
                    }
                    _ => {}
                }
                if _self.pc_idx == max_pc_idx {
                    _self.set_status(SimulatorStatus::Stopped);
                    _self.update(Optional {
                        success: true,
                        message: "finished running".to_string(),
                    });
                    break;
                }
                if _self.get_status() != SimulatorStatus::Running {
                    _self.update(Optional {
                        success: true,
                        message: "stopped".to_string(),
                    });
                    break;
                }
            }
        }));
    }

    fn update(&mut self, res: Optional) {
        let paused = self.get_status() == SimulatorStatus::Paused;
        match simulator_update(self, res, paused) {
            Ok(_) => {}
            Err(e) => {
                dprintln!("{}", e);
            }
        }
    }
}
