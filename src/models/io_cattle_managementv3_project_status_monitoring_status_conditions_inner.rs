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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoCattleManagementv3ProjectStatusMonitoringStatusConditionsInner {
    /// Last time the condition transitioned from one status to another.
    #[serde(rename = "lastTransitionTime", skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<String>,
    /// The last time this condition was updated.
    #[serde(rename = "lastUpdateTime", skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,
    /// Human-readable message indicating details about last transition
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The reason for the condition's last transition.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Status of the condition, one of True, False, Unknown.
    #[serde(rename = "status")]
    pub status: String,
    /// Type of cluster condition.
    #[serde(rename = "type")]
    pub r#type: String,
}

impl IoCattleManagementv3ProjectStatusMonitoringStatusConditionsInner {
    pub fn new(status: String, r#type: String) -> IoCattleManagementv3ProjectStatusMonitoringStatusConditionsInner {
        IoCattleManagementv3ProjectStatusMonitoringStatusConditionsInner {
            last_transition_time: None,
            last_update_time: None,
            message: None,
            reason: None,
            status,
            r#type,
        }
    }
}

