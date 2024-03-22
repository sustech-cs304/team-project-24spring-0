trait Frontend {
    //TODO
    // file operation
    //fn file_operation(&mut self, op: FileOp, path: String) -> String;
    //fn assembler_operation(&mut self, op: AssemblerOp, path: String) -> String;
    //fn char_insert(&mut self, pos: (i32, i32), content: char) -> bool;
    //fn char_delete(&mut self, pos: (i32, i32)) -> bool;
    //fn text_insert(&mut self, pos: (i32, i32), content: String) -> bool;
    //fn text_delete(&mut self, beg: (i32, i32), end: (i32, i32)) -> bool;
    //fn simulator_operation(&mut self, op: SimulatorOp) -> String;

    // assembler operation
    fn assemble(&mut self) -> String;
}
