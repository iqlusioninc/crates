//! Connections to HTTP servers

#[cfg(feature = "logger")]
use slog::Logger;
use std::{
    fmt::Write as FmtWrite,
    io::Write,
    net::{TcpStream, ToSocketAddrs},
    ops::DerefMut,
    string::String,
    sync::Mutex,
    time::{Duration, Instant},
    vec::Vec,
};

use super::{HTTP_VERSION, USER_AGENT};
use crate::error::Error;
use crate::path::PathBuf;
use crate::request;
use crate::response;

/// Default timeout in milliseconds (5 seconds)
const DEFAULT_TIMEOUT_MS: u64 = 5000;

/// Options when building a `Connection`
pub struct ConnectionOptions {
    timeout: Duration,

    #[cfg(feature = "logger")]
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
    #[cfg(feature = "logger")]
    pub fn logger(&mut self, l: Logger) {
        self.logger = Some(l)
    }
}

impl Default for ConnectionOptions {
    fn default() -> Self {
        Self {
            timeout: Duration::from_millis(DEFAULT_TIMEOUT_MS),

            #[cfg(feature = "logger")]
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
    #[cfg(feature = "logger")]
    logger: Option<Logger>,
}

impl Connection {
    /// Create a new connection to an HTTP server
    pub fn open(addr: &str, port: u16, opts: &ConnectionOptions) -> Result<Self, Error> {
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
            #[cfg(feature = "logger")]
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
    pub fn get<P: Into<PathBuf>>(
        &self,
        into_path: P,
        body: &request::Body,
    ) -> Result<response::Body, Error> {
        let path = into_path.into();
        let mut request = String::new();

        if !body.0.is_empty() {
            panic!("GET request bodies unsupported!");
        }

        writeln!(request, "GET {} {}\r", path, HTTP_VERSION)?;
        writeln!(request, "Host: {}\r", self.host)?;
        writeln!(request, "User-Agent: {}\r", USER_AGENT)?;
        writeln!(request, "Content-Length: {}\r", body.0.len())?;
        writeln!(request, "\r")?;

        #[cfg(feature = "logger")]
        let request_start = Instant::now();

        let mut socket = self.socket.lock().unwrap();
        socket.write_all(request.as_bytes())?;

        let response_body = response::Reader::new(socket.deref_mut())?.into_body();

        #[cfg(feature = "logger")]
        self.log("GET", &path, request_start);

        Ok(response_body)
    }

    /// Make an HTTP POST request to the given path
    pub fn post<P: Into<PathBuf>>(
        &self,
        into_path: P,
        body: &request::Body,
    ) -> Result<response::Body, Error> {
        let path = into_path.into();
        let mut headers = String::new();

        writeln!(headers, "POST {} {}\r", path, HTTP_VERSION)?;
        writeln!(headers, "Host: {}\r", self.host)?;
        writeln!(headers, "User-Agent: {}\r", USER_AGENT)?;
        writeln!(headers, "Content-Length: {}\r", body.0.len())?;
        writeln!(headers, "\r")?;

        // Make a Nagle-friendly request by combining headers and body
        let mut request: Vec<u8> = headers.into();
        request.extend_from_slice(body.0.as_slice());

        #[cfg(feature = "logger")]
        let request_start = Instant::now();

        let mut socket = self.socket.lock().unwrap();
        socket.write_all(&request)?;

        let response_body = response::Reader::new(socket.deref_mut())?.into_body();

        #[cfg(feature = "logger")]
        self.log("POST", &path, request_start);

        Ok(response_body)
    }

    /// Log information about a request (if `logger` feature is enabled)
    #[cfg(feature = "logger")]
    fn log(&self, method: &str, path: &PathBuf, started_at: Instant) {
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
