use crate::simulator::dram::*;

/// The address which dram starts, same as QEMU virt machine.
pub const DRAM_BASE: u64 = 0x8000_0000;

/// The system bus.
pub struct Bus {
    dram: Dram,
}

// impl Bus {
//     pub fn new(code: usize) -> Bus {
//         Self {
//             dram: Dram::new(code),
//         }
//     }

//     pub fn load(&self, addr: u64, size: u64) -> Result<u64, ()> {
//         if DRAM_BASE <= addr {
//             return self.dram.load(addr, size);
//         }
//         Err(())
//     }
//     pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), ()> {
//         if DRAM_BASE <= addr {
//             return self.dram.store(addr, size, value);
//         }
//         Err(())
//     }
// }
