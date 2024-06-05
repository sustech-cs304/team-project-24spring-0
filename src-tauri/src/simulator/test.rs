mod memory {
    use super::super::memory::Memory;

    #[test]
    fn test() {
        let mut mem = Memory::new();
        mem[0] = 1;
        assert_eq!(mem[0], 1);
        assert_eq!(mem[1], 0);
        assert_eq!(mem[0x54365453u32], 0);
        let start = 0x95478806u32;
        mem.set_range(start, &[1, 2]);
        assert_eq!(mem.get_range(start - 2, 6), [0, 0, 1, 2, 0, 0]);
        let start = 0x07637855u32;
        const LEN: usize = 16666;
        mem.set_range(start, &[1; LEN]);
        assert_eq!(mem.get_range(start, LEN as u32), [1; 16666]);
        mem.reset();
    }
}
