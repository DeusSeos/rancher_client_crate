# IoCattleManagementv3ProjectRoleTemplateBinding

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_version** | Option<**String**> | APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources | [optional]
**group_name** | Option<**String**> | GroupName is the name of the group subject added to the project. Immutable. | [optional]
**group_principal_name** | Option<**String**> | GroupPrincipalName is the name of the group principal subject added to the project. Immutable. | [optional]
**kind** | Option<**String**> | Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds | [optional]
**metadata** | Option<[**kube::api::ObjectMeta**](ioK8sApimachineryPkgApisMetaV1ObjectMeta.md)> |  | [optional]
**project_name** | **String** | ProjectName is the name of the project to which a subject is added. Immutable. | 
**role_template_name** | **String** | RoleTemplateName is the name of the role template that defines permissions to perform actions on resources in the project. Immutable. | 
**service_account** | Option<**String**> | ServiceAccount is the name of the service account bound as a subject. Immutable. Deprecated. | [optional]
**user_name** | Option<**String**> | UserName is the name of the user subject added to the project. Immutable. | [optional]
**user_principal_name** | Option<**String**> | UserPrincipalName is the name of the user principal subject added to the project. Immutable. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


