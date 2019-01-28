use std::net::*;

pub fn server(addr: Ipv4Addr, port: u16) {
    let server_addr = SocketAddrV4::new(addr, port);
    let socket = match bind_socket(server_addr) {
        Ok(socket) => socket,
        Err(e) => {
            eprintln!("{}", e);
            return;
        },
    };
    loop {
        let caller_addr = match recv_addr(&socket) {
            Ok(addr) => addr,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            },
        };
        match send_echo(&socket, caller_addr) {
            Ok(_) => continue,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            },
        };
    }
}

fn bind_socket(addr: SocketAddrV4) -> Result<UdpSocket, Error> {
    match UdpSocket::bind(addr) {
        Ok(socket) => Ok(socket),
        Err(_) => Err(Error::BindFailure(addr)),
    }
}

fn recv_addr(socket: &UdpSocket) -> Result<SocketAddrV4, Error> {
    let mut buffer = [0u8; 8];
    match socket.recv_from(&mut buffer) {
        Ok((_size, client_addr)) => {
            match client_addr {
                SocketAddr::V4(addr) => Ok(addr),
                SocketAddr::V6(addr) => Err(Error::InvalidAddress(addr)),
            }
        },
        Err(_) => {
            let sock_addr = socket.local_addr().unwrap();
            Err(Error::ReceiveError(sock_addr))
        },
    }
}

fn send_echo(socket: &UdpSocket,
             addr: SocketAddrV4) -> Result<(), Error> {
    let send_data = octets_from_v4_addr(addr);
    match socket.send_to(&send_data, addr) {
        Ok(size) => {
            match size {
                4 => Ok(()),
                _ => Err(Error::ShortSend(addr, size)),
            }
        },
        Err(_) => Err(Error::SendFailure(addr)),
    }
}

fn octets_from_v4_addr(sockaddr: SocketAddrV4) -> [u8; 4] {
    sockaddr.ip().octets()
}

#[derive(Clone, PartialEq, Debug)]
enum Error {
    BindFailure(SocketAddrV4),
    ReceiveError(SocketAddr),
    InvalidAddress(SocketAddrV6),
    SendFailure(SocketAddrV4),
    ShortSend(SocketAddrV4, usize),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Error::*;
        match self {
            BindFailure(addr) =>
                write!(f, "unable to bind to {}", addr),
            ReceiveError(addr) =>
                write!(f, "error recieving data on {}", addr),
            InvalidAddress(addr) =>
                write!(f, "received invalid address {}", addr),
            SendFailure(addr) => 
                write!(f, "error sending data to {}", addr),
            ShortSend(addr, size) =>
                write!(f, "only sent {} of 4 bytes to {}", size, addr),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::{spawn, sleep};
    use std::time::Duration;

    #[test]
    fn test_bind_socket() {
        let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0);
        let result = bind_socket(addr);
        assert!(result.is_ok());
        let socket = result.unwrap();
        let result = socket.local_addr();
        assert!(result.is_ok());
        let addr = result.unwrap();
        let ip = addr.ip();
        assert_eq!(ip, Ipv4Addr::LOCALHOST);
    }

    #[test]
    fn test_recv_addr() {
        let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0);
        let server = UdpSocket::bind(addr).unwrap();
        let server_addr = server.local_addr().unwrap();
        let client = UdpSocket::bind(addr).unwrap();
        let client_addr = match client.local_addr().unwrap() {
            SocketAddr::V4(addr) => addr,
            SocketAddr::V6(_) => panic!(),
        };

        let _child = spawn(move || {
            sleep(Duration::from_millis(100));
            client.send_to(&[0u8; 0], server_addr)
        });

        let result = recv_addr(&server);
        assert!(result.is_ok());
        let addr = result.unwrap();
        assert_eq!(addr, client_addr);
    }

    #[test]
    fn test_send_echo() {
        let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0);
        let server = UdpSocket::bind(addr).unwrap();
        let server_addr = server.local_addr().unwrap();
        let client = UdpSocket::bind(addr).unwrap();
        let client_addr = match client.local_addr().unwrap() {
            SocketAddr::V4(addr) => addr,
            SocketAddr::V6(_) => panic!(),
        };

        let child = spawn(move || {
            let mut buffer = [0u8; 5];
            let result = client.recv_from(&mut buffer);
            (buffer, result)
        });

        sleep(Duration::from_millis(100));
        let result = send_echo(&server, client_addr);
        assert_eq!(Ok(()), result);

        let result = child.join();
        assert!(result.is_ok());
        let (buffer, recv_result) = result.unwrap();
        assert!(recv_result.is_ok());
        let (size, addr) = recv_result.unwrap();
        assert_eq!(addr, server_addr);
        assert_eq!(size, 4);
        assert_eq!(buffer, [127, 0, 0, 1, 0]);
    }

    #[test]
    fn test_octets_from_v4_addr() {
        let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0);
        assert_eq!(octets_from_v4_addr(addr), [127, 0, 0, 1]);
    }

    #[test]
    fn test_error_format() {
        use Error::*;

        let v4addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0);
        let v6addr = SocketAddrV6::new(Ipv6Addr::LOCALHOST, 0, 0, 0);
        let addr = SocketAddr::V4(v4addr);

        let msg = format!("{}", BindFailure(v4addr));
        assert!(msg.ends_with("127.0.0.1:0"));

        let msg = format!("{}", ReceiveError(addr));
        assert!(msg.ends_with("127.0.0.1:0"));

        let msg = format!("{}", InvalidAddress(v6addr));
        assert!(msg.ends_with("[::1]:0"));

        let msg = format!("{}", SendFailure(v4addr));
        assert!(msg.ends_with("127.0.0.1:0"));

        let msg = format!("{}", ShortSend(v4addr, 3));
        assert!(msg.ends_with("127.0.0.1:0"));
        assert!(msg.contains("3 of 4"));
    }
}
