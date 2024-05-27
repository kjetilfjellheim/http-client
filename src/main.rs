use url::Url;
use clap::Parser;
use std::time::Duration;
use crate::connection::TcpConnection;

mod connection;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Url to connect to.
    #[arg(short, long)]
    url: String,
    
    /// Connection timeout
    #[arg(short, long)]
    connection_timeout: u64
}

fn main() {
    // Parsing arguments
    let args = Args::parse();

    // Parsing url
    let url_parts: Url = if let Ok(url_result) = Url::parse(&args.url) {
        println!("Successfully parsed url");
        url_result
    } else {
        panic!("Failed could not parse url");
    };
 
    let connection_timeout = Duration::from_millis(args.connection_timeout);
    let host = url_parts.host_str().unwrap_or("localhost").to_string();
    let port = url_parts.port_or_known_default().unwrap_or(80);

    let connection = TcpConnection::new(host.clone(), port, connection_timeout);

    let connect = connection.connect();
    let _tcp_stream = match connect {
        Ok(tcp_stream) => { tcp_stream },
        Err(err) => { panic!("Connection failed: {}", err.message);  }
    };
    println!("Connection successful {}:{}", &host, port)

}
