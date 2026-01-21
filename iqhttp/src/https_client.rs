//! HTTPS client

use crate::{Path, Query, Result, USER_AGENT};
use hyper::{
    body::Buf,
    client::{Client, HttpConnector},
    header::{self, HeaderMap, HeaderName, HeaderValue},
    Body, Request, Response,
};
use hyper_rustls::HttpsConnector;

#[cfg(feature = "json")]
use serde::de::DeserializeOwned;

#[cfg(feature = "proxy")]
use {
    crate::{Error, Uri},
    hyper_proxy::{Intercept, Proxy, ProxyConnector},
};

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
    pub fn new(hostname: impl Into<String>) -> Self {
        let client = Client::builder().build(Self::https_connector());

        Self {
            inner: InnerClient::Https(client),
            hostname: hostname.into(),
            headers: default_headers(),
        }
    }

    /// Create a new HTTPS client which makes requests to the given hostname
    /// via the provided HTTP CONNECT proxy.
    // TODO(tarcieri): proxy auth
    #[cfg(feature = "proxy")]
    pub fn new_with_proxy(hostname: impl Into<String>, proxy_uri: Uri) -> Result<Self> {
        let connector = Self::https_connector();
        let proxy = Proxy::new(Intercept::All, proxy_uri);
        let proxy_connector = ProxyConnector::from_proxy(connector, proxy).map_err(Error::Proxy)?;
        let client = Client::builder().build(proxy_connector);

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
    pub async fn request(&self, mut request: Request<Body>) -> Result<Response<Body>> {
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
    pub async fn get(&self, path: &Path, query: &Query) -> Result<Response<Body>> {
        let uri = query.to_request_uri(&self.hostname, path);

        // TODO(tarcieri): better errors
        let request = Request::builder()
            .method("GET")
            .uri(&uri)
            .body(Body::empty())?;

        self.request(request).await
    }

    /// Perform HTTP GET request and return the response body.
    pub async fn get_body(&self, path: &Path, query: &Query) -> Result<impl Buf + use<>> {
        // TODO(tarcieri): timeouts
        let response = self.get(path, query).await?;
        Ok(hyper::body::aggregate(response.into_body()).await?)
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
            .body(Body::empty())?;

        request
            .headers_mut()
            .append(header::CONTENT_TYPE, "application/json".parse()?);

        let response = self.request(request).await?;
        let body = hyper::body::aggregate(response.into_body()).await?;
        Ok(serde_json::from_reader(body.reader())?)
    }

    /// Get the `HttpsConnector` to use.
    fn https_connector() -> HttpsConnector<HttpConnector> {
        HttpsConnector::with_native_roots()
    }
}

/// Inner client type which abstracts over the presence or absence of a proxy
enum InnerClient {
    /// HTTPS client (no proxy)
    Https(Client<HttpsConnector<HttpConnector>, Body>),

    /// HTTPS client with proxy
    #[cfg(feature = "proxy")]
    HttpsViaProxy(Client<ProxyConnector<HttpsConnector<HttpConnector>>, Body>),
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
