use std::net::Ipv4Addr;

use editor_rpc::editor_client::EditorClient;
use editor_rpc::{
    AuthorizeReply, AuthorizeRequest, SetCursorReply, SetCursorRequest, UpdateContentReply,
    UpdateContentRequest,
};

use crate::interface::remote::RpcClient;

pub mod editor_rpc {
    tonic::include_proto!("editor");
}

struct RpcClientImpl {
    addr: String,
    client: Option<EditorClient<tonic::transport::Channel>>,
}
impl RpcClientImpl {
    pub fn new(ipv4: &str, port: u16) -> Self {
        Self {
            addr: format!("http://{}:{}", ipv4, port),
            client: None,
        }
    }
}
impl RpcClient for RpcClientImpl {
    async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let client = EditorClient::connect(self.addr.parse::<tonic::transport::Uri>()?).await?;
        self.client = Some(client);
        Ok(())
    }

    fn disconnect(&mut self) {
        self.client = None;
    }
}
