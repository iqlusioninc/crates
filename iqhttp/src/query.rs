//! HTTP query strings.

use crate::Uri;
use std::{
    collections::BTreeMap as Map,
    fmt::{self, Display},
    iter::FromIterator,
};

/// HTTP query string: parameters to a request included in the URL.
///
/// <https://en.wikipedia.org/wiki/Query_string>
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Query(Map<String, String>);

impl Query {
    /// Create params
    pub fn new() -> Self {
        Self::default()
    }

    /// Add params
    pub fn add(&mut self, field: impl Into<String>, value: impl Into<String>) -> bool {
        // TODO(tarcieri): return result?
        self.0.insert(field.into(), value.into()).is_none()
    }

    /// Compute [`Uri`]
    // TODO(tarcieri): factor this onto a prospective `Path` type
    pub(crate) fn to_request_uri(&self, hostname: &str, path: &str) -> Uri {
        let path_and_query = format!("{}?{}", path, self);

        Uri::builder()
            .scheme("https")
            .authority(hostname)
            .path_and_query(path_and_query.as_str())
            .build()
            .expect("error building request from hostname and Query")
    }
}

impl Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, (field, value)) in self.0.iter().enumerate() {
            write!(f, "{}={}", field, value)?;

            if i < self.0.len() - 1 {
                write!(f, "&")?;
            }
        }

        Ok(())
    }
}

impl<'a> FromIterator<&'a (String, String)> for Query {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a (String, String)>,
    {
        let mut params = Self::new();

        for (field, value) in iter {
            params.add(field, value);
        }

        params
    }
}

#[cfg(test)]
mod tests {
    use super::{FromIterator, Query};

    #[test]
    fn params_to_string() {
        let params = Query::from_iter(&[
            ("foo".to_owned(), "value_1".to_owned()),
            ("bar".to_owned(), "value_2".to_owned()),
        ]);

        let serialized_params = params.to_string();
        assert_eq!(&serialized_params, "bar=value_2&foo=value_1");
    }
}
