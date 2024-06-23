use std::io::{Read, Write};
use std::net::{ ToSocketAddrs, TcpStream };
use std::time::Duration;
use std::option::Option;

use crate::common::{ ClientError, ClientErrorType};


/**
 * Handles tcp connection to a host and port, with a connection timeout.
 * Reads and writes data to the tcp stream.
 */
pub struct TcpConnection {
    host: String,
    port: u16,
    connection_timeout: Duration,
    tcp_stream: Option<TcpStream>
}

impl TcpConnection {
    /**
     * Creates a new TcpConnection with a host, port and connection timeout.
     */
    pub fn new(host: String, port: u16, connection_timeout: Duration) -> TcpConnection {
        TcpConnection {
            host,
            port,
            connection_timeout,
            tcp_stream: None
        }
    }

    /**
     * Connects to the host and port. Stores the tcp stream for later use.
     * Returns an error if the connection fails.
     */
    pub fn connect(&mut self) -> Result<(), ClientError> {
        let connect_str = self.get_connect_str();
        let socket_addr = &connect_str.to_socket_addrs();
        let socket_addr= match socket_addr {
            Ok(socket_addr) => { socket_addr.clone().next() },
            Err(err) => return Err(ClientError::new(ClientErrorType::IncorrectSocketAddr, err.to_string()))
        };
        let socket_addr = match socket_addr {
            Some(socket_addr) => { socket_addr },
            None => { return Err(ClientError::new(ClientErrorType::IncorrectSocketAddr,"Could not get socket address".to_string())) }
        };
        let stream_result: Result<TcpStream, std::io::Error> = TcpStream::connect_timeout(&socket_addr, self.connection_timeout);
        self.tcp_stream = match stream_result {
            Ok(stream) => Some(stream),
            Err(err) => return Err(ClientError::new(ClientErrorType::ConnectionFailure, err.to_string()))
        };
        Ok(())
    }

    /**
     * Writes a request string to the tcp stream.
     * Returns an error if the write fails.
     */
    pub fn write(&mut self, request_str: &String) -> Result<(), ClientError>{
        let mut tcp_stream = match &self.tcp_stream {
            None => return Err(ClientError::new(ClientErrorType::NoAvailableTcpStream, "Could not retrieve tcp stream".to_string())),
            Some(tcp_stream) => tcp_stream
        };
        let write_result = tcp_stream.write_all(request_str.as_bytes());
        match write_result {
            Ok(_) => Ok(()),
            Err(_) => Err(ClientError::new(ClientErrorType::WriteError, "Could not write data".to_string()))
        }
    }

    /**
     * Reads data from the tcp stream.
     * Returns an error if the read fails.
     */
    pub fn read(self) -> Result<String, ClientError> {
        let mut tcp_stream = match self.tcp_stream {
            None => return Err(ClientError::new(ClientErrorType::NoAvailableTcpStream, "Could not retrieve tcp stream".to_string())),
            Some(tcp_stream) => tcp_stream
        };
        let _ = tcp_stream.set_read_timeout(Some(Duration::from_secs(5)));
        let mut buffer:Vec<u8> = Vec::new();
        let read_result = tcp_stream.read_to_end(buffer.as_mut());
        match read_result {
            Ok(_) => {
                Ok(String::from_utf8(buffer).unwrap())
            },
            Err(_) => Err(ClientError::new(ClientErrorType::WriteError, "Could not read data".to_string()))
        }
    }

    /**
     * Returns true if the tcp stream is not connected.
     */
    pub fn is_not_connected(&self) -> bool  {
        !self.tcp_stream.is_some()
    }

    /**
     * Returns a string with the host and port.
     */
    fn get_connect_str(&self) -> String {
        let mut connect_str = String::new();
        connect_str.push_str(&self.host);
        connect_str.push(':');
        connect_str.push_str(&self.port.to_string());
        connect_str
    }

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_connect_str() {
        let tcp_connection = TcpConnection::new("localhost".to_string(), 8080, Duration::from_secs(5));
        assert_eq!(tcp_connection.get_connect_str(), "localhost:8080");
    }

    #[test]
    fn test_is_not_connected() {
        let mut tcp_connection = TcpConnection::new("localhost".to_string(), 80, Duration::from_secs(5));
        assert_eq!(tcp_connection.is_not_connected(), true);
        tcp_connection.connect().unwrap();
        assert_eq!(tcp_connection.is_not_connected(), false);
        tcp_connection.write(&"GET / HTTP/1.1\r\n\r\n".to_string()).unwrap();
        tcp_connection.read().unwrap();
    }


}
