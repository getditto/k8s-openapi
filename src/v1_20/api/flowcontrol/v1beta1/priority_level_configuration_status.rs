// Generated from definition io.k8s.api.flowcontrol.v1beta1.PriorityLevelConfigurationStatus

/// PriorityLevelConfigurationStatus represents the current state of a "request-priority".
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema), schemars(rename_all = "camelCase"))]
pub struct PriorityLevelConfigurationStatus {
    /// `conditions` is the current state of "request-priority".
    #[cfg_attr(feature = "schema", schemars(default = "Vec::<crate::api::flowcontrol::v1beta1::PriorityLevelConfigurationCondition>::new"))]
    pub conditions: Vec<crate::api::flowcontrol::v1beta1::PriorityLevelConfigurationCondition>,
}

impl<'de> crate::serde::Deserialize<'de> for PriorityLevelConfigurationStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_conditions,
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
                            "conditions" => Field::Key_conditions,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PriorityLevelConfigurationStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PriorityLevelConfigurationStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_conditions: Option<Vec<crate::api::flowcontrol::v1beta1::PriorityLevelConfigurationCondition>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PriorityLevelConfigurationStatus {
                    conditions: value_conditions.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "PriorityLevelConfigurationStatus",
            &[
                "conditions",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PriorityLevelConfigurationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PriorityLevelConfigurationStatus",
            usize::from(!self.conditions.is_empty()),
        )?;
        if !self.conditions.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", &self.conditions)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}
