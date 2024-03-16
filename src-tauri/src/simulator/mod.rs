trait LoadAssembly {
    fn load_assembly<T>(&self, assembly: Vec<T>);
    fn load_memory<T>(&self, memory: Vec<T>);
}

trait RunAssembly {
    fn run_all<T>(&self);
    fn run_step<T>(&self);
}

trait ReadStatus {
    fn read_all_regs<T>(&self) -> Vec<T>;
    fn read_memory<T>(&self, range: (T, T));
}

trait IO {
    fn input(&self, input: String);
    fn set_output<T>(&self, output: T);
}

mod mips;
