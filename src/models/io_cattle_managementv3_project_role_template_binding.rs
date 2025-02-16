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

/// IoCattleManagementv3ProjectRoleTemplateBinding : ProjectRoleTemplateBinding is the object representing membership of a subject in a project with permissions specified by a given role template.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoCattleManagementv3ProjectRoleTemplateBinding {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// GroupName is the name of the group subject added to the project. Immutable.
    #[serde(rename = "groupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// GroupPrincipalName is the name of the group principal subject added to the project. Immutable.
    #[serde(rename = "groupPrincipalName", skip_serializing_if = "Option::is_none")]
    pub group_principal_name: Option<String>,
    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<models::IoK8sApimachineryPkgApisMetaV1ObjectMeta>>,
    /// ProjectName is the name of the project to which a subject is added. Immutable.
    #[serde(rename = "projectName")]
    pub project_name: String,
    /// RoleTemplateName is the name of the role template that defines permissions to perform actions on resources in the project. Immutable.
    #[serde(rename = "roleTemplateName")]
    pub role_template_name: String,
    /// ServiceAccount is the name of the service account bound as a subject. Immutable. Deprecated.
    #[serde(rename = "serviceAccount", skip_serializing_if = "Option::is_none")]
    pub service_account: Option<String>,
    /// UserName is the name of the user subject added to the project. Immutable.
    #[serde(rename = "userName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// UserPrincipalName is the name of the user principal subject added to the project. Immutable.
    #[serde(rename = "userPrincipalName", skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
}

impl IoCattleManagementv3ProjectRoleTemplateBinding {
    /// ProjectRoleTemplateBinding is the object representing membership of a subject in a project with permissions specified by a given role template.
    pub fn new(project_name: String, role_template_name: String) -> IoCattleManagementv3ProjectRoleTemplateBinding {
        IoCattleManagementv3ProjectRoleTemplateBinding {
            api_version: None,
            group_name: None,
            group_principal_name: None,
            kind: None,
            metadata: None,
            project_name,
            role_template_name,
            service_account: None,
            user_name: None,
            user_principal_name: None,
        }
    }
}

