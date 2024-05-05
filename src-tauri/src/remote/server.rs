use crate::interface::remote::RpcServer;
use crate::types::middleware_types::{CurTabName, Tab, TabMap};
use crate::utility::ptr::Ptr;
use crate::APP_HANDLE;
use editor_rpc::editor_server::{Editor, EditorServer};
use editor_rpc::{
    AuthorizeReply, AuthorizeRequest, DisconnectReply, DisconnectRequest, GetContentReply,
    GetContentRequest, SetCursorReply, SetCursorRequest, UpdateContentReply, UpdateContentRequest,
};
use std::collections::LinkedList;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tauri::Manager;
use tokio::task::JoinHandle;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub mod editor_rpc {
    tonic::include_proto!("editor");
}

#[derive(Default)]
struct ServerHandle {
    cur_tab: Mutex<(String, Option<Ptr<TabMap>>)>,
    password: Mutex<String>,
    clients: Mutex<Vec<SocketAddr>>,
    cursor_pos: Mutex<LinkedList<(SocketAddr, u64, u64)>>,
}

impl ServerHandle {
    /// Change password for server.
    fn change_password(&self, password: &str) {
        let mut lock = self.password.lock().unwrap();
        *lock = password.to_string();
    }

    /// Update host cursor position.
    fn udpate_host_cursor(&mut self, row: u64, column: u64) {
        let mut lock = self.cursor_pos.lock().unwrap();
        if let Some((_, pos)) = lock
            .iter_mut()
            .enumerate()
            .find(|(_, x)| x.0 == "127.0.0.1".parse().unwrap())
        {
            pos.1 = row;
            pos.2 = column;
        } else {
            lock.push_back(("127.0.0.1".parse().unwrap(), row, column));
        }
    }
}

#[tonic::async_trait]
impl Editor for Arc<Mutex<ServerHandle>> {
    async fn authorize(
        &self,
        request: Request<AuthorizeRequest>,
    ) -> Result<Response<AuthorizeReply>, Status> {
        let handler = self.lock().unwrap();
        if request.get_ref().password == handler.password.lock().unwrap().as_str() {
            let mut file = "Server have not iniitilized".to_string();
            let mut success = false;
            if let Some(app_handle) = APP_HANDLE.lock().unwrap().as_ref() {
                let mut clients = handler.clients.lock().unwrap();
                clients.push(request.remote_addr().unwrap());
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
        request: Request<DisconnectRequest>,
    ) -> Result<Response<DisconnectReply>, Status> {
        let handler = self.lock().unwrap();
        let mut clients = handler.clients.lock().unwrap();
        let mut success = false;
        if let Some(pos) = clients
            .iter()
            .position(|x| *x == request.remote_addr().unwrap())
        {
            clients.remove(pos);
            success = true;
        }
        Ok(Response::new(DisconnectReply { success }))
    }

    async fn set_cursor(
        &self,
        request: Request<SetCursorRequest>,
    ) -> Result<Response<SetCursorReply>, Status> {
        let handler = self.lock().unwrap();
        let mut cursor_lock = handler.cursor_pos.lock().unwrap();
        if let Some(_) = cursor_lock
            .iter()
            .enumerate()
            .find(|(_, x)| x.0 == request.remote_addr().unwrap())
        {
            Ok(Response::new(editor_rpc::SetCursorReply { success: false }))
        } else {
            cursor_lock.push_back((
                request.remote_addr().unwrap(),
                request.get_ref().row,
                request.get_ref().column,
            ));
            Ok(Response::new(editor_rpc::SetCursorReply { success: true }))
        }
    }

    async fn get_content(
        &self,
        request: Request<GetContentRequest>,
    ) -> Result<Response<GetContentReply>, Status> {
        Ok(Response::new(GetContentReply {
            need_update: true,
            content: "".to_string(),
        }))
    }

    async fn update_content(
        &self,
        request: Request<UpdateContentRequest>,
    ) -> Result<Response<UpdateContentReply>, Status> {
        todo!("update_content");
        Ok(Response::new(UpdateContentReply {
            success: true,
            content: "foo".to_string(),
        }))
    }
}
pub struct RpcServerImpl {
    port: u16,
    tokio_runtime: tokio::runtime::Runtime,
    server_handle: Option<JoinHandle<()>>,
    shared_handler: Arc<Mutex<ServerHandle>>,
}

impl RpcServerImpl {
    fn default() -> Self {
        Self {
            port: 11451,
            tokio_runtime: tokio::runtime::Builder::new_multi_thread()
                .worker_threads(4)
                .enable_all()
                .build()
                .unwrap(),
            server_handle: None,
            shared_handler: Arc::new(Mutex::new(Default::default())),
        }
    }

    pub fn start_service(
        &mut self,
        cur_tab_name: String,
        tab_map: Ptr<TabMap>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.shared_handler.lock().unwrap().cur_tab = Mutex::new((cur_tab_name, Some(tab_map)));
        self.start()?;
        Ok(())
    }

    pub fn stop_service(&mut self) {
        self.stop();
    }

    pub fn change_port(&mut self, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        self.stop();
        self.port = port;
        self.start()?;
        Ok(())
    }

    pub fn change_password(&mut self, new_password: &str) -> bool {
        let handler = self.shared_handler.lock().unwrap();
        handler.change_password(new_password);
        true
    }
}

impl Default for RpcServerImpl {
    fn default() -> Self {
        Self::default()
    }
}

impl RpcServer for RpcServerImpl {
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("0.0.0.0:{:?}", self.port).parse().unwrap();
        println!("Server listening on: {:?}", addr);
        let handler = Arc::clone(&self.shared_handler);

        let server_handle = self.tokio_runtime.spawn(async move {
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
