use editor_rpc::editor_server::{Editor, EditorServer};
use editor_rpc::{
    AuthorizeReply, AuthorizeRequest, DisconnectReply, DisconnectRequest, SetCursorReply,
    SetCursorRequest, UpdateContentReply, UpdateContentRequest,
};
use std::net::SocketAddr;
use std::sync::RwLock;
use tauri::Manager;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use crate::interface::remote::RpcServer;
use crate::types::middleware_types::CurTabName;
use crate::APP_HANDLE;

pub mod editor_rpc {
    tonic::include_proto!("editor");
}

#[derive(Debug, Default)]
struct ServerImpl {
    pub password: RwLock<String>,
    pub clients: RwLock<Vec<SocketAddr>>,
}

#[tonic::async_trait]
impl Editor for ServerImpl {
    async fn authorize(
        &self,
        request: Request<AuthorizeRequest>,
    ) -> Result<Response<AuthorizeReply>, Status> {
        if request.get_ref().password == self.password.read().unwrap().as_str() {
            let mut file = "Server have not iniitilized".to_string();
            let mut success = false;
            if let Some(app_handle) = APP_HANDLE.lock().unwrap().as_ref() {
                let mut lock = self.clients.write().unwrap();
                lock.push(request.remote_addr().unwrap());
                let cur_tab_name = app_handle.state::<CurTabName>();
                let lock = cur_tab_name.name.lock().unwrap();
                file = lock.clone();
                success = true;
            }
            Ok(Response::new(AuthorizeReply { success, file }))
        } else {
            Ok(Response::new(AuthorizeReply {
                success: false,
                file: "Unauthorized".to_string(),
            }))
        }
    }

    async fn disconnect(
        &self,
        request: Request<editor_rpc::DisconnectRequest>,
    ) -> Result<Response<editor_rpc::DisconnectReply>, Status> {
        let mut lock = self.clients.write().unwrap();
        let mut success = false;
        if let Some(pos) = lock
            .iter()
            .position(|x| *x == request.remote_addr().unwrap())
        {
            lock.remove(pos);
            success = true;
        }
        Ok(Response::new(editor_rpc::DisconnectReply { success }))
    }

    async fn set_cursor(
        &self,
        request: Request<editor_rpc::SetCursorRequest>,
    ) -> Result<Response<editor_rpc::SetCursorReply>, Status> {
        Ok(Response::new(editor_rpc::SetCursorReply { success: true }))
    }

    async fn update_content(
        &self,
        request: Request<editor_rpc::UpdateContentRequest>,
    ) -> Result<Response<editor_rpc::UpdateContentReply>, Status> {
        Ok(Response::new(editor_rpc::UpdateContentReply {
            success: true,
            content: "foo".to_string(),
        }))
    }
}

impl ServerImpl {
    pub fn change_password(&self, password: String) {
        let mut lock = self.password.write().unwrap();
        *lock = password;
    }
}

pub struct RpcServerImpl {
    addr: SocketAddr,
    server_handle: Option<JoinHandle<()>>,
}

impl RpcServerImpl {
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            addr,
            server_handle: None,
        }
    }
}

impl RpcServer for RpcServerImpl {
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = self.addr;
        let handler = ServerImpl::default();

        let server_handle = tokio::spawn(async move {
            Server::builder()
                .add_service(EditorServer::new(handler))
                .serve(addr)
                .await
                .unwrap();
        });

        self.server_handle = Some(server_handle);

        Ok(())
    }

    fn stop(&mut self) {
        if let Some(handle) = self.server_handle.take() {
            handle.abort();
        }
    }
}
