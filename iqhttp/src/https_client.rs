//! HTTPS client

use crate::{Path, Query, Result, USER_AGENT};
use http_body_util::{BodyExt, Full};
use hyper::{
    Request, Response,
    body::{Buf, Bytes, Incoming},
    header::{self, HeaderMap, HeaderName, HeaderValue},
};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use hyper_util::{
    client::legacy::{Client, connect::HttpConnector},
    rt::TokioExecutor,
};

#[cfg(feature = "json")]
use serde::de::DeserializeOwned;

#[cfg(feature = "proxy")]
use {crate::Uri, hyper_util::client::legacy::connect::proxy::Tunnel};

/// HTTPS client.
///
/// This type provides a persistent connection to a particular hostname and
/// allows requests by path and query string.
pub struct HttpsClient {
    /// Enum over possible `hyper` clients.
    inner: InnerClient,

    /// Hostname this client is making requests to.
    hostname: String,

    /// Headers to send in the request.
    headers: HeaderMap,
}

impl HttpsClient {
    /// Create a new HTTPS client which makes requests to the given hostname.
    ///
    /// Returns an error if the operating system's root certificates can't be
    /// loaded.
    pub fn new(hostname: impl Into<String>) -> Result<Self> {
        let connector = Self::https_connector_builder()?.build();
        let client = Client::builder(TokioExecutor::new()).build(connector);

        Ok(Self {
            inner: InnerClient::Https(client),
            hostname: hostname.into(),
            headers: default_headers(),
        })
    }

    /// Create a new HTTPS client which makes requests to the given hostname
    /// via the provided HTTP CONNECT proxy.
    // TODO(tarcieri): proxy auth
    #[cfg(feature = "proxy")]
    pub fn new_with_proxy(hostname: impl Into<String>, proxy_uri: Uri) -> Result<Self> {
        // TLS runs end-to-end with the destination over the CONNECT tunnel
        let tunnel = Tunnel::new(proxy_uri, HttpConnector::new());
        let connector = Self::https_connector_builder()?.wrap_connector(tunnel);
        let client = Client::builder(TokioExecutor::new()).build(connector);

        Ok(Self {
            inner: InnerClient::HttpsViaProxy(client),
            hostname: hostname.into(),
            headers: default_headers(),
        })
    }

    /// Borrow the request headers mutably.
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }

    /// Add a header to this request context.
    pub fn add_header(&mut self, name: HeaderName, value: &str) -> Result<Option<HeaderValue>> {
        Ok(self.headers.insert(name, value.parse()?))
    }

    /// Perform a low-level request using hyper's types directly.
    pub async fn request(&self, mut request: Request<Full<Bytes>>) -> Result<Response<Incoming>> {
        if request.headers().is_empty() {
            *request.headers_mut() = self.headers.clone();
        } else {
            for (name, value) in &self.headers {
                request.headers_mut().append(name, value.clone());
            }
        }

        Ok(match &self.inner {
            InnerClient::Https(client) => client.request(request),
            #[cfg(feature = "proxy")]
            InnerClient::HttpsViaProxy(client) => client.request(request),
        }
        .await?)
    }

    /// Perform HTTP GET request for the given [`Path`] and [`Query`].
    pub async fn get(&self, path: &Path, query: &Query) -> Result<Response<Incoming>> {
        let uri = query.to_request_uri(&self.hostname, path);

        // TODO(tarcieri): better errors
        let request = Request::builder()
            .method("GET")
            .uri(&uri)
            .body(Full::default())?;

        self.request(request).await
    }

    /// Perform HTTP GET request and return the response body.
    pub async fn get_body(&self, path: &Path, query: &Query) -> Result<impl Buf + use<>> {
        // TODO(tarcieri): timeouts
        let response = self.get(path, query).await?;
        Ok(response.into_body().collect().await?.aggregate())
    }

    /// Perform HTTP GET request and parse the response as JSON.
    #[cfg(feature = "json")]
    pub async fn get_json<T>(&self, path: &Path, query: &Query) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uri = query.to_request_uri(&self.hostname, path);

        let mut request = Request::builder()
            .method("GET")
            .uri(&uri)
            .body(Full::default())?;

        request
            .headers_mut()
            .append(header::CONTENT_TYPE, "application/json".parse()?);

        let response = self.request(request).await?;
        let body = response.into_body().collect().await?.aggregate();
        Ok(serde_json::from_reader(body.reader())?)
    }

    /// Get an HTTPS-only connector builder using native roots and the `ring`
    /// crypto provider.
    ///
    /// The provider is passed explicitly rather than relying on a process-wide
    /// default, which panics if more than one provider is enabled.
    fn https_connector_builder()
    -> Result<HttpsConnectorBuilder<hyper_rustls::builderstates::WantsProtocols2>> {
        Ok(HttpsConnectorBuilder::new()
            .with_provider_and_native_roots(rustls::crypto::ring::default_provider())?
            .https_only()
            .enable_http1())
    }
}

/// Inner client type which abstracts over the presence or absence of a proxy
enum InnerClient {
    /// HTTPS client (no proxy)
    Https(Client<HttpsConnector<HttpConnector>, Full<Bytes>>),

    /// HTTPS client with proxy
    #[cfg(feature = "proxy")]
    HttpsViaProxy(Client<HttpsConnector<Tunnel<HttpConnector>>, Full<Bytes>>),
}

/// Default headers to send with requests.
fn default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        USER_AGENT.parse().expect("USER_AGENT invalid"),
    );
    headers
}
