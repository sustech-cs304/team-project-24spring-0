use std::ops::Index;

use once_cell::sync::Lazy;
use RV32IInstruction::*;

use super::simulator::*;
use crate::{
    interface::assembler::Operand,
    modules::riscv::{
        basic::interface::parser::RISCV,
        middleware::backend_api::{syscall_input_request, syscall_output_print},
        rv32i::constants::*,
    },
    utility::{enum_map::EnumMap, ptr::Ptr},
};

type InstHandler = fn(InstHandlerArg) -> Result<SimulatorStatus, String>;
pub(super) struct InstHandlerArg<'a> {
    pub sim: Ptr<RISCVSimulator>,
    pub args: &'a Vec<Operand<RISCV>>,
    pub history: Ptr<History>,
}

pub(super) static INST_HANDLER_MAP: Lazy<EnumMap<RV32IInstruction, InstHandler>> =
    Lazy::new(|| {
        EnumMap::new(
            &[
                (Add, add_handler as InstHandler),
                (Addi, addi_handler as InstHandler),
                (And, and_handler as InstHandler),
                (Andi, andi_handler as InstHandler),
                (Auipc, auipc_handler as InstHandler),
                (Beq, beq_handler as InstHandler),
                (Bge, bge_handler as InstHandler),
                (Bgeu, bgeu_handler as InstHandler),
                (Blt, blt_handler as InstHandler),
                (Bltu, bltu_handler as InstHandler),
                (Bne, bne_handler as InstHandler),
                (Csrrc, csrrc_handler as InstHandler),
                (Csrrci, csrrci_handler as InstHandler),
                (Csrrs, csrrs_handler as InstHandler),
                (Csrrsi, csrrsi_handler as InstHandler),
                (Csrrs, csrrs_handler as InstHandler),
                (Csrrsi, csrrsi_handler as InstHandler),
                (Csrrw, csrrw_handler as InstHandler),
                (Csrrwi, csrrwi_handler as InstHandler),
                (Ebreak, ebreak_handler as InstHandler),
                (Ecall, ecall_handler as InstHandler),
                (Fence, fence_handler as InstHandler),
                (FenceI, fence_i_handler as InstHandler),
                (Jal, jal_handler as InstHandler),
                (Jalr, jalr_handler as InstHandler),
                (Lb, lb_handler as InstHandler),
                (Lbu, lbu_handler as InstHandler),
                (Lh, lh_handler as InstHandler),
                (Lhu, lhu_handler as InstHandler),
                (Lui, lui_handler as InstHandler),
                (Lw, lw_handler as InstHandler),
                (Or, or_handler as InstHandler),
                (Ori, ori_handler as InstHandler),
                (Sb, sb_handler as InstHandler),
                (Sh, sh_handler as InstHandler),
                (Sll, sll_handler as InstHandler),
                (Slli, slli_handler as InstHandler),
                (Slt, slt_handler as InstHandler),
                (Slti, slti_handler as InstHandler),
                (Sltiu, sltiu_handler as InstHandler),
                (Sltu, sltu_handler as InstHandler),
                (Sra, sra_handler as InstHandler),
                (Srai, srai_handler as InstHandler),
                (Srl, srl_handler as InstHandler),
                (Srli, srli_handler as InstHandler),
                (Sub, sub_handler as InstHandler),
                (Sw, sw_handler as InstHandler),
                (Xor, xor_handler as InstHandler),
                (Xori, xori_handler as InstHandler),
            ],
            |ele| (ele.0, ele.1),
        )
    });

macro_rules! load_helper {
    ($arg:expr, $size:expr, $t:ty) => {{
        let addr = $arg.reg($arg[2]) + $arg[1] as u32;
        let sim = $arg.sim.as_ref();
        if sim.in_data_segment(addr, $size) || sim.in_stack_segment(addr, $size) {
            let mut buf = [0u8; $size];
            for i in 0..$size {
                buf[i] = sim.mem[addr + i as u32];
            }
            *$arg.reg_mut($arg[0]) = unsafe { std::mem::transmute::<_, $t>(buf) as u32 };
            $arg.pc_step();
            Ok(SimulatorStatus::Running)
        } else {
            Err("Invalid memory access".to_string())
        }
    }};
}

