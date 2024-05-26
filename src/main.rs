use std::net::{ToSocketAddrs, TcpStream};
use url::Url;
use clap::Parser;
use std::time::Duration;

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

    let mut use_socket_addr = String::new();
    use_socket_addr.push_str(url_parts.host_str().expect("Missing host")); 
    use_socket_addr.push(':');
    use_socket_addr.push_str(url_parts.port_or_known_default().expect("Missing port").to_string().as_str());

    let socket_addr = use_socket_addr.to_socket_addrs().unwrap().next();
    println!("Connecting to host");

    match socket_addr {
        Some(socket_addr) => {
            let stream_result: Result<TcpStream, std::io::Error> = TcpStream::connect_timeout(&socket_addr, connection_timeout);
            match stream_result {
                Ok(_stream) => {
                    println!("Connected to host");
                },
                Err(err) => {
                    println!("Failed connecting to host: {}", err);
                }
            };
        },
        None => { panic!("No socket address could be parsed") }

    }

}
