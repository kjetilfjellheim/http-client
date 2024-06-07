use crate::connection::TcpConnection;

pub struct HttpClient {
    tcp_onnection: TcpConnection
}

impl HttpClient {
    pub fn new(tcp_onnection: TcpConnection) -> HttpClient {
        HttpClient {
            tcp_onnection
        }
    }
}