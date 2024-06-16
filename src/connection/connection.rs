use std::io::{Read, Write};
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

    pub fn connect(&mut self) -> Result<(), ClientError> {
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
        println!("Connecting {}", &connect_str);
        let stream_result: Result<TcpStream, std::io::Error> = TcpStream::connect_timeout(&socket_addr, self.connection_timeout);
        self.tcp_stream = match stream_result {
            Ok(stream) => {
                Some(stream)
            },
            Err(err) => {
                return Err(ClientError::new(ClientErrorType::ConnectionFailure, err.to_string()))
            }
        };
        println!("Connected {}", &connect_str);
        Ok(())
    }

    pub fn write(&mut self, request_str: &String) -> Result<(), ClientError>{
        let mut tcp_stream = match &self.tcp_stream {
            None => return Err(ClientError::new(ClientErrorType::NoAvailableTcpStream, "Could not retrieve tcp stream".to_string())),
            Some(tcp_stream) => tcp_stream
        };
        tcp_stream.set_read_timeout(Some(Duration::from_millis(1)));
        println!("Writing request {}", request_str);
        let write_result = tcp_stream.write_all(request_str.as_bytes());
        match write_result {
            Ok(_) => Ok(()),
            Err(_) => Err(ClientError::new(ClientErrorType::WriteError, "Could not write data".to_string()))
        }
    }

    pub fn read(self) -> Result<String, ClientError> {
        let mut tcp_stream = match self.tcp_stream {
            None => return Err(ClientError::new(ClientErrorType::NoAvailableTcpStream, "Could not retrieve tcp stream".to_string())),
            Some(tcp_stream) => tcp_stream
        };
        let mut buffer:Vec<u8> = Vec::new();
        let read_result = tcp_stream.read_to_end(buffer.as_mut());
        match read_result {
            Ok(_) => {
                let response = String::from_utf8(buffer).unwrap();
                println!("Reading response {}", response);
                Ok(response) 
            },
            Err(_) => Err(ClientError::new(ClientErrorType::WriteError, "Could not read data".to_string()))
        }
    }

    pub fn is_not_connected(&self) -> bool  {
        !self.tcp_stream.is_some()
    }

    fn get_connect_str(&self) -> String {
        let mut connect_str = String::new();
        connect_str.push_str(&self.host);
        connect_str.push(':');
        connect_str.push_str(&self.port.to_string());
        connect_str
    }

}



