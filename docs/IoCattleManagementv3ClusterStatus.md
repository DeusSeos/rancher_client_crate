# IoCattleManagementv3ClusterStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aad_client_cert_secret** | Option<**String**> |  | [optional]
**aad_client_secret** | Option<**String**> |  | [optional]
**agent_features** | Option<**std::collections::HashMap<String, bool>**> |  | [optional]
**agent_image** | Option<**String**> |  | [optional]
**aks_status** | Option<[**serde_json::Value**](.md)> |  | [optional]
**allocatable** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**api_endpoint** | Option<**String**> |  | [optional]
**applied_agent_env_vars** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**applied_cluster_agent_deployment_customization** | Option<[**serde_json::Value**](.md)> |  | [optional]
**applied_enable_network_policy** | Option<**bool**> |  | [optional]
**applied_spec** | Option<[**serde_json::Value**](.md)> |  | [optional]
**auth_image** | Option<**String**> |  | [optional]
**ca_cert** | Option<**String**> |  | [optional]
**capabilities** | Option<[**serde_json::Value**](.md)> |  | [optional]
**capacity** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**certificates_expiration** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**component_statuses** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**conditions** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**current_cis_run_name** | Option<**String**> |  | [optional]
**driver** | Option<**String**> |  | [optional]
**eks_status** | Option<[**serde_json::Value**](.md)> |  | [optional]
**failed_spec** | Option<[**serde_json::Value**](.md)> |  | [optional]
**gke_status** | Option<[**serde_json::Value**](.md)> |  | [optional]
**istio_enabled** | Option<**bool**> |  | [optional]
**limits** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**linux_worker_count** | Option<**i32**> |  | [optional]
**node_count** | Option<**i32**> |  | [optional]
**node_version** | Option<**i32**> |  | [optional]
**open_stack_secret** | Option<**String**> |  | [optional]
**private_registry_secret** | Option<**String**> |  | [optional]
**provider** | Option<**String**> |  | [optional]
**requested** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**s3_credential_secret** | Option<**String**> |  | [optional]
**service_account_token_secret** | Option<**String**> |  | [optional]
**version** | Option<[**serde_json::Value**](.md)> |  | [optional]
**virtual_center_secret** | Option<**String**> |  | [optional]
**vsphere_secret** | Option<**String**> |  | [optional]
**weave_password_secret** | Option<**String**> |  | [optional]
**windows_worker_count** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


