use editor_rpc::editor_client::EditorClient;
use editor_rpc::{
    AuthorizeReply, AuthorizeRequest, SetCursorReply, SetCursorRequest, UpdateContentReply,
    UpdateContentRequest,
};

pub mod editor_rpc {
    tonic::include_proto!("editor");
}

async fn foo() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = EditorClient::connect("http://[::1]:50051").await?;

    //println!("RESPONSE={:?}", response);

    Ok(())
}
