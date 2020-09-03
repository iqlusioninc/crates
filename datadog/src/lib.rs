use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde::{ser, Serialize};
use std::collections::BTreeMap as Map;

#[derive(Debug, Serialize)]
pub struct Event {
    pub ddsource: String,
    pub service: String,
    #[serde(serialize_with = "serialize_ddtags")]
    pub ddtags: DdTags,
    pub hostname: String,
    pub message: String,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Error {
    pub code: u16,
}

pub type DdTags = Map<String, String>;

fn serialize_ddtags<S>(ddtags: &DdTags, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    ddtags
        .iter()
        .map(|(k, v)| [k.clone(), v.clone()].join(":"))
        .collect::<Vec<_>>()
        .join(",")
        .serialize(serializer)
}

pub async fn send_event<T>(value: &T, dd_api_key: String) -> Result<(), Error>
where
    T: Serialize,
{
    let event = serde_json::to_string(&value).unwrap();
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
        Err(Error {
            code: response.status().as_u16(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{send_event, Event};
    use std::collections::BTreeMap;
    use std::env;

    fn block_on<F: std::future::Future>(f: F) -> F::Output {
        tokio::runtime::Builder::new()
            .basic_scheduler()
            .enable_all()
            .build()
            .unwrap()
            .block_on(f)
    }

    // Set env var with `export DD_API_KEY=<YOUR_DATADOG_API_KEY>`
    // Run test locally with `cargo test -- --ignored`
    #[test]
    #[ignore]
    fn test_send_event() {
        let dd_api_key = env::var("DD_API_KEY").unwrap();
        let mut ddtags = BTreeMap::new();
        ddtags.insert("env".to_owned(), "staging".to_owned());
        ddtags.insert("user".to_owned(), "datadog_crate".to_owned());

        let event = Event {
            ddsource: "datadog_crate".to_owned(),
            service: "datadog_crate".to_owned(),
            ddtags: ddtags,
            hostname: "127.0.0.1".to_owned(),
            message: "hello world! datadog crate test blob!!".to_owned(),
        };

        let event = block_on(send_event(&event, dd_api_key));
        assert_eq!(event, Ok(()));
    }
}
