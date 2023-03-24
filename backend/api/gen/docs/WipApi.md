# \WipApi

All URIs are relative to *https://ux55hiuu9e.execute-api.eu-central-1.amazonaws.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_content_task_task_id_answer_post**](WipApi.md#v1_content_task_task_id_answer_post) | **Post** /v1/content/task/{task_id}/answer | 



## v1_content_task_task_id_answer_post

> Vec<crate::models::AnswerValidationResultDto> v1_content_task_task_id_answer_post(task_id, answer_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** |  | [required] |
**answer_dto** | [**AnswerDto**](AnswerDto.md) |  | [required] |

### Return type

[**Vec<crate::models::AnswerValidationResultDto>**](AnswerValidationResultDto.md)

### Authorization

[just-code-dev-moderator-authorizer](../README.md#just-code-dev-moderator-authorizer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

