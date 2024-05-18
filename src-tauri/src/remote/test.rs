use std::net::{IpAddr, SocketAddr};

use rand::random;

use super::{ClientCursor, GetCmpType};

fn gen_client_cursor(len: u16, ip: IpAddr) -> Vec<ClientCursor> {
    let mut v: Vec<ClientCursor> = Vec::new();
    for i in 0..len {
        v.push(ClientCursor {
            ip: SocketAddr::new(ip, i),
            row: random::<u64>() % 1000000,
            col: random::<u64>() % 1000000,
        });
    }
    v
}

#[test]
fn test_insert() {
    let mut list: LinkedList<ClientCursor> = Default::default();
    let clients = gen_client_cursor(u16::pow(2, 12), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    for c in clients.iter() {
        list_insert_or_replace_asc::<CursorCMP>(&mut list, c.clone());
    }
    let mut first = list.iter();
    let mut second = list.iter().skip(1);
    loop {
        match (first.next(), second.next()) {
            (Some(f), Some(s)) => {
                assert!(f.row <= s.row);
            }
            _ => break,
        }
    }
}

#[test]
fn test_delete() {
    let mut list: LinkedList<ClientCursor> = Default::default();
    let clients = gen_client_cursor(u16::pow(2, 12), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    for c in clients.iter() {
        list_insert_or_replace_asc::<CursorCMP>(&mut list, c.clone());
    }
    let clients = gen_client_cursor(u16::pow(2, 12), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    for c in clients.iter() {
        let val = list_check_and_del::<CursorCMP>(&mut list, c);
        assert_eq!(val, true);
    }
    assert_eq!(list.len(), 0);
    let val = list_check_and_del::<CursorCMP>(
        &mut list,
        &ClientCursor {
            ip: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0),
            row: 0,
            col: 0,
        },
    );
    assert_eq!(val, false);
}

#[test]
fn test_replace() {
    let mut list: LinkedList<ClientCursor> = Default::default();
    let clients = gen_client_cursor(u16::pow(2, 12), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    for c in clients.iter() {
        list_insert_or_replace_asc::<CursorCMP>(&mut list, c.clone());
    }
    let clients = gen_client_cursor(u16::pow(2, 12), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    for c in clients.iter() {
        list_insert_or_replace_asc::<CursorCMP>(&mut list, c.clone());
    }
    let mut first = list.iter();
    let mut second = list.iter().skip(1);
    loop {
        match (first.next(), second.next()) {
            (Some(f), Some(s)) => {
                assert!(f.row <= s.row);
            }
            _ => break,
        }
    }
}
