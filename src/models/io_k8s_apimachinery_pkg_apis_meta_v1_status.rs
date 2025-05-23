/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.27.5+k3s1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

/// IoK8sApimachineryPkgApisMetaV1Status : Status is a return value for calls that don't return other objects.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, SerdeDiff)]
pub struct IoK8sApimachineryPkgApisMetaV1Status {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// Suggested HTTP return code for this status, 0 if not set.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    #[serde_diff(opaque)]
    pub details: Option<Box<models::IoK8sApimachineryPkgApisMetaV1StatusDetails>>,
    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// A human-readable description of the status of this operation.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    #[serde_diff(opaque)]
    pub  metadata: Option<Box<models::IoK8sApimachineryPkgApisMetaV1ListMeta>>,
    /// A machine-readable description of why this operation is in the \"Failure\" status. If this value is empty there is no information available. A Reason clarifies an HTTP status code but does not override it.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Status of the operation. One of: \"Success\" or \"Failure\". More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl IoK8sApimachineryPkgApisMetaV1Status {
    /// Status is a return value for calls that don't return other objects.
    pub fn new() -> IoK8sApimachineryPkgApisMetaV1Status {
        IoK8sApimachineryPkgApisMetaV1Status {
            api_version: None,
            code: None,
            details: None,
            kind: None,
            message: None,
            metadata: None,
            reason: None,
            status: None,
        }
    }
}

