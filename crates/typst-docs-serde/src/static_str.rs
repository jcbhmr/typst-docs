use serde::{Deserialize, Deserializer};
use serde_with::DeserializeAs;

pub(crate) type StaticStr = &'static str;

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

impl<'de> DeserializeAs<'de, &'static str> for StaticStrDef {
    fn deserialize_as<D>(deserializer: D) -> Result<&'static str, D::Error>
        where
            D: Deserializer<'de> {
        StaticStrDef::deserialize(deserializer)
    }
}

pub(crate) type StaticStrSlice = &'static [&'static str];

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

impl<'de> DeserializeAs<'de, &'static [&'static str]> for StaticStrSliceDef {
    fn deserialize_as<D>(deserializer: D) -> Result<&'static [&'static str], D::Error>
        where
            D: Deserializer<'de> {
        StaticStrSliceDef::deserialize(deserializer)
    }
}