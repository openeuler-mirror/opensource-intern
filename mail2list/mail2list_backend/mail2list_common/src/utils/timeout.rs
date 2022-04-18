extern crate imap;
extern crate native_tls;

use imap::Client;
use native_tls::TlsConnector;
use native_tls::TlsStream;
use std::error::Error;
use std::fmt;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;

pub struct Timeout{}

impl Timeout {
    // resolve address and try to connect to all in order
    pub fn connect_all_timeout<A: ToSocketAddrs, S: AsRef<str>>(
        addr: A,
        domain: S,
        ssl_connector: &TlsConnector,
        timeout: Duration,
    ) -> Result<Client<TlsStream<TcpStream>>, Box<dyn Error>> {
        let addrs = addr.to_socket_addrs()?;

        for addr in addrs {
            match connect_timeout(&addr, &domain, ssl_connector, timeout) {
                Ok(client) => return Ok(client),
                Err(error) => eprintln!("couldn't connect to {}: {}", addr, error),
            }
        }

        Err(Box::new(TimeoutError))
    }

}

pub fn connect_timeout<S: AsRef<str>>(
    addr: &SocketAddr,
    domain: S,
    ssl_connector: &TlsConnector,
    timeout: Duration,
) -> Result<Client<TlsStream<TcpStream>>, Box<dyn Error>> {
    // the timeout is actually used with the initial TcpStream
    let tcp_stream = TcpStream::connect_timeout(addr, timeout)?;

    let tls_stream = TlsConnector::connect(ssl_connector, domain.as_ref(), tcp_stream)?;

    let mut client = Client::new(tls_stream);

    // don't forget to wait for the IMAP protocol server greeting ;)
    client.read_greeting()?;

    Ok(client)
}

// very simple timeout error; instead of printing the errors immediately like in
// `connect_all_timeout`, you may want to collect and return them
#[derive(Debug)]
struct TimeoutError;

impl fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "all addresses failed to connect")
    }
}

impl Error for TimeoutError {}