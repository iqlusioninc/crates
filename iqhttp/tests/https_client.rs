//! `HttpsClient` tests which don't require network access.

use iqhttp::{HttpsClient, Request, body::Full};

#[tokio::test]
async fn new_loads_native_roots() {
    HttpsClient::new("example.com").expect("client should build");
}

#[tokio::test]
async fn plaintext_http_is_refused() {
    let client = HttpsClient::new("example.com").expect("client should build");
    let request = Request::get("http://127.0.0.1:1/")
        .body(Full::default())
        .expect("request should build");

    assert!(client.request(request).await.is_err());
}

#[cfg(feature = "proxy")]
#[tokio::test]
async fn proxy_issues_connect_to_destination() {
    use iqhttp::Query;
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
    };

    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let proxy_uri = format!("http://{}", listener.local_addr().expect("addr"))
        .parse()
        .expect("proxy URI");

    let proxy = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.expect("accept");
        let mut buf = vec![0u8; 1024];
        let n = stream.read(&mut buf).await.expect("read");
        stream
            .write_all(b"HTTP/1.1 403 Forbidden\r\ncontent-length: 0\r\n\r\n")
            .await
            .expect("write");
        String::from_utf8_lossy(&buf[..n]).into_owned()
    });

    let client = HttpsClient::new_with_proxy("example.com", proxy_uri).expect("client");
    let result = client.get("/", &Query::new()).await;
    assert!(result.is_err(), "proxy refused the tunnel");

    let connect = proxy.await.expect("proxy task");
    assert!(
        connect.starts_with("CONNECT example.com:443 HTTP/1.1\r\n"),
        "unexpected proxy request: {connect:?}"
    );
}
