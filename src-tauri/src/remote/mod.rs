/// Client module for p2p text editor
pub mod client;
/// Server module for p2p text editor
pub mod server;
pub mod utils;

use std::{cmp::Ordering, collections::LinkedList, net::SocketAddr};

use server::editor_rpc::OperationType;

use crate::middleware_types::TextPosition;

struct History {
    version: u64,
    op: OperationType,
    start: TextPosition,
    end: TextPosition,
    content: String,
}

#[derive(Eq, PartialOrd)]
struct ClientCursor {
    ip: SocketAddr,
    row: u64,
    col: u64,
}

impl Ord for ClientCursor {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.row
            .cmp(&other.row)
            .then(self.col.cmp(&other.col))
            .then(self.ip.cmp(&other.ip))
    }
}

impl PartialEq for ClientCursor {
    fn eq(&self, other: &Self) -> bool {
        self.ip == other.ip
    }
}

fn list_insert_asc<T>(list: &mut LinkedList<T>, value: T)
where
    T: Ord,
{
    let mut cursor = list.cursor_front_mut();
    loop {
        match cursor.current() {
            Some(current_value) if *current_value >= value => {
                cursor.insert_before(value);
                break;
            }
            Some(_) => cursor.move_next(),
            None => {
                cursor.insert_after(value);
                break;
            }
        }
    }
}

fn list_check<T>(list: &mut LinkedList<T>, value: &T) -> bool
where
    T: Eq,
{
    let mut cursor = list.cursor_front_mut();
    loop {
        match cursor.current() {
            Some(current_value) if *current_value == *value => {
                return true;
            }
            Some(_) => cursor.move_next(),
            None => return false,
        }
    }
}

fn list_insert_or_replace_asc<T>(list: &mut LinkedList<T>, value: T)
where
    T: PartialEq + Ord,
{
    let mut cursor = list.cursor_front_mut();
    let mut inserted = false;
    let mut removed = false;
    loop {
        match cursor.current() {
            Some(current_value) if *current_value >= value => {
                cursor.insert_before(value);
                break;
            }
            Some(_) => {
                cursor.move_next();
            }
            None => {
                cursor.insert_after(value);
                break;
            }
        }
    }
}
