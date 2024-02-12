use serde::{Deserialize, Deserializer};
use serde_with::DeserializeAs;
use typst_docs::Html;

pub struct HtmlDef;

impl HtmlDef {
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Html, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string: String = Deserialize::deserialize(deserializer)?;
        Ok(Html::new(string))
    }
}

impl<'de> DeserializeAs<'de, Html> for HtmlDef {
    fn deserialize_as<D>(deserializer: D) -> Result<Html, D::Error>
        where
            D: Deserializer<'de> {
        HtmlDef::deserialize(deserializer)
    }
}