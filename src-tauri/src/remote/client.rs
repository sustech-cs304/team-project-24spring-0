use std::{convert::TryFrom, net::SocketAddr, sync::Mutex, time::Duration};

use editor_rpc::editor_client::EditorClient;
use tauri::async_runtime::block_on;
use tokio::time::timeout;
use tonic::{transport::Endpoint, Request};

use crate::{interface::remote::RpcClient, types::ResultVoid};

pub mod editor_rpc {
    tonic::include_proto!("editor");
}

pub struct RpcClientImpl {
    server_addr: Mutex<SocketAddr>,
    client: Option<EditorClient<tonic::transport::Channel>>,
}

impl Default for RpcClientImpl {
    fn default() -> Self {
        Self {
            server_addr: Mutex::new(SocketAddr::new("127.0.0.1".parse().unwrap(), 0)),
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

    pub fn start(&mut self) -> ResultVoid {
        self.should_not_running()?;
        block_on(self.connect())?;
        Ok(())
    }

    pub fn stop(&mut self) -> ResultVoid {
        self.should_running()?;
        self.disconnect();
        Ok(())
    }

    pub fn set_server_addr(&mut self, server_addr: SocketAddr) -> ResultVoid {
        self.should_not_running()?;
        *self.server_addr.lock().unwrap() = server_addr;
        Ok(())
    }

    pub async fn send_authorize(&mut self, password: &str) -> ResultVoid {
        self.should_running()?;
        let req = Request::new(editor_rpc::AuthorizeRequest {
            password: password.to_string(),
        });
        let res = match timeout(
            Duration::from_secs(2),
            self.client.as_mut().unwrap().authorize(req),
        )
        .await
        {
            Ok(res) => res?,
            Err(_) => return Err("Timeout".into()),
        };
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
