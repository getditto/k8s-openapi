// Generated from definition io.k8s.api.autoscaling.v2beta1.MetricSpec

/// MetricSpec specifies how to scale based on a single metric (only `type` and one other matching field should be set at once).
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema), schemars(rename_all = "camelCase"))]
pub struct MetricSpec {
    /// container resource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing a single container in each pod of the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source. This is an alpha feature and can be enabled by the HPAContainerMetrics feature flag.
    pub container_resource: Option<crate::api::autoscaling::v2beta1::ContainerResourceMetricSource>,

    /// external refers to a global metric that is not associated with any Kubernetes object. It allows autoscaling based on information coming from components running outside of cluster (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster).
    pub external: Option<crate::api::autoscaling::v2beta1::ExternalMetricSource>,

    /// object refers to a metric describing a single kubernetes object (for example, hits-per-second on an Ingress object).
    pub object: Option<crate::api::autoscaling::v2beta1::ObjectMetricSource>,

    /// pods refers to a metric describing each pod in the current scale target (for example, transactions-processed-per-second).  The values will be averaged together before being compared to the target value.
    pub pods: Option<crate::api::autoscaling::v2beta1::PodsMetricSource>,

    /// resource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing each pod in the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.
    pub resource: Option<crate::api::autoscaling::v2beta1::ResourceMetricSource>,

    /// type is the type of metric source.  It should be one of "ContainerResource", "External", "Object", "Pods" or "Resource", each mapping to a matching field in the object. Note: "ContainerResource" type is available on when the feature-gate HPAContainerMetrics is enabled
    pub type_: String,
}

impl<'de> crate::serde::Deserialize<'de> for MetricSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_container_resource,
            Key_external,
            Key_object,
            Key_pods,
            Key_resource,
            Key_type_,
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
                            "containerResource" => Field::Key_container_resource,
                            "external" => Field::Key_external,
                            "object" => Field::Key_object,
                            "pods" => Field::Key_pods,
                            "resource" => Field::Key_resource,
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = MetricSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("MetricSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_container_resource: Option<crate::api::autoscaling::v2beta1::ContainerResourceMetricSource> = None;
                let mut value_external: Option<crate::api::autoscaling::v2beta1::ExternalMetricSource> = None;
                let mut value_object: Option<crate::api::autoscaling::v2beta1::ObjectMetricSource> = None;
                let mut value_pods: Option<crate::api::autoscaling::v2beta1::PodsMetricSource> = None;
                let mut value_resource: Option<crate::api::autoscaling::v2beta1::ResourceMetricSource> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_container_resource => value_container_resource = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_external => value_external = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_object => value_object = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_pods => value_pods = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource => value_resource = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_type_ => value_type_ = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(MetricSpec {
                    container_resource: value_container_resource,
                    external: value_external,
                    object: value_object,
                    pods: value_pods,
                    resource: value_resource,
                    type_: value_type_.ok_or_else(|| crate::serde::de::Error::missing_field("type"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "MetricSpec",
            &[
                "containerResource",
                "external",
                "object",
                "pods",
                "resource",
                "type",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for MetricSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "MetricSpec",
            1 +
            self.container_resource.as_ref().map_or(0, |_| 1) +
            self.external.as_ref().map_or(0, |_| 1) +
            self.object.as_ref().map_or(0, |_| 1) +
            self.pods.as_ref().map_or(0, |_| 1) +
            self.resource.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.container_resource {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "containerResource", value)?;
        }
        if let Some(value) = &self.external {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "external", value)?;
        }
        if let Some(value) = &self.object {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "object", value)?;
        }
        if let Some(value) = &self.pods {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "pods", value)?;
        }
        if let Some(value) = &self.resource {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resource", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}
