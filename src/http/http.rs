use std::collections::HashMap;
use std::time::Duration;

use crate::connection::TcpConnection;
use crate::common::{ ClientError, ClientErrorType };

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

    /**
     * Sends http request.
     * 
     * If the tcp connection is not connected, the client will attempt to connect.
     * Returns an error if the connection fails.
     */
    pub fn send(mut self, http_request: HttpRequest) -> Result<HttpResponse, ClientError> {
        if self.tcp_connection.is_not_connected() {
            self.tcp_connection.connect()?
        }
        let request_str: String = self.get_request_string(&http_request);
        let _ = &self.tcp_connection.write(&request_str)?;
        let read_result = &self.tcp_connection.read()?;
        if read_result.len() > 0 {
            HttpResponse::new(&read_result)
        } else {
            Err(ClientError::new(ClientErrorType::NoResponse,"No response".to_string()))
        }       
    }

    /**
     * Get request string from http request.
     * Example: GET / HTTP/1.1\nHost: localhost\n\n
     */
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

#[derive(Debug)]
pub struct HttpResponse {
    pub response_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Option<String>
}

impl HttpResponse {
    pub fn new(response: &String) -> Result<HttpResponse, ClientError> {
        let lines: std::str::Lines = response.lines();
        let mut response_code = 500;
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut body: Option<String> = None; 

        let mut body_start = false;

        for (index,line) in lines.enumerate() {
            if index == 0 {
                response_code = Self::get_response_code(&line);
            }            
            if line.is_empty() {
                body_start = true;
            } else if index > 0{
                if body_start {
                    body = Self::append_body(body, line.to_string());
                } else {
                    let header = Self::get_header(&line);
                    headers.insert(header.0, header.1);
                }
            }
        }

        Ok(HttpResponse {
            response_code,
            headers,
            body
        })
    }

    /**
     * Get response code from first line of response.
     * If the response code cannot be parsed, the default is 500.
     * The default is 500 because the response code is required.
     * If the response code is not provided, the server is not following the http protocol.
     */
    fn get_response_code(line: &str) -> u16 {
        line.split_whitespace().nth(1).unwrap_or("500").parse::<u16>().unwrap_or(500)
    }
    /**
     * Get header from line. 
     */
    fn get_header(line: &str) -> (String, String) {
        let header_parts: Vec<&str> = line.split(":").collect();
        (header_parts[0].to_string(), header_parts[1].to_string())
    }

    /**
     * Append line to body.
     */
    fn append_body(body: Option<String>, line: String) -> Option<String> {
        match body {
            Some(body) => {
                let mut new_body = body.clone();
                new_body.push_str(&line);
                Some(new_body)
            },
            None => Some(line)
        }
    }
}
