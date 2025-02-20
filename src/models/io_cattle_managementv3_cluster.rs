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
use kube::Resource;
use serde::{Deserialize, Serialize};

/// IoCattleManagementv3Cluster : Cluster is a representation of a Rancher Kubernetes cluster.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoCattleManagementv3Cluster {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "spec")]
    pub spec: models::IoCattleManagementv3ClusterSpec,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::IoCattleManagementv3ClusterStatus>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<kube::api::ObjectMeta>,
}

impl Resource for IoCattleManagementv3Cluster {

    type DynamicType = ();

    type Scope = ();

    fn kind(_dt: &Self::DynamicType) -> std::borrow::Cow<'_, str> {
        std::borrow::Cow::Borrowed("Cluster")
    }

    fn group(_dt: &Self::DynamicType) -> std::borrow::Cow<'_, str> {
        std::borrow::Cow::Borrowed("management.cattle.io")
    }

    fn version(_dt: &Self::DynamicType) -> std::borrow::Cow<'_, str> {
        std::borrow::Cow::Borrowed("v3")
    }

    fn plural(_dt: &Self::DynamicType) -> std::borrow::Cow<'_, str> {
        std::borrow::Cow::Borrowed("clusters")
    }


    fn meta(&self) -> &kube::api::ObjectMeta {
        self.metadata.as_ref().unwrap()
    }

    fn meta_mut(&mut self) -> &mut kube::api::ObjectMeta {
        self.metadata.as_mut().unwrap()
    }



}

impl IoCattleManagementv3Cluster {
    /// Cluster is a representation of a Rancher Kubernetes cluster.
    pub fn new(spec: models::IoCattleManagementv3ClusterSpec) -> IoCattleManagementv3Cluster {
        IoCattleManagementv3Cluster {
            api_version: None,
            kind: None,
            spec,
            status: None,
            metadata: None,
        }
    }
}

