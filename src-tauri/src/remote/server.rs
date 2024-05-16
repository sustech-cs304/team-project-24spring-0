use std::{
    collections::LinkedList,
    error::Error,
    net::SocketAddr,
    sync::{atomic, Arc, Mutex},
};

use editor_rpc::{
    editor_server::{Editor, EditorServer},
    AuthorizeReply,
    AuthorizeRequest,
    DisconnectReply,
    DisconnectRequest,
    GetContentReply,
    GetContentRequest,
    OperationType,
    OperationType::{Delete, Insert, Replace},
    SetCursorReply,
    SetCursorRequest,
    UpdateContentReply,
    UpdateContentRequest,
};
use tauri::utils::debug_eprintln;
use tokio::task::JoinHandle;
use tonic::{transport::Server, Request, Response, Status};

use super::{ClientCursor, History};
use crate::{
    interface::remote::RpcServer,
    types::middleware_types::{CurTabName, Tab, TabMap, TextPosition},
    utility::ptr::Ptr,
};

pub mod editor_rpc {
    tonic::include_proto!("editor");
}

#[derive(Default)]
struct ServerHandle {
    map_state: Mutex<(String, Option<Ptr<TabMap>>)>,
    version: atomic::AtomicUsize,
    password: Mutex<String>,
    clients: Mutex<Vec<SocketAddr>>,
    cursor_pos: Mutex<LinkedList<ClientCursor>>,
    history: Mutex<Vec<History>>,
}

impl ServerHandle {
    /// Change password for server.
    fn change_password(&self, password: &str) {
        let mut lock = self.password.lock().unwrap();
        *lock = password.to_string();
    }

    /// Update host cursor position.
    fn udpate_host_cursor(&mut self, row: u64, col: u64) {
        let mut lock = self.cursor_pos.lock().unwrap();
        if let Some((_, pos)) = lock
            .iter_mut()
            .enumerate()
            .find(|(_, x)| x.ip == "127.0.0.1".parse().unwrap())
        {
            pos.row = row;
            pos.col = col;
        } else {
            lock.push_back(ClientCursor {
                ip: "127.0.0.1".parse().unwrap(),
                row,
                col,
            });
        }
    }

    /// Handle logic current tab wht a `&mut Tab` as lambda parameter.
    /// Only use to bypass the fxxking borrow checker.
    fn handle_with_cur_tab<F, R>(&self, handle: F) -> Result<R, String>
    where
        F: Fn(&mut Tab) -> Result<R, String>,
    {
        let map_state_lock = self.map_state.lock().unwrap();
        match map_state_lock.1 {
            Some(tab_map_ptr) => {
                let tab_map = tab_map_ptr.as_ref();
                let mut tabs_lock = tab_map.tabs.lock().unwrap();
                let mut tab = tabs_lock.get_mut(&map_state_lock.0).unwrap();
                handle(&mut tab)
            }
            None => Err("TabMap have not been iniitilized".to_string()),
        }
    }

    fn handle_rpc_with_cur_tab<F, R>(&self, handle: F) -> Result<tonic::Response<R>, Status>
    where
        F: Fn(&mut Tab) -> Result<R, String>,
    {
        match self.handle_with_cur_tab(handle) {
            Ok(success) => Ok(tonic::Response::new(success)),
            Err(err) => Err(Status::internal(err)),
        }
    }

