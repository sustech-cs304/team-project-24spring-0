/// Client module for p2p text editor
pub mod client;
/// Server module for p2p text editor
pub mod server;
/// Utils module for p2p text editor
pub mod utils;

mod test;

use std::{cmp::Ordering, net::SocketAddr};

use server::editor_rpc::{ContentPosition, OperationType, Pos, UpdateContentRequest};

use crate::middleware_types::CursorPosition;

pub trait GetCmpType {
    type Type: Clone;

    fn new(t: &Self::Type) -> Self;
}

#[derive(Clone)]
struct OpRange {
    start: CursorPosition,
    end: CursorPosition,
}

#[derive(Clone)]
struct History {
    version: u64,
    op: OperationType,
    op_range: OpRange,
    modified_content: String,
}

#[derive(Clone, Debug)]
struct ClientCursor {
    ip: SocketAddr,
    row: u64,
    col: u64,
}

struct CursorCMP(ClientCursor);

impl Into<Pos> for CursorPosition {
    fn into(self) -> Pos {
        Pos {
            row: self.row,
            col: self.col,
        }
    }
}

impl Into<ContentPosition> for OpRange {
    fn into(self) -> ContentPosition {
        ContentPosition {
            start: Option::Some(self.start.into()),
            end: Option::Some(self.end.into()),
        }
    }
}

impl Into<UpdateContentRequest> for History {
    fn into(self) -> UpdateContentRequest {
        UpdateContentRequest {
            version: self.version,
            op: self.op.into(),
            op_range: Option::Some(self.op_range.into()),
            modified_content: self.modified_content,
        }
    }
}

impl GetCmpType for CursorCMP {
    type Type = ClientCursor;

    fn new(t: &Self::Type) -> Self {
        CursorCMP { 0: t.clone() }
    }
}

impl PartialEq for CursorCMP {
    fn eq(&self, other: &Self) -> bool {
        self.0.ip == other.0.ip
    }
}

impl Eq for CursorCMP {}

impl PartialOrd for CursorCMP {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Option::Some(
            self.0
                .row
                .cmp(&other.0.row)
                .then(self.0.col.cmp(&other.0.col))
                .then(self.0.ip.cmp(&other.0.ip)),
        )
    }
}
impl Ord for CursorCMP {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .row
            .cmp(&other.0.row)
            .then(self.0.col.cmp(&other.0.col))
            .then(self.0.ip.cmp(&other.0.ip))
    }
}
