// Generated from definition io.k8s.api.core.v1.NodeSelector

/// A node selector represents the union of the results of one or more label queries over a set of nodes; that is, it represents the OR of the selectors represented by the node selector terms.
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema), schemars(rename_all = "camelCase"))]
pub struct NodeSelector {
    /// Required. A list of node selector terms. The terms are ORed.
    #[cfg_attr(feature = "schema", schemars(default = "Vec::<crate::api::core::v1::NodeSelectorTerm>::new"))]
    pub node_selector_terms: Vec<crate::api::core::v1::NodeSelectorTerm>,
}

impl<'de> crate::serde::Deserialize<'de> for NodeSelector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_node_selector_terms,
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
                            "nodeSelectorTerms" => Field::Key_node_selector_terms,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = NodeSelector;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("NodeSelector")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_node_selector_terms: Option<Vec<crate::api::core::v1::NodeSelectorTerm>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_node_selector_terms => value_node_selector_terms = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(NodeSelector {
                    node_selector_terms: value_node_selector_terms.ok_or_else(|| crate::serde::de::Error::missing_field("nodeSelectorTerms"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "NodeSelector",
            &[
                "nodeSelectorTerms",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for NodeSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "NodeSelector",
            1,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nodeSelectorTerms", &self.node_selector_terms)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}
