use std::net::{ ToSocketAddrs, TcpStream };
use std::time::Duration;

pub struct ConnectionError {
    pub message: String
}

impl ConnectionError {
    fn new(message: String) -> ConnectionError {
        ConnectionError {
            message
        }
    }
}

pub struct TcpConnection {
    host: String,
    port: u16,
    connection_timeout: Duration
}

impl TcpConnection {

    pub fn new(host: String, port: u16, connection_timeout: Duration) -> TcpConnection {
        TcpConnection {
            host,
            port,
            connection_timeout
        }
    }

    pub fn connect(&self) -> Result<TcpStream, ConnectionError> {
        let connect_str = self.get_connect_str();
        let socket_addr = &connect_str.to_socket_addrs();
        let socket_addr= match socket_addr {
            Ok(socket_addr) => { socket_addr.clone().next() },
            Err(err) => { return Err(ConnectionError::new(err.to_string())) }
        };

        let socket_addr = match socket_addr {
            Some(socket_addr) => { socket_addr },
            None => { return Err(ConnectionError::new("Could not get socket address".to_string())) }
        };

        let stream_result: Result<TcpStream, std::io::Error> = TcpStream::connect_timeout(&socket_addr, self.connection_timeout);
        let tcp_stream = match stream_result {
            Ok(stream) => {
                stream
            },
            Err(err) => {
                return Err(ConnectionError::new(err.to_string()))
            }
        };
        Ok(tcp_stream)
    }

    fn get_connect_str(&self) -> String {
        let mut connect_str = String::new();
        connect_str.push_str(&self.host);
        connect_str.push(':');
        connect_str.push_str(&self.port.to_string());
        connect_str
    }

}