macro_rules! store_helper {
    ($arg:expr, $size:expr, $t:ty) => {{
        let addr = $arg.reg($arg[2]) + $arg[1] as u32;
        let sim = $arg.sim.as_mut();
        if sim.in_data_segment(addr, $size) || sim.in_stack_segment(addr, $size) {
            let history = $arg.history.as_mut();
            history.mem_addr = addr;
            history.mem_len = $size;
            let mut buf = ($arg.reg($arg[0]) as $t).to_le_bytes();
            for i in 0..$size {
                history.mem[i] = sim.mem[addr + i as u32];
                sim.mem[addr + i as u32] = buf[i];
            }
            $arg.pc_step();
            Ok(SimulatorStatus::Running)
        } else {
            Err("Invalid memory access".to_string())
        }
    }};
}

pub(super) fn add_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    *arg.reg_mut(arg[0]) = arg.reg(arg[1]) + arg.reg(arg[2]);
    arg.pc_step();
    Ok(SimulatorStatus::Running)
}

pub(super) fn addi_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    *arg.reg_mut(arg[0]) = arg.reg(arg[1]) + arg[2] as u32;
    arg.pc_step();
    Ok(SimulatorStatus::Running)
}

pub(super) fn and_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    *arg.reg_mut(arg[0]) = arg.reg(arg[1]) & arg.reg(arg[2]);
    arg.pc_step();
    Ok(SimulatorStatus::Running)
}

pub(super) fn andi_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    *arg.reg_mut(arg[0]) = arg.reg(arg[1]) & arg[2] as u32;
    arg.pc_step();
    Ok(SimulatorStatus::Running)
}

pub(super) fn auipc_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    *arg.reg_mut(arg[0]) = arg.pc() + arg[1] as u32;
    arg.pc_step();
    Ok(SimulatorStatus::Running)
}

pub(super) fn beq_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    if arg.reg(arg[0]) == arg.reg(arg[1]) {
        jump_helper(&arg, arg[2] as u32)?;
    }
    Ok(SimulatorStatus::Running)
}

pub(super) fn bge_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    if arg.reg(arg[0]) as i32 >= arg.reg(arg[1]) as i32 {
        jump_helper(&arg, arg[2] as u32)?;
    }
    Ok(SimulatorStatus::Running)
}

pub(super) fn bgeu_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    if arg.reg(arg[0]) >= arg.reg(arg[1]) {
        jump_helper(&arg, arg[2] as u32)?;
    }
    Ok(SimulatorStatus::Running)
}

pub(super) fn blt_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    if (arg.reg(arg[0]) as i32) < (arg.reg(arg[1]) as i32) {
        jump_helper(&arg, arg[2] as u32)?;
    }
    Ok(SimulatorStatus::Running)
}

pub(super) fn bltu_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    if arg.reg(arg[0]) < arg.reg(arg[1]) {
        jump_helper(&arg, arg[2] as u32)?;
    }
    Ok(SimulatorStatus::Running)
}

pub(super) fn bne_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    if arg.reg(arg[0]) != arg.reg(arg[1]) {
        jump_helper(&arg, arg[2] as u32)?;
    }
    Ok(SimulatorStatus::Running)
}

pub(super) fn csrrc_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    unimplemented!();
}

pub(super) fn csrrci_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    unimplemented!();
}

pub(super) fn csrrs_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    unimplemented!();
}

pub(super) fn csrrsi_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    unimplemented!();
}

pub(super) fn csrrw_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    unimplemented!();
}

pub(super) fn csrrwi_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    unimplemented!();
}

pub(super) fn ebreak_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    arg.pc_step();
    Ok(SimulatorStatus::Paused)
}

