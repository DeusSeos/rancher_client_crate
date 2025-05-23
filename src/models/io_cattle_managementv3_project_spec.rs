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

/// IoCattleManagementv3ProjectSpec : Spec is the specification of the desired configuration for the project.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, SerdeDiff)]
pub struct IoCattleManagementv3ProjectSpec {
    /// ClusterName is the name of the cluster the project belongs to. Immutable.
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    #[serde(rename = "containerDefaultResourceLimit", skip_serializing_if = "Option::is_none")]
    #[serde_diff(opaque)]
    pub container_default_resource_limit: Option<models::IoCattleManagementv3ProjectSpecContainerDefaultResourceLimit>,
    /// Description is a human-readable description of the project.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// DisplayName is the human-readable name for the project.
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// EnableProjectMonitoring indicates whether Monitoring V1 should be enabled for this project. Deprecated. Use the Monitoring V2 app instead. Defaults to false.
    #[serde(rename = "enableProjectMonitoring", skip_serializing_if = "Option::is_none")]
    #[serde_diff(skip)]
    pub enable_project_monitoring: Option<bool>,
    #[serde(rename = "namespaceDefaultResourceQuota", skip_serializing_if = "Option::is_none")]
    #[serde_diff(opaque)]
    pub namespace_default_resource_quota: Option<models::IoCattleManagementv3ProjectSpecNamespaceDefaultResourceQuota>,
    #[serde(rename = "resourceQuota", skip_serializing_if = "Option::is_none")]
    pub resource_quota: Option<models::IoCattleManagementv3ProjectSpecResourceQuota>,
}

impl IoCattleManagementv3ProjectSpec {
    /// Spec is the specification of the desired configuration for the project.
    pub fn new(cluster_name: String, display_name: String) -> IoCattleManagementv3ProjectSpec {
        IoCattleManagementv3ProjectSpec {
            cluster_name,
            container_default_resource_limit: None,
            description: None,
            display_name,
            enable_project_monitoring: None,
            namespace_default_resource_quota: None,
            resource_quota: None,
        }
    }
}

