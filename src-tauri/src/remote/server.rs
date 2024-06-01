use std::{
    error::Error,
    net::SocketAddr,
    sync::{
        atomic::{self, AtomicU16},
        Arc,
        Mutex,
    },
};

use editor_rpc::{
    editor_server::{Editor, EditorServer},
    AuthorizeReply,
    AuthorizeRequest,
    DisconnectReply,
    DisconnectRequest,
    GetContentReply,
    GetContentRequest,
    SetCursorReply,
    SetCursorRequest,
    UpdateContentReply,
    UpdateContentRequest,
};
use tauri::{Manager, Window};
use tokio::task::JoinHandle;
use tonic::{transport::Server, Request, Response, Status};

use super::{
    utils::priority_lsit::{list_check_and_del, list_insert_or_replace_asc},
    ClientCursor,
    CursorAsc,
    Modification,
};
use crate::{
    dprintln,
    interface::remote::RpcServer,
    types::{
        middleware_types::{Tab, TabMap, UpdateContent},
        rpc_types::CursorList,
        ResultVoid,
    },
    utility::ptr::Ptr,
    APP_HANDLE,
};

pub mod editor_rpc {
    tonic::include_proto!("editor");
}

#[derive(Default)]
struct ServerHandle {
    window: Option<Window>,
    map_state: Mutex<(String, Option<Ptr<TabMap>>)>,
    version: atomic::AtomicUsize,
    password: Mutex<String>,
    clients: Mutex<Vec<SocketAddr>>,
    cursor_list: Arc<Mutex<CursorList>>,
    history: Mutex<Vec<Modification>>,
}

impl ServerHandle {
    /// Change password for server.
    fn change_password(&self, password: &str) {
        let mut lock = self.password.lock().unwrap();
        *lock = password.to_string();
    }

    /// Update host cursor position.
    fn set_host_cursor(&mut self, row: u64, col: u64, port: u16) {
        let mut lock = self.cursor_list.lock().unwrap();
        list_insert_or_replace_asc::<CursorAsc>(
            &mut *lock,
            ClientCursor {
                addr: format!("0.0.0.0:{}", port).parse().unwrap(),
                row,
                col,
            },
        );
    }

