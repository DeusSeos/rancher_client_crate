# IoCattleManagementv3ProjectSpecResourceQuotaLimit

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**config_maps** | Option<**String**> | ConfigMaps is the total number of ReplicationControllers that can exist in the namespace. | [optional]
**limits_cpu** | Option<**String**> | LimitsCPU is the CPU limits across all pods in a non-terminal state. | [optional]
**limits_memory** | Option<**String**> | LimitsMemory is the memory limits across all pods in a non-terminal state. | [optional]
**persistent_volume_claims** | Option<**String**> | PersistentVolumeClaims is the total number of PersistentVolumeClaims that can exist in the namespace. | [optional]
**pods** | Option<**String**> | Pods is the total number of Pods in a non-terminal state that can exist in the namespace. A pod is in a terminal state if .status.phase in (Failed, Succeeded) is true. | [optional]
**replication_controllers** | Option<**String**> | ReplicationControllers is total number of ReplicationControllers that can exist in the namespace. | [optional]
**requests_cpu** | Option<**String**> | RequestsCPU is the CPU requests limit across all pods in a non-terminal state. | [optional]
**requests_memory** | Option<**String**> | RequestsMemory is the memory requests limit across all pods in a non-terminal state. | [optional]
**requests_storage** | Option<**String**> | RequestsStorage is the storage requests limit across all persistent volume claims. | [optional]
**secrets** | Option<**String**> | Secrets is the total number of ReplicationControllers that can exist in the namespace. | [optional]
**services** | Option<**String**> | Services is the total number of Services that can exist in the namespace. | [optional]
**services_load_balancers** | Option<**String**> | ServicesLoadBalancers is the total number of Services of type LoadBalancer that can exist in the namespace. | [optional]
**services_node_ports** | Option<**String**> | ServiceNodePorts is the total number of Services of type NodePort that can exist in the namespace. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


