use clap::Parser;

pub enum ClientErrorType {
    IncorrectSocketAddr,
    ConnectionFailure,
    UnsupportedScheme,
    Unimplemented,
    NoAvailableTcpStream,
    WriteError
}

pub struct ClientError {
    pub error_type: ClientErrorType,
    pub message: String
}

impl ClientError {
    pub fn new(error_type: ClientErrorType, message: String) -> ClientError {
        ClientError {
            error_type,
            message
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about = "Http client to test tls implementation.", long_about = None)]
pub struct Arguments {
    // Url to connect to. Example tcp://localhost:8080 or http://localhost:8080
    #[arg(short = 'u', long = "url")]
    pub url: String,
    
    // Connection timeout. Default 1000
    #[arg(short = 'c', long="connection_timeout")]
    pub connection_timeout: Option<u64>,

     // Proxy: Example localhost:7000
     #[arg(short = 'p', long = "proxy")]
     pub proxy: Option<String>,

    // Method: Example GET. Default GET
    #[arg(short = 'm', long = "method")]
    pub method: Option<String>,

    // Body: Example {}. 
    #[arg(short = 'b', long = "body")]
    pub body: Option<String>,

    // Headers, comma separated: Example Accept: application/json, Content-Type: text/xml . 
    #[arg(short = 'n', long = "headers")]
    pub headers: Option<String>,

}


