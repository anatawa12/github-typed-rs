use serde::{Deserialize, Serialize};

// load generated code now.
include!("./schema_generated.rs");

mod map_from_status_code {
    use http::StatusCode;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::collections::BTreeMap;

    #[derive(Ord, PartialOrd, Eq, PartialEq)]
    struct StatusCodeWrap(StatusCode);

    pub fn serialize<S, T: Serialize>(
        value: &BTreeMap<StatusCode, T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let wrapped = unsafe {
            &*(value as *const BTreeMap<StatusCode, T> as *const BTreeMap<StatusCodeWrap, T>)
        };
        Serialize::serialize(wrapped, serializer)
    }

    pub fn deserialize<'de, D, T: Deserialize<'de>>(
        deserializer: D,
    ) -> Result<BTreeMap<StatusCode, T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut wrapped = <BTreeMap<StatusCodeWrap, T> as Deserialize>::deserialize(deserializer)?;
        unsafe {
            let wrapped_ref = &mut *(&mut wrapped as *mut BTreeMap<StatusCodeWrap, T>
                as *mut BTreeMap<StatusCode, T>);
            Ok(std::mem::take(wrapped_ref))
        }
    }

    impl<'de> Deserialize<'de> for StatusCodeWrap {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
        where
            D: Deserializer<'de>,
        {
            super::status_code_by_string::deserialize(deserializer).map(|x| StatusCodeWrap(x))
        }
    }

    impl Serialize for StatusCodeWrap {
        fn serialize<S>(
            &self,
            serializer: S,
        ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where
            S: Serializer,
        {
            super::status_code_by_string::serialize(&self.0, serializer)
        }
    }
}

mod status_code_by_string {
    use std::fmt;

    use http::StatusCode;
    use serde::de::{Error, Unexpected, Visitor};
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(value: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(value.as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<StatusCode, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VisitorImpl;
        impl<'de> Visitor<'de> for VisitorImpl {
            type Value = StatusCode;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                writeln!(formatter, "string or number of 100..=999")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if v < 100 || v >= 1000 {
                    return Err(Error::invalid_value(Unexpected::Unsigned(v), &self));
                }
                Ok(StatusCode::from_u16(v as u16)
                    .map_err(|_| Error::invalid_value(Unexpected::Unsigned(v), &self))?)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let value = v
                    .parse::<u16>()
                    .map_err(|_| Error::invalid_value(Unexpected::Str(v), &self))?;
                Ok(StatusCode::from_u16(value)
                    .map_err(|_| Error::invalid_value(Unexpected::Str(v), &self))?)
            }
        }
        deserializer.deserialize_any(VisitorImpl {})
    }
}
