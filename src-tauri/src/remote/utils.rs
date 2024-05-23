use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddrV4, TcpListener},
};

use rand::random;

pub fn get_free_port(ip: Ipv4Addr, try_times: usize) -> Result<u16, Box<dyn Error + Send + Sync>> {
    let port = random::<u16>();
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

pub mod priority_lsit {
    use std::{
        collections::{linked_list, LinkedList},
        fmt::Debug,
    };

    use super::super::GetCmpType;

    fn list_insert_asc<C>(list: &mut LinkedList<C::Type>, value: C::Type)
    where
        C: Ord + GetCmpType,
        C::Type: Debug,
    {
        let mut cursor = list.cursor_front_mut();
        loop {
            match cursor.current() {
                Some(v) => {
                    if C::new(v) > C::new(&value) {
                        cursor.insert_before(value);
                        return;
                    }
                    cursor.move_next();
                }
                None => {
                    cursor.insert_before(value);
                    return;
                }
            }
        }
    }

    pub fn list_check_and_del<C>(list: &mut LinkedList<C::Type>, value: &C::Type) -> bool
    where
        C: PartialEq + GetCmpType,
        C::Type: Debug,
    {
        let mut cursor = list.cursor_front_mut();
        loop {
            match cursor.current() {
                Some(v) => {
                    if C::new(v) == C::new(&value) {
                        cursor.remove_current();
                        return true;
                    }
                    cursor.move_next();
                }
                None => return false,
            }
        }
    }

    fn list_check<C>(list: &mut LinkedList<C::Type>, value: &C::Type) -> bool
    where
        C: PartialEq + GetCmpType,
        C::Type: Debug,
    {
        let mut cursor = list.cursor_front_mut();
        loop {
            match cursor.current() {
                Some(v) => {
                    if C::new(v) == C::new(value) {
                        return true;
                    }
                    cursor.move_next();
                }
                None => return false,
            }
        }
    }

    pub fn list_insert_or_replace_asc<C>(list: &mut LinkedList<C::Type>, value: C::Type)
    where
        C: Ord + Eq + GetCmpType,
        C::Type: Debug,
    {
        list_check_and_del::<C>(list, &value);
        list_insert_asc::<C>(list, value);
    }

    pub fn get_cursor<'a, C>(
          list: &'a mut LinkedList<C::Type>,
        value: &C::Type,
    ) -> Option<linked_list::CursorMut<'a, C::Type>>
    where
        C: PartialEq + GetCmpType,
        C::Type: Debug,
    {
        let mut cursor = list.cursor_front_mut();
        loop {
            match cursor.current() {
                Some(v) => {
                    if C::new(v) == C::new(value) {
                        cursor.move_prev();
                        return Some(cursor);
                    }
                    cursor.move_next();
                }
                None => return None,
            }
        }
    }
}
