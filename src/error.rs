use std::net::*;

#[derive(Clone, PartialEq, Debug)]
pub enum Error {
    InvalidAddress(SocketAddr),
    BindFailure(SocketAddrV4),
    SendFailure(SocketAddrV4),
    ReceiveFailure(SocketAddr),
    MismatchedSendSize(SocketAddrV4, usize, usize),
    MismatchedRecvSize(SocketAddrV4, usize, usize),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Error::*;
        match self {
            InvalidAddress(addr) =>
                write!(f, "received invalid address {}", addr),
            BindFailure(addr) =>
                write!(f, "unable to bind socket to {}", addr),
            SendFailure(addr) =>
                write!(f, "error sending data to {}", addr),
            ReceiveFailure(addr) =>
                write!(f, "error recieving data on {}", addr),
            MismatchedSendSize(addr, size, expected) =>
                write!(f, "sent {} of {} bytes to {}",
                       size, expected, addr),
            MismatchedRecvSize(addr, size, expected) =>
                write!(f, "received {} of {} bytes from {}",
                       size, expected, addr),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_format() {
        use Error::*;

        let v4addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0);
        let addr = SocketAddr::V4(v4addr);

        let msg = format!("{}", InvalidAddress(addr));
        assert!(msg.ends_with("127.0.0.1:0"));

        let msg = format!("{}", BindFailure(v4addr));
        assert!(msg.ends_with("127.0.0.1:0"));

        let msg = format!("{}", SendFailure(v4addr));
        assert!(msg.ends_with("127.0.0.1:0"));

        let msg = format!("{}", ReceiveFailure(addr));
        assert!(msg.ends_with("127.0.0.1:0"));

        let msg = format!("{}", MismatchedSendSize(v4addr, 500, 1000));
        assert!(msg.ends_with("127.0.0.1:0"));
        assert!(msg.contains("500 of 1000"));

        let msg = format!("{}", MismatchedRecvSize(v4addr, 500, 1000));
        assert!(msg.ends_with("127.0.0.1:0"));
        assert!(msg.contains("500 of 1000"));
    }
}
