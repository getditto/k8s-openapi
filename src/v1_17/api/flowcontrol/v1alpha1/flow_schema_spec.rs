// Generated from definition io.k8s.api.flowcontrol.v1alpha1.FlowSchemaSpec

/// FlowSchemaSpec describes how the FlowSchema's specification looks like.
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema), schemars(rename_all = "camelCase"))]
pub struct FlowSchemaSpec {
    /// `distinguisherMethod` defines how to compute the flow distinguisher for requests that match this schema. `nil` specifies that the distinguisher is disabled and thus will always be the empty string.
    pub distinguisher_method: Option<crate::api::flowcontrol::v1alpha1::FlowDistinguisherMethod>,

    /// `matchingPrecedence` is used to choose among the FlowSchemas that match a given request. The chosen FlowSchema is among those with the numerically lowest (which we take to be logically highest) MatchingPrecedence.  Each MatchingPrecedence value must be non-negative. Note that if the precedence is not specified or zero, it will be set to 1000 as default.
    pub matching_precedence: Option<i32>,

    /// `priorityLevelConfiguration` should reference a PriorityLevelConfiguration in the cluster. If the reference cannot be resolved, the FlowSchema will be ignored and marked as invalid in its status. Required.
    pub priority_level_configuration: crate::api::flowcontrol::v1alpha1::PriorityLevelConfigurationReference,

    /// `rules` describes which requests will match this flow schema. This FlowSchema matches a request if and only if at least one member of rules matches the request. if it is an empty slice, there will be no requests matching the FlowSchema.
    #[cfg_attr(feature = "schema", schemars(default = "Vec::<crate::api::flowcontrol::v1alpha1::PolicyRulesWithSubjects>::new"))]
    pub rules: Vec<crate::api::flowcontrol::v1alpha1::PolicyRulesWithSubjects>,
}

impl<'de> crate::serde::Deserialize<'de> for FlowSchemaSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_distinguisher_method,
            Key_matching_precedence,
            Key_priority_level_configuration,
            Key_rules,
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
                            "distinguisherMethod" => Field::Key_distinguisher_method,
                            "matchingPrecedence" => Field::Key_matching_precedence,
                            "priorityLevelConfiguration" => Field::Key_priority_level_configuration,
                            "rules" => Field::Key_rules,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = FlowSchemaSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FlowSchemaSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_distinguisher_method: Option<crate::api::flowcontrol::v1alpha1::FlowDistinguisherMethod> = None;
                let mut value_matching_precedence: Option<i32> = None;
                let mut value_priority_level_configuration: Option<crate::api::flowcontrol::v1alpha1::PriorityLevelConfigurationReference> = None;
                let mut value_rules: Option<Vec<crate::api::flowcontrol::v1alpha1::PolicyRulesWithSubjects>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_distinguisher_method => value_distinguisher_method = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_matching_precedence => value_matching_precedence = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_priority_level_configuration => value_priority_level_configuration = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_rules => value_rules = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FlowSchemaSpec {
                    distinguisher_method: value_distinguisher_method,
                    matching_precedence: value_matching_precedence,
                    priority_level_configuration: value_priority_level_configuration.ok_or_else(|| crate::serde::de::Error::missing_field("priorityLevelConfiguration"))?,
                    rules: value_rules.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "FlowSchemaSpec",
            &[
                "distinguisherMethod",
                "matchingPrecedence",
                "priorityLevelConfiguration",
                "rules",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for FlowSchemaSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FlowSchemaSpec",
            1 +
            self.distinguisher_method.as_ref().map_or(0, |_| 1) +
            self.matching_precedence.as_ref().map_or(0, |_| 1) +
            usize::from(!self.rules.is_empty()),
        )?;
        if let Some(value) = &self.distinguisher_method {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "distinguisherMethod", value)?;
        }
        if let Some(value) = &self.matching_precedence {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "matchingPrecedence", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "priorityLevelConfiguration", &self.priority_level_configuration)?;
        if !self.rules.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "rules", &self.rules)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}