pub(super) fn ecall_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    match arg.reg(RV32IRegister::A7 as i32) {
        1 => {
            syscall_output_print(
                arg.get_path(),
                &((arg.reg(RV32IRegister::A0 as i32) as i32).to_string()),
            )?;
            arg.pc_step();
            Ok(SimulatorStatus::Running)
        }
        4 => {
            let addr = arg.reg(RV32IRegister::A0 as i32);
            let sim = arg.sim.as_ref();
            let mut buf = Vec::new();
            for i in 0.. {
                let byte = sim.mem[addr + i as u32];
                if byte == 0 {
                    break;
                }
                buf.push(byte);
            }
            let s = String::from_utf8(buf).unwrap();
            syscall_output_print(arg.get_path(), &s)?;
            arg.pc_step();
            Ok(SimulatorStatus::Running)
        }
        5 => {
            arg.sim.as_mut().wait_input = WaitStatus::Int;
            syscall_input_request(arg.get_path())?;
            Ok(SimulatorStatus::Paused)
        }
        8 => {
            arg.sim.as_mut().wait_input = WaitStatus::String;
            syscall_input_request(arg.get_path())?;
            Ok(SimulatorStatus::Paused)
        }
        10 => Ok(SimulatorStatus::Stopped),
        11 => {
            syscall_output_print(
                arg.get_path(),
                &((arg.reg(RV32IRegister::A0 as i32) as u8 as char).to_string()),
            )?;
            arg.pc_step();
            Ok(SimulatorStatus::Running)
        }
        12 => {
            arg.sim.as_mut().wait_input = WaitStatus::Char;
            syscall_input_request(arg.get_path())?;
            Ok(SimulatorStatus::Paused)
        }
        34 => {
            syscall_output_print(
                arg.get_path(),
                &format!("0x{:08x}", arg.sim.as_ref().reg[RV32IRegister::A0 as usize]),
            )?;
            arg.pc_step();
            Ok(SimulatorStatus::Running)
        }
        35 => {
            syscall_output_print(
                arg.get_path(),
                &format!(
                    "0x{:032b}",
                    arg.sim.as_ref().reg[RV32IRegister::A0 as usize]
                ),
            )?;
            arg.pc_step();
            Ok(SimulatorStatus::Running)
        }
        36 => {
            syscall_output_print(
                arg.get_path(),
                &((arg.reg(RV32IRegister::A0 as i32)).to_string()),
            )?;
            arg.pc_step();
            Ok(SimulatorStatus::Running)
        }
        _ => Err("Invalid ecall number".to_string()),
    }
}

pub(super) fn fence_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    arg.pc_step();
    Ok(SimulatorStatus::Running)
}

pub(super) fn fence_i_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    arg.pc_step();
    Ok(SimulatorStatus::Running)
}

pub(super) fn jal_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    let pc = arg.pc();
    *arg.reg_mut(arg[0]) = pc + 4;
    jump_helper(&arg, pc + arg[1] as u32)?;
    Ok(SimulatorStatus::Running)
}

pub(super) fn jalr_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    *arg.reg_mut(arg[0]) = arg.pc() + 4;
    jump_helper(&arg, (arg.reg(arg[1]) + arg[2] as u32) & !1)?;
    Ok(SimulatorStatus::Running)
}

pub(super) fn lb_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    load_helper!(arg, 1, i8)
}

pub(super) fn lbu_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    load_helper!(arg, 1, u8)
}

pub(super) fn lh_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    load_helper!(arg, 2, i16)
}

pub(super) fn lhu_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    load_helper!(arg, 2, u16)
}

pub(super) fn lui_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    *arg.reg_mut(arg[0]) = (arg[1] << 12) as u32;
    arg.pc_step();
    Ok(SimulatorStatus::Running)
}

pub(super) fn lw_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    load_helper!(arg, 4, u32)
}

pub(super) fn or_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    *arg.reg_mut(arg[0]) = arg.reg(arg[1]) | arg.reg(arg[2]);
    arg.pc_step();
    Ok(SimulatorStatus::Running)
}

pub(super) fn ori_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    *arg.reg_mut(arg[0]) = arg.reg(arg[1]) | arg[2] as u32;
    arg.pc_step();
    Ok(SimulatorStatus::Running)
}

pub(super) fn sb_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    store_helper!(arg, 1, u8)
}

pub(super) fn sh_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    store_helper!(arg, 2, u16)
}

pub(super) fn sll_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    *arg.reg_mut(arg[0]) = arg.reg(arg[1]) << (arg.reg(arg[2]) & 0x1f);
    arg.pc_step();
    Ok(SimulatorStatus::Running)
}

pub(super) fn slli_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    *arg.reg_mut(arg[0]) = arg.reg(arg[1]) << arg[2];
    arg.pc_step();
    Ok(SimulatorStatus::Running)
}

