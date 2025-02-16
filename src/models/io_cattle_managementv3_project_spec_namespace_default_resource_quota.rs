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

/// IoCattleManagementv3ProjectSpecNamespaceDefaultResourceQuota : NamespaceDefaultResourceQuota is a specification of the default ResourceQuota that a namespace will receive if none is provided. Must provide ResourceQuota if NamespaceDefaultResourceQuota is specified. See https://kubernetes.io/docs/concepts/policy/resource-quotas/ for more details.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoCattleManagementv3ProjectSpecNamespaceDefaultResourceQuota {
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<models::IoCattleManagementv3ProjectSpecNamespaceDefaultResourceQuotaLimit>,
}

impl IoCattleManagementv3ProjectSpecNamespaceDefaultResourceQuota {
    /// NamespaceDefaultResourceQuota is a specification of the default ResourceQuota that a namespace will receive if none is provided. Must provide ResourceQuota if NamespaceDefaultResourceQuota is specified. See https://kubernetes.io/docs/concepts/policy/resource-quotas/ for more details.
    pub fn new() -> IoCattleManagementv3ProjectSpecNamespaceDefaultResourceQuota {
        IoCattleManagementv3ProjectSpecNamespaceDefaultResourceQuota {
            limit: None,
        }
    }
}

