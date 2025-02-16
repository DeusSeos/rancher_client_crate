# IoCattleManagementv3ProjectSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cluster_name** | **String** | ClusterName is the name of the cluster the project belongs to. Immutable. | 
**container_default_resource_limit** | Option<[**models::IoCattleManagementv3ProjectSpecContainerDefaultResourceLimit**](ioCattleManagementv3Project_spec_containerDefaultResourceLimit.md)> |  | [optional]
**description** | Option<**String**> | Description is a human-readable description of the project. | [optional]
**display_name** | **String** | DisplayName is the human-readable name for the project. | 
**enable_project_monitoring** | Option<**bool**> | EnableProjectMonitoring indicates whether Monitoring V1 should be enabled for this project. Deprecated. Use the Monitoring V2 app instead. Defaults to false. | [optional]
**namespace_default_resource_quota** | Option<[**models::IoCattleManagementv3ProjectSpecNamespaceDefaultResourceQuota**](ioCattleManagementv3Project_spec_namespaceDefaultResourceQuota.md)> |  | [optional]
**resource_quota** | Option<[**models::IoCattleManagementv3ProjectSpecResourceQuota**](ioCattleManagementv3Project_spec_resourceQuota.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


