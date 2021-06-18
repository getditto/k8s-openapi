// Generated from definition io.k8s.api.discovery.v1.EndpointHints

/// EndpointHints provides hints describing how an endpoint should be consumed.
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema), schemars(rename_all = "camelCase"))]
pub struct EndpointHints {
    /// forZones indicates the zone(s) this endpoint should be consumed by to enable topology aware routing.
    #[cfg_attr(feature = "schema", schemars(default = "Vec::<crate::api::discovery::v1::ForZone>::new"))]
    pub for_zones: Vec<crate::api::discovery::v1::ForZone>,
}

impl<'de> crate::serde::Deserialize<'de> for EndpointHints {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_for_zones,
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
                            "forZones" => Field::Key_for_zones,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = EndpointHints;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("EndpointHints")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_for_zones: Option<Vec<crate::api::discovery::v1::ForZone>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_for_zones => value_for_zones = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(EndpointHints {
                    for_zones: value_for_zones.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "EndpointHints",
            &[
                "forZones",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for EndpointHints {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "EndpointHints",
            usize::from(!self.for_zones.is_empty()),
        )?;
        if !self.for_zones.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "forZones", &self.for_zones)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}
