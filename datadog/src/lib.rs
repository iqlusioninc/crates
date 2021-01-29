//! **datadog.rs**: Rust bindings to Datadog API.
//!
//! # About
//!
//! **datadog.rs** is an API wrapper which provides support for sending HTTPS log
//! events and stream events to Datadog. Post Stream event enables Pagerduty integration.
//! Future work will include error report integration and datadog-agent support.
//! Currently very alpha, though iqlusion will test in prod.
//!

use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde::{ser, Serialize};
use std::collections::BTreeMap as Map;
use std::time::SystemTime;

#[derive(Debug, Serialize)]
pub enum AlertType {
    Error,
    Warning,
    Info,
    Success,
    Recommendation,
    Snapshot,
}

#[derive(Debug, Serialize)]
pub enum Priority {
    Normal,
    Low,
}

/// Event struct
/// https://docs.datadoghq.com/api/v1/logs/#send-logs
#[derive(Debug, Serialize)]
pub struct Event {
    pub ddsource: String,
    pub service: String,
    #[serde(serialize_with = "serialize_ddtags")]
    pub ddtags: Option<DdTags>,
    pub hostname: String,
    pub message: String,
}

/// Stream Event struct
/// https://docs.datadoghq.com/api/latest/events/#post-an-event
#[derive(Debug, Serialize)]
pub struct StreamEvent {
    pub aggregation_key: Option<String>,
    pub alert_type: Option<AlertType>,
    pub date_happened: Option<SystemTime>,
    pub device_name: Option<String>,
    pub host: Option<String>,
    pub priority: Option<Priority>,
    pub related_event_id: Option<u64>,
    #[serde(serialize_with = "serialize_ddtags")]
    pub tags: Option<DdTags>,
    // Required
    pub text: String,
    // Required
    pub title: String,
}

/// Error struct
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Error {
    pub code: u16,
}

/// DdTags type
pub type DdTags = Map<String, String>;

fn serialize_ddtags<S>(ddtags: &Option<DdTags>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    if let Some(tags) = ddtags {
        tags.iter()
            .map(|(k, v)| [k.clone(), v.clone()].join(":"))
            .collect::<Vec<_>>()
            .join(",")
            .serialize(serializer)
    } else {
        serializer.serialize_none()
    }
}

/// Send a log event to Datadog via HTTPS. Requires DD_API_KEY env variable set.
/// https://docs.datadoghq.com/api/v1/logs/#send-logs
pub async fn send_event(value: &Event, dd_api_key: String) -> Result<(), Error> {
    let event = serde_json::to_string(&value).unwrap();

    let req = Request::builder()
        .method(Method::POST)
        .uri("https://http-intake.logs.datadoghq.com/v1/input")
        .header("Content-Type", "application/json")
        .header("DD-API-KEY", dd_api_key)
        .body(Body::from(event))
        .unwrap();

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

/// Send a stream event to Datadog via HTTPS. Requires DD_API_KEY env variable set.
/// https://docs.datadoghq.com/api/latest/events/#post-an-event
pub async fn send_stream_event(value: &StreamEvent, dd_api_key: String) -> Result<(), Error> {
    let stream_event = serde_json::to_string(&value).unwrap();

    let request = Request::builder()
        .method(Method::POST)
        .uri("https://api.datadoghq.com/api/v1/events")
        .header("Content-Type", "application/json")
        .header("DD-API-KEY", dd_api_key)
        .body(Body::from(stream_event))
        .unwrap();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let response = client.request(request).await.unwrap();
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
    use super::{send_event, send_stream_event, Event, StreamEvent};
    use crate::AlertType::Error;
    use crate::Priority::Normal;
    use hostname;
    use std::collections::BTreeMap;
    use std::env;
    use std::time::SystemTime;

    fn block_on<F: std::future::Future>(f: F) -> F::Output {
        tokio::runtime::Builder::new_current_thread()
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
            ddtags: Some(ddtags),
            hostname: "127.0.0.1".to_owned(),
            message: "hello world! datadog crate test blob!!".to_owned(),
        };

        let event = block_on(send_event(&event, dd_api_key));
        assert_eq!(event, Ok(()));
    }

    // Set env var with `export DD_API_KEY=<YOUR_DATADOG_API_KEY>`
    // Run test locally with `cargo test -- --ignored`
    #[test]
    #[ignore]
    fn test_send_stream_event() {
        let dd_api_key = env::var("DD_API_KEY").unwrap();
        let mut ddtags = BTreeMap::new();
        ddtags.insert("env".to_owned(), "staging".to_owned());

        let hostname = hostname::get().unwrap();

        let stream_event = StreamEvent {
            aggregation_key: None,
            alert_type: Some(Error),
            date_happened: None,
            device_name: None,
            host: Some(hostname.to_string_lossy().to_string()),
            priority: Some(Normal),
            related_event_id: None,
            tags: Some(ddtags),
            // Text field must contain @pagerduty to trigger alert.
            // Limited to 4000 characters, supports markdown.
            // To use markdown, start the text block with %%% \n and end the text block with \n %%%.
            text: "@pagerduty 💾🐶📦".to_owned(),
            title: "datadog 💾🐶📦 test".to_owned(),
        };

        let stream_event = block_on(send_stream_event(&stream_event, dd_api_key));
        assert_eq!(stream_event, Ok(()));
    }
}
