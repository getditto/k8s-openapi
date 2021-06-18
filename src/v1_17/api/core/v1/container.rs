// Generated from definition io.k8s.api.core.v1.Container

/// A single application container that you want to run within a pod.
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema), schemars(rename_all = "camelCase"))]
pub struct Container {
    /// Arguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
    #[cfg_attr(feature = "schema", schemars(default = "Vec::<String>::new"))]
    pub args: Vec<String>,

    /// Entrypoint array. Not executed within a shell. The docker image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
    #[cfg_attr(feature = "schema", schemars(default = "Vec::<String>::new"))]
    pub command: Vec<String>,

    /// List of environment variables to set in the container. Cannot be updated.
    #[cfg_attr(feature = "schema", schemars(default = "Vec::<crate::api::core::v1::EnvVar>::new"))]
    pub env: Vec<crate::api::core::v1::EnvVar>,

    /// List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated.
    #[cfg_attr(feature = "schema", schemars(default = "Vec::<crate::api::core::v1::EnvFromSource>::new"))]
    pub env_from: Vec<crate::api::core::v1::EnvFromSource>,

    /// Docker image name. More info: https://kubernetes.io/docs/concepts/containers/images This field is optional to allow higher level config management to default or override container images in workload controllers like Deployments and StatefulSets.
    pub image: Option<String>,

    /// Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
    pub image_pull_policy: Option<String>,

    /// Actions that the management system should take in response to container lifecycle events. Cannot be updated.
    pub lifecycle: Option<crate::api::core::v1::Lifecycle>,

    /// Periodic probe of container liveness. Container will be restarted if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    pub liveness_probe: Option<crate::api::core::v1::Probe>,

    /// Name of the container specified as a DNS_LABEL. Each container in a pod must have a unique name (DNS_LABEL). Cannot be updated.
    pub name: String,

    /// List of ports to expose from the container. Exposing a port here gives the system additional information about the network connections a container uses, but is primarily informational. Not specifying a port here DOES NOT prevent that port from being exposed. Any port which is listening on the default "0.0.0.0" address inside a container will be accessible from the network. Cannot be updated.
    #[cfg_attr(feature = "schema", schemars(default = "Vec::<crate::api::core::v1::ContainerPort>::new"))]
    pub ports: Vec<crate::api::core::v1::ContainerPort>,

    /// Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    pub readiness_probe: Option<crate::api::core::v1::Probe>,

    /// Compute Resources required by this container. Cannot be updated. More info: https://kubernetes.io/docs/concepts/configuration/manage-compute-resources-container/
    pub resources: Option<crate::api::core::v1::ResourceRequirements>,

    /// Security options the pod should run with. More info: https://kubernetes.io/docs/concepts/policy/security-context/ More info: https://kubernetes.io/docs/tasks/configure-pod-container/security-context/
    pub security_context: Option<crate::api::core::v1::SecurityContext>,

    /// StartupProbe indicates that the Pod has successfully initialized. If specified, no other probes are executed until this completes successfully. If this probe fails, the Pod will be restarted, just as if the livenessProbe failed. This can be used to provide different probe parameters at the beginning of a Pod's lifecycle, when it might take a long time to load data or warm a cache, than during steady-state operation. This cannot be updated. This is an alpha feature enabled by the StartupProbe feature flag. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    pub startup_probe: Option<crate::api::core::v1::Probe>,

    /// Whether this container should allocate a buffer for stdin in the container runtime. If this is not set, reads from stdin in the container will always result in EOF. Default is false.
    pub stdin: Option<bool>,

    /// Whether the container runtime should close the stdin channel after it has been opened by a single attach. When stdin is true the stdin stream will remain open across multiple attach sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the first client attaches to stdin, and then remains open and accepts data until the client disconnects, at which time stdin is closed and remains closed until the container is restarted. If this flag is false, a container processes that reads from stdin will never receive an EOF. Default is false
    pub stdin_once: Option<bool>,

    /// Optional: Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.
    pub termination_message_path: Option<String>,

    /// Indicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated.
    pub termination_message_policy: Option<String>,

    /// Whether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is false.
    pub tty: Option<bool>,

    /// volumeDevices is the list of block devices to be used by the container. This is a beta feature.
    #[cfg_attr(feature = "schema", schemars(default = "Vec::<crate::api::core::v1::VolumeDevice>::new"))]
    pub volume_devices: Vec<crate::api::core::v1::VolumeDevice>,

