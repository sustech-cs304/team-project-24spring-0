pub trait RpcServer {
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn stop(&mut self);
}

pub trait RpcClient {
    async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn disconnect(&mut self);
}
