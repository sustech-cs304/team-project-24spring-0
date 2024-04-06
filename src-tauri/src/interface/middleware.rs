pub trait Middleware<OP, DATA> {
    fn syscall_out(&mut self, op: OP, data: DATA);
}
