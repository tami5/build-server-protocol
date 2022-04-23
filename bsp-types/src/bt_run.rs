use super::BuildTargetIdentifier;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The run request is sent from the client to the server to run a build target. The server
/// communicates during the initialize handshake whether this method is supported or not.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BuildTargetRun {
    /// The build target to run.
    target: BuildTargetIdentifier,

    /// A unique identifier generated by the client to identify this request.
    ///  * The server may include this id in triggered notifications or responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_id: Option<String>,

    /// Optional arguments to the executed application.
    #[serde(skip_serializing_if = "Option::is_none")]
    arguments: Option<String>,

    /// Kind of data to expect in the data field. If this field is not set, the kind of data is not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    data_kind: Option<String>,

    /// Language-specific metadata for this execution.
    ///  * See https://github.com/build-server-protocol/build-server-protocol/blob/master/bsp4j/src/main/xtend-gen/ch/epfl/scala/bsp4j/ScalaMainClass.java
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

impl BuildTargetRun {
    pub fn new(
        target: BuildTargetIdentifier,
        origin_id: Option<String>,
        arguments: Option<String>,
        data_kind: Option<String>,
        data: Option<Value>,
    ) -> Self {
        Self {
            target,
            origin_id,
            arguments,
            data_kind,
            data,
        }
    }
    pub fn new_simple(target: BuildTargetIdentifier) -> Self {
        Self {
            target,
            origin_id: None,
            arguments: None,
            data_kind: None,
            data: None,
        }
    }

    /// Get a reference to the bsp btrun params's target.
    pub fn target(&self) -> &BuildTargetIdentifier {
        &self.target
    }

    /// Set the bsp btrun params's target.
    pub fn set_target(&mut self, target: BuildTargetIdentifier) {
        self.target = target;
    }

    /// Get a reference to the bsp btrun params's origin id.
    pub fn origin_id(&self) -> Option<&String> {
        self.origin_id.as_ref()
    }

    /// Set the bsp btrun params's origin id.
    pub fn set_origin_id(&mut self, origin_id: Option<String>) {
        self.origin_id = origin_id;
    }

    /// Get a reference to the bsp btrun params's arguments.
    pub fn arguments(&self) -> Option<&String> {
        self.arguments.as_ref()
    }

    /// Set the bsp btrun params's arguments.
    pub fn set_arguments(&mut self, arguments: Option<String>) {
        self.arguments = arguments;
    }

    /// Get a reference to the bsp btrun params's data kind.
    pub fn data_kind(&self) -> Option<&String> {
        self.data_kind.as_ref()
    }

    /// Set the bsp btrun params's data kind.
    pub fn set_data_kind(&mut self, data_kind: Option<String>) {
        self.data_kind = data_kind;
    }

    /// Get a reference to the bsp btrun params's data.
    pub fn data(&self) -> Option<&Value> {
        self.data.as_ref()
    }

    /// Set the bsp btrun params's data.
    pub fn set_data(&mut self, data: Option<Value>) {
        self.data = data;
    }
}

/// Note that an empty run request is valid. Run will be executed in the target as specified in the build tool.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BuildTargetRunResult {
    /** An optional request id to know the origin of this report. */
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_id: Option<String>,

    /** A status code for the execution. */
    status_code: usize,
}

impl BuildTargetRunResult {
    pub fn new(origin_id: Option<String>, status_code: usize) -> Self {
        Self {
            origin_id,
            status_code,
        }
    }

    /// Get a reference to the bsp btrun result's origin id.
    pub fn origin_id(&self) -> Option<&String> {
        self.origin_id.as_ref()
    }

    /// Get the bsp btrun result's status code.
    pub fn status_code(&self) -> usize {
        self.status_code
    }

    /// Set the bsp btrun result's origin id.
    pub fn set_origin_id(&mut self, origin_id: Option<String>) {
        self.origin_id = origin_id;
    }

    /// Set the bsp btrun result's status code.
    pub fn set_status_code(&mut self, status_code: usize) {
        self.status_code = status_code;
    }
}
