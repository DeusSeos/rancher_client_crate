# IoCattleManagementv3ClusterSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent_env_vars** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**agent_image_override** | Option<**String**> |  | [optional]
**aks_config** | Option<[**serde_json::Value**](.md)> |  | [optional]
**amazon_elastic_container_service_config** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**answers** | Option<[**serde_json::Value**](.md)> |  | [optional]
**azure_kubernetes_service_config** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**cluster_agent_deployment_customization** | Option<[**serde_json::Value**](.md)> |  | [optional]
**cluster_secrets** | Option<[**serde_json::Value**](.md)> |  | [optional]
**cluster_template_id** | Option<**String**> |  | [optional]
**cluster_template_revision_id** | Option<**String**> |  | [optional]
**default_cluster_role_for_project_members** | Option<**String**> |  | [optional]
**default_pod_security_admission_configuration_template_name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**desired_agent_image** | Option<**String**> |  | [optional]
**desired_auth_image** | Option<**String**> |  | [optional]
**display_name** | **String** | The display name of the cluster. | 
**docker_root_dir** | Option<**String**> |  | [optional][default to /var/lib/docker]
**eks_config** | Option<[**serde_json::Value**](.md)> |  | [optional]
**enable_network_policy** | Option<**bool**> |  | [optional][default to false]
**fleet_agent_deployment_customization** | Option<[**serde_json::Value**](.md)> |  | [optional]
**fleet_workspace_name** | Option<**String**> |  | [optional]
**generic_engine_config** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**gke_config** | Option<[**serde_json::Value**](.md)> |  | [optional]
**google_kubernetes_engine_config** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**imported_config** | Option<[**serde_json::Value**](.md)> |  | [optional]
**internal** | Option<**bool**> |  | [optional][default to false]
**k3s_config** | Option<[**serde_json::Value**](.md)> |  | [optional]
**local_cluster_auth_endpoint** | Option<[**serde_json::Value**](.md)> |  | [optional]
**questions** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**rancher_kubernetes_engine_config** | Option<[**serde_json::Value**](.md)> |  | [optional]
**rke2_config** | Option<[**serde_json::Value**](.md)> |  | [optional]
**windows_prefered_cluster** | Option<**bool**> |  | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


