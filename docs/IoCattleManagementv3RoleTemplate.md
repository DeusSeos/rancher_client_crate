# IoCattleManagementv3RoleTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**administrative** | Option<**bool**> | Administrative if false, and context is set to cluster this RoleTemplate will not grant access to \"CatalogTemplates\" and \"CatalogTemplateVersions\" for any project in the cluster. Default is false. | [optional]
**api_version** | Option<**String**> | APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources | [optional]
**builtin** | Option<**bool**> | Builtin if true specifies that this RoleTemplate was created by Rancher and is immutable. Default to false. | [optional]
**cluster_creator_default** | Option<**bool**> | ClusterCreatorDefault if true, a binding with this RoleTemplate will be created for a users when they create a new cluster. ClusterCreatorDefault is only evaluated if the context of the RoleTemplate is set to cluster. Default to false. | [optional]
**context** | Option<**String**> | Context describes if the roleTemplate applies to clusters or projects. Valid values are \"project\", \"cluster\" or \"\". | [optional]
**description** | Option<**String**> | Description holds text that describes the resource. | [optional]
**display_name** | Option<**String**> | DisplayName is the human-readable name displayed in the UI for this resource. | [optional]
**external** | Option<**bool**> | External if true specifies that rules for this RoleTemplate should be gathered from a ClusterRole with the matching name. If set to true the Rules on the template will not be evaluated. External's value is only evaluated if the RoleTemplate's context is set to \"cluster\" Default to false. | [optional]
**hidden** | Option<**bool**> | Hidden if true informs the Rancher UI not to display this RoleTemplate. Default to false. | [optional]
**kind** | Option<**String**> | Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds | [optional]
**locked** | Option<**bool**> | Locked if true, new bindings will not be able to use this RoleTemplate. Default to false. | [optional]
**metadata** | Option<[**models::IoK8sApimachineryPkgApisMetaV1ObjectMeta**](ioK8sApimachineryPkgApisMetaV1ObjectMeta.md)> |  | [optional]
**project_creator_default** | Option<**bool**> | ProjectCreatorDefault if true, a binding with this RoleTemplate will be created for a user when they create a new project. ProjectCreatorDefault is only evaluated if the context of the RoleTemplate is set to project. Default to false. | [optional]
**role_template_names** | Option<**Vec<String>**> | RoleTemplateNames list of RoleTemplate names that this RoleTemplate will inherit. This RoleTemplate will grant all rules defined in an inherited RoleTemplate. Inherited RoleTemplates must already exist. | [optional]
**rules** | Option<[**Vec<models::IoCattleManagementv3GlobalRoleRulesInner>**](ioCattleManagementv3GlobalRole_rules_inner.md)> | Rules hold all the PolicyRules for this RoleTemplate. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


