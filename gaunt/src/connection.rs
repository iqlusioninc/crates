//! Connections to HTTP servers

#[cfg(feature = "slog")]
use slog::Logger;
use std::{
    fmt::Write as FmtWrite,
    io::Write,
    net::{TcpStream, ToSocketAddrs},
    sync::Mutex,
    time::{Duration, Instant},
};

use super::{HTTP_VERSION, USER_AGENT};
use error::Error;
use path::Path;
use response::ResponseReader;

/// Default timeout in milliseconds (5 seconds)
const DEFAULT_TIMEOUT_MS: u64 = 5000;

/// Options when building a `Connection`
pub struct ConnectionOptions {
    timeout: Duration,

    #[cfg(feature = "slog")]
    logger: Option<Logger>,
}

impl ConnectionOptions {
    /// Get default connection options
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the timeout
    pub fn timeout(&mut self, t: Duration) {
        self.timeout = t;
    }

    /// Set the logger
    #[cfg(feature = "slog")]
    pub fn logger(&mut self, l: Logger) {
        self.logger = Some(l)
    }
}

impl Default for ConnectionOptions {
    fn default() -> Self {
        Self {
            timeout: Duration::from_millis(DEFAULT_TIMEOUT_MS),

            #[cfg(feature = "slog")]
            logger: None,
        }
    }
}

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

    /// Logger for recording request details
    #[cfg(feature = "slog")]
    logger: Option<Logger>,
}

// TODO: use clippy's scoped lints once they work on stable
#[allow(unknown_lints, renamed_and_removed_lints, write_with_newline)]
impl Connection {
    /// Create a new connection to an HTTP server
    pub fn new(addr: &str, port: u16, opts: &ConnectionOptions) -> Result<Self, Error> {
        let host = format!("{}:{}", addr, port);

        let socketaddr = &host.to_socket_addrs()?.next().ok_or_else(|| {
            err!(
                AddrInvalid,
                "couldn't resolve DNS for {}",
                host.split(':').next().unwrap()
            )
        })?;

        // TODO: better timeout handling?
        let socket = TcpStream::connect_timeout(socketaddr, opts.timeout)?;
        socket.set_read_timeout(Some(opts.timeout))?;
        socket.set_write_timeout(Some(opts.timeout))?;

        Ok(Self {
            host,
            socket: Mutex::new(socket),
            created_at: Instant::now(),
            request_count: 0,
            #[cfg(feature = "slog")]
            logger: opts.logger.clone(),
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

        write!(request, "GET {} {}\r\n", path, HTTP_VERSION)?;
        write!(request, "Host: {}\r\n", self.host)?;
        write!(request, "User-Agent: {}\r\n", USER_AGENT)?;
        write!(request, "Content-Length: 0\r\n\r\n")?;

        #[cfg(feature = "slog")]
        let request_start = Instant::now();

        let mut socket = self.socket.lock().unwrap();
        socket.write_all(request.as_bytes())?;

        let response = ResponseReader::new(&mut socket)?.into();

        #[cfg(feature = "slog")]
        self.log("GET", &path, request_start);

        Ok(response)
    }

    /// Make an HTTP POST request to the given path
    pub fn post<P: Into<Path>>(&self, into_path: P, mut body: Vec<u8>) -> Result<Vec<u8>, Error> {
        let path = into_path.into();
        let mut headers = String::new();

        write!(headers, "POST {} {}\r\n", path, HTTP_VERSION)?;
        write!(headers, "Host: {}\r\n", self.host)?;
        write!(headers, "User-Agent: {}\r\n", USER_AGENT)?;
        write!(headers, "Content-Length: {}\r\n\r\n", body.len())?;

        // Make a Nagle-friendly request by combining headers and body
        let mut request: Vec<u8> = headers.into();
        request.append(&mut body);

        #[cfg(feature = "slog")]
        let request_start = Instant::now();

        let mut socket = self.socket.lock().unwrap();
        socket.write_all(&request)?;

        let response = ResponseReader::new(&mut socket)?.into();

        #[cfg(feature = "slog")]
        self.log("POST", &path, request_start);

        Ok(response)
    }

    /// Log information about a request (if `slog` feature is enabled)
    #[cfg(feature = "slog")]
    fn log(&self, method: &str, path: &Path, started_at: Instant) {
        let duration = Instant::now().duration_since(started_at);

        if let Some(log) = self.logger.as_ref() {
            debug!(
                log,
                "{method} {scheme}://{host}{path} ({duration_ms}ms)",
                method = method,
                scheme = "http", // TODO: https
                host = &self.host,
                path = path.as_ref(),
                duration_ms = duration.as_secs() * 1000 + u64::from(duration.subsec_millis())
            );
        }
    }
}
