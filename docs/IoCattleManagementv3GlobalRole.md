# IoCattleManagementv3GlobalRole

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_version** | Option<**String**> | APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources | [optional]
**builtin** | Option<**bool**> | Builtin specifies that this GlobalRole was created by Rancher if true. Immutable. | [optional]
**description** | Option<**String**> | Description holds text that describes the resource. | [optional]
**display_name** | Option<**String**> | DisplayName is the human-readable name displayed in the UI for this resource. | [optional]
**inherited_cluster_roles** | Option<**Vec<String>**> | InheritedClusterRoles are the names of RoleTemplates whose permissions are granted by this GlobalRole in every cluster besides the local cluster. To grant permissions in the local cluster, use the Rules field. | [optional]
**kind** | Option<**String**> | Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds | [optional]
**metadata** | Option<[**models::IoK8sApimachineryPkgApisMetaV1ObjectMeta**](ioK8sApimachineryPkgApisMetaV1ObjectMeta.md)> |  | [optional]
**new_user_default** | Option<**bool**> | NewUserDefault specifies that all new users created should be bound to this GlobalRole if true. | [optional]
**rules** | Option<[**Vec<models::IoCattleManagementv3GlobalRoleRulesInner>**](ioCattleManagementv3GlobalRole_rules_inner.md)> | Rules holds a list of PolicyRules that are applied to the local cluster only. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


