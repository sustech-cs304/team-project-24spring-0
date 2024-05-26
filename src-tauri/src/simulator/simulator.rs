use std::{collections::VecDeque, sync::atomic::AtomicU8};

use super::{
    instruction::{InstHandlerArg, INST_HANDLER_MAP},
    memory::Memory,
};
use crate::{
    interface::{assembler::AssembleResult, simulator::Simulator},
    modules::riscv::{
        basic::interface::parser::{ParserRISCVInstOp, RV32IRegister, RISCV},
        middleware::backend_api::simulator_update,
    },
    types::middleware_types::AssemblerConfig,
    utility::ptr::Ptr,
};

pub const MAX_HISTORY_SIZE: usize = 100;

pub(super) const STATUS_UNLOADED: u8 = 0;
pub(super) const STATUS_LOADING: u8 = 1;
pub(super) const STATUS_RUNNING: u8 = 2;
pub(super) const STATUS_PAUSED: u8 = 3;
pub(super) const STATUS_STOPPED: u8 = 4;
pub(super) const STATUS_STOPPING: u8 = 5;
pub(super) const STATUS_UNDO: u8 = 6;

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
        }
    }

    pub(super) fn in_data_segment(&self, addr: u32, len: u32) -> bool {
        let data_start = self.conf.dot_data_base_address as u32;
        let data_end = self.conf.data_segment_limit_address as u32;
        addr >= data_start && addr < data_end && addr + len <= data_end
    }

    pub(super) fn in_stack_segment(&self, addr: u32, len: u32) -> bool {
        let stack_start = self.conf.stack_base_address as u32;
        let stack_end = self.conf.stack_limit_address as u32;
        addr <= stack_start && addr > stack_end && addr + len <= stack_start
    }

    pub(super) fn text_range(&self) -> (u32, u32) {
        (
            self.conf.dot_text_base_address as u32,
            self.conf.dot_text_base_address as u32
                + self.inst.as_ref().unwrap().instruction.len() as u32 * 4,
        )
    }

    pub(super) fn to_text_idx(&self, addr: u32) -> Option<usize> {
        let (text_start, text_end) = self.text_range();
        if addr >= text_start && addr < text_end && addr % 4 == 0 {
            Some(((addr - text_start) / 4) as usize)
        } else {
            None
        }
    }

    pub(super) fn to_text_addr(&self, idx: usize) -> u32 {
        (self.conf.dot_data_base_address as usize + idx * 4) as u32
    }
}

impl Simulator for RISCVSimulator {
    fn load_inst(&mut self, inst: AssembleResult<RISCV>) -> Result<(), String> {
        if !self.cas_status(STATUS_UNLOADED, STATUS_LOADING)
            && !self.cas_status(STATUS_STOPPED, STATUS_LOADING)
        {
            return Err("Simulator is still running".to_string());
        }
        self.breakpoints = vec![false; inst.instruction.len()];
        self.inst = Some(inst);
        self._reset();
        self.set_status(STATUS_STOPPED);
        Ok(())
    }

    fn update_config(&mut self, config: &AssemblerConfig) -> Result<(), String> {
        let old_status;
        if !self.cas_status(STATUS_UNLOADED, STATUS_LOADING) {
            old_status = STATUS_UNLOADED;
        } else if !self.cas_status(STATUS_STOPPED, STATUS_LOADING) {
            old_status = STATUS_STOPPED;
        } else {
            return Err("Simulator is still running".to_string());
        }
        if config.dot_text_base_address % 4 != 0 {
            return Err("Invalid text base address".to_string());
        }
        self.conf = config.clone();
        if old_status == STATUS_STOPPED {
            self._reset();
        }
        self.set_status(old_status);
        Ok(())
    }

    fn run(&mut self) -> Result<(), String> {
        if !self.cas_status(STATUS_STOPPED, STATUS_RUNNING) {
            return Err("Invalid operation".to_string());
        }
        self.debug = false;
        self._reset();
        self._start();
        Ok(())
    }

    fn debug(&mut self) -> Result<(), String> {
        if !self.cas_status(STATUS_STOPPED, STATUS_RUNNING) {
            return Err("Invalid operation".to_string());
        }
        self.debug = true;
        self._reset();
        self._start();
        Ok(())
    }

    fn stop(&mut self) -> Result<(), String> {
        if !self.cas_status(STATUS_RUNNING, STATUS_STOPPING) {
            return Err("Simulator not running".to_string());
        }
        if let Some(t) = self.thread.take() {
            t.join().unwrap();
        }
        self.set_status(STATUS_STOPPED);
        Ok(())
    }

    fn resume(&mut self) -> Result<(), String> {
        if self.wait_input != WaitStatus::Not {
            return Err("Waiting for input".to_string());
        }
        if !self.cas_status(STATUS_PAUSED, STATUS_RUNNING) {
            return Err("Simulator not paused".to_string());
        }
        self._start();
        Ok(())
    }

