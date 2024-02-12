use serde::Deserialize;

pub(crate) type StaticStr = &'static str;
pub(crate) type StaticStrSlice = &'static [&'static str];

/// LEAKS MEMORY!
pub(crate) fn deserialize_static_str<'de, D>(deserializer: D) -> Result<&'static str, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let string: String = Deserialize::deserialize(deserializer)?;
    Ok(Box::leak(string.into_boxed_str()))
}

/// LEAKS MEMORY!
pub(crate) fn deserialize_static_str_option<'de, D>(deserializer: D) -> Result<Option<&'static str>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let res: Option<String> = Deserialize::deserialize(deserializer)?;
    Ok(res.map(|s| Box::leak(s.into_boxed_str()) as _))
}

/// LEAKS MEMORY!
pub(crate) fn deserialize_static_str_vec<'de, D>(deserializer: D) -> Result<Vec<&'static str>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let strings: Vec<String> = Deserialize::deserialize(deserializer)?;
    let static_strs: Vec<&'static str> = strings
        .into_iter()
        .map(|s| Box::leak(s.into_boxed_str()) as _)
        .collect();
    Ok(static_strs)
}

/// LEAKS MEMORY!
pub(crate) fn deserialize_static_str_slice<'de, D>(deserializer: D) -> Result<&'static [&'static str], D::Error>
where
    D: serde::Deserializer<'de>,
{
    let strings: Vec<String> = Deserialize::deserialize(deserializer)?;
    let static_strs: Vec<&'static str> = strings
        .into_iter()
        .map(|s| Box::leak(s.into_boxed_str()) as _)
        .collect();
    Ok(Box::leak(static_strs.into_boxed_slice()))
}