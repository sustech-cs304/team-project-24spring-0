use tonic::transport::Server;
use tonic::{Request, Response, Status};

use editor_rpc::editor_server::{Editor, EditorServer};
use editor_rpc::{AuthorizeReply, AuthorizeRequest, SetCursorReply, SetCursorRequest};

pub mod editor_rpc {
    tonic::include_proto!("editor");
}

#[derive(Debug, Default)]
pub struct ServerImpl {}

#[tonic::async_trait]
impl Editor for ServerImpl {
    async fn authorize(
        &self,
        request: Request<AuthorizeRequest>,
    ) -> Result<Response<AuthorizeReply>, Status> {
        Ok(Response::new(AuthorizeReply {
            success: true,
            file: "foo".to_string(),
        }))
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

async fn foo(addr: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", addr, port).parse()?;
    let handler = ServerImpl::default();

    Server::builder()
        .add_service(EditorServer::new(handler))
        .serve(addr)
        .await?;
    Ok(())
}
