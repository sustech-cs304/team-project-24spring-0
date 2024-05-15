use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use editor_rpc::editor_client::EditorClient;
use editor_rpc::OperationType::{Delete, Insert, Replace};
use editor_rpc::{
    AuthorizeReply, AuthorizeRequest, OperationType, SetCursorReply, SetCursorRequest,
    UpdateContentReply, UpdateContentRequest,
};

use crate::interface::remote::RpcClient;
use std::convert::TryFrom;
use tonic::transport::Endpoint;

pub mod editor_rpc {
    tonic::include_proto!("editor");
}

struct RpcClientImpl {
    addr: SocketAddr,
    password: String,
    client: Option<EditorClient<tonic::transport::Channel>>,
}

impl RpcClientImpl {
    pub fn new(ip: IpAddr, port: u16, password: &str) -> Self {
        Self {
            addr: SocketAddr::new(ip, port),
            password: password.to_string(),
            client: None,
        }
    }
}

impl RpcClient for RpcClientImpl {
    async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let uri = format!("http://{}", self.addr);
        let endpoint = Endpoint::try_from(uri)?;
        let client = EditorClient::connect(endpoint).await?;
        self.client = Some(client);
        Ok(())
    }

    fn disconnect(&mut self) {
        self.client = None;
    }
}
