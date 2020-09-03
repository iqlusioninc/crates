use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde::Serialize;
use serde_json::json;

pub async fn send_event<T>(value: &T, dd_api_key: String) -> Result<(), ()>
where
    T: Serialize,
{
    let event_json = json!({"ddsource":"datadog_crate", "service":"datadog_crate", "ddtags":"env:staging,user:datadog_crate", "hostname":"127.0.0.1", "message":value});
    let event = serde_json::to_string(&event_json).unwrap();
    println!("{:?}", event);

    // https://docs.datadoghq.com/api/v1/logs/#send-logs
    let req = Request::builder()
        .method(Method::POST)
        .uri("https://http-intake.logs.datadoghq.com/v1/input")
        .header("Content-Type", "application/json")
        .header("DD-API-KEY", dd_api_key)
        .body(Body::from(event))
        .unwrap();
    println!("{:?}", &req);

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let response = client.request(req).await.unwrap();
    if response.status().is_success() {
        Ok(())
    } else {
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::send_event;
    use serde_json::json;
    use std::env;

    fn block_on<F: std::future::Future>(f: F) -> F::Output {
        tokio::runtime::Builder::new()
            .basic_scheduler()
            .enable_all()
            .build()
            .unwrap()
            .block_on(f)
    }

    // Run test locally with `cargo test -- --ignored`
    #[test]
    #[ignore]
    fn test_send_event() {
        let dd_api_key = env::var("DD_API_KEY").unwrap();
        let message = "hello world! datadog crate test blob!!";

        let event = block_on(send_event(&message, dd_api_key));
        assert_eq!(event, Ok(()));
    }
}
