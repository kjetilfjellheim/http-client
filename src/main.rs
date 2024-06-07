use url::Url;
use std::time::Duration;

use clap::Parser;

use crate::common::{ ClientError, ClientErrorType, Arguments };
use crate::connection::TcpConnection;
use crate::http::HttpClient;

mod connection;
mod http;
mod common;

fn main() {
    // Parsing arguments
    let args = Arguments::parse();

    // Parsing url
    let url_parts: Url = if let Ok(url_result) = Url::parse(&args.url) {
        url_result
    } else {
        panic!("Failed could not parse url");
    };
 
    //Connect
    let connection_timeout = Duration::from_millis(args.connection_timeout);
    let host = url_parts.host_str().unwrap_or("localhost").to_string();
    let port = url_parts.port_or_known_default().unwrap_or(80);
    let tcp_connection = TcpConnection::new(host.clone(), port, connection_timeout);

    let result: Result<(), ClientError>  = match url_parts.scheme() {
        "http" => { 
            let  _http_client: HttpClient = HttpClient::new(tcp_connection);
            Ok(())
         },
        "https" => { 
            let  _http_client: HttpClient = HttpClient::new(tcp_connection);
            Ok(())
         },
        "tcp" => { 
            tcp_connection.connect()

        },
        _ => { Err(ClientError::new(ClientErrorType::UnsupportedScheme, "Unsupported scheme".to_string())) }

    };
    
    match result {
        Ok(_) => { println!("Success") },
        Err(err) => { panic!("Failed {}", err.message); }
    }

}
