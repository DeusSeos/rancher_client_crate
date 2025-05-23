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

/// IoCattleManagementv3ProjectStatusMonitoringStatus : MonitoringStatus is the status of the Monitoring V1 app.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, SerdeDiff)]
pub struct IoCattleManagementv3ProjectStatusMonitoringStatus {
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<models::IoCattleManagementv3ProjectStatusMonitoringStatusConditionsInner>>,
    #[serde(rename = "grafanaEndpoint", skip_serializing_if = "Option::is_none")]
    pub grafana_endpoint: Option<String>,
}

impl IoCattleManagementv3ProjectStatusMonitoringStatus {
    /// MonitoringStatus is the status of the Monitoring V1 app.
    pub fn new() -> IoCattleManagementv3ProjectStatusMonitoringStatus {
        IoCattleManagementv3ProjectStatusMonitoringStatus {
            conditions: None,
            grafana_endpoint: None,
        }
    }
}

