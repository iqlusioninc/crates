//! HTTPS client

use crate::{Path, Query, Result, USER_AGENT};
use hyper::{
    body::Buf,
    client::{Client, HttpConnector},
    header, Body, Request, Response,
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
    inner: InnerClient,
    hostname: String,
}

impl HttpsClient {
    /// Create a new HTTPS client which makes requests to the given hostname.
    pub fn new(hostname: impl Into<String>) -> Self {
        let client = Client::builder().build(Self::https_connector());

        Self {
            inner: InnerClient::Https(client),
            hostname: hostname.into(),
        }
    }

    /// Create a new HTTPS client which makes requests to the given hostname
    /// via the provided HTTP CONNECT proxy.
    // TODO(tarcieri): proxy auth
    #[cfg(feature = "proxy")]
    #[cfg_attr(docsrs, doc(cfg(feature = "proxy")))]
    pub fn new_with_proxy(hostname: impl Into<String>, proxy_uri: Uri) -> Result<Self> {
        let connector = Self::https_connector();
        let proxy = Proxy::new(Intercept::All, proxy_uri);
        let proxy_connector = ProxyConnector::from_proxy(connector, proxy).map_err(Error::Proxy)?;
        let client = Client::builder().build(proxy_connector);

        Ok(Self {
            inner: InnerClient::HttpsViaProxy(client),
            hostname: hostname.into(),
        })
    }

    /// Perform a low-level request using hyper's types directly.
    pub async fn request(&self, mut request: Request<Body>) -> Result<Response<Body>> {
        // TODO(tarcieri): avoid clobbering existing User-Agent header?
        add_header(&mut request, header::USER_AGENT, USER_AGENT)?;

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

        Ok(self.request(request).await?)
    }

    /// Perform HTTP GET request and return the response body.
    pub async fn get_body(&self, path: &Path, query: &Query) -> Result<impl Buf> {
        // TODO(tarcieri): timeouts
        let response = self.get(path, query).await?;
        Ok(hyper::body::aggregate(response.into_body()).await?)
    }

    /// Perform HTTP GET request and parse the response as JSON.
    #[cfg(feature = "json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "json")))]
    pub async fn get_json<T>(&self, path: &Path, query: &Query) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uri = query.to_request_uri(&self.hostname, path);

        let mut request = Request::builder()
            .method("GET")
            .uri(&uri)
            .body(Body::empty())?;

        add_header(&mut request, header::CONTENT_TYPE, "application/json")?;

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

/// Add an HTTP header to a request
// TODO(tarcieri): factor this onto a request wrapper type? better value type?
fn add_header(req: &mut Request<Body>, name: header::HeaderName, value: &str) -> Result<bool> {
    let headers = req.headers_mut();
    Ok(headers.insert(name, value.parse()?).is_some())
}
