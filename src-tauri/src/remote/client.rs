use std::{
    convert::TryFrom,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use editor_rpc::{
    editor_client::EditorClient,
    AuthorizeReply,
    AuthorizeRequest,
    OperationType,
    OperationType::{Delete, Insert, Replace},
    SetCursorReply,
    SetCursorRequest,
    UpdateContentReply,
    UpdateContentRequest,
};
use tonic::transport::Endpoint;

use crate::interface::remote::RpcClient;

pub mod editor_rpc {
    tonic::include_proto!("editor");
}

pub struct RpcClientImpl {
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
