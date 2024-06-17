use std::collections::HashMap;
use std::time::Duration;

use crate::connection::TcpConnection;
use crate::common::ClientError;

/**
 * Http client.
 * Sends http requests.
 */
pub struct HttpClient {
    tcp_connection: TcpConnection
}

impl HttpClient {
    pub fn new(host: String, port: u16, connection_timeout: Duration) -> HttpClient {
        let tcp_connection = TcpConnection::new(host, port, connection_timeout);
        HttpClient {
            tcp_connection
        }
    }

    pub fn send(mut self, http_request: HttpRequest) -> Result<(), ClientError> {
        if self.tcp_connection.is_not_connected() {
            self.tcp_connection.connect()?
        }
        let request_str: String = self.get_request_string(&http_request);
        let _ = &self.tcp_connection.write(&request_str)?;
        let read_result = &self.tcp_connection.read()?;
        let body = if read_result.len() > 0 {
            Some(read_result.clone())
        } else {
            None
        };
        Ok(()) 
    }

    pub fn get_request_string(&self, http_request: &HttpRequest) -> String {
        let mut request_string = http_request.method.clone();
        request_string.push_str( " ");
        request_string.push_str(&http_request.path.clone());
        request_string.push_str(" HTTP/1.1\n");
        http_request.headers.iter().for_each(|header| { 
            request_string.push_str(header.0);
            request_string.push_str(": ");
            request_string.push_str(header.1);
        });
        request_string.push_str("\n\n");
        request_string
    }

}

#[derive(Debug)]
pub struct HttpRequest {
    pub path: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>
}

impl HttpRequest {
    pub fn new(path: String, method: String, headers: HashMap<String, String>, body: Option<String>) -> HttpRequest {
        HttpRequest {
            path,
            method,
            headers,
            body
        }
    }
}
