use base64::engine::general_purpose;
use base64::Engine;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::Deref;

// https://users.rust-lang.org/t/serialize-a-vec-u8-to-json-as-base64/57781/5
pub struct Base64(pub Vec<u8>);

impl Deref for Base64 {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for Base64 {
    fn eq(&self, other: &Base64) -> bool {
        self.0 == other.0
    }
}

impl Serialize for Base64 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&general_purpose::STANDARD.encode(&self.0))
    }
}

impl<'de> Deserialize<'de> for Base64 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Vis;
        impl serde::de::Visitor<'_> for Vis {
            type Value = Base64;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a base64 string")
            }

            fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
                general_purpose::STANDARD
                    .decode(v)
                    .map(Base64)
                    .map_err(Error::custom)
            }
        }
        deserializer.deserialize_str(Vis)
    }
}
