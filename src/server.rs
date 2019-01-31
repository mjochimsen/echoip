use std::net::*;
use super::error::Error;
use super::error::Error::*;

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
        Err(_) => Err(BindFailure(addr)),
    }
}

fn recv_addr(socket: &UdpSocket) -> Result<SocketAddrV4, Error> {
    let mut buffer = [0u8; 8];
    match socket.recv_from(&mut buffer) {
        Ok((_size, client_addr)) => {
            match client_addr {
                SocketAddr::V4(addr) => Ok(addr),
                SocketAddr::V6(_) => Err(InvalidAddress(client_addr)),
            }
        },
        Err(_) => {
            let sock_addr = socket.local_addr().unwrap();
            Err(ReceiveFailure(sock_addr))
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
                _ => Err(MismatchedSendSize(addr, size, 4)),
            }
        },
        Err(_) => Err(SendFailure(addr)),
    }
}

fn octets_from_v4_addr(sockaddr: SocketAddrV4) -> [u8; 4] {
    sockaddr.ip().octets()
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
}
