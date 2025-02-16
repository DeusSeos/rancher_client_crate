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

/// IoCattleManagementv3ProjectSpecResourceQuotaUsedLimit : UsedLimit is the currently allocated quota for all namespaces in the project.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoCattleManagementv3ProjectSpecResourceQuotaUsedLimit {
    /// ConfigMaps is the total number of ReplicationControllers that can exist in the namespace.
    #[serde(rename = "configMaps", skip_serializing_if = "Option::is_none")]
    pub config_maps: Option<String>,
    /// LimitsCPU is the CPU limits across all pods in a non-terminal state.
    #[serde(rename = "limitsCpu", skip_serializing_if = "Option::is_none")]
    pub limits_cpu: Option<String>,
    /// LimitsMemory is the memory limits across all pods in a non-terminal state.
    #[serde(rename = "limitsMemory", skip_serializing_if = "Option::is_none")]
    pub limits_memory: Option<String>,
    /// PersistentVolumeClaims is the total number of PersistentVolumeClaims that can exist in the namespace.
    #[serde(rename = "persistentVolumeClaims", skip_serializing_if = "Option::is_none")]
    pub persistent_volume_claims: Option<String>,
    /// Pods is the total number of Pods in a non-terminal state that can exist in the namespace. A pod is in a terminal state if .status.phase in (Failed, Succeeded) is true.
    #[serde(rename = "pods", skip_serializing_if = "Option::is_none")]
    pub pods: Option<String>,
    /// ReplicationControllers is total number of ReplicationControllers that can exist in the namespace.
    #[serde(rename = "replicationControllers", skip_serializing_if = "Option::is_none")]
    pub replication_controllers: Option<String>,
    /// RequestsCPU is the CPU requests limit across all pods in a non-terminal state.
    #[serde(rename = "requestsCpu", skip_serializing_if = "Option::is_none")]
    pub requests_cpu: Option<String>,
    /// RequestsMemory is the memory requests limit across all pods in a non-terminal state.
    #[serde(rename = "requestsMemory", skip_serializing_if = "Option::is_none")]
    pub requests_memory: Option<String>,
    /// RequestsStorage is the storage requests limit across all persistent volume claims.
    #[serde(rename = "requestsStorage", skip_serializing_if = "Option::is_none")]
    pub requests_storage: Option<String>,
    /// Secrets is the total number of ReplicationControllers that can exist in the namespace.
    #[serde(rename = "secrets", skip_serializing_if = "Option::is_none")]
    pub secrets: Option<String>,
    /// Services is the total number of Services that can exist in the namespace.
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Option<String>,
    /// ServicesLoadBalancers is the total number of Services of type LoadBalancer that can exist in the namespace.
    #[serde(rename = "servicesLoadBalancers", skip_serializing_if = "Option::is_none")]
    pub services_load_balancers: Option<String>,
    /// ServiceNodePorts is the total number of Services of type NodePort that can exist in the namespace.
    #[serde(rename = "servicesNodePorts", skip_serializing_if = "Option::is_none")]
    pub services_node_ports: Option<String>,
}

impl IoCattleManagementv3ProjectSpecResourceQuotaUsedLimit {
    /// UsedLimit is the currently allocated quota for all namespaces in the project.
    pub fn new() -> IoCattleManagementv3ProjectSpecResourceQuotaUsedLimit {
        IoCattleManagementv3ProjectSpecResourceQuotaUsedLimit {
            config_maps: None,
            limits_cpu: None,
            limits_memory: None,
            persistent_volume_claims: None,
            pods: None,
            replication_controllers: None,
            requests_cpu: None,
            requests_memory: None,
            requests_storage: None,
            secrets: None,
            services: None,
            services_load_balancers: None,
            services_node_ports: None,
        }
    }
}

