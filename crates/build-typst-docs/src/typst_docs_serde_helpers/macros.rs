macro_rules! deserialize_as_impl {
    ($t:ident, $t_def:ident) => {
        impl<'de> serde_with::DeserializeAs<'de, $t> for $t_def {
            fn deserialize_as<D>(deserializer: D) -> Result<$t, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                <$t_def>::deserialize(deserializer)
            }
        }
    };
}
pub(crate) use deserialize_as_impl;