    fn step(&mut self) -> Result<(), String> {
        if !self.cas_status(STATUS_STOPPED, STATUS_RUNNING) {
            if !self.cas_status(STATUS_PAUSED, STATUS_RUNNING) {
                return Err("Invalid operation".to_string());
            } else {
                if self.wait_input != WaitStatus::Not {
                    self.set_status(STATUS_PAUSED);
                    return Err("Waiting for input".to_string());
                }
            }
        }
        let res = self._step();
        self.set_status(STATUS_PAUSED);
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn reset(&mut self) -> Result<(), String> {
        if !self.cas_status(STATUS_STOPPED, STATUS_STOPPING)
            && !self.cas_status(STATUS_PAUSED, STATUS_STOPPING)
        {
            return Err("Invalid operation".to_string());
        }
        self._reset();
        self.set_status(STATUS_STOPPED);
        Ok(())
    }

    fn undo(&mut self) -> Result<(), String> {
        if self.history.is_empty() {
            return Err("No history".to_string());
        }
        if self.cas_status(STATUS_STOPPED, STATUS_UNDO) {
            if self.history.is_empty() {
                self.set_status(STATUS_STOPPED);
                return Err("No history".to_string());
            }
        } else if self.cas_status(STATUS_PAUSED, STATUS_UNDO) {
            if self.history.is_empty() {
                self.set_status(STATUS_PAUSED);
                return Err("No history".to_string());
            }
        } else {
            return Err("Invalid operation".to_string());
        }
        let h = self.history.pop_front().unwrap();
        if h.reg_idx != -1 {
            self.reg[h.reg_idx as usize] = h.reg_val;
        }
        self.pc_idx = h.pc_idx;
        if h.mem_len != 0 {
            self.mem.set_range(h.mem_addr, &h.mem[..h.mem_len as usize]);
        }
        self.set_status(STATUS_PAUSED);
        Ok(())
    }

    fn set_breakpoint(&mut self, idx: usize) -> Result<(), String> {
        if idx >= self.breakpoints.len() {
            return Err("Invalid breakpoint index".to_string());
        }
        self.breakpoints[idx] ^= true;
        Ok(())
    }

    fn syscall_input(&mut self, input: &str) -> Result<(), String> {
        match self.wait_input {
            WaitStatus::Not => Err("No input required".to_string()),
            WaitStatus::Int => {
                if let Ok(val) = input.parse::<u32>() {
                    self.reg[RV32IRegister::A0 as usize] = val;
                    self.wait_input = WaitStatus::Not;
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
                self.resume()
            }
            WaitStatus::Char => {
                let addr = self.reg[RV32IRegister::A0 as usize];
                if !self.in_data_segment(addr, 1) {
                    return Err("Invalid memory access".to_string());
                }
                self.mem[addr] = input.as_bytes()[0];
                self.wait_input = WaitStatus::Not;
                self.resume()
            }
        }
    }

    fn get_register(&self) -> &[u32] {
        &self.reg
    }

    fn get_memory(&self, begin: u32, end: u32) -> Vec<u32> {
        if let (Some(begin_idx), Some(end_idx)) = (self.to_text_idx(begin), self.to_text_idx(end)) {
            self.inst.as_ref().unwrap().instruction[begin_idx..end_idx]
                .iter()
                .map(|inst| inst.code)
                .collect()
        } else if let Some(idx) = self.to_text_idx(begin) {
            let mut res = self.inst.as_ref().unwrap().instruction[idx..]
                .iter()
                .map(|inst| inst.code)
                .collect::<Vec<u32>>();
            res.extend(
                self.mem
                    .get_range(self.text_range().1, end)
                    .chunks(4)
                    .map(|data| u32::from_le_bytes([data[0], data[1], data[2], data[3]])),
            );
            res
        } else if let Some(idx) = self.to_text_idx(end) {
            let mut res = self
                .mem
                .get_range(begin, self.text_range().0)
                .chunks(4)
                .map(|data| u32::from_le_bytes([data[0], data[1], data[2], data[3]]))
                .collect::<Vec<u32>>();
            res.extend(
                self.inst.as_ref().unwrap().instruction[..idx]
                    .iter()
                    .map(|inst| inst.code),
            );
            res
        } else {
            self.mem
                .get_range(begin, end)
                .chunks(4)
                .map(|data| u32::from_le_bytes([data[0], data[1], data[2], data[3]]))
                .collect()
        }
    }
}

impl RISCVSimulator {
    fn get_status(&self) -> u8 {
        self.status.load(std::sync::atomic::Ordering::Acquire)
    }

    fn set_status(&self, status: u8) {
        self.status
            .store(status, std::sync::atomic::Ordering::Release);
    }

    fn cas_status(&self, current: u8, new: u8) -> bool {
        self.status
            .compare_exchange(
                current,
                new,
                std::sync::atomic::Ordering::AcqRel,
                std::sync::atomic::Ordering::Acquire,
            )
            .is_ok()
    }

    fn _step(&mut self) -> Result<u8, String> {
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
            if matches!(res, Ok(STATUS_RUNNING)) && self.debug && self.breakpoints[self.pc_idx] {
                Ok(STATUS_PAUSED)
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
        self.reg[RV32IRegister::Sp as usize] = self.conf.stack_base_address as u32;
        self.pc_idx = 0;
        self.mem.reset();
        self.mem.set_range(
            self.conf.dot_text_base_address as u32,
            self.inst.as_ref().unwrap().data.as_slice(),
        );
        if let Some(t) = self.thread.take() {
            t.join().unwrap();
        }
        self.wait_input = WaitStatus::Not;
    }

    fn _start(&mut self) {
        let self_ptr = Ptr::new(self);
        self.thread = Some(std::thread::spawn(move || loop {
            let _self = self_ptr.as_mut();
            match _self._step() {
                Ok(STATUS_PAUSED) => {
                    _self.set_status(STATUS_PAUSED);
                    simulator_update(Ok(()));
                    break;
                }
                Err(e) => {
                    _self.set_status(STATUS_STOPPED);
                    simulator_update(Err(e));
                    break;
                }
                _ => {}
            }
            if _self.get_status() != STATUS_RUNNING {
                simulator_update(Err("Simulator stopped".to_string()));
                break;
            }
        }));
    }
}
