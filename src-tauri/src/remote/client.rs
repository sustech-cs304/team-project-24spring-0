use std::{convert::TryFrom, error::Error, net::SocketAddr, sync::Mutex, time::Duration};

use tauri::async_runtime::block_on;
use tokio::time::timeout;
use tonic::{transport::Endpoint, Request};

use super::{
    server::editor_rpc::{
        editor_client::EditorClient,
        AuthorizeRequest,
        ContentPosition,
        DisconnectRequest,
        GetContentReply,
        GetContentRequest,
        SetCursorRequest,
        UpdateContentRequest,
    },
    Modification,
};
use crate::{
    interface::remote::RpcClient,
    remote::server::editor_rpc::UpdateContentReply,
    types::ResultVoid,
};

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

    pub async fn send_authorize(
        &mut self,
        password: &str,
    ) -> Result<(String, u64, String), Box<dyn Error>> {
        self.should_running()?;
        let request = Request::new(AuthorizeRequest {
            password: password.to_string(),
        });
        let reply = match timeout(
            Duration::from_secs(2),
            self.client.as_mut().unwrap().authorize(request),
        )
        .await
        {
            Ok(reply) => reply?,
            Err(_) => return Err("Timeout".into()),
        };
        let reply_ref = reply.get_ref();
        if reply_ref.success {
            Ok((
                reply_ref.file_name.to_owned(),
                reply_ref.version,
                reply_ref.content.to_owned(),
            ))
        } else {
            Err("Authorize failed".into())
        }
    }

    pub async fn send_disconnect(&mut self) -> ResultVoid {
        let request = Request::new(DisconnectRequest {});

        let reply = match timeout(
            Duration::from_secs(2),
            self.client.as_mut().unwrap().disconnect(request),
        )
        .await
        {
            Ok(reply) => reply?,
            Err(_) => return Err("Timeout".into()),
        };
        let reply_ref = reply.get_ref();
        if !reply_ref.success {
            return Err("Disconnect failed".into());
        } else {
            Ok(())
        }
    }

    pub async fn send_set_cursor(&mut self, row: u64, col: u64) -> ResultVoid {
        let request = Request::new(SetCursorRequest { row, col });
        let reply = match timeout(
            Duration::from_secs(1),
            self.client.as_mut().unwrap().set_cursor(request),
        )
        .await
        {
            Ok(reply) => reply?,
            Err(_) => return Err("Timeout".into()),
        };
        if !reply.get_ref().success {
            Err("Failed to set cursor, the line already use by others".into())
        } else {
            Ok(())
        }
    }

    pub async fn send_get_content(
        &mut self,
        version: u64,
    ) -> Result<GetContentReply, Box<dyn Error>> {
        let request = Request::new(GetContentRequest {
            version,
            full_content: false,
        });
        let reply = match timeout(
            Duration::from_secs(1),
            self.client.as_mut().unwrap().get_content(request),
        )
        .await
        {
            Ok(reply) => reply?,
            Err(_) => return Err("Timeout".into()),
        };
        Ok(reply.get_ref().clone())
    }

    pub async fn send_update_content(
        &mut self,
        version: u64,
        history: &Modification,
    ) -> Result<UpdateContentReply, Box<dyn Error>> {
        let pos: ContentPosition = history.op_range.clone().into();
        let request = Request::new(UpdateContentRequest {
            version,
            op: history.op.clone().into(),
            op_range: Some(pos),
            modified_content: history.modified_content.clone(),
        });
        let reply = match timeout(
            Duration::from_secs(1),
            self.client.as_mut().unwrap().update_content(request),
        )
        .await
        {
            Ok(reply) => reply?,
            Err(_) => return Err("Timeout".into()),
        };
        Ok(reply.get_ref().clone())
    }
}

impl RpcClient for RpcClientImpl {
    async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        let uri = format!("https://{}", self.server_addr.lock().unwrap());
        let endpoint = Endpoint::try_from(uri)?;
        let client = EditorClient::connect(endpoint).await?;
        self.client = Some(client);
        Ok(())
    }

    fn disconnect(&mut self) {
        self.client = None;
    }
}
