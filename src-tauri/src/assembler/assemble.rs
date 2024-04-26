use crate::interface::assembler::Assembler;
pub struct RiscVAssembler;

impl RiscVAssembler {
    fn assemble<IN, OUT, SET, ERR>(&mut self, ast: &IN) -> Result<OUT, ERR> {
        unimplemented!()
    }
    fn dump<IN, OUT, SET, ERR>(&self, ast: &IN) -> Result<String, ERR> {
        unimplemented!()
    }

    fn update_setting<IN, OUT, SET, ERR>(
        &mut self,
        settings: &SET,
    ) -> Result<bool, String> {
        unimplemented!()
    }
}

impl<IN, OUT, SET, ERR> Assembler<IN, OUT, SET, ERR> for RiscVAssembler {
    fn assemble(&mut self, ast: &IN) -> Result<OUT, ERR> {
        unimplemented!()
    }

    fn dump(&self, ast: &IN) -> Result<String, ERR> {
        unimplemented!()
    }

    fn update_setting(&mut self, settings: &SET) -> Result<bool, String> {
        unimplemented!()
    }
}
