use crate::macros::deserialize_as_impl;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{DeserializeAs, SerializeAs};

pub(crate) type StaticStr = &'static str;
pub(crate) type StaticStrSlice = &'static [&'static str];

pub(crate) struct StaticStrDef;
impl StaticStrDef {
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<&'static str, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string: String = Deserialize::deserialize(deserializer)?;
        Ok(Box::leak(string.into_boxed_str()))
    }
}
deserialize_as_impl!(StaticStr, StaticStrDef);

pub(crate) struct StaticStrSliceDef;
impl StaticStrSliceDef {
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<&'static [&'static str], D::Error>
    where
        D: Deserializer<'de>,
    {
        let strings: Vec<String> = Deserialize::deserialize(deserializer)?;
        let static_strs: Vec<&'static str> = strings
            .into_iter()
            .map(|s| Box::leak(s.into_boxed_str()) as _)
            .collect();
        Ok(Box::leak(static_strs.into_boxed_slice()))
    }
}
deserialize_as_impl!(StaticStrSlice, StaticStrSliceDef);