    /// Handle logic current tab wht a `&mut Tab` as lambda parameter.
    /// Only use to bypass the fucking borrow checker.
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
            None => Err("TabMap has not been initialized".to_string()),
        }
    }

    fn handle_rpc_with_cur_tab<F, R>(&self, handle: F) -> Result<Response<R>, Status>
    where
        F: Fn(&mut Tab) -> Result<R, String>,
    {
        match self.handle_with_cur_tab(handle) {
            Ok(success) => Ok(Response::new(success)),
            Err(err) => Err(Status::internal(err)),
        }
    }

    fn check_ip_authorized(&self, ip: SocketAddr) -> Result<(), Status> {
        let clients = self.clients.lock().unwrap();
        if clients.iter().any(|x| *x == ip) {
            Ok(())
        } else {
            Err(Status::unauthenticated("Unauthorized"))
        }
    }

    fn get_history_since<T>(&self, version: usize) -> Vec<T>
    where
        Modification: Into<T> + Clone,
    {
        let lock = self.history.lock().unwrap();
        lock[version..]
            .to_vec()
            .into_iter()
            .map(Into::into)
            .collect()
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
            if request.get_ref().password == handler.password.lock().unwrap().as_str()
                || handler.password.lock().unwrap().is_empty()
            {
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
                    file_name: String::new(),
                    version: 0,
                    content: String::new(),
                })
            }
        })
    }

    async fn disconnect(
        &self,
        request: Request<DisconnectRequest>,
    ) -> Result<Response<DisconnectReply>, Status> {
        let handler = self.lock().unwrap();
        handler.check_ip_authorized(request.remote_addr().unwrap())?;
        let mut clients = handler.clients.lock().unwrap();
        let mut success = false;
        if let Some(pos) = clients
            .iter()
            .position(|x| *x == request.remote_addr().unwrap())
        {
            clients.remove(pos);
            success = true;
            let mut cursor_lock = handler.cursor_list.lock().unwrap();
            _ = list_check_and_del::<CursorAsc>(
                &mut cursor_lock,
                &ClientCursor {
                    addr: request.remote_addr().unwrap(),
                    row: 0,
                    col: 0,
                },
            );
        }
        Ok(Response::new(DisconnectReply { success }))
    }

    async fn set_cursor(
        &self,
        request: Request<SetCursorRequest>,
    ) -> Result<Response<SetCursorReply>, Status> {
        let handler = self.lock().unwrap();
        handler.check_ip_authorized(request.remote_addr().unwrap())?;
        let mut cursor_lock = handler.cursor_list.lock().unwrap();
        list_insert_or_replace_asc::<CursorAsc>(
            &mut *cursor_lock,
            ClientCursor {
                addr: request.remote_addr().unwrap(),
                row: request.get_ref().row,
                col: request.get_ref().col,
            },
        );
        dprintln!("Set cursor: {:?}", cursor_lock);
        Ok(Response::new(SetCursorReply { success: true }))
    }

    async fn get_content(
        &self,
        request: Request<GetContentRequest>,
    ) -> Result<Response<GetContentReply>, Status> {
        let handler = self.lock().unwrap();
        handler.check_ip_authorized(request.remote_addr().unwrap())?;
        return handler.handle_rpc_with_cur_tab(
            |tab: &mut Tab| -> Result<GetContentReply, String> {
                if request.get_ref().full_content {
                    return Ok(GetContentReply {
                        history: vec![],
                        full_content: tab.text.to_string(),
                    });
                } else if request.get_ref().version
                    == handler.version.load(atomic::Ordering::Relaxed) as u64
                {
                    Ok(GetContentReply {
                        history: vec![],
                        full_content: String::new(),
                    })
                } else {
                    Ok(GetContentReply {
                        history: handler.get_history_since(request.get_ref().version as usize),
                        full_content: String::new(),
                    })
                }
            },
        );
    }

    async fn update_content(
        &self,
        request: Request<UpdateContentRequest>,
    ) -> Result<Response<UpdateContentReply>, Status> {
        let handler = self.lock().unwrap();
        handler.check_ip_authorized(request.remote_addr().unwrap())?;
        let map_state_lock = handler.map_state.lock().unwrap();

        match map_state_lock.1 {
            Some(tab_map_ptr) => {
                let tab_map = tab_map_ptr.as_ref();
                let mut tabs_lock = tab_map.tabs.lock().unwrap();
                let tab = tabs_lock.get_mut(&map_state_lock.0).unwrap();
                let mut cursor_lock = handler.cursor_list.lock().unwrap();
                let request_ref = request.get_ref();
                let content_position = request_ref.op_range.clone().unwrap();
                let start = content_position.start.unwrap();
                let end = content_position.end.unwrap();

                // check version correct(up to date)
                if request_ref.version != handler.version.load(atomic::Ordering::Relaxed) as u64 {
                    return Ok(Response::new(UpdateContentReply {
                        success: false,
                        message: "Version mismatch".to_string(),
                    }));
                }

                // check cursor position correct
                // for cursor in &*cursor_lock {
                //     if cursor.addr == request.remote_addr().unwrap()
                //         && (start.row != cursor.row || start.col != cursor.col)
                //         && (end.row != cursor.row || end.col != cursor.col)
                //     {
                //         return Ok(Response::new(UpdateContentReply {
                //             success: false,
                //             message: "miss matched cursor position".to_string(),
                //         }));
                //     }
                // }

                // handle operation
                match tab.text.merge_history(
                    &vec![Modification::from(request_ref.clone())],
                    &mut *cursor_lock,
                ) {
                    Ok(_) => {
                        #[cfg(not(test))]
                        {
                            let _ = APP_HANDLE.lock().unwrap().as_ref().unwrap().emit_all(
                                "front_update_content",
                                UpdateContent {
                                    file_name: tab.text.get_path_str(),
                                    op: request_ref.op,
                                    start: (start.row, start.col),
                                    end: (end.row, end.col),
                                    content: request_ref.modified_content.clone(),
                                },
                            );
                        }
                        Ok(Response::new(UpdateContentReply {
                            success: true,
                            message: String::new(),
                        }))
                    }
                    Err(e) => Ok(Response::new(UpdateContentReply {
                        success: false,
                        message: e.to_string(),
                    })),
                }
            }
            None => Err(Status::internal("TabMap have not been initialized")),
        }
    }
}

