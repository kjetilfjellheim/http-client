use clap::Parser;
use std::collections::HashMap;
use std::time::Duration;
use url::Url;

#[derive(Debug, PartialEq)]
pub enum ClientErrorType {
    IncorrectSocketAddr,
    UnparseableUrl,
    ConnectionFailure,
    UnsupportedScheme,
    Unimplemented,
    NoAvailableTcpStream,
    WriteError,
    NoResponse,
}

/**
 * Client error.
 */
#[derive(Debug, PartialEq)]
 pub struct ClientError {
    pub error_type: ClientErrorType,
    pub message: String,
}

/**
 * Client error implementation.
 */
impl ClientError {
    pub fn new(error_type: ClientErrorType, message: String) -> ClientError {
        ClientError {
            error_type,
            message,
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
    #[arg(long = "connection-timeout", default_value = "1000")]
    pub connection_timeout: Option<u64>,

    // Proxy host: Example localhost
    #[arg(long = "proxyhost")]
    pub proxyhost: Option<String>,

    // Proxyport: Example 8080
    #[arg(long = "proxyport")]
    pub proxyport: Option<u16>,

    // Method: Example GET. Default GET
    #[arg(long = "method", default_value = "GET")]
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
    pub body: Option<String>,
}

impl Parameters {

    const DEFAULT_CONNECTION_TIMEOUT: u64 = 1000;
    const DEFAULT_HOST: &'static str = "localhost";
    const DEFAULT_METHOD: &'static str = "GET";

    pub fn new(arguments: &Arguments) -> Result<Parameters, ClientError> {
        // Parsing url
        let url_parts = Self::get_url_parts(&arguments.url)?;
        //Connect
        let connection_timeout = Self::get_connection_timeout(&arguments.connection_timeout);
        let connect_host = &arguments
            .proxyhost
            .clone()
            .unwrap_or(url_parts.host_str().unwrap_or(Parameters::DEFAULT_HOST).to_string())
            .clone();
        let connect_port = &arguments
            .proxyport
            .clone()
            .unwrap_or(url_parts.port_or_known_default().unwrap_or(80));
        let headers = Self::get_headers(&arguments.headers);
        let path = Self::get_use_path(&url_parts, arguments.proxyhost.clone());
        let method = arguments.method.clone().unwrap_or(Parameters::DEFAULT_METHOD.to_string());
        let body = arguments.body.clone();
        Ok(Parameters {
            scheme: url_parts.scheme().to_string(),
            connect_host: connect_host.clone(),
            connect_port: connect_port.clone(),
            path,
            connection_timeout,
            method,
            headers,
            body
        })
    }

    /**
     * Get url parts from the arguments.
     */
    fn get_url_parts(url: &str) -> Result<Url, ClientError> {
        if let Ok(url_result) = Url::parse(url) {
            Ok(url_result)
        } else {
            Err(ClientError::new(ClientErrorType::UnparseableUrl, "Url could not be parsed".to_string()))
        }
    }

    /**
     * Get connection timeout from the arguments.
     * If no connection timeout is provided, the default is 1000.
     */
    fn get_connection_timeout(connection_timeout: &Option<u64>) -> Duration {
        Duration::from_millis(match connection_timeout {
            Some(connection_timeout) => connection_timeout.clone(),
            None => Self::DEFAULT_CONNECTION_TIMEOUT,
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
            Some(headers) => headers
                .split(',')
                .map(|header| {
                    let header_parts: Vec<&str> = header.split(":").collect();
                    (header_parts[0].trim().to_string(), header_parts[1].trim().to_string())
                })
                .collect(),
            None => HashMap::new(),
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_use_path_no_proxy() -> Result<(), String> {
        let url = Url::parse("http://localhost:8080").unwrap();
        let path = Parameters::get_use_path(&url, None);
        assert_eq!(path, "/");
        Ok(())  
    }

    #[test]
    fn test_get_use_path_proxy() -> Result<(), String> {
        let url = Url::parse("http://localhost:8080/test").unwrap();
        let proxyhost = Some("http://localhost:8888".to_string());
        let path = Parameters::get_use_path(&url, proxyhost);
        assert_eq!(path, "http://localhost:8080/test");
        Ok(())  
    }

    #[test]
    fn test_get_headers_none() -> Result<(), String> {
        let headers = Parameters::get_headers(&None);
        assert_eq!(0, headers.len());
        Ok(())  
    }

    #[test]
    fn test_get_headers_one_header() -> Result<(), String> {
        let headers = Parameters::get_headers(&Some("accept: application/json".to_string()));
        assert_eq!(1, headers.len());
        assert_eq!(headers.get("accept").unwrap(), "application/json");
        Ok(())  
    }

    #[test]
    fn test_get_headers_multiple_headers() -> Result<(), String> {
        let headers = Parameters::get_headers(&Some("accept: application/json, content-type: text/xml".to_string()));
        assert_eq!(2, headers.len());
        assert_eq!(headers.get("accept").unwrap(), "application/json");
        assert_eq!(headers.get("content-type").unwrap(), "text/xml");
        Ok(())  
    }

    #[test]
    fn test_get_connection_timeout_none() -> Result<(), String> {
        let connection_timeout = Parameters::get_connection_timeout(&None);
        assert_eq!(u128::from(Parameters::DEFAULT_CONNECTION_TIMEOUT), connection_timeout.as_millis());
        Ok(())  
    }

    #[test]
    fn test_get_connection_timeout_with_value() -> Result<(), String> {
        let connection_timeout = Parameters::get_connection_timeout(&Some(2000));
        assert_eq!(2000, connection_timeout.as_millis());
        Ok(())  
    }

    #[test]
    fn test_successful_url() {
        let url = "http://localhost:8080/test";
        let url_parts = Parameters::get_url_parts(&url);
        assert!(url_parts.is_ok());
    }

    #[test]
    fn test_arguments_to_parameters() {
        let arguments = Arguments {
            url: "http://localhost:8080/test".to_string(),
            connection_timeout: Some(2000),
            proxyhost: Some("localhost".to_string()),
            proxyport: Some(8888),
            method: Some("GET".to_string()),
            headers: Some("accept: application/json, content-type: text/xml".to_string()),
            body: Some("{}".to_string()),
        };
        let parameters = Parameters::new(&arguments);
        assert!(parameters.is_ok());
        assert_eq!(parameters.as_ref().unwrap().scheme, "http");
        assert_eq!(parameters.as_ref().unwrap().connect_host, "localhost");
        assert_eq!(parameters.as_ref().unwrap().connect_port, 8888);
        assert_eq!(parameters.as_ref().unwrap().path, "http://localhost:8080/test");
        assert_eq!(parameters.as_ref().unwrap().connection_timeout.as_millis(), 2000);
        assert_eq!(parameters.as_ref().unwrap().method, "GET");
        assert_eq!(parameters.as_ref().unwrap().headers.len(), 2);
        assert!(parameters.as_ref().unwrap().body.is_some());
    }

    #[test]
    fn test_create_client_error() {
        let client_error = ClientError::new(ClientErrorType::UnparseableUrl, "Url could not be parsed".to_string());
        assert_eq!(client_error.error_type, ClientErrorType::UnparseableUrl);
        assert!(client_error.message == "Url could not be parsed");
    }
}