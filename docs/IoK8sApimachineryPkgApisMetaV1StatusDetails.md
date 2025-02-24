# IoK8sApimachineryPkgApisMetaV1StatusDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**causes** | Option<[**Vec<kube::core::StatusCause>**](ioK8sApimachineryPkgApisMetaV1StatusCause.md)> | The Causes array includes more details associated with the StatusReason failure. Not all StatusReasons may provide detailed causes. | [optional]
**group** | Option<**String**> | The group attribute of the resource associated with the status StatusReason. | [optional]
**kind** | Option<**String**> | The kind attribute of the resource associated with the status StatusReason. On some operations may differ from the requested resource Kind. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds | [optional]
**name** | Option<**String**> | The name attribute of the resource associated with the status StatusReason (when there is a single name which can be described). | [optional]
**retry_after_seconds** | Option<**i32**> | If specified, the time in seconds before the operation should be retried. Some errors may indicate the client must take an alternate action - for those errors this field may indicate how long to wait before taking the alternate action. | [optional]
**uid** | Option<**String**> | UID of the resource. (when there is a single resource which can be described). More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names#uids | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


