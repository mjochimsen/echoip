use std::net::*;
use std::time::Duration;
use std::process::exit;
use super::error::Error;
use super::error::Error::*;

const RECV_TIMEOUT: u64 = 5;

pub fn client(server_addr: Ipv4Addr, port: u16) {
    // Bind client socket.
    let socket = match bind_socket() {
        Ok(socket) => socket,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        },
    };
    // Set the read timeout on the socket.
    let timeout = Duration::from_secs(RECV_TIMEOUT);
    match socket.set_read_timeout(Some(timeout)) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }

    // Send to server.
    let server_addr = SocketAddrV4::new(server_addr, port);
    match send(&socket, server_addr) {
        Ok(()) => true,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        },
    };

    // Receive echo data.
    let echo_data = match recv_from(&socket, server_addr) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        },
    };

    // Decode data to IP address and print.
    match decode_addr(echo_data) {
        Ok(addr) => println!("{}", addr),
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        },
    }
}

fn bind_socket() -> Result<UdpSocket, Error> {
    let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);
    let result = UdpSocket::bind(addr);
    if result.is_err() {
        Err(BindFailure(addr))
    } else {
        Ok(result.unwrap())
    }
}

fn send(socket: &UdpSocket, addr: SocketAddrV4) -> Result<(), Error> {
    match socket.send_to(&[0u8; 0], addr) {
        Ok(0) => Ok(()),
        Ok(sent) => Err(MismatchedSendSize(addr, sent, 0)),
        Err(_) => Err(SendFailure(addr)),
    }
}

fn recv_from(socket: &UdpSocket,
             addr: SocketAddrV4) -> Result<Vec<u8>, Error> {
    let mut buffer = [0u8; 16];
    match socket.recv_from(&mut buffer) {
        Ok((size, recv_addr)) =>
            if recv_addr == SocketAddr::V4(addr) {
                if size == 4 {
                    Ok(Vec::from(&buffer[0..size]))
                } else {
                    Err(MismatchedRecvSize(addr, size, 4))
                }
            } else {
                let addr = socket.local_addr().unwrap();
                Err(ReceiveFailure(addr))
            },
        Err(_) => {
            let addr = socket.local_addr().unwrap();
            Err(ReceiveFailure(addr))
        },
    }
}

fn decode_addr(addr_data: Vec<u8>) -> Result<Ipv4Addr, Error> {
    if addr_data.len() == 4 {
        let mut addr_buffer = [0u8; 4];
        addr_buffer.copy_from_slice(&addr_data[0..4]);
        let addr = Ipv4Addr::from(addr_buffer);
        Ok(addr)
    } else {
        let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);
        Err(MismatchedRecvSize(addr, addr_data.len(), 4))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::{spawn, sleep};
    use std::time::Duration;

    #[test]
    fn test_bind_socket() {
        let result = bind_socket();
        assert!(result.is_ok());
        let socket = result.unwrap();
        let result = socket.local_addr();
        assert!(result.is_ok());
        let addr = result.unwrap();
        let ip = addr.ip();
        assert_eq!(ip, Ipv4Addr::UNSPECIFIED);
    }

    #[test]
    fn test_send() {
        let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0);
        let client = UdpSocket::bind(addr).unwrap();
        let client_addr = client.local_addr().unwrap();
        let server = UdpSocket::bind(addr).unwrap();
        let server_addr = match server.local_addr().unwrap() {
            SocketAddr::V4(addr) => addr,
            SocketAddr::V6(_) => panic!(),
        };

        let child = spawn(move || {
            let mut buffer = [0u8; 4];
            server.recv_from(&mut buffer)
        });

        sleep(Duration::from_millis(100));
        let result = send(&client, server_addr);
        assert_eq!(Ok(()), result);

        let result = child.join();
        assert!(result.is_ok());
        let recv_result = result.unwrap();
        assert!(recv_result.is_ok());
        let (size, addr) = recv_result.unwrap();
        assert_eq!(addr, client_addr);
        assert_eq!(size, 0);
    }

    #[test]
    fn test_recv_from() {
        let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0);
        let client = UdpSocket::bind(addr).unwrap();
        let client_addr = client.local_addr().unwrap();
        let server = UdpSocket::bind(addr).unwrap();
        let server_addr = match server.local_addr().unwrap() {
            SocketAddr::V4(addr) => addr,
            SocketAddr::V6(_) => panic!(),
        };

        let child = spawn(move || {
            sleep(Duration::from_millis(100));
            server.send_to(&[127, 0, 0, 1], client_addr)
        });

        let result = recv_from(&client, server_addr);
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data, vec![127, 0, 0, 1]);

        let result = child.join();
        assert!(result.is_ok());
        let send_result = result.unwrap();
        assert!(send_result.is_ok());
        let size = send_result.unwrap();
        assert_eq!(size, 4);
    }

    #[test]
    fn test_decode_addr() {
        let result = decode_addr(vec![127, 0, 0, 1]);
        assert!(result.is_ok());
        let addr = result.unwrap();
        assert_eq!(Ipv4Addr::LOCALHOST, addr);
    }
}
