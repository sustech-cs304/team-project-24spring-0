use std::{
    collections::LinkedList,
    error::Error,
    net::{Ipv4Addr, SocketAddrV4, TcpListener},
    str::FromStr,
};

use rand::random;

use super::GetCmpType;

pub fn get_free_port(ip: Ipv4Addr, try_times: usize) -> Result<u16, Box<dyn Error>> {
    let mut port = random::<u16>();
    for _ in 0..try_times {
        let addr = SocketAddrV4::new(ip, port);
        match TcpListener::bind(addr) {
            Ok(_) => {
                return Ok(port);
            }
            Err(_) => continue,
        }
    }
    Err("No free port found".into())
}

fn list_insert_asc<C>(list: &mut LinkedList<C::Type>, value: C::Type)
where
    C: PartialOrd + GetCmpType,
{
    let mut cursor = list.cursor_front_mut();
    loop {
        match cursor.current() {
            Some(current_value) if C::new(current_value) >= C::new(&value) => {
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

pub fn list_check_and_del<C>(list: &mut LinkedList<C::Type>, value: C::Type) -> bool
where
    C: Eq + GetCmpType,
{
    let mut cursor = list.cursor_front_mut();
    loop {
        match cursor.current() {
            Some(current_value) if C::new(current_value) == C::new(&value) => {
                cursor.remove_current();
                return true;
            }
            Some(_) => cursor.move_next(),
            None => return false,
        }
    }
}

fn list_check<C>(list: &mut LinkedList<C::Type>, value: &C::Type) -> bool
where
    C: Eq + GetCmpType,
{
    let mut cursor = list.cursor_front_mut();
    loop {
        match cursor.current() {
            Some(current_value) if C::new(current_value) == C::new(value) => {
                return true;
            }
            Some(_) => cursor.move_next(),
            None => return false,
        }
    }
}

pub fn list_insert_or_replace_asc<C>(list: &mut LinkedList<C::Type>, value: C::Type)
where
    C: PartialOrd + Eq + GetCmpType,
{
    let mut cursor = list.cursor_front_mut();
    loop {
        match cursor.current() {
            Some(current_value) if C::new(current_value) == C::new(&value) => {
                cursor.remove_current();
            }
            Some(_) => cursor.move_next(),
            None => break,
        }
    }
    list_insert_asc::<C>(list, value);
}
