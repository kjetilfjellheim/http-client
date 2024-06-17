use common::Parameters;

use clap::Parser;

use crate::common::{ ClientError, ClientErrorType, Arguments };
use crate::http::HttpRequest;
use crate::http::HttpClient;

mod connection;
mod http;
mod common;

fn main() {
    // Parsing arguments
    let arguments = Arguments::parse();
    // Converting arguments to parameters used by the client
    let parameters = Parameters::new(&arguments);
    // Creating http client
    let http_client_result = get_http_client(&parameters);
    // Sending request
    send_request(http_client_result, parameters);
}

fn send_request(http_client_result: Result<HttpClient, ClientError>, parameters: Parameters) {
    match http_client_result {
        Ok(http_client) => { 
            let http_result = http_client.send(HttpRequest::new(parameters.path, parameters.method, parameters.headers, parameters.body));
            match http_result {
                Ok(http_response) => {
                    println!("Response: {:?}", http_response);
                },
                Err(err) => { println!("Failed {}", err.message); }
            }
        },
        Err(err) => { println!("Failed {}", err.message); }
    }
}

fn get_http_client(parameters: &Parameters) -> Result<HttpClient, ClientError> {
    match parameters.scheme.as_str() {
        "http" => { 
            Ok(HttpClient::new( parameters.connect_host.clone(), parameters.connect_port.clone(), parameters.connection_timeout))
         },
        "https" => { 
            Ok(HttpClient::new(parameters.connect_host.clone(), parameters.connect_port.clone(), parameters.connection_timeout))
         },
        _ => { Err(ClientError::new(ClientErrorType::UnsupportedScheme, "Unsupported scheme".to_string())) }
    }
}