    fn check_ip_authorized(&self, ip: SocketAddr) -> Result<(), Status> {
        let clients = self.clients.lock().unwrap();
        println!("{:?}", clients);
        if clients.iter().any(|x| *x == ip) {
            Ok(())
        } else {
            Err(Status::unauthenticated("Unauthorized"))
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
        handler.handle_rpc_with_cur_tab(|tab| {
            if request.get_ref().password == handler.password.lock().unwrap().as_str() {
                if let Ok(_) = handler.check_ip_authorized(request.remote_addr().unwrap()) {
                    Ok(AuthorizeReply {
                        success: true,
                        file_name: tab.text.get_path_str(),
                        version: handler.version.load(atomic::Ordering::Relaxed) as u64,
                        content: tab.text.to_string(),
                    })
                } else {
                    let mut client_lock = handler.clients.lock().unwrap();
                    client_lock.push(request.remote_addr().unwrap());
                    Ok(AuthorizeReply {
                        success: true,
                        file_name: tab.text.get_path_str(),
                        version: handler.version.load(atomic::Ordering::Relaxed) as u64,
                        content: tab.text.to_string(),
                    })
                }
            } else {
                Ok(AuthorizeReply {
                    success: false,
                    file_name: "".to_string(),
                    version: 0,
                    content: "".to_string(),
                })
            }
        })
    }

    async fn disconnect(
        &self,
        request: Request<DisconnectRequest>,
    ) -> Result<Response<DisconnectReply>, Status> {
        let handler = self.lock().unwrap();
        let _ = handler.check_ip_authorized(request.remote_addr().unwrap())?;
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
        let _ = handler.check_ip_authorized(request.remote_addr().unwrap())?;
        let mut cursor_lock = handler.cursor_pos.lock().unwrap();
        if let Some(_) = cursor_lock
            .iter()
            .enumerate()
            .find(|(_, x)| x.ip == request.remote_addr().unwrap())
        {
            Ok(Response::new(editor_rpc::SetCursorReply { success: false }))
        } else {
            cursor_lock.push_back(ClientCursor {
                ip: request.remote_addr().unwrap(),
                row: request.get_ref().row,
                col: request.get_ref().col,
            });
            Ok(Response::new(editor_rpc::SetCursorReply { success: true }))
        }
    }

    async fn get_content(
        &self,
        request: Request<GetContentRequest>,
    ) -> Result<Response<GetContentReply>, Status> {
        let handler = self.lock().unwrap();
        let _ = handler.check_ip_authorized(request.remote_addr().unwrap())?;
        return handler.handle_rpc_with_cur_tab(
            |tab: &mut Tab| -> Result<GetContentReply, String> {
                let content = tab.text.to_string();
                if request.get_ref().version
                    == handler.version.load(atomic::Ordering::Relaxed) as u64
                {
                    todo!("Implement history");
                    Ok(editor_rpc::GetContentReply { history: vec![] })
                } else {
                    Ok(editor_rpc::GetContentReply { history: vec![] })
                }
            },
        );
    }

    async fn update_content(
        &self,
        request: Request<UpdateContentRequest>,
    ) -> Result<Response<UpdateContentReply>, Status> {
        let handler = self.lock().unwrap();
        let map_state_lock = handler.map_state.lock().unwrap();
        match map_state_lock.1 {
            Some(tab_map_ptr) => {
                let tab_map = tab_map_ptr.as_ref();
                let mut tabs_lock = tab_map.tabs.lock().unwrap();
                let tab = tabs_lock.get_mut(&map_state_lock.0).unwrap();
                let cursor_lock = handler.cursor_pos.lock().unwrap();
                let request_ref = request.get_ref();
                let content_position = request_ref.pos.clone().unwrap();
                let start = content_position.start.unwrap();

                // check cursor position correct
                for cursor in &*cursor_lock {
                    if cursor.ip == request.remote_addr().unwrap() {
                        if start.row != cursor.row || start.col != cursor.col {
                            return Ok(Response::new(UpdateContentReply {
                                success: false,
                                content: "miss matched cursor position".to_string(),
                            }));
                        }
                    }
                }

                // handle operation
                let raw_rope = tab.text.get_raw();
                let char_idx = raw_rope.line_to_char(start.row as usize) + start.col as usize;
                match request_ref.op() {
                    Insert => {
                        raw_rope.insert(char_idx, &request_ref.content);
                    }
                    Delete => {
                        todo!("Implement delete content");
                        //raw_rope.remove(char_idx, char_idx +
                        // request_ref.content.len());
                    }
                    Replace => {
                        todo!("Implement replace content");
                        //raw_rope.remove(char_idx, char_idx + request_ref.content.len());
                        raw_rope.insert(char_idx, &request_ref.content);
                    }
                }
                Ok(Response::new(UpdateContentReply {
                    success: true,
                    content: "".to_string(),
                }))
            }
            None => Err(Status::internal("TabMap have not been iniitilized")),
        }
    }
}

pub struct RpcServerImpl {
    port: u16, //TODO: add mutex or use atomic
    tokio_runtime: tokio::runtime::Runtime,
    server_handle: Option<JoinHandle<()>>,
    shared_handler: Arc<Mutex<ServerHandle>>,
}

impl RpcServerImpl {
    fn default(thread_num: usize) -> Self {
        Self {
            port: 0,
            tokio_runtime: tokio::runtime::Builder::new_multi_thread()
                .worker_threads(thread_num)
                .enable_all()
                .build()
                .unwrap(),
            server_handle: None,
            shared_handler: Arc::new(Mutex::new(Default::default())),
        }
    }

    /// Start the service with a tab map.
    ///
    /// If the server is already running, return an error.
    pub fn start_service(
        &mut self,
        cur_tab_name: String,
        tab_map: Ptr<TabMap>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.shared_handler.lock().unwrap().map_state = Mutex::new((cur_tab_name, Some(tab_map)));
        if self.is_running() {
            return Err("Server already running".into());
        }
        self.start()?;
        Ok(())
    }

    pub fn stop_service(&mut self) {
        self.stop();
    }

    /// Set the port for the server.
    ///
    /// If the server is already running, return an error.
    pub fn set_port(&mut self, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_running() {
            return Err("Server already running".into());
        }
        self.port = port;
        Ok(())
    }

    /// Change the password for the server.
    pub fn change_password(&mut self, new_password: &str) {
        let handler = self.shared_handler.lock().unwrap();
        handler.change_password(new_password);
    }

    /// Check if the server is running.
    pub fn is_running(&self) -> bool {
        self.server_handle.is_some()
    }

    /// Get the port of the server.
    pub fn get_port(&self) -> u16 {
        self.port
    }

    /// Update the host cursor position.
    ///
    /// - `row`: The row of the cursor.
    /// - `col`: The column of the cursor.
    /// TODO:  add out of range check
    pub fn udpate_host_cursor(&mut self, row: u64, col: u64) {
        self.shared_handler
            .lock()
            .unwrap()
            .udpate_host_cursor(row, col);
    }
}

impl Default for RpcServerImpl {
    fn default() -> Self {
        Self::default(8)
    }
}

impl RpcServer for RpcServerImpl {
    fn start(&mut self) -> Result<(), Box<dyn Error>> {
        if self.server_handle.is_some() {
            return Err("Server already running".into());
        }
        let addr = format!("0.0.0.0:{}", self.port).parse().unwrap();
        debug_eprintln!("Server listening on: {}", addr);
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
