use std::{convert::TryFrom, net::SocketAddr, sync::Mutex};

use editor_rpc::editor_client::EditorClient;
use tonic::transport::Endpoint;

use crate::{interface::remote::RpcClient, types::ResultVoid};

pub mod editor_rpc {
    tonic::include_proto!("editor");
}

pub struct RpcClientImpl {
    server_addr: Mutex<SocketAddr>,
    password: Mutex<String>,
    client: Option<EditorClient<tonic::transport::Channel>>,
}

impl Default for RpcClientImpl {
    fn default() -> Self {
        Self {
            server_addr: Mutex::new(SocketAddr::new("127.0.0.1".parse().unwrap(), 0)),
            password: Default::default(),
            client: None,
        }
    }
}
impl RpcClientImpl {
    fn should_not_running(&self) -> ResultVoid {
        if self.client.is_some() {
            return Err("Client is running, you need disconnect first".into());
        }
        Ok(())
    }

    fn should_running(&self) -> ResultVoid {
        if self.client.is_none() {
            return Err("Client is not running, you need connect first".into());
        }
        Ok(())
    }

    pub fn set_server_addr(&mut self, server_addr: SocketAddr) -> ResultVoid {
        self.should_not_running()?;
        *self.server_addr.lock().unwrap() = server_addr;
        Ok(())
    }

    pub fn set_password(&mut self, password: String) -> ResultVoid {
        self.should_not_running()?;
        *self.password.lock().unwrap() = password;
        Ok(())
    }

    pub fn send_authorize(&mut self) -> ResultVoid {
        self.should_running()?;
        //TODO
        Ok(())
    }
}

impl RpcClient for RpcClientImpl {
    async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let uri = format!("http://{}", self.server_addr.lock().unwrap());
        let endpoint = Endpoint::try_from(uri)?;
        let client = EditorClient::connect(endpoint).await?;
        self.client = Some(client);
        Ok(())
    }

    fn disconnect(&mut self) {
        self.client = None;
    }
}
