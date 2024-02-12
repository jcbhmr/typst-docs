use crate::macros::deserialize_as_impl;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{DeserializeAs, SerializeAs};
use typst_docs::Html;

pub struct HtmlDef;
impl HtmlDef {
    pub fn serialize<S>(data: &Html, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        data.serialize(serializer)
    }
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Html, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string: String = Deserialize::deserialize(deserializer)?;
        Ok(Html::new(string))
    }
}
deserialize_as_impl!(Html, HtmlDef);
