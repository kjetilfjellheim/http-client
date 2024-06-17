use clap::Parser;
use std::collections::HashMap;
use std::time::Duration;
use url::Url;

pub enum ClientErrorType {
    IncorrectSocketAddr,
    ConnectionFailure,
    UnsupportedScheme,
    Unimplemented,
    NoAvailableTcpStream,
    WriteError
}

/**
 * Client error.
 */
pub struct ClientError {
    pub error_type: ClientErrorType,
    pub message: String
}

/**
 * Client error implementation.
 */
impl ClientError {
    pub fn new(error_type: ClientErrorType, message: String) -> ClientError {
        ClientError {
            error_type,
            message
        }
    }
}

/**
 * Arguments for the client.
 * Example: http-client --url http://localhost:8080
 */
#[derive(Parser, Debug)]
#[command(version, about = "Http client to test tls implementation.", long_about = None)]
pub struct Arguments {
    // Url to connect to. Example tcp://localhost:8080 or http://localhost:8080
    #[arg(long = "url")]
    pub url: String,
    
    // Connection timeout. Default 1000
    #[arg(long="connection_timeout")]
    pub connection_timeout: Option<u64>,

     // Proxy host: Example localhost
     #[arg(long = "proxyhost")]
     pub proxyhost: Option<String>,
     
     // Proxyport: Example 8080
     #[arg(long = "proxyport")]
     pub proxyport: Option<u16>,

    // Method: Example GET. Default GET
    #[arg(long = "method")]
    pub method: Option<String>,

    // Body: Example {}. 
    #[arg(long = "body")]
    pub body: Option<String>,

    // Headers, comma separated: Example Accept: application/json, Content-Type: text/xml . 
    #[arg(long = "headers")]
    pub headers: Option<String>,
}

/**
 * Parameters for the http client after parsing the arguments.
 */
pub struct Parameters {
    pub scheme: String,
    pub connect_host: String,
    pub connect_port: u16,
    pub path: String,
    pub connection_timeout: Duration,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>
}

impl Parameters {
    pub fn new(arguments: &Arguments) -> Parameters {
        // Parsing url
        let url_parts = get_url_parts(arguments);
        //Connect
        let connection_timeout = get_connection_timeout(&arguments.connection_timeout);
        let connect_host = &arguments.proxyhost.clone().unwrap_or(url_parts.host_str().unwrap_or("localhost").to_string()).clone();
        let connect_port = &arguments.proxyport.clone().unwrap_or(url_parts.port_or_known_default().unwrap_or(80));
        let headers = get_headers(&arguments.headers);
        let path = get_use_path(&url_parts, arguments.proxyhost.clone());
        let method = arguments.method.clone().unwrap_or("GET".to_string());

        Parameters {
            scheme: url_parts.scheme().to_string(),
            connect_host: connect_host.clone(),
            connect_port: connect_port.clone(),
            path,
            connection_timeout,
            method,
            headers,
            body: None
        }
    }


}

/**
 * Get url parts from the arguments.
 * If the url cannot be parsed, the program will panic.
 */
fn get_url_parts(arguments: &Arguments) -> Url {
    if let Ok(url_result) = Url::parse(&arguments.url) {
        url_result
    } else {
        panic!("Failed could not parse url");
    }
}

/**
 * Get connection timeout from the arguments.
 * If no connection timeout is provided, the default is 1000.
 */
fn get_connection_timeout(connection_timeout: &Option<u64>) -> Duration {
    Duration::from_millis(match connection_timeout {
        Some(connection_timeout) => connection_timeout.clone(),
        None => 1000
    })
}

/**
 * Get headers from the arguments.
 * Headers are comma separated.
 * Example: Accept: application/json, Content-Type: text/xml
 * 
 * Returns a hashmap with the headers.
 */
fn get_headers(headers: &Option<String>) -> HashMap<String, String> {
    match headers {
        Some(headers) => {
            headers.split(',').map(|header| {
                let header_parts: Vec<&str> = header.split(":").collect();
                (header_parts[0].to_string(), header_parts[1].to_string())
            }).collect()
        },
        None => HashMap::new()
    }
}
/**
 * Get the path to use for the request.
 * If a proxy host is provided, the path will include the scheme, host, port and path.
 * If no proxy host is provided, the path will only include the path.
 * 
 * Returns the path used in the request.
 */
fn get_use_path(url_parts: &Url, proxyhost: Option<String>) -> String {
    let mut path = String::new();
    if proxyhost.is_some() {
        path.push_str(&url_parts.scheme());
        path.push_str("://");
        path.push_str(&url_parts.host_str().unwrap_or("localhost"));
        path.push_str(":");
        path.push_str(&url_parts.port_or_known_default().unwrap_or(80).to_string());
        path.push_str(&url_parts.path());
    } else {
        path.push_str(&url_parts.path());
    }
    path
}

