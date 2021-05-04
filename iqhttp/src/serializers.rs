//! Serde serialization helpers.

/// Helpers for serializing fields containing a [`Uri`][`crate::Uri`].
///
/// Annotate fields with `#[serde(with = "iqhttp::serializers::uri)]` to use these.
pub mod uri {
    use crate::Uri;
    use serde::{de, ser, Deserialize, Serialize};

    /// Deserialize [`Uri`].
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Uri, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }

    /// Serialize [`Uri`].
    pub fn serialize<S>(uri: &Uri, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        uri.to_string().serialize(serializer)
    }
}

/// Helpers for serializing fields containing an optional [`Uri`][`crate::Uri`].
///
/// Annotate fields with `#[serde(with = "iqhttp::serializers::uri_optional)]` to use these.
pub mod uri_optional {
    use crate::Uri;
    use serde::{de, ser, Deserialize};

    /// Deserialize an optional [`Uri`].
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Uri>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Option::<String>::deserialize(deserializer)?
            .map(|uri| uri.parse().map_err(de::Error::custom))
            .transpose()
    }

    /// Serialize an optional [`Uri`].
    pub fn serialize<S>(maybe_uri: &Option<Uri>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match maybe_uri {
            Some(uri) => serializer.serialize_some(&uri.to_string()),
            None => serializer.serialize_none(),
        }
    }
}
