use url::Url;
use std::collections::HashMap;
use std::time::Duration;

use clap::Parser;

use crate::common::{ ClientError, ClientErrorType, Arguments };
use crate::http::HttpRequest;
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
    let connection_timeout = Duration::from_millis(match args.connection_timeout {
        Some(connection_timeout) => connection_timeout,
        None => 1000
    });
    let host = url_parts.host_str().unwrap_or("localhost").to_string();
    let port: u16 = url_parts.port_or_known_default().unwrap_or(80);

    let http_client_result = match url_parts.scheme() {
        "http" => { 
            Ok(HttpClient::new(host, port, connection_timeout))
         },
        "https" => { 
            Ok(HttpClient::new(host, port, connection_timeout))
         },
        _ => { Err(ClientError::new(ClientErrorType::UnsupportedScheme, "Unsupported scheme".to_string())) }
    };
    
    match http_client_result {
        Ok(http_client) => { 
            http_client.send(HttpRequest::new(url_parts.path().to_string(), args.method.unwrap_or("GET".to_string()), HashMap::new(), None));
        },
        Err(err) => { panic!("Failed {}", err.message); }
    }

}
