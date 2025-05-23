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

/// IoCattleManagementv3ClusterStatus : Status is the most recently observed status of the cluster.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, SerdeDiff)]
pub struct IoCattleManagementv3ClusterStatus {
    #[serde(rename = "aadClientCertSecret", skip_serializing_if = "Option::is_none")]
    pub aad_client_cert_secret: Option<String>,
    #[serde(rename = "aadClientSecret", skip_serializing_if = "Option::is_none")]
    pub aad_client_secret: Option<String>,
    #[serde(rename = "agentFeatures", skip_serializing_if = "Option::is_none")]
    pub agent_features: Option<std::collections::HashMap<String, bool>>,
    #[serde(rename = "agentImage", skip_serializing_if = "Option::is_none")]
    pub agent_image: Option<String>,
    #[serde(rename = "aksStatus", skip_serializing_if = "Option::is_none")]
    #[serde_diff(opaque)]
    pub aks_status: Option<serde_json::Value>,
    #[serde(rename = "allocatable", skip_serializing_if = "Option::is_none")]
    pub allocatable: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "apiEndpoint", skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,
    #[serde(rename = "appliedAgentEnvVars", skip_serializing_if = "Option::is_none")]
    #[serde_diff(opaque)]
    pub applied_agent_env_vars: Option<Vec<serde_json::Value>>,
    #[serde(rename = "appliedClusterAgentDeploymentCustomization", skip_serializing_if = "Option::is_none")]
    #[serde_diff(opaque)]
    pub applied_cluster_agent_deployment_customization: Option<serde_json::Value>,
    #[serde(rename = "appliedEnableNetworkPolicy", skip_serializing_if = "Option::is_none")]
    pub applied_enable_network_policy: Option<bool>,
    #[serde(rename = "appliedSpec", skip_serializing_if = "Option::is_none")]
    #[serde_diff(opaque)]
    pub applied_spec: Option<serde_json::Value>,
    #[serde(rename = "authImage", skip_serializing_if = "Option::is_none")]
    pub auth_image: Option<String>,
    #[serde(rename = "caCert", skip_serializing_if = "Option::is_none")]
    pub ca_cert: Option<String>,
    #[serde(rename = "capabilities", skip_serializing_if = "Option::is_none")]
    #[serde_diff(opaque)]
    pub capabilities: Option<serde_json::Value>,
    #[serde(rename = "capacity", skip_serializing_if = "Option::is_none")]
    pub capacity: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "certificatesExpiration", skip_serializing_if = "Option::is_none")]
    #[serde_diff(opaque)]
    pub certificates_expiration: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "componentStatuses", skip_serializing_if = "Option::is_none")]
    #[serde_diff(opaque)]
    pub component_statuses: Option<Vec<serde_json::Value>>,
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    #[serde_diff(opaque)]
    pub conditions: Option<Vec<serde_json::Value>>,
    #[serde(rename = "currentCisRunName", skip_serializing_if = "Option::is_none")]
    pub current_cis_run_name: Option<String>,
    #[serde(rename = "driver", skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "eksStatus", skip_serializing_if = "Option::is_none")]
    #[serde_diff(opaque)]
    pub eks_status: Option<serde_json::Value>,
    #[serde(rename = "failedSpec", skip_serializing_if = "Option::is_none")]
    #[serde_diff(opaque)]
    pub failed_spec: Option<serde_json::Value>,
    #[serde(rename = "gkeStatus", skip_serializing_if = "Option::is_none")]
    #[serde_diff(opaque)]
    pub gke_status: Option<serde_json::Value>,
    #[serde(rename = "istioEnabled", skip_serializing_if = "Option::is_none")]
    pub istio_enabled: Option<bool>,
    #[serde(rename = "limits", skip_serializing_if = "Option::is_none")]
    pub limits: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "linuxWorkerCount", skip_serializing_if = "Option::is_none")]
    pub linux_worker_count: Option<i32>,
    #[serde(rename = "nodeCount", skip_serializing_if = "Option::is_none")]
    pub node_count: Option<i32>,
    #[serde(rename = "nodeVersion", skip_serializing_if = "Option::is_none")]
    pub node_version: Option<i32>,
    #[serde(rename = "openStackSecret", skip_serializing_if = "Option::is_none")]
    pub open_stack_secret: Option<String>,
    #[serde(rename = "privateRegistrySecret", skip_serializing_if = "Option::is_none")]
    pub private_registry_secret: Option<String>,
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename = "requested", skip_serializing_if = "Option::is_none")]
    pub requested: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "s3CredentialSecret", skip_serializing_if = "Option::is_none")]
    pub s3_credential_secret: Option<String>,
    #[serde(rename = "serviceAccountTokenSecret", skip_serializing_if = "Option::is_none")]
    pub service_account_token_secret: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    #[serde_diff(opaque)]
    pub version: Option<serde_json::Value>,
    #[serde(rename = "virtualCenterSecret", skip_serializing_if = "Option::is_none")]
    pub virtual_center_secret: Option<String>,
    #[serde(rename = "vsphereSecret", skip_serializing_if = "Option::is_none")]
    pub vsphere_secret: Option<String>,
    #[serde(rename = "weavePasswordSecret", skip_serializing_if = "Option::is_none")]
    pub weave_password_secret: Option<String>,
    #[serde(rename = "windowsWorkerCount", skip_serializing_if = "Option::is_none")]
    pub windows_worker_count: Option<i32>,
}

impl IoCattleManagementv3ClusterStatus {
    /// Status is the most recently observed status of the cluster.
    pub fn new() -> IoCattleManagementv3ClusterStatus {
        IoCattleManagementv3ClusterStatus {
            aad_client_cert_secret: None,
            aad_client_secret: None,
            agent_features: None,
            agent_image: None,
            aks_status: None,
            allocatable: None,
            api_endpoint: None,
            applied_agent_env_vars: None,
            applied_cluster_agent_deployment_customization: None,
            applied_enable_network_policy: None,
            applied_spec: None,
            auth_image: None,
            ca_cert: None,
            capabilities: None,
            capacity: None,
            certificates_expiration: None,
            component_statuses: None,
            conditions: None,
            current_cis_run_name: None,
            driver: None,
            eks_status: None,
            failed_spec: None,
            gke_status: None,
            istio_enabled: None,
            limits: None,
            linux_worker_count: None,
            node_count: None,
            node_version: None,
            open_stack_secret: None,
            private_registry_secret: None,
            provider: None,
            requested: None,
            s3_credential_secret: None,
            service_account_token_secret: None,
            version: None,
            virtual_center_secret: None,
            vsphere_secret: None,
            weave_password_secret: None,
            windows_worker_count: None,
        }
    }
}

