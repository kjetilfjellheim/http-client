use clap::Parser;

pub enum ClientErrorType {
    IncorrectSocketAddr,
    ConnectionFailure,
    DisconnectionFailed,
    UnsupportedScheme
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
#[command(version, about, long_about = None)]
pub struct Arguments {
    /// Url to connect to.
    #[arg(short, long)]
    pub url: String,
    
    /// Connection timeout
    #[arg(short, long)]
    pub connection_timeout: u64
}