pub(super) fn slt_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    *arg.reg_mut(arg[0]) = if (arg.reg(arg[1]) as i32) < (arg.reg(arg[2]) as i32) {
        1
    } else {
        0
    };
    arg.pc_step();
    Ok(SimulatorStatus::Running)
}

pub(super) fn slti_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    *arg.reg_mut(arg[0]) = if (arg.reg(arg[1]) as i32) < arg[2] as i32 {
        1
    } else {
        0
    };
    arg.pc_step();
    Ok(SimulatorStatus::Running)
}

pub(super) fn sltiu_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    *arg.reg_mut(arg[0]) = if arg.reg(arg[1]) < arg[2] as u32 {
        1
    } else {
        0
    };
    arg.pc_step();
    Ok(SimulatorStatus::Running)
}

pub(super) fn sltu_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    *arg.reg_mut(arg[0]) = if arg.reg(arg[1]) < arg.reg(arg[2]) {
        1
    } else {
        0
    };
    arg.pc_step();
    Ok(SimulatorStatus::Running)
}

pub(super) fn sra_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    *arg.reg_mut(arg[0]) = (arg.reg(arg[1]) as i32 >> (arg.reg(arg[2]) & 0x1f)) as u32;
    arg.pc_step();
    Ok(SimulatorStatus::Running)
}

pub(super) fn srai_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    *arg.reg_mut(arg[0]) = (arg.reg(arg[1]) as i32 >> arg[2] as i32) as u32;
    arg.pc_step();
    Ok(SimulatorStatus::Running)
}

pub(super) fn srl_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    *arg.reg_mut(arg[0]) = arg.reg(arg[1]) >> (arg.reg(arg[2]) & 0x1f);
    arg.pc_step();
    Ok(SimulatorStatus::Running)
}

pub(super) fn srli_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    *arg.reg_mut(arg[0]) = arg.reg(arg[1]) >> arg[2];
    arg.pc_step();
    Ok(SimulatorStatus::Running)
}

pub(super) fn sub_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    *arg.reg_mut(arg[0]) = arg.reg(arg[1]) - arg.reg(arg[2]);
    arg.pc_step();
    Ok(SimulatorStatus::Running)
}

pub(super) fn sw_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    store_helper!(arg, 4, u32)
}

pub(super) fn xor_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    *arg.reg_mut(arg[0]) = arg.reg(arg[1]) ^ arg.reg(arg[2]);
    arg.pc_step();
    Ok(SimulatorStatus::Running)
}

pub(super) fn xori_handler(arg: InstHandlerArg) -> Result<SimulatorStatus, String> {
    *arg.reg_mut(arg[0]) = arg.reg(arg[1]) ^ arg[2] as u32;
    arg.pc_step();
    Ok(SimulatorStatus::Running)
}

fn jump_helper(arg: &InstHandlerArg, offset: u32) -> Result<(), String> {
    if arg.set_pc(arg.pc() + offset) {
        Ok(())
    } else {
        Err("Invalid aim pc".to_string())
    }
}

impl<'a> Index<usize> for InstHandlerArg<'a> {
    type Output = Operand<RISCV>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.args[index]
    }
}

impl<'a> InstHandlerArg<'a> {
    fn reg(&self, index: Operand<RISCV>) -> u32 {
        self.sim.as_ref().reg[index as usize]
    }

    fn reg_mut(&self, index: Operand<RISCV>) -> &mut u32 {
        let sim = self.sim.as_mut();
        let history = self.history.as_mut();
        history.reg_idx = index as i32;
        history.reg_val = sim.reg[index as usize];
        &mut sim.reg[index as usize]
    }

    fn pc_idx(&self) -> usize {
        self.sim.as_ref().pc_idx
    }

    fn pc(&self) -> u32 {
        self.sim.as_ref().to_text_addr(self.sim.as_ref().pc_idx)
    }

    fn set_pc(&self, addr: u32) -> bool {
        let sim = self.sim.as_mut();
        match sim.to_text_idx(addr) {
            Some(idx) => {
                sim.pc_idx = idx;
                true
            }
            None => false,
        }
    }

    fn pc_step(&self) {
        self.sim.as_mut().pc_idx += 1;
    }

    fn get_path(&self) -> &str {
        &self.sim.as_ref().file
    }
}
