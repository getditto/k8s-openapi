// Generated from definition io.k8s.api.authorization.v1.NonResourceAttributes

/// NonResourceAttributes includes the authorization attributes available for non-resource requests to the Authorizer interface
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema), schemars(rename_all = "camelCase"))]
pub struct NonResourceAttributes {
    /// Path is the URL path of the request
    pub path: Option<String>,

    /// Verb is the standard HTTP verb
    pub verb: Option<String>,
}

impl<'de> crate::serde::Deserialize<'de> for NonResourceAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_path,
            Key_verb,
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
                            "path" => Field::Key_path,
                            "verb" => Field::Key_verb,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NonResourceAttributes;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("NonResourceAttributes")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_path: Option<String> = None;
                let mut value_verb: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_path => value_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_verb => value_verb = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NonResourceAttributes {
                    path: value_path,
                    verb: value_verb,
                })
            }
        }

        deserializer.deserialize_struct(
            "NonResourceAttributes",
            &[
                "path",
                "verb",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NonResourceAttributes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NonResourceAttributes",
            self.path.as_ref().map_or(0, |_| 1) +
            self.verb.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.path {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "path", value)?;
        }
        if let Some(value) = &self.verb {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "verb", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}
