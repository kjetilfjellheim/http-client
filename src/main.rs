use common::Parameters;

use clap::Parser;

use crate::common::{ ClientError, ClientErrorType, Arguments };
use crate::http::HttpRequest;
use crate::http::HttpClient;

mod connection;
mod http;
mod common;

fn main() -> Result<(), ClientError> {
    // Parsing arguments
    let arguments = Arguments::parse();
    // Converting arguments to parameters used by the client
    let parameters = Parameters::new(&arguments)?;
    // Creating http client
    let http_client = get_http_client(&parameters)?;
    // Sending request
    send_request(http_client, parameters)?;
    // Ok
    Ok(())
}

fn send_request(http_client: HttpClient, parameters: Parameters) -> Result<(), ClientError> {
    let http_request = HttpRequest::new(parameters.path, parameters.method, parameters.headers, parameters.body);
    println!("Http request : {:?}", http_request);
    let http_result = http_client.send(http_request);
    match http_result {
        Ok(http_response) => {
            println!("Http response : {:?}", http_response);
            Ok(())
        },
        Err(err) => { Err(err) }
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


