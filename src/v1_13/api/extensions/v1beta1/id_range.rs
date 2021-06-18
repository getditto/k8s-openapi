// Generated from definition io.k8s.api.extensions.v1beta1.IDRange

/// IDRange provides a min/max of an allowed range of IDs. Deprecated: use IDRange from policy API Group instead.
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema), schemars(rename_all = "camelCase"))]
pub struct IDRange {
    /// max is the end of the range, inclusive.
    pub max: i64,

    /// min is the start of the range, inclusive.
    pub min: i64,
}

impl<'de> crate::serde::Deserialize<'de> for IDRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_max,
            Key_min,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "max" => Field::Key_max,
                            "min" => Field::Key_min,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = IDRange;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("IDRange")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_max: Option<i64> = None;
                let mut value_min: Option<i64> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_max => value_max = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_min => value_min = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(IDRange {
                    max: value_max.ok_or_else(|| crate::serde::de::Error::missing_field("max"))?,
                    min: value_min.ok_or_else(|| crate::serde::de::Error::missing_field("min"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "IDRange",
            &[
                "max",
                "min",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for IDRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "IDRange",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "max", &self.max)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "min", &self.min)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}
