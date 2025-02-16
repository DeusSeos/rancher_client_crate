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

/// IoCattleManagementv3ProjectRoleTemplateBindingList : ProjectRoleTemplateBindingList is a list of ProjectRoleTemplateBinding
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoCattleManagementv3ProjectRoleTemplateBindingList {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// List of projectroletemplatebindings. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md
    #[serde(rename = "items")]
    pub items: Vec<models::IoCattleManagementv3ProjectRoleTemplateBinding>,
    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<models::IoK8sApimachineryPkgApisMetaV1ListMeta>>,
}

impl IoCattleManagementv3ProjectRoleTemplateBindingList {
    /// ProjectRoleTemplateBindingList is a list of ProjectRoleTemplateBinding
    pub fn new(items: Vec<models::IoCattleManagementv3ProjectRoleTemplateBinding>) -> IoCattleManagementv3ProjectRoleTemplateBindingList {
        IoCattleManagementv3ProjectRoleTemplateBindingList {
            api_version: None,
            items,
            kind: None,
            metadata: None,
        }
    }
}

