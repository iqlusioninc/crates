//! Connections to HTTP servers

use std::{
    fmt::Write as FmtWrite,
    io::Write,
    net::{TcpStream, ToSocketAddrs},
    sync::Mutex,
    time::{Duration, Instant},
};

use super::USER_AGENT;
use error::Error;
use path::Path;
use response::ResponseReader;

/// Default timeout in milliseconds (5 seconds)
const DEFAULT_TIMEOUT_MS: u64 = 5000;

/// HTTP connection to a remote host
pub struct Connection {
    /// Host header to send in HTTP requests
    host: String,

    /// Open TCP socket to remote host
    socket: Mutex<TcpStream>,

    /// Session creation timestamp
    created_at: Instant,

    /// Number of requests performed since the connection was opened
    request_count: usize,
}

// TODO: use clippy's scoped lints once they work on stable
#[allow(unknown_lints, renamed_and_removed_lints, write_with_newline)]
impl Connection {
    /// Create a new connection to an HTTP server
    pub fn new(addr: &str, port: u16) -> Result<Self, Error> {
        let host = format!("{}:{}", addr, port);
        let timeout = Duration::from_millis(DEFAULT_TIMEOUT_MS);

        let socketaddr = &host.to_socket_addrs()?.next().ok_or_else(|| {
            err!(
                AddrInvalid,
                "couldn't resolve DNS for {}",
                host.split(':').next().unwrap()
            )
        })?;

        let socket = TcpStream::connect_timeout(socketaddr, timeout)?;
        socket.set_read_timeout(Some(timeout))?;
        socket.set_write_timeout(Some(timeout))?;

        Ok(Self {
            host,
            socket: Mutex::new(socket),
            created_at: Instant::now(),
            request_count: 0,
        })
    }

    /// How long has this connection been open?
    pub fn duration(&self) -> Duration {
        Instant::now().duration_since(self.created_at)
    }

    /// Number of requests we've made since opening connection
    pub fn request_count(&self) -> usize {
        self.request_count
    }

    /// Make an HTTP GET request to the given path
    pub fn get<P: Into<Path>>(&self, into_path: P) -> Result<Vec<u8>, Error> {
        let path = into_path.into();
        let mut request = String::new();

        write!(request, "GET {} HTTP/1.1\r\n", path)?;
        write!(request, "Host: {}\r\n", self.host)?;
        write!(request, "User-Agent: {}\r\n", USER_AGENT)?;
        write!(request, "Content-Length: 0\r\n\r\n")?;

        let mut socket = self.socket.lock().unwrap();

        let request_start = Instant::now();
        socket.write_all(request.as_bytes())?;

        let response = ResponseReader::new(&mut socket)?;
        let elapsed_time = Instant::now().duration_since(request_start);

        debug!(
            "host={} method=GET path={} t={}ms",
            &self.host,
            path,
            elapsed_time.as_secs() * 1000 + u64::from(elapsed_time.subsec_millis())
        );

        Ok(response.into())
    }

    /// Make an HTTP POST request to the given path
    pub fn post<P: Into<Path>>(&self, into_path: P, mut body: Vec<u8>) -> Result<Vec<u8>, Error> {
        let path = into_path.into();
        let mut headers = String::new();

        write!(headers, "POST {} HTTP/1.1\r\n", path)?;
        write!(headers, "Host: {}\r\n", self.host)?;
        write!(headers, "User-Agent: {}\r\n", USER_AGENT)?;
        write!(headers, "Content-Length: {}\r\n\r\n", body.len())?;

        // Make a Nagle-friendly request by combining headers and body
        let mut request: Vec<u8> = headers.into();
        request.append(&mut body);

        let mut socket = self.socket.lock().unwrap();
        socket.write_all(&request)?;

        Ok(ResponseReader::new(&mut socket)?.into())
    }
}