    /// Pod volumes to mount into the container's filesystem. Cannot be updated.
    #[cfg_attr(feature = "schema", schemars(default = "Vec::<crate::api::core::v1::VolumeMount>::new"))]
    pub volume_mounts: Vec<crate::api::core::v1::VolumeMount>,

    /// Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated.
    pub working_dir: Option<String>,
}

impl<'de> crate::serde::Deserialize<'de> for Container {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_args,
            Key_command,
            Key_env,
            Key_env_from,
            Key_image,
            Key_image_pull_policy,
            Key_lifecycle,
            Key_liveness_probe,
            Key_name,
            Key_ports,
            Key_readiness_probe,
            Key_resources,
            Key_security_context,
            Key_startup_probe,
            Key_stdin,
            Key_stdin_once,
            Key_termination_message_path,
            Key_termination_message_policy,
            Key_tty,
            Key_volume_devices,
            Key_volume_mounts,
            Key_working_dir,
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
                            "args" => Field::Key_args,
                            "command" => Field::Key_command,
                            "env" => Field::Key_env,
                            "envFrom" => Field::Key_env_from,
                            "image" => Field::Key_image,
                            "imagePullPolicy" => Field::Key_image_pull_policy,
                            "lifecycle" => Field::Key_lifecycle,
                            "livenessProbe" => Field::Key_liveness_probe,
                            "name" => Field::Key_name,
                            "ports" => Field::Key_ports,
                            "readinessProbe" => Field::Key_readiness_probe,
                            "resources" => Field::Key_resources,
                            "securityContext" => Field::Key_security_context,
                            "startupProbe" => Field::Key_startup_probe,
                            "stdin" => Field::Key_stdin,
                            "stdinOnce" => Field::Key_stdin_once,
                            "terminationMessagePath" => Field::Key_termination_message_path,
                            "terminationMessagePolicy" => Field::Key_termination_message_policy,
                            "tty" => Field::Key_tty,
                            "volumeDevices" => Field::Key_volume_devices,
                            "volumeMounts" => Field::Key_volume_mounts,
                            "workingDir" => Field::Key_working_dir,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Container;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Container")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_args: Option<Vec<String>> = None;
                let mut value_command: Option<Vec<String>> = None;
                let mut value_env: Option<Vec<crate::api::core::v1::EnvVar>> = None;
                let mut value_env_from: Option<Vec<crate::api::core::v1::EnvFromSource>> = None;
                let mut value_image: Option<String> = None;
                let mut value_image_pull_policy: Option<String> = None;
                let mut value_lifecycle: Option<crate::api::core::v1::Lifecycle> = None;
                let mut value_liveness_probe: Option<crate::api::core::v1::Probe> = None;
                let mut value_name: Option<String> = None;
                let mut value_ports: Option<Vec<crate::api::core::v1::ContainerPort>> = None;
                let mut value_readiness_probe: Option<crate::api::core::v1::Probe> = None;
                let mut value_resources: Option<crate::api::core::v1::ResourceRequirements> = None;
                let mut value_security_context: Option<crate::api::core::v1::SecurityContext> = None;
                let mut value_startup_probe: Option<crate::api::core::v1::Probe> = None;
                let mut value_stdin: Option<bool> = None;
                let mut value_stdin_once: Option<bool> = None;
                let mut value_termination_message_path: Option<String> = None;
                let mut value_termination_message_policy: Option<String> = None;
                let mut value_tty: Option<bool> = None;
                let mut value_volume_devices: Option<Vec<crate::api::core::v1::VolumeDevice>> = None;
                let mut value_volume_mounts: Option<Vec<crate::api::core::v1::VolumeMount>> = None;
                let mut value_working_dir: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_args => value_args = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_command => value_command = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_env => value_env = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_env_from => value_env_from = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_image => value_image = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_image_pull_policy => value_image_pull_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_lifecycle => value_lifecycle = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_liveness_probe => value_liveness_probe = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_ports => value_ports = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_readiness_probe => value_readiness_probe = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resources => value_resources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_security_context => value_security_context = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_startup_probe => value_startup_probe = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_stdin => value_stdin = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_stdin_once => value_stdin_once = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_termination_message_path => value_termination_message_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_termination_message_policy => value_termination_message_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_tty => value_tty = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_devices => value_volume_devices = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_mounts => value_volume_mounts = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_working_dir => value_working_dir = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Container {
                    args: value_args.unwrap_or_default(),
                    command: value_command.unwrap_or_default(),
                    env: value_env.unwrap_or_default(),
                    env_from: value_env_from.unwrap_or_default(),
                    image: value_image,
                    image_pull_policy: value_image_pull_policy,
                    lifecycle: value_lifecycle,
                    liveness_probe: value_liveness_probe,
                    name: value_name.ok_or_else(|| crate::serde::de::Error::missing_field("name"))?,
                    ports: value_ports.unwrap_or_default(),
                    readiness_probe: value_readiness_probe,
                    resources: value_resources,
                    security_context: value_security_context,
                    startup_probe: value_startup_probe,
                    stdin: value_stdin,
                    stdin_once: value_stdin_once,
                    termination_message_path: value_termination_message_path,
                    termination_message_policy: value_termination_message_policy,
                    tty: value_tty,
                    volume_devices: value_volume_devices.unwrap_or_default(),
                    volume_mounts: value_volume_mounts.unwrap_or_default(),
                    working_dir: value_working_dir,
                })
            }
        }

        deserializer.deserialize_struct(
            "Container",
            &[
                "args",
                "command",
                "env",
                "envFrom",
                "image",
                "imagePullPolicy",
                "lifecycle",
                "livenessProbe",
                "name",
                "ports",
                "readinessProbe",
                "resources",
                "securityContext",
                "startupProbe",
                "stdin",
                "stdinOnce",
                "terminationMessagePath",
                "terminationMessagePolicy",
                "tty",
                "volumeDevices",
                "volumeMounts",
                "workingDir",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Container {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Container",
            1 +
            usize::from(!self.args.is_empty()) +
            usize::from(!self.command.is_empty()) +
            usize::from(!self.env.is_empty()) +
            usize::from(!self.env_from.is_empty()) +
            self.image.as_ref().map_or(0, |_| 1) +
            self.image_pull_policy.as_ref().map_or(0, |_| 1) +
            self.lifecycle.as_ref().map_or(0, |_| 1) +
            self.liveness_probe.as_ref().map_or(0, |_| 1) +
            usize::from(!self.ports.is_empty()) +
            self.readiness_probe.as_ref().map_or(0, |_| 1) +
            self.resources.as_ref().map_or(0, |_| 1) +
            self.security_context.as_ref().map_or(0, |_| 1) +
            self.startup_probe.as_ref().map_or(0, |_| 1) +
            self.stdin.as_ref().map_or(0, |_| 1) +
            self.stdin_once.as_ref().map_or(0, |_| 1) +
            self.termination_message_path.as_ref().map_or(0, |_| 1) +
            self.termination_message_policy.as_ref().map_or(0, |_| 1) +
            self.tty.as_ref().map_or(0, |_| 1) +
            usize::from(!self.volume_devices.is_empty()) +
            usize::from(!self.volume_mounts.is_empty()) +
            self.working_dir.as_ref().map_or(0, |_| 1),
        )?;
        if !self.args.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "args", &self.args)?;
        }
        if !self.command.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "command", &self.command)?;
        }
        if !self.env.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "env", &self.env)?;
        }
        if !self.env_from.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "envFrom", &self.env_from)?;
        }
        if let Some(value) = &self.image {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "image", value)?;
        }
        if let Some(value) = &self.image_pull_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "imagePullPolicy", value)?;
        }
        if let Some(value) = &self.lifecycle {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "lifecycle", value)?;
        }
        if let Some(value) = &self.liveness_probe {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "livenessProbe", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if !self.ports.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ports", &self.ports)?;
        }
        if let Some(value) = &self.readiness_probe {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "readinessProbe", value)?;
        }
        if let Some(value) = &self.resources {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resources", value)?;
        }
        if let Some(value) = &self.security_context {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "securityContext", value)?;
        }
        if let Some(value) = &self.startup_probe {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "startupProbe", value)?;
        }
        if let Some(value) = &self.stdin {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "stdin", value)?;
        }
        if let Some(value) = &self.stdin_once {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "stdinOnce", value)?;
        }
        if let Some(value) = &self.termination_message_path {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "terminationMessagePath", value)?;
        }
        if let Some(value) = &self.termination_message_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "terminationMessagePolicy", value)?;
        }
        if let Some(value) = &self.tty {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "tty", value)?;
        }
        if !self.volume_devices.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeDevices", &self.volume_devices)?;
        }
        if !self.volume_mounts.is_empty() {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeMounts", &self.volume_mounts)?;
        }
        if let Some(value) = &self.working_dir {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "workingDir", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}
