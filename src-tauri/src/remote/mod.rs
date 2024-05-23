/// Client module for p2p text editor
pub mod client;
/// Server module for p2p text editor
pub mod server;
/// Utils module for p2p text editor
pub mod utils;

#[cfg(test)]
mod test;

use std::{cmp::Ordering, net::SocketAddr};

use server::editor_rpc::{ContentPosition, OperationType, Pos, UpdateContentRequest};

use crate::middleware_types::CursorPosition;

pub trait GetCmpType {
    type Type: Clone;

    fn new(t: &Self::Type) -> Self;
}

#[derive(Clone)]
pub struct OpRange {
    pub start: CursorPosition,
    pub end: CursorPosition,
}

#[derive(Clone)]
pub struct History {
    pub version: u64,
    pub op: OperationType,
    pub op_range: OpRange,
    pub modified_content: String,
}

#[derive(Clone, Debug)]
pub struct ClientCursor {
    pub addr: SocketAddr,
    pub row: u64,
    pub col: u64,
}

struct CursorAsc(ClientCursor);

pub struct CursorRowEq(ClientCursor);

impl Into<Pos> for CursorPosition {
    fn into(self) -> Pos {
        Pos {
            row: self.row,
            col: self.col,
        }
    }
}

impl From<Pos> for CursorPosition {
    fn from(pos: Pos) -> Self {
        CursorPosition {
            row: pos.row,
            col: pos.col,
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

impl From<ContentPosition> for OpRange {
    fn from(pos: ContentPosition) -> Self {
        OpRange {
            start: pos.start.unwrap().into(),
            end: pos.end.unwrap().into(),
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

impl From<UpdateContentRequest> for History {
    fn from(req: UpdateContentRequest) -> Self {
        History {
            version: req.version,
            op: req.op(),
            op_range: req.op_range.unwrap().into(),
            modified_content: req.modified_content,
        }
    }
}

impl GetCmpType for CursorAsc {
    type Type = ClientCursor;

    fn new(t: &Self::Type) -> Self {
        CursorAsc { 0: t.clone() }
    }
}

impl PartialEq for CursorAsc {
    fn eq(&self, other: &Self) -> bool {
        self.0.addr == other.0.addr
    }
}

impl Eq for CursorAsc {}

impl PartialOrd for CursorAsc {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Option::Some(
            self.0
                .row
                .cmp(&other.0.row)
                .then(self.0.col.cmp(&other.0.col))
                .then(self.0.addr.cmp(&other.0.addr)),
        )
    }
}

impl Ord for CursorAsc {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .row
            .cmp(&other.0.row)
            .then(self.0.col.cmp(&other.0.col))
            .then(self.0.addr.cmp(&other.0.addr))
    }
}

impl GetCmpType for CursorRowEq {
    type Type = ClientCursor;

    fn new(t: &Self::Type) -> Self {
        CursorRowEq { 0: t.clone() }
    }
}

impl PartialEq for CursorRowEq {
    fn eq(&self, other: &Self) -> bool {
        self.0.row == other.0.row
    }
}

impl Eq for CursorRowEq {}
