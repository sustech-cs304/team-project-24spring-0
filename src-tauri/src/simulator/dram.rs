/// Default dram size (64MiB).
pub const DRAM_SIZE: u32 = 1024 * 1024 * 64;
/// The address which dram starts, same as QEMU virt machine.
pub const DRAM_BASE: u32 = 0x8000_0000;
pub struct Dram {
    pub dram: Vec<u8>,
}

impl Dram {
    pub fn new(mem_size: usize) -> Self {
        Dram {
            dram: vec![0; mem_size * 1024 * 1024],
        }
    }

    /// Load bytes from the little-endiam dram.
    pub fn load(&self, addr: u32, size: u32) -> Result<u32, ()> {
        match size {
            8 => Ok(self.load8(addr)),
            16 => Ok(self.load16(addr)),
            32 => Ok(self.load32(addr)),
            64 => Ok(self.load64(addr)),
            _ => Err(()),
        }
    }

    /// Load a byte from the little-endian dram.
    fn load8(&self, addr: u32) -> u32 {
        let index = (addr - DRAM_BASE) as usize;
        self.dram[index] as u32
    }

    /// Load 2 bytes from the little-endian dram.
    fn load16(&self, addr: u32) -> u32 {
        let index = (addr - DRAM_BASE) as usize;
        return (self.dram[index] as u32) | ((self.dram[index + 1] as u32) << 8);
    }

    /// Load 4 bytes from the little-endian dram.
    fn load32(&self, addr: u32) -> u32 {
        let index = (addr - DRAM_BASE) as usize;
        return (self.dram[index] as u32)
            | ((self.dram[index + 1] as u32) << 8)
            | ((self.dram[index + 2] as u32) << 16)
            | ((self.dram[index + 3] as u32) << 24);
    }

    /// Load 8 bytes from the little-endian dram.
    fn load64(&self, addr: u32) -> u32 {
        let index = (addr - DRAM_BASE) as usize;
        return (self.dram[index] as u32)
            | ((self.dram[index + 1] as u32) << 8)
            | ((self.dram[index + 2] as u32) << 16)
            | ((self.dram[index + 3] as u32) << 24)
            | ((self.dram[index + 4] as u32) << 32)
            | ((self.dram[index + 5] as u32) << 40)
            | ((self.dram[index + 6] as u32) << 48)
            | ((self.dram[index + 7] as u32) << 56);
    }


    // Store bytes to the little-endiam dram.
    // pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), ()> {
    //     match size {
    //         8 => Ok(self.store8(addr, value)),
    //         16 => Ok(self.store16(addr, value)),
    //         32 => Ok(self.store32(addr, value)),
    //         64 => Ok(self.store64(addr, value)),
    //         _ => Err(()),
    //     }
    // }

    // /// Store a byte to the little-endian dram.
    // fn store8(&mut self, addr: u64, value: u64) {
    //     let index = (addr - DRAM_BASE) as usize;
    //     self.dram[index] = value as u8
    // }

    // /// Store 2 bytes to the little-endian dram.
    // fn store16(&mut self, addr: u64, value: u64) {
    //     let index = (addr - DRAM_BASE) as usize;
    //     self.dram[index] = (value & 0xff) as u8;
    //     self.dram[index + 1] = ((value >> 8) & 0xff) as u8;
    // }

    // /// Store 4 bytes to the little-endian dram.
    // fn store32(&mut self, addr: u64, value: u64) {
    //     let index = (addr - DRAM_BASE) as usize;
    //     self.dram[index] = (value & 0xff) as u8;
    //     self.dram[index + 1] = ((value >> 8) & 0xff) as u8;
    //     self.dram[index + 2] = ((value >> 16) & 0xff) as u8;
    //     self.dram[index + 3] = ((value >> 24) & 0xff) as u8;
    // }

    // /// Store 8 bytes to the little-endian dram.
    // fn store64(&mut self, addr: u64, value: u64) {
    //     let index = (addr - DRAM_BASE) as usize;
    //     self.dram[index] = (value & 0xff) as u8;
    //     self.dram[index + 1] = ((value >> 8) & 0xff) as u8;
    //     self.dram[index + 2] = ((value >> 16) & 0xff) as u8;
    //     self.dram[index + 3] = ((value >> 24) & 0xff) as u8;
    //     self.dram[index + 4] = ((value >> 32) & 0xff) as u8;
    //     self.dram[index + 5] = ((value >> 40) & 0xff) as u8;
    //     self.dram[index + 6] = ((value >> 48) & 0xff) as u8;
    //     self.dram[index + 7] = ((value >> 56) & 0xff) as u8;
    // }
}