pub struct RpcServerImpl {
    port: AtomicU16,
    tokio_runtime: tokio::runtime::Runtime,
    rpc_server_handle: Option<JoinHandle<()>>,
    shared_handler: Arc<Mutex<ServerHandle>>,
}

impl RpcServerImpl {
    /// Start the service with a tab map.
    ///
    /// If the server is already running, return an error.
    pub fn start_server(
        &mut self,
        cur_tab_name: String,
        tab_map: Ptr<TabMap>,
        cursor_list: &Arc<Mutex<CursorList>>,
    ) -> ResultVoid {
        if self.is_running() {
            return Err("Server already running".into());
        }
        self.shared_handler.lock().unwrap().map_state = Mutex::new((cur_tab_name, Some(tab_map)));
        self.shared_handler.lock().unwrap().cursor_list = cursor_list.clone();
        self.start()?;
        Ok(())
    }

    pub fn stop_server(&mut self) {
        self.stop();
    }

    /// Set the port for the server.
    ///
    /// If the server is already running, return an error.
    pub fn set_port(&mut self, port: u16) -> Result<(), Box<dyn Error>> {
        if self.is_running() {
            return Err("Server already running".into());
        }
        self.port.store(port, atomic::Ordering::Relaxed);
        Ok(())
    }

    /// Change the password for the server.
    pub fn change_password(&mut self, new_password: &str) {
        let handler = self.shared_handler.lock().unwrap();
        handler.change_password(new_password);
    }

    /// Check if the server is running.
    pub fn is_running(&self) -> bool {
        self.rpc_server_handle.is_some()
    }

    /// Get the port of the server.
    pub fn get_port(&self) -> u16 {
        self.port.load(atomic::Ordering::Relaxed)
    }

    /// Update the host cursor position.
    ///
    /// - `row`: The row of the cursor.
    /// - `col`: The column of the cursor.
    /// TODO:  add out of range check
    pub fn set_host_cursor(&mut self, row: u64, col: u64) {
        self.shared_handler.lock().unwrap().set_host_cursor(
            row,
            col,
            self.port.load(atomic::Ordering::Relaxed),
        );
    }

    const fn get_ip() -> &'static str {
        #[cfg(test)]
        {
            "127.0.0.1"
        }
        #[cfg(not(test))]
        {
            "0.0.0.0"
        }
    }
}

impl Default for RpcServerImpl {
    fn default() -> Self {
        Self {
            port: AtomicU16::new(0),
            tokio_runtime: tokio::runtime::Builder::new_multi_thread()
                .worker_threads(8)
                .enable_all()
                .build()
                .unwrap(),
            rpc_server_handle: None,
            shared_handler: Arc::new(Mutex::new(Default::default())),
        }
    }
}

impl RpcServer for RpcServerImpl {
    fn start(&mut self) -> Result<(), Box<dyn Error>> {
        if self.rpc_server_handle.is_some() {
            return Err("Server already running".into());
        }

        let addr = format!(
            "{}:{}",
            RpcServerImpl::get_ip(),
            self.port.load(atomic::Ordering::Relaxed)
        )
        .parse()
        .unwrap();
        dprintln!("Server listening on: {}", addr);
        let handler = Arc::clone(&self.shared_handler);

        let server_handle = self.tokio_runtime.spawn(async move {
            Server::builder()
                .add_service(EditorServer::new(handler))
                .serve(addr)
                .await
                .unwrap();
        });
        self.rpc_server_handle = Some(server_handle);
        Ok(())
    }

    fn stop(&mut self) {
        if let Some(handle) = self.rpc_server_handle.take() {
            handle.abort();
        }
    }
}
