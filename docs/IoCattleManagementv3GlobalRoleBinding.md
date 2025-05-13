# IoCattleManagementv3GlobalRoleBinding

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_version** | Option<**String**> | APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources | [optional]
**global_role_name** | **String** | GlobalRoleName is the name of the Global Role that the subject will be bound to. Immutable. | 
**group_principal_name** | Option<**String**> | GroupPrincipalName is the name of the group principal subject to be bound. Immutable. | [optional]
**kind** | Option<**String**> | Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds | [optional]
**metadata** | Option<[**kube::api::ObjectMeta**](ioK8sApimachineryPkgApisMetaV1ObjectMeta.md)> |  | [optional]
**user_name** | Option<**String**> | UserName is the name of the user subject to be bound. Immutable. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


