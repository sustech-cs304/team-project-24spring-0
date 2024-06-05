use std::sync::{Condvar, Mutex};

use crate::{
    interface::simulator::{FakeMiddlewareTrait, Simulator},
    simulator::simulator::RISCVSimulator,
    utility::ptr::Ptr,
};

pub struct FakeMiddleware {
    pub input: Option<String>,
    pub input_res: Option<Result<(), String>>,
    pub output: Option<String>,
    pub sim_ptr: Ptr<RISCVSimulator>,
    pub success: bool,
    pub cv: (Condvar, Mutex<()>),
}

impl FakeMiddlewareTrait for FakeMiddleware {
    fn request_input(&mut self) {
        let self_ptr = Ptr::new(self);
        std::thread::spawn(move || {
            let _self = self_ptr.as_mut();
            std::thread::sleep(std::time::Duration::from_millis(100));
            _self.input_res = Some(
                _self
                    .sim_ptr
                    .as_mut()
                    .syscall_input(_self.input.as_ref().unwrap()),
            );
        });
    }

    fn output(&mut self, output: &str) {
        self.output = Some(output.to_string());
    }

    fn update(&mut self, res: crate::types::middleware_types::Optional) {
        self.success = res.success;
        self.cv.0.notify_one();
    }
}
