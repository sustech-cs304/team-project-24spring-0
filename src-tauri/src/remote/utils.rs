use std::{
    collections::LinkedList,
    error::Error,
    net::{Ipv4Addr, SocketAddrV4, TcpListener},
    str::FromStr,
};

use rand::random;

pub fn lines(content: &str) -> usize {
    let mut line_count: usize = 0;
    for c in content.chars() {
        if c == '\n' {
            line_count += 1;
        }
    }
    line_count
}

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
