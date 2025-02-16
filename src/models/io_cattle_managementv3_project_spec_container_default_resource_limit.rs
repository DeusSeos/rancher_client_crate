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

/// IoCattleManagementv3ProjectSpecContainerDefaultResourceLimit : ContainerDefaultResourceLimit is a specification for the default LimitRange for the namespace. See https://kubernetes.io/docs/concepts/policy/limit-range/ for more details.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoCattleManagementv3ProjectSpecContainerDefaultResourceLimit {
    /// LimitsCPU is the CPU limits across all pods in a non-terminal state.
    #[serde(rename = "limitsCpu", skip_serializing_if = "Option::is_none")]
    pub limits_cpu: Option<String>,
    /// LimitsMemory is the memory limits across all pods in a non-terminal state.
    #[serde(rename = "limitsMemory", skip_serializing_if = "Option::is_none")]
    pub limits_memory: Option<String>,
    /// RequestsCPU is the CPU requests limit across all pods in a non-terminal state.
    #[serde(rename = "requestsCpu", skip_serializing_if = "Option::is_none")]
    pub requests_cpu: Option<String>,
    /// RequestsMemory is the memory requests limit across all pods in a non-terminal state.
    #[serde(rename = "requestsMemory", skip_serializing_if = "Option::is_none")]
    pub requests_memory: Option<String>,
}

impl IoCattleManagementv3ProjectSpecContainerDefaultResourceLimit {
    /// ContainerDefaultResourceLimit is a specification for the default LimitRange for the namespace. See https://kubernetes.io/docs/concepts/policy/limit-range/ for more details.
    pub fn new() -> IoCattleManagementv3ProjectSpecContainerDefaultResourceLimit {
        IoCattleManagementv3ProjectSpecContainerDefaultResourceLimit {
            limits_cpu: None,
            limits_memory: None,
            requests_cpu: None,
            requests_memory: None,
        }
    }
}

