use std::net::{ ToSocketAddrs, TcpStream };
use std::time::Duration;
use std::option::Option;
use crate::common::{ ClientError, ClientErrorType};

pub struct TcpConnection {
    host: String,
    port: u16,
    connection_timeout: Duration,
    tcp_stream: Option<TcpStream>
}

impl TcpConnection {

    pub fn new(host: String, port: u16, connection_timeout: Duration) -> TcpConnection {
        TcpConnection {
            host,
            port,
            connection_timeout,
            tcp_stream: None
        }
    }

    pub fn connect(mut self) -> Result<(), ClientError> {
        let connect_str = self.get_connect_str();
        let socket_addr = &connect_str.to_socket_addrs();
        let socket_addr= match socket_addr {
            Ok(socket_addr) => { socket_addr.clone().next() },
            Err(err) => { return Err(ClientError::new(ClientErrorType::IncorrectSocketAddr, err.to_string())) }
        };

        let socket_addr = match socket_addr {
            Some(socket_addr) => { socket_addr },
            None => { return Err(ClientError::new(ClientErrorType::IncorrectSocketAddr,"Could not get socket address".to_string())) }
        };

        let stream_result: Result<TcpStream, std::io::Error> = TcpStream::connect_timeout(&socket_addr, self.connection_timeout);
        self.tcp_stream = match stream_result {
            Ok(stream) => {
                Some(stream)
            },
            Err(err) => {
                return Err(ClientError::new(ClientErrorType::ConnectionFailure, err.to_string()))
            }
        };
        Ok(())
    }

    pub fn disconnect(mut self, ignore_error: bool) -> Result<(), ClientError> {
        if !ignore_error && self.tcp_stream.is_some() {
            return Err(ClientError::new(ClientErrorType::DisconnectionFailed, "No connection open".to_string()));
        }
        self.tcp_stream = None;
        Ok(())
    }

    fn get_connect_str(&self) -> String {
        let mut connect_str = String::new();
        connect_str.push_str(&self.host);
        connect_str.push(':');
        connect_str.push_str(&self.port.to_string());
        connect_str
    }

}



