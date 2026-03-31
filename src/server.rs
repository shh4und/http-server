use libc::sockaddr_in;

pub struct Server {
    pub domain: i32,
    pub service: i32,
    pub protocol: i32,
    pub interface: u64,
    pub port: i32,
    pub backlog: i32,
    pub address: sockaddr_in,
    pub socket: i32,
}

impl Server {
    pub fn new(
        domain: i32,
        service: i32,
        protocol: i32,
        interface: u64,
        port: i32,
        backlog: i32,
        socket: i32,
    ) -> Self {
        Self {
            domain,
            service,
            protocol,
            interface,
            port,
            backlog,
            socket,
        }
    }
}